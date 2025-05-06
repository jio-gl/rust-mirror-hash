use mirror_hash::Mirror256;
use std::env;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let message = if args.len() > 1 {
        args[1].clone()
    } else {
        "This is the canary.".to_string()
    };

    println!("Message: {}", message);
    
    // Create a new hasher with the message
    let hasher = Mirror256::new(Some(&message), None, None, true);
    
    // Get and print the digest
    println!("Hash: {}", hasher.hexdigest());
    
    // Benchmark
    benchmark();
}

fn benchmark() {
    println!("\nBenchmarking...");
    
    let start = Instant::now();
    let mut count = 0;
    
    loop {
        let random_string = random_alphanumeric_string(32);
        let hasher = Mirror256::new(Some(&random_string), None, None, true);
        
        let _digest = hasher.hexdigest();
        count += 1;
        
        if start.elapsed() >= Duration::from_secs(1) {
            break;
        }
    }
    
    println!("{} hashes per second!", count);
    
    // Example with a specific message
    let example_message = "This is the canary #42. asdfasdfasdfasdfasdfqwerqwerqwerdfnnjkdfnjldljknsvv";
    let example_hasher = Mirror256::new(Some(example_message), None, None, true);
    println!("Example message: {}", example_message);
    println!("Example digest: {}", example_hasher.hexdigest());
    
    // Example with a random string
    let random_message = random_alphanumeric_string(32);
    let random_hasher = Mirror256::new(Some(&random_message), None, None, true);
    println!("Example message: {}", random_message);
    println!("Example digest: {}", random_hasher.hexdigest());
}

fn random_alphanumeric_string(length: usize) -> String {
    use rand::{Rng, thread_rng};
    
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut rng = thread_rng();
    
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
} 