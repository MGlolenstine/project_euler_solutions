fn main() {
    let mut sum = 0;
    for i in 2..2_000_000 {
        if is_prime(i) {
            sum += i;
        }
    }
    println!("sum: {sum}");
}

fn is_prime(input: u64) -> bool {
    let sqrt = (input as f64).sqrt().floor() as u64;
    for i in 2..=sqrt {
        if input % i == 0 {
            return false;
        }
    }
    true
}
