use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Message {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Terminate,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
        self.sender
            .send(Message::Job(Box::new(f)))
            .expect("Failed to send job")
    }
    fn terminate(&self) {
        for _ in 0..self.workers.len() {
            self.sender
                .send(Message::Terminate)
                .expect("Failed to send terminate")
        }
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            if let Ok(r) = receiver.lock() {
                if let Ok(message) = r.recv() {
                    match message {
                        Message::Job(job) => job(),
                        Message::Terminate => break,
                    }
                }
            }
        });
        Self { id, thread }
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(worker_count);
    for &s in input {
        let s = s.to_string();
        let sender = mpsc::Sender::clone(&tx);
        pool.execute(move || {
            let mut hm = HashMap::new();
            for chr in s.chars().filter(|c| c.is_alphabetic()) {
                if let Some(c) = chr.to_lowercase().next() {
                    *hm.entry(c).or_insert(0) += 1;
                }
            }
            sender.send(hm).expect("Failed to send results of job")
        });
    }
    pool.terminate();
    drop(tx);

    let mut ret = HashMap::new();
    for received in rx {
        for (k, v) in received {
            *ret.entry(k).or_insert(0) += v;
        }
    }
    ret
}
