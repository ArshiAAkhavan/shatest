#![allow(dead_code, unused_variables)]

use loom::sync::atomic::{AtomicI8, AtomicU8, AtomicUsize, Ordering};

struct System {
    gpu_tail: AtomicU8,
    host_status: AtomicI8,
    call_back_queue: AtomicUsize,
}
impl System {
    fn new() -> Self {
        Self {
            gpu_tail: AtomicU8::new(0),
            host_status: AtomicI8::new(0),
            call_back_queue: AtomicUsize::new(0),
        }
    }

    fn gpu_process(&self) {
        self.gpu_tail.fetch_add(1, Ordering::Release);
    }

    fn schedule_callback(&self) {
        self.call_back_queue.fetch_add(1, Ordering::Release);
    }

    fn gpu_run(&self) {
        dbg!("im in gpu");
        for _ in 0..ITERATIONS {
            if self.gpu_tail.load(Ordering::Acquire) < CAPACITY {
                self.gpu_process()
            }
            if self.gpu_tail.load(Ordering::Acquire) == CAPACITY {
                self.host_status
                    .fetch_add(CAPACITY as i8, Ordering::Release);
            }
        }
    }

    fn host_run(&self) {
        dbg!("im in host");
        for _ in 0..ITERATIONS {
            if self.host_status.load(Ordering::Acquire) > 0 {
                assert!(self.host_status.load(Ordering::Acquire) > 0);
                self.host_status.fetch_add(i8::MIN, Ordering::Release);
                // process ...
                self.schedule_callback();
            }
        }
    }

    fn callback_run(&self) {
        dbg!("im in callback");
        for _ in 0..ITERATIONS {
            if self.call_back_queue.load(Ordering::Acquire) > 0 {
                self.call_back_queue.fetch_sub(1, Ordering::Release);
                self.host_status.store(0, Ordering::Release);
                self.gpu_tail.store(0, Ordering::Release);
            }
        }
    }
}

const ITERATIONS: usize = 2;
const CAPACITY: u8 = 1;

#[cfg(test)]
mod test {
    use super::*;
    use loom::{sync::Arc, thread};

    #[test]
    fn model_check() {
        loom::model(system_sim);
    }

    fn system_sim() {
        let system_main = Arc::new(System::new());

        let system = system_main.clone();
        let h1 = thread::spawn(move || {
            system.gpu_run();
        });
        let system = system_main.clone();
        let h2 = thread::spawn(move || {
            system.host_run();
        });
        let system = system_main.clone();
        let h3 = thread::spawn(move || {
            system.callback_run();
        });
        h1.join().unwrap();
        h2.join().unwrap();
        h3.join().unwrap();
    }
}

