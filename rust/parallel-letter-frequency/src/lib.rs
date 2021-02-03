use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() -> Option<()> + Send + 'static>;

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
        F: FnOnce() -> Option<()> + Send + 'static,
    {
        self.sender.send(Box::new(f)).ok();
    }
    fn terminate(&self) {
        for _ in 0..self.workers.len() {
            self.sender.send(Box::new(|| None)).ok();
        }
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
                    if job().is_none() {
                        break;
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
            sender.send(hm).ok()
        });
    }
    pool.terminate();
    drop(tx);

    let mut ret = HashMap::new();
    for received in rx.iter() {
        for (k, v) in received {
            *ret.entry(k).or_insert(0) += v;
        }
    }
    ret
}
