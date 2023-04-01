mod dinning;
mod philosopher;

use std::mem::transmute;
use std::sync::{Arc, Mutex};
use std::thread;
use dinning::*;
use philosopher::*;
const PSNUM: usize = 5;
const NAMES: [&str; PSNUM] = ["Mark", "Hegel", "Lenin", "Heidegger", "Althusse"];

fn main() {
    print!("将用Semaphore重新实现一次");
    let mut ps: Vec<Philosopher> = vec![];
    let chops = Arc::new(Table::new(PSNUM));
    for (idx, n) in NAMES.into_iter().enumerate() {
        ps.push(Philosopher::new(n, idx, (idx + 1) % PSNUM));
    }

    let handlers: Vec<thread::JoinHandle<()>> = ps.into_iter()
            .map(|mut philosopher| {
                thread::spawn(move || {
                    philosopher.run(&chops, None);
                })
            }).collect();
    for thread2bhandled in handlers {
        let Ok(_) = thread2bhandled.join() else {
            panic!("Failed to join!");
        };
    } 
}

mod tests {

}