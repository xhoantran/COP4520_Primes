# Parallel Prime Number Finder in Rust

## Description

This program finds all prime numbers up to `10^8`. This program is meant to be a demonstration of how to use Rust's concurrency features safely. It might not be the most efficient way to find prime numbers.

## Usage

```bash
git clone https://github.com/xhoantran/COP4520_Primes
cd COP4520_Primes
cargo build --release
cargo run --release
```

## Proof of Correctness

### Prime Checker

The program checks if a number is prime by checking if it is divisible by any number up to its square root. This is because if a number is not prime, it can be factored into two numbers, one of which is less than or equal to its square root. If a number is not divisible by any number up to its square root, then it is prime.

### Dynamic Load Balancing

The program uses dynamic load balancing to distribute the work of checking if a number is prime. Each thread acquires a number to check from the counter mutex. The counter mutex is locked when a thread get the value and unlocked as soon as the thread acquires a number to check. This ensures that each thread gets a unique number to check. The sum and count of prime numbers are thead-local variables. This ensures that the threads do not have to wait for each other to update the sum and count of prime numbers. After the threads are done checking if a number is prime, the threads update the sum and count of prime numbers by acquiring the sum and count mutex.

## Efficiency

By using mutex, the works are evenly distributed among threads. But the mutex also slows down the program. The program is faster than the sequential version when the number of threads is less than 3. The program is slower than the sequential version when the number of threads is greater than 3. This is because the overhead of using mutex is greater than the benefit of using multiple threads.

## Experiment Results

**OS**: macOS Sonoma 14.1.1
**CPU**: Apple M1 Pro 8-Core CPU

| Number of Threads | Time (seconds) |
| :---------------: | :------------: |
|         1         |     10.33      |
|         2         |     6.434      |
|         3         |     5.201      |
|         4         |     8.910      |
|         5         |     9.607      |
|         6         |     9.984      |
|         7         |     10.09      |
|         8         |     10.10      |
