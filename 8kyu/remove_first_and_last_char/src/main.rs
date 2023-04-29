fn main() {
    println!("Hello, world!");
}

pub fn remove_char(s: &str) -> String {
    // Your code here!
    let s = &s[1..];
    let s = &s[..s.len() - 1];
    String::from(s)
}

pub fn remove_char_best_practice(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_cases() {
        assert_eq!(remove_char_best_practice("eloquent"), "loquen");
        assert_eq!(remove_char_best_practice("country"), "ountr");
        assert_eq!(remove_char_best_practice("person"), "erso");
        assert_eq!(remove_char_best_practice("place"), "lac");
        assert_eq!(remove_char_best_practice("ok"), "");
        assert_eq!(remove_char_best_practice("ooopsss"), "oopss");
    }
}
