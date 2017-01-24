pub fn square_of_sum(n: usize) -> u64 {
    u64::pow((1..).take(n).sum(), 2)
}

pub fn sum_of_squares(n: usize) -> u64 {
    (1..).take(n).fold(0, |acc, x| acc + x*x)
}

pub fn difference(n: usize) -> u64 {
    square_of_sum(n) - sum_of_squares(n)
}
