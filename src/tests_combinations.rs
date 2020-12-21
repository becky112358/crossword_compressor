#[cfg(test)]
mod tests {
    use crate::combinations::*;
    use factorial::Factorial;

    #[test]
    fn test_get_combinations() {
        let n = 6;
        let combinations = get_combinations(n);

        assert_eq!(n.factorial(), combinations.len());

        for combination in &combinations {
            assert_eq!(n, combination.len());
            for m in 0..n {
                assert!(combination.contains(&m));
            }
        }

        for i in 0..combinations.len() {
            for j in i+1..combinations.len() {
                assert!(!combinations[i].eq(&combinations[j]));
            }
        }
    }
}
