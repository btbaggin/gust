use std::thread;
use crate::logger::*;
use std::collections::HashSet;

//This is used to pass a raw pointer to the assetslot between threads
//Use very rarely when mutability and lifetimes cause issues
#[derive(Clone, Copy)]
pub struct RawDataPointer(*mut u8);
unsafe impl std::marker::Send for RawDataPointer {}
impl RawDataPointer {
    pub fn new<T>(item: &mut T) -> RawDataPointer {
        unsafe {
            let layout = std::alloc::Layout::new::<usize>();
            let slot_ptr = std::alloc::alloc(layout);
            *(slot_ptr as *mut &T) = item;
            RawDataPointer(slot_ptr)
        }
    }
    pub fn get_inner<'a, T>(&self) -> &'a mut T {
        unsafe { &mut *(self.0 as *mut &mut T) }
    }
}

pub struct JobQueue {
    queue: spmc::Sender<JobType>,
    set: HashSet<String>,
} 
impl JobQueue {
    /// Sends a message to the job system for asynchronous processing
    /// Each new message type needs custom handling
    pub fn send(&mut self, job: JobType) {
        self.queue.send(job).unwrap()
    }
}

/// Starts a single producer multiple consumer job threading system
/// Jobs can be sent to this system using the returned JobQueue
pub fn start_job_system() -> (JobQueue, std::sync::mpsc::Receiver<u8>) {
    const NUM_THREADS: u32 = 8;

    let (tx, rx) = spmc::channel();
    let (notify_tx, notify_rx) = std::sync::mpsc::channel();
    for _ in 0..NUM_THREADS {
        let rx = rx.clone();
        let notify_tx = notify_tx.clone();
        thread::spawn(move || {
            poll_pending_jobs(rx, notify_tx)
        });
    }

    (JobQueue { queue: tx, set: HashSet::new() }, notify_rx)
}

fn poll_pending_jobs(queue: spmc::Receiver<JobType>, notify: std::sync::mpsc::Sender<u8>) {
    loop {
        let msg = queue.recv().log_and_panic();
        match msg {
            JobType::LoadImage((path, slot)) => crate::assets::load_image_async(path, slot),
        }

        notify.send(0).log("Unable to notify main loop about finished job");
    }
}

pub enum JobType {
    /// Loads an image synchronously
    /// Should only be called through the asset system
    /// We copy the AssetPathType from the slot so 
    /// the locks on the slot are shorter
    LoadImage((&'static str, RawDataPointer)),
}