use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use rand::prelude::*;



fn main() {
    // spawn 100 tasks on threads
    let thread_handlers: Vec<JoinHandle<_>> =
    (1..=100)
    .map(|index| {
        // 1-20 secunds of delay
        let delay = rand::thread_rng().gen_range(1000..20000);

        // name of the thread
        let builder = thread::Builder::new().name(format!("Thread-0x{}", index));

        // Spawn a new thread
        builder.spawn(move || {
            // simulate execution with a delay
            thread::sleep(Duration::from_millis(delay));

            // message when the thread is finished
            println!("☑️ Thread-0x{}, delay {}ms for done", thread::current().name().unwrap(), delay);
        }).unwrap()
    }).collect();

    for job in thread_handlers {
        let _ = job.join();
    };

    println!("✅ all job finished");
}
