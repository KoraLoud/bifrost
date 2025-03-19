//bifrost - Kora Loudermilk 2025

use bifrost::http_resource::HttpResource;
use bifrost::thread_pool::ThreadPool;
use bifrost::{DirectoryReadError, http_parse_error::HttpParseError};
use core::error;
use http::{Method, Request, Response, Uri, Version};
use std::io::Write;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufReader, Read},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

const LF: u8 = 0x0a;
const CR: u8 = 0x0d;

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        //not correct amount of arguments
        println!("One path argument is required to build the route table.");
        println!("Usage: bifrost <directory>");
        return;
    }
    //check if provided path is valid
    let routing_table = Arc::new(build_route_table(&argv[1]).unwrap()); //hashmap of uri paths and their corresponding html
    //file localtion. routes will be created at the beginning of the program to provide security
    //against path traversal exploits
    let listener = TcpListener::bind("192.168.1.81:80").unwrap();
    let threadpool = ThreadPool::new(4);
    println!("listening");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let rt = Arc::clone(&routing_table);
        let request_handler = || handle_connection(stream, rt).unwrap();
        threadpool.execute(request_handler);
    }
}

fn build_route_table(
    dir_string: &String,
) -> Result<HashMap<String, HttpResource>, bifrost::DirectoryReadError> {
    let root_dir = fs::read_dir(dir_string).expect("Problem reading directory");
    let mut route_map = HashMap::new();
    recursive_add_route(root_dir, &mut route_map, dir_string)?;
    Ok(route_map)
}

fn recursive_add_route(
    dir: fs::ReadDir,
    map: &mut HashMap<String, HttpResource>,
    root_dir: &String,
) -> Result<(), DirectoryReadError> {
    for entry in dir.into_iter().flatten() {
        println!(
            "indexing file {:?}",
            entry.file_name().into_string().unwrap()
        );
        if entry.file_type()?.is_file() {
            let (route_string, path_resource) = file_to_route(&entry, root_dir)?;
            if map.contains_key(&route_string) {
                return Err(DirectoryReadError {
                    msg: String::from(
                        "Route already exists. Do you have two html files in the same directory?",
                    ),
                });
            }
            map.insert(route_string, path_resource);
        } else if entry.file_type()?.is_dir() {
            recursive_add_route(fs::read_dir(entry.path()).unwrap(), map, root_dir)?;
        }
    }
    Ok(())
}

fn file_to_route(
    entry: &fs::DirEntry,
    root_dir: &String,
) -> Result<(String, HttpResource), DirectoryReadError> {
    let mut route_string = String::new();
    let mut route_http_resource: Option<HttpResource> = None;
    if let Some(entry_ext) = entry.path().extension() {
        let path_resource = entry.path().canonicalize().unwrap();
        route_http_resource = Some(HttpResource::new(path_resource.to_str().unwrap()));
        route_string = String::from(entry.path().to_str().unwrap()).replace(root_dir, "");
        if entry_ext == "html" {
            //route is directory name
            route_string = String::from(entry.path().parent().unwrap().to_str().unwrap())
                .replace(root_dir, "");
            if route_string.is_empty() {
                route_string = String::from("/");
            }
        }
    }
    if (route_http_resource.is_none()) {
        return Err(DirectoryReadError {
            msg: String::from("Failure to read file"),
        });
    }
    Ok((route_string, route_http_resource.unwrap()))
}

fn invalid_request_handler(steam: TcpStream) {
    //send error html page
}

fn parse_http_packet(http_packet: Vec<String>) -> Result<Request<()>, HttpParseError> {
    let first_line: Vec<&str> = http_packet
        .first()
        .ok_or(HttpParseError::ParseError)?
        .split(' ')
        .collect();
    let method = Method::from_bytes(
        first_line
            .first()
            .ok_or(HttpParseError::ParseError)?
            .as_bytes(),
    )?;
    let version = match *first_line.get(2).ok_or(HttpParseError::ParseError)? {
        "HTTP/0.9" => Version::HTTP_09,
        "HTTP/1.0" => Version::HTTP_10,
        "HTTP/1.1" => Version::HTTP_11,
        "HTTP/2.0" => Version::HTTP_2,
        "HTTP/3.0" => Version::HTTP_3,
        _ => return Err(HttpParseError::ParseError),
    };
    let uri = first_line
        .get(1)
        .ok_or(HttpParseError::ParseError)?
        .parse::<Uri>()?;
    let mut request = Request::builder().version(version).uri(uri).method(method);
    for line in 1..http_packet.len() {
        if let Some(line_text) = http_packet.get(line) {
            let line_key_value: Vec<String> =
                line_text.split(": ").map(|s| s.to_string()).collect();

            request = request.header(
                line_key_value.first().ok_or(HttpParseError::ParseError)?,
                line_key_value.last().ok_or(HttpParseError::ParseError)?,
            );
        }
    }
    Ok(request.body(())?)
}

fn read_stream(mut reader: BufReader<&TcpStream>) -> Result<Vec<String>, io::Error> {
    let mut packet: Vec<String> = Vec::new();
    let mut line_buff: Vec<u8> = Vec::new();
    loop {
        let mut byte = [0; 1];
        reader.read_exact(&mut byte)?;
        //if last two reads were CRLF
        line_buff.push(byte[0]);
        if line_buff.len() >= 2 && line_buff.as_slice()[line_buff.len() - 2..].to_vec() == [CR, LF]
        {
            if line_buff.len() == 2 {
                break;
            }

            packet.push(String::from_utf8_lossy(&line_buff[0..line_buff.len() - 2]).to_string());
            line_buff.clear();
        }

        //println!("{:?}", line_buff);
    }
    Ok(packet)
}

fn handle_connection(
    stream: TcpStream,
    routing_table: Arc<HashMap<String, HttpResource>>,
) -> Result<(), Box<dyn error::Error>> {
    println!("got connection!");
    println!("{:#?}", stream.peer_addr().unwrap());
    let reader = BufReader::new(&stream);
    //read tcp stream into Strings
    let http_request_string = read_stream(reader)?;
    //parse packet into request struct
    if let Ok(req) = parse_http_packet(http_request_string) {
        let uri_path = &req.uri().path().to_string();
        if routing_table.contains_key(uri_path) {
            send_res(stream, &routing_table[uri_path]);
            //requested resource exists
            /*let response = Response::builder()
            .status(200)
            .body(routing_table[uri_path].file_data.as_bytes())
            .unwrap();*/
        } else {
            //requested resource does not exist
            println!("ermmm i dont have that one");
        }
    } else {
        println!("http parse error");
    }
    Ok(())
}

fn send_res(mut stream: TcpStream, file: &HttpResource) {}
