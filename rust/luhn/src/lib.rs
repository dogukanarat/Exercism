/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut is_valid = true;
    let mut numeric_list : Vec<u8> = Vec::new();

    let code_without_space = code.trim().replace(" ", "");

    code_without_space.chars().for_each(|c| {
        if let Some(single_digit) = c.to_digit(10)
        {
            numeric_list.push(single_digit as u8);
        }
        else 
        {
            is_valid = false;
        }
    });

    if numeric_list.len() < 1
    {
        is_valid = false;
    }

    if numeric_list.len() == 1 && numeric_list[0] == 0
    {
        is_valid = false;
    }

    numeric_list.reverse();

    if is_valid
    {
        numeric_list.iter().enumerate().for_each(|(index, digit)| {
            let mut modified_digit = *digit;
            if index % 2 == 1
            {
                modified_digit = modified_digit * 2;

                if modified_digit > 9
                {
                    modified_digit = modified_digit - 9;
                }
            }

            sum = sum + modified_digit;

        });

        if !(sum % 10 == 0)
        {
            is_valid = false;
        }
    }

    return is_valid;
}
