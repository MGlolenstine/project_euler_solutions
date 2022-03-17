fn main() {
    let mut biggest = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            if is_palindrome(i * j) {
                if i * j > biggest {
                    biggest = i * j;
                }
                println!("{i}+{j} = {}", i * j);
            }
        }
    }
    println!("Biggest: {biggest}");
}

fn is_palindrome(num: u32) -> bool {
    let s = num.to_string();
    s == s.chars().rev().collect::<String>()
}
