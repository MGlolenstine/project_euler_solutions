fn main() {
    let mut sum_squares = 0;
    let mut square_sums = 0;
    for i in 0..=100u32 {
        sum_squares += i.pow(2);
        square_sums += i;
    }
    square_sums = square_sums.pow(2);
    println!("{}", square_sums - sum_squares);
}
