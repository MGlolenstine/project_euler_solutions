fn main() {
    let limit = 20;
    // let num = 2520;
    for i in 1..u32::MAX {
        let mut pass = true;
        for j in 2..limit {
            if i % j != 0 {
                pass = false;
                break;
            }
        }
        if pass {
            println!("{i}");
            break;
        }
    }
}
