use std::{io, thread};

pub struct ThreadPool {
    size: usize,
}

impl ThreadPool {
    /// Creates new thread pool
    ///
    /// # Panics
    /// if the size is smaller than zero
    ///
    pub fn new(size: usize) -> Result<ThreadPool, io::Error> {
        assert!(size > 0);

        Ok(ThreadPool { size })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        thread::spawn(job);
    }
}
