// Take 2 strings s1 and s2 including only letters from ato z. Return a new sorted string, the longest possible, containing distinct letters - each taken only once - coming from s1 or s2.
//
// Examples:
// a = "xyaabbbccccdefww"
// b = "xxxxyyyyabklmopq"
// longest(a, b) -> "abcdefklmopqwxy"
//
// a = "abcdefghijklmnopqrstuvwxyz"
// longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"
//
use std::collections::HashSet;

// T: O(Nlog(N)), N is length of a1 + a2
// S: O(N)
fn longest(a1: &str, a2: &str) -> String {
    let mut chars = HashSet::with_capacity(a1.len() + a2.len());
    a1.chars().for_each(|c| {
        chars.insert(c);
    });
    a2.chars().for_each(|c| {
        chars.insert(c);
    });
    let mut sorted_chars = chars.into_iter().collect::<Vec<char>>();
    sorted_chars.sort();
    sorted_chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }
}
