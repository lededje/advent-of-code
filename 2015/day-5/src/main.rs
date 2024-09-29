use std::{collections::HashSet, env, fs, path::Path};
use once_cell::sync::Lazy;

static VOWELS: Lazy<HashSet<char>> = Lazy::new(|| {
    HashSet::from(['a', 'e', 'i', 'o', 'u'])
});

fn str_has_three_vowels(test_string: &str) -> bool {
    let number_of_vowels = test_string.chars().fold(0, |acc, char| {
        if VOWELS.contains(&char) {
            return acc +1;
        }
        acc
    });

    number_of_vowels >= 3
}

fn str_has_duplicate_chars(test_string: &str) -> bool {
    let mut previous_char = '\0';

    return test_string.chars().any(|char| {
        if previous_char == char {
            return true;
        }

        previous_char = char;

        false
    });
}

static ILLEGAL_SUBSTRINGS: Lazy<HashSet<&str>> = Lazy::new(|| {
    HashSet::from(["ab", "cd", "pq", "xy"])
});

fn str_has_no_illegal_substrings(test_string: &str) -> bool {
    let mut previous_char = '\0';
    
    let has_illegal_substrings = test_string.chars().any(|char| {
        let key = format!("{}{}", previous_char, char);

        if ILLEGAL_SUBSTRINGS.contains(key.as_str()) {
            return true;
        }

        previous_char = char;

        false
    });

    has_illegal_substrings == false
}

fn str_has_repeating_couple(test_string: &str) -> bool {
    let mut seen_strings: HashSet<String> = HashSet::new();
    let mut previous_chars = ['\0'; 2];

    test_string.chars().any(|char| {
        let newest_key = format!("{}{}", previous_chars[1], char);
        
        if seen_strings.contains(newest_key.as_str()) {
            return true;
        }

        seen_strings.insert(previous_chars.iter().collect());
        previous_chars[0] = previous_chars[1];
        previous_chars[1] = char;

        false
    })
}

fn str_has_repeating_char_one_space_apart(test_string: &str) -> bool {
    let mut previous_chars = ['\0'; 2];

    test_string.chars().any(|char| {
        
        if previous_chars[0] == char {
            return true;
        }

        previous_chars[0] = previous_chars[1];
        previous_chars[1] = char;

        false
    })
}

static DETERMINATION_TESTS_V1: Lazy<Vec<fn(test_string: &str) -> bool>> = Lazy::new(|| {
    vec![str_has_three_vowels, str_has_duplicate_chars, str_has_no_illegal_substrings]
});

fn str_is_nice_v1(test_string: &str) -> bool {
    DETERMINATION_TESTS_V1.iter().all(|&test| test(test_string)) 
}

static DETERMINATION_TESTS_V2: Lazy<Vec<fn(test_string: &str) -> bool>> = Lazy::new(|| {
    vec![str_has_repeating_couple, str_has_repeating_char_one_space_apart]
});

fn str_is_nice_v2(test_string: &str) -> bool {
    DETERMINATION_TESTS_V2.iter().all(|&test| test(test_string)) 
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let puzzle_path = Path::new(&cwd).join("2015/day-5/src/puzzle_input.txt");

    let puzzle_input = fs::read_to_string(puzzle_path).expect("Expected puzzle_input.txt");

    let list = puzzle_input.lines();

    let number_of_nice_v1 = list.clone().into_iter().fold(0, |acc, text_str| {
        if str_is_nice_v1(text_str) == true {
            return acc + 1;
        }

        acc
    });

    println!("Number of nice strings v1: {}", number_of_nice_v1);

    let number_of_nice_v2 = list.clone().into_iter().fold(0, |acc, text_str| {
        if str_is_nice_v2(text_str) == true {
            return acc + 1;
        }

        acc
    });

    println!("Number of nice strings v2: {}", number_of_nice_v2);


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_has_three_vowels() {
        let test_cases = vec![
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("", false),
            ("aa", false)
        ];
        
        test_cases.iter().for_each(|(input, expected_output)| {
            assert_eq!(str_has_three_vowels(input), *expected_output)
        });
    }

    #[test]
    fn test_str_has_duplicate() {
        let test_cases = vec![
            ("ugknbfddgicrmopn", true),
            ("aa", true),
            ("a a", false),
            ("", false),
            ("  ", true),
        ];

        test_cases.iter().for_each(|(input, expected_output)| {
            assert_eq!(str_has_duplicate_chars(input), *expected_output)
        });
    }

    #[test]
    fn test_str_has_no_illegal_substrings() {
        let test_cases = vec![
            ("haegwjzuvuyypxyu", false),
            ("ugknbfddgicrmopn", true)
        ];

        test_cases.iter().for_each(|(input, expected_output)| {
            assert_eq!(str_has_no_illegal_substrings(input), *expected_output)
        });
    }

    #[test]
    fn test_string_has_repeating_double() {
        let test_cases = vec![
            ("xyxy", true),
            ("aabcdefgaa", true),
            ("aaa", false),
        ];

        test_cases.iter().for_each(|(input, expected_output)| {
            assert_eq!(str_has_repeating_couple(input), *expected_output)
        });
    }

    #[test]
    fn test_str_has_repeating_char_one_space_apart() {
        let test_cases = vec![
            ("xyx", true),
            ("abcdefeghi", true),
            ("aaa", true),
            ("uurcxstgmygtbstg", false),
        ];

        test_cases.iter().for_each(|(input, expected_output)| {
            assert_eq!(str_has_repeating_char_one_space_apart(input), *expected_output);
        });
    }
}