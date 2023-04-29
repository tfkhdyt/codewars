fn main() {
    let result = plural(2.5);
    println!("Result = {}", result)
}

fn plural(n: f64) -> bool {
    if n == 1.0 {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural() {
        assert_eq!(plural(0.0), true);
        assert_eq!(plural(0.5), true);
        assert_eq!(plural(1.0), false);
        assert_eq!(plural(100.0), true);
    }
}
