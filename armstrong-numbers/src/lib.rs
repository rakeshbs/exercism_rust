pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    let mut sum = 0;
    let mut n = num;
    while n > 0 {
        sum += u32::pow(n % 10, len);
        n /= 10;
    }
    if sum == num {
        return true;
    }
    return false;
}
