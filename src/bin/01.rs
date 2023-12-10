advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut first_digit:Option<u32> = None;
        let mut last_digit:Option<u32> = None;

        for char in line.chars() {
            if !char.is_digit(10) {
                continue;
            }

            last_digit = char.to_digit(10);
            if first_digit == None {
                first_digit = last_digit;
            }
        }

        if first_digit.is_some() && last_digit.is_some()
        {
            let number = first_digit.unwrap() * 10 + last_digit.unwrap();
            sum += number;
        }
    }

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let number_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        // Find first digit
        let mut first_digit_index = line.find(|c: char| c.is_digit(10));
        let mut first_digit = line.chars().nth(first_digit_index.unwrap())?.to_digit(10);

        for (pos, word) in number_words.iter().enumerate() {
            let index = line.find(word);
            if index.is_some() && index < first_digit_index {
                first_digit_index = index;
                first_digit = Some(pos as u32 + 1);
            }
        }

        // Find last digit
        let mut last_digit_index = line.rfind(|c: char| c.is_digit(10));
        let mut last_digit = line.chars().nth(last_digit_index.unwrap())?.to_digit(10);

        for (pos, word) in number_words.iter().enumerate() {
            let index = line.rfind(word);
            if index > last_digit_index {
                last_digit_index = index;
                last_digit = Some(pos as u32 + 1);
            }
        }

        // Add to sum
        if first_digit.is_some() && last_digit.is_some()
        {
            let number = first_digit.unwrap() * 10 + last_digit.unwrap();
            sum += number;
        }
    }

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
