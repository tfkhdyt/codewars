fn get_age(age: &str) -> u32 {
    // Your code here
    // let age = age.split(" ").collect::<Vec<&str>>();
    // let age_int: u32 = age[0].parse().unwrap_or(0);
    // age_int
    age[..1].parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(get_age("2 years old"), 2);
        assert_eq!(get_age("4 years old"), 4);
        assert_eq!(get_age("5 years old"), 5);
        assert_eq!(get_age("7 years old"), 7);
    }
}
