fn main() {
    let limit = 1_000u32;
    for a in 0u32..limit {
        for b in a + 1u32..limit {
            for c in b + 1u32..limit {
                let a2 = a.pow(2);
                let b2 = b.pow(2);
                let c2 = c.pow(2);
                match c2.cmp(&(a2 + b2)) {
                    std::cmp::Ordering::Equal => {
                        println!("A pytagorean triplet: {a}² + {b}² = {c}²");
                        if a + b + c == 1000 {
                            println!("Found the right triplet! {}", a * b * c);
                            return;
                        }
                    }
                    std::cmp::Ordering::Greater => break,
                    _ => {}
                }
            }
        }
    }
}
