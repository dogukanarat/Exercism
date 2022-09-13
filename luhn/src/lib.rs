/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut is_invalid = false;
    let mut numeric_list : Vec<u32> = vec![0];

    let code_without_space = code.replace(" ", "");

    code_without_space.chars().for_each(|c| {
        if let Some(single_digit) = c.to_digit(10)
        {
            numeric_list.push(single_digit);
        }
        else 
        {
            is_invalid = true;
        }
    });

    if numeric_list.len() <= 1
    {
        is_invalid = true;
    }

    if is_invalid
    {
        return false;
    }

    numeric_list.iter().for_each(|digit| {

    });
}
