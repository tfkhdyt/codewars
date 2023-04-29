fn main() {
    println!("Hello, world!");
}

fn positive_sum(slice: &[i32]) -> i32 {
    if slice.len() == 0 {
        return 0;
    }
    slice.iter().filter(|&x| x >= &0).sum()
}

fn positive_sum_best_practice(slice: &[i32]) -> i32 {
    if slice.len() == 0 {
        return 0;
    }
    slice.iter().filter(|&x| x.is_positive()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum_best_practice(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(positive_sum_best_practice(&[1, -2, 3, 4, 5]), 13);
        assert_eq!(positive_sum_best_practice(&[-1, 2, 3, 4, -5]), 9);
    }

    #[test]
    fn empty_list() {
        assert_eq!(positive_sum_best_practice(&[]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(positive_sum_best_practice(&[-1, -2, -3, -4, -5]), 0);
    }
}
