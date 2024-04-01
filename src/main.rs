use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args[1].parse::<usize>().unwrap();
    let mut _primes = vec![true; num - 1];
    for _prime in 2..(((num as f64).sqrt().ceil() as u32) + 1) {
        if _primes[(_prime - 2) as usize] {
            // mark the multiples
            let mut k = 2;
            while k*_prime <= (num as u32) {
                _primes[(k*_prime - 2) as usize] = false;
                k += 1;
            }
        }
    }

    // print primes
    print!("Prime numbers up to {}: ", num);
    for index in 0..num-1 {
        if _primes[index] {
            print!("{} ", index + 2);
        }
    }
}
