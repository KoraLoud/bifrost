//bifrost - Kora Loudermilk 2025

use bifrost::http_parse_error::HttpParseError;
use core::error;
use http::{Method, Request, Uri, Version};
use std::{
    io::{BufReader, Read},
    net::{TcpListener, TcpStream},
    thread,
};

const LF: u8 = 0x0a;
const CR: u8 = 0x0d;

fn main() {
    let listener = TcpListener::bind("192.168.1.81:80").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| match handle_connection(stream) {
            Err(error) => println!("error in thread: {}", error),
            _ => (),
        });
    }
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

fn read_stream(mut reader: BufReader<&TcpStream>) -> Result<Vec<String>, Box<dyn error::Error>> {
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

fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn error::Error>> {
    println!("got connection!");
    println!("{:#?}", stream.peer_addr().unwrap());
    let reader = BufReader::new(&stream);
    //let read_timeout = Duration::from_millis(1000);
    //stream.set_read_timeout(Some(read_timeout))?;
    let http_request_string = read_stream(reader)?;
    println!("got stuff: {:?}", http_request_string);
    //println!("{:?}", http_request);
    let http_request = parse_http_packet(http_request_string);
    if http_request.is_err() {
        println!("invlid request");
    } else {
        println!("{:#?}", http_request.unwrap());
    }
    Ok(())
}
