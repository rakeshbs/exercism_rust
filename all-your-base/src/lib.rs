#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    } else if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    } else {
        let mut value: u64 = 0;
        let mut power = number.len() as i32 - 1;
        for digit in number {
            if *digit >= from_base {
                return Err(Error::InvalidDigit(*digit));
            }
            value += (digit * from_base.pow(power as u32)) as u64;
            power -= 1;
        }

        let mut result: Vec<u32> = Vec::new();

        loop {
            let digit = value % (to_base as u64);
            result.push(digit as u32);
            value /= to_base as u64;
            if value <= 0 {
                break;
            }
        }
        result.reverse();
        Ok(result)
    }
}
