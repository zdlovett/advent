/*
You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.

However, they do remember a few key facts about the password:

It is a six-digit number.
The value is within the range given in your puzzle input.
Two adjacent digits are the same (like 22 in 122345).
Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
Other than the range rule, the following are true:

111111 meets these criteria (double 11, never decreases).
223450 does not meet these criteria (decreasing pair of digits 50).
123789 does not meet these criteria (no double).
How many different passwords within the range given in your puzzle input meet these criteria?

Your puzzle input is 138241-674034.

---
part 2

An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.

Given this additional criterion, but still ignoring the range rule, the following are now true:

112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
How many different passwords within the range given in your puzzle input meet all of the criteria?


*/
use std::cmp::min;


fn matches(input: i32) -> bool {
    // return true or false if a given number is a possible password
    // println!("checking:{}", input);
    // println!("{}", input);

    let mut counts = [0; 10];

    let mut input = input;
    let mut last = input % 10;
    counts[last as usize] += 1;
    input = input / 10;

    while input != 0 {
        let digit = input % 10;
        input = input / 10;

        counts[digit as usize] += 1;

        if digit > last {
            return false
        }

        last = digit;
    }

    if (counts.iter().sum::<i32>() == 6) & counts.contains(&2) {
        return true;
    } else {
        return false
    }
}


fn count_matching(start: i32, end: i32) -> u32 {
    // return the number of numbers that meet the above criteria
    let mut count = 0;

    for i in start..end {
        if matches(i) {
            count += 1;
        }
    }

    count
}



fn main() {
    println!("Number of possible passwords: {}", count_matching(138241, 674034));
}



#[cfg(test)]
mod test {
    use super::matches;

    #[test]
    fn test_matching(){

        assert_eq!(matches(111111), false);
        assert_eq!(matches(223456), true);
        assert_eq!(matches(223345), true);

        assert_eq!(matches(223450), false);
        assert_eq!(matches(123789), false);

        // too short
        assert_eq!(matches(111), false);
        assert_eq!(matches(12233), false);

        // too long
        assert_eq!(matches(11223344), false);
        assert_eq!(matches(1123450), false);

        assert_eq!(matches(0), false);

        assert_eq!(matches(112233), true); // meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
        assert_eq!(matches(123444), false); // no longer meets the criteria (the repeated 44 is part of a larger group of 444).
        assert_eq!(matches(111122), true); // meets the criteria (even though 1 is repeated more than twice, it still contains a double 22)
    }
}