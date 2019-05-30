pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit).fold(0, |acc, x| {
        let mut is_multiple = false;
        for number in factors.into_iter() {
            if *number != 0 {
                if x % number == 0 {
                    is_multiple = true;
                }
            }
        }
        if is_multiple {
            return acc + x;
        }
        acc
    })
}
