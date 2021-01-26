use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Self { workers, sender }
    }
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).ok();
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            if let Ok(r) = receiver.lock() {
                if let Ok(job) = r.recv() {
                    job();
                }
            }
        });
        Self { id, thread }
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let len = input.len();
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(worker_count);
    for s in input {
        let s = s.to_string();
        let sender = mpsc::Sender::clone(&tx);
        pool.execute(move || {
            let mut hm = HashMap::new();
            for chr in s.chars().filter(|c| c.is_alphabetic()) {
                if let Some(c) = chr.to_lowercase().next() {
                    *hm.entry(c).or_insert(0) += 1;
                }
            }
            sender.send(hm).ok();
        });
    }
    let mut ret = HashMap::new();
    for (i, received) in rx.iter().enumerate() {
        for (k, v) in received {
            *ret.entry(k).or_insert(0) += v;
        }
        if i == len - 1 {
            return ret;
        }
    }
    unreachable!();
}
