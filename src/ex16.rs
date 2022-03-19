use uint::construct_uint;

construct_uint! {
	pub struct U1024(16);
}

fn main() {
    let pow: u32 = 1000;
    let bigint = U1024::from(2u32);
    let number = bigint.pow(U1024::from(pow)).to_string();
    let sum = number
        .chars()
        .map(|c| c.to_string().parse::<u128>().unwrap())
        .sum::<u128>();
    println!("{sum}");
}
