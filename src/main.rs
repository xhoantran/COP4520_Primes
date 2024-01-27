use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

fn is_prime(n: u64) -> bool {
    if n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    let sqrt_n = (n as f64).sqrt() as u64;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i + 6;
    }
    return true;
}

struct Counter {
    val: u64,
    next_increment: u8,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            val: 5,
            next_increment: 2,
        }
    }

    fn increment(&mut self) {
        self.val += self.next_increment as u64;
        self.next_increment = 6 - self.next_increment;
    }
}

fn main() {
    let nthreads = 3;
    let n = 100000000;

    // Start the timer.
    let start = std::time::Instant::now();

    // Create a vector to hold the thread handles.
    let mut handles = vec![];

    // Add 2 to the sum of primes. 2 is the only even prime.
    let counter = Arc::new(Mutex::new(Counter::new()));
    let num_primes = Arc::new(Mutex::new(2));
    let sum_primes = Arc::new(Mutex::new(2 + 3));
    let largest_primes = Arc::new(Mutex::new(vec![0; 10]));

    for _ in 0..nthreads {
        // Clone-protected mutexes.
        let counter = counter.clone();
        let num_primes: Arc<Mutex<u32>> = num_primes.clone();
        let sum_primes = sum_primes.clone();
        let largest_primes = largest_primes.clone();

        // Spawn a thread to find primes.
        let handle = thread::spawn(move || {
            // Thread local variables.
            let mut thread_count_primes = 0;
            let mut thread_sum_primes = 0;
            let mut thread_largest_primes = vec![0; 10];
            let mut thread_largest_primes_index = 0;
            loop {
                // Safely increment the counter. Create a new variable val
                // to avoid holding the lock
                let mut counter = counter.lock().unwrap();
                let val = counter.val;
                counter.increment();
                std::mem::drop(counter);

                // If val is greater than n, stop the thread.
                if val > n {
                    // Aquire the lock and update the global variables.
                    let mut num = num_primes.lock().unwrap();
                    *num += thread_count_primes;
                    std::mem::drop(num);
                    let mut sum = sum_primes.lock().unwrap();
                    *sum += thread_sum_primes;
                    std::mem::drop(sum);

                    // Move the index to the largest prime in the thread.
                    if thread_largest_primes_index == 0 {
                        thread_largest_primes_index = 9;
                    } else {
                        thread_largest_primes_index -= 1;
                    }

                    let mut largest = largest_primes.lock().unwrap();
                    for i in 0..largest.len() {
                        // If the current largest prime is less than the
                        // smallest prime in the thread, replace it.
                        if largest[i] < thread_largest_primes[thread_largest_primes_index] {
                            largest[i] = thread_largest_primes[thread_largest_primes_index];
                            if thread_largest_primes_index == 0 {
                                thread_largest_primes_index = 9;
                            } else {
                                thread_largest_primes_index -= 1;
                            }
                        }
                    }
                    largest.sort();
                    std::mem::drop(largest);
                    break;
                }

                if is_prime(val) {
                    thread_count_primes += 1;
                    thread_sum_primes += val;
                    thread_largest_primes[thread_largest_primes_index] = val;
                    thread_largest_primes_index += 1;
                    if thread_largest_primes_index == 10 {
                        thread_largest_primes_index = 0;
                    }
                }
            }
        });

        // Add the thread handle to the vector.
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // Stop the timer.
    let duration = start.elapsed();

    // Write to primes.txt
    let mut file = File::create("primes.txt").unwrap();

    let num = num_primes.lock().unwrap();
    let sum = sum_primes.lock().unwrap();
    file.write_all(format!("{} {} {}\n", duration.as_secs_f64(), *num, *sum).as_bytes())
        .unwrap();

    let largest = largest_primes.lock().unwrap();
    for i in 0..largest.len() {
        file.write_all(format!("{} ", largest[i]).as_bytes())
            .unwrap();
    }
    file.write_all(b"\n").unwrap();
}
