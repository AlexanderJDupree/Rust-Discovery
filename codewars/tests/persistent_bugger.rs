// Write a function, persistence, that takes in a positive parameter num and returns its multiplicative persistence, which is the number of times you must multiply the digits in num until you reach a single digit.
//
// For example:
//
// persistence(39) // returns 3, because 3*9=27, 2*7=14, 1*4=4
//                // and 4 has only one digit
//
// persistence(999) // returns 4, because 9*9*9=729, 7*2*9=126,
//                 // 1*2*6=12, and finally 1*2=2
//
// persistence(4) // returns 0, because 4 is already a one-digit number
//

// T: O(N * M), N is number of digits, M is persistence
// S: O(M)
fn persistence(mut num: u64) -> u64 {
    if num < 10 {
        return 0;
    }

    let mut digits = vec![];
    while num >= 10 {
        digits.push(num % 10);
        num = num / 10;
    }
    digits.push(num);
    1 + persistence(digits.iter().fold(1, |acc, n| acc * n))
}

#[test]
fn sample_tests() {
    assert_eq!(persistence(39), 3);
    assert_eq!(persistence(4), 0);
    assert_eq!(persistence(25), 2);
    assert_eq!(persistence(999), 4);
    assert_eq!(persistence(101), 1);
}
