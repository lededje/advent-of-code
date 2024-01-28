fn find_lowest_nonce(seed: &str, difficulty: usize) -> Option<i32> {
    let mut current_iteration = 0;
    let max_iterations = 100_000_000;

    if difficulty > 16 {
        return None;
    }

    while current_iteration < max_iterations {
        let data = format!("{seed}{current_iteration}");
        let hash = md5::compute(data);

        let hash_u128: u128 = u128::from_be_bytes(hash.0);

        let bit_shifted_result = hash_u128 >> (128 - difficulty * 4);

        if bit_shifted_result == 0 {
            return Some(current_iteration);
        }

        current_iteration += 1;
    }

    None
}

fn main() {
    let puzzle_input = "bgvyzdsv";

    if let Some(lowest) = find_lowest_nonce(&puzzle_input, 6) {
        print!("The lowest nonce for {} is {}", puzzle_input, lowest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_known_seeds() {
        assert_eq!(find_lowest_nonce("abcdef", 5), Some(609043));
        assert_eq!(find_lowest_nonce("pqrstuv", 5), Some(1048970));
    }
}
