/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;

    let splitted = code.split(" ").collect::<Vec<&str>>();
    
    let mut every_digit : Vec<i32> = Vec::new();
    let mut is_successful = true;

    if code.len() == 1 {
        return false;
    }

    let mut number_of_digits = 0;

    code.chars().for_each(|c| {
        if c.is_digit(10)
        {
            number_of_digits = number_of_digits + 1;
        }
    });

    if number_of_digits <= 1
    {
        return false;
    }

    splitted.iter().for_each(|each_str| {
        let str_value : &str = *each_str;

        str_value.chars().for_each(|y| {
            let single_char = y.to_digit(10);
            if single_char.is_some() {
                every_digit.push(single_char.unwrap() as i32);
            }
            else
            {
                is_successful = false;
            }
        });
    });

    let every_second_digits = every_digit.iter().enumerate().filter(|&(i, _)| i % 2 == 0).map(|(_, x)| x).collect::<Vec<&i32>>();
    let every_first_digits = every_digit.iter().skip(1).enumerate().filter(|&(i, _)| i % 2 == 0).map(|(_, x)| x).collect::<Vec<&i32>>();
    

    every_first_digits.iter().for_each(|&digit| {
        sum += digit;
    });

    every_second_digits.iter().for_each(|digit| {
        let mut x = *digit * 2;
        if x > 9 {
            x = x - 9;
        }
        sum += x;
    });

    for i in 0..every_first_digits.len() {
        let digit = every_first_digits[i];
        sum += digit;
    }

    if is_successful {
        sum % 10 == 0
    }
    else
    {
        false
    }
}
