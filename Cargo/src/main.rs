use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use sha2::{Sha256, Digest};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'N', long)]
    zeros: usize,

    #[arg(short = 'F', long)]
    results: usize,
}

fn main() {

    let args = Args::parse();

    let counter = Arc::new(AtomicU64::new(1));

    let found = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for _ in 0..num_cpus::get() {
        let counter = Arc::clone(&counter);
        let found = Arc::clone(&found);
        let zeros = args.zeros;
        let results = args.results;

        let handle = thread::spawn(move || {
            loop {
                if found.load(Ordering::SeqCst) >= results as u64 {
                    break;
                }

                let num = counter.fetch_add(1, Ordering::SeqCst);


                let hash = calculate_sha256(num);

                if hash.ends_with(&"0".repeat(zeros)) {
                    println!("{}, \"{}\"", num, hash);

                    found.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
fn calculate_sha256(num: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(num.to_string());
    let result = hasher.finalize();
    format!("{:x}", result)
}