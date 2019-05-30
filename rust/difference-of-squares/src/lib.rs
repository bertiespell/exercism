pub fn square_of_sum(n: u32) -> u32 {
    let number = (0..n + 1).fold(0, |acc, x| acc + x);
    number * number
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..n+1).fold(0, |acc, x| acc + (x*x))
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
