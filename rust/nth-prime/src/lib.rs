pub fn nth(n: u32) -> u32 {
    let mut primes = vec!(2);
    let mut possible_prime: u32 = 3;
    while primes.len() - 1 < n as usize { // // account for 0 based indexing
        if test_prime_efficient(possible_prime, &primes) {
            primes.push(possible_prime);
        }
        possible_prime = possible_prime + 1;
    }
    match primes.pop() {
        Some(n) => n as u32,
        None => panic!("No primes were found"),
    }
}

// a less efficient solution
fn test_prime(number: u32) -> bool {
    let mut number_to_test = number - 1;
    let mut prime = true;
    while number_to_test > 1 {
        if number % number_to_test == 0 {
            prime = false;
        }
        number_to_test = number_to_test - 1;
    }
    prime
}

fn test_prime_efficient(number: u32, primes: &Vec<u32>) -> bool {
    // check whether any prime integer m from 2 to √n evenly divides n (the division leaves no remainder)
    let mut is_prime = true;
    for prime in primes.into_iter() {
        if *prime <= (number as f64).sqrt() as u32 { // is there a better way to do this
            // println!()
            if number % prime == 0 {
                // if it is divisible by any prime then it is composite
                is_prime = false;
            }
        }
    }
    is_prime
}