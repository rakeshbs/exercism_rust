/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned = code.replace(" ", "");
    let mut sum = 0;
    for (i, c) in cleaned.chars().rev().enumerate() {
        match c.to_digit(10) {
            Some(mut num) => {
                println!("n : {}", num);
                if i % 2 != 0 {
                    num *= 2;
                    if num > 9 {
                        num -= 9;
                    }
                }
                sum += num;
            }
            None => return false,
        }
    }

    sum % 10 == 0 && cleaned.len() > 1
}
