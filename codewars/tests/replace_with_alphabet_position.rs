// Welcome.
// In this kata you are required to, given a string, replace every letter with its position in the alphabet.
//
// If anything in the text isn't a letter, ignore it and don't return it.
//
// "a" = 1, "b" = 2, etc.
//
// Example
// alphabet_position("The sunset sets at twelve o' clock.")
// Should return "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11" (as a string)

// T: O(1)
fn get_alphabet_position(c: char) -> Option<String> {
    let pos = (c.to_ascii_lowercase() as u32).wrapping_sub('a' as u32) + 1;
    if pos > 0 && pos <= 26 {
        Some(pos.to_string())
    } else {
        None
    }
}

// T: O(N), N is length of text
// S: O(N), N is length of text
fn alphabet_position(text: &str) -> String {
    text.chars()
        .filter_map(get_alphabet_position)
        .collect::<Vec<String>>()
        .join(" ")
}

#[test]
fn returns_expected() {
    assert_eq!(
        alphabet_position("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
}
