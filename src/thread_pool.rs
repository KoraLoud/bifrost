use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

//threadpool object spawns a set amount of threads and has them read out a mpsc queue
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(workers: usize) -> ThreadPool {
        assert!(workers > 0);
        let (sender, reciever) = mpsc::channel();
        let mut worker_list: Vec<Worker> = Vec::with_capacity(workers);
        let worker_mutex: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(reciever));
        for i in 0..workers {
            worker_list.push(Worker::new(i, Arc::clone(&worker_mutex)));
        }
        ThreadPool {
            workers: worker_list,
            sender,
        }
    }
    pub fn execute<T>(&self, job: T)
    where
        T: FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static,
    {
        let job = Box::new(job);
        self.sender.send(job).unwrap();
    }
}

pub struct Worker {
    thread_num: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(thread_num: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = reciever.lock().unwrap().recv().unwrap(); //get mutex and attempt to read
                //queue
                job();
                println!("request brought to you by: thread {thread_num}");
            }
        });
        Worker { thread_num, thread }
    }
}

pub type Job = Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static>;
