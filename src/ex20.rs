use uint::construct_uint;

construct_uint! {
	pub struct U1024(16);
}

fn main(){
    let f = factorial(100);
    println!("{f}");
    let sum = f.to_string().chars().filter_map(|a|a.to_string().parse::<u32>().ok()).sum::<u32>();
    println!("{sum}");
}

fn factorial(num: u64) -> U1024{
    let mut bigint = U1024::from(1u32);
    for i in 2..num{
        bigint *= i;
    }
    bigint
}