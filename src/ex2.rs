fn main() {
    let mut vals = vec![1, 2];
    println!("{}", vals[0]);
    let mut sum_even = 0;
    while vals[1] < 4_000_000 {
        println!("{}", vals[1]);
        if vals[1] % 2 == 0 {
            sum_even += vals[1];
        }
        let tmp = vals[0] + vals[1];
        vals[0] = vals[1];
        vals[1] = tmp;
    }
    println!("sum even: {sum_even}");
}
