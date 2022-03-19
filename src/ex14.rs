fn main() {
    let mut longest_chain = 0;
    let mut longest_chain_number = 0;
    for i in 1..1_000_000{
        let chain = calculate_chain(i);
        if chain > longest_chain{
            longest_chain = chain;
            longest_chain_number = i;
        }
    }
    println!("{longest_chain_number}");
} 

fn calculate_chain(mut num: u32) -> usize {
    let mut counter = 1;
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }
        counter += 1;
    }
    counter
}
