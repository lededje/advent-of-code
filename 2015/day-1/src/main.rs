fn parse_floors(directions: &str) -> (i32, Option<u32>) {
    let mut floor = 0;
    let mut position_santa_enters_basement: Option<u32> = None;

    for (index, c) in directions.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                println!("Found unsupported character '{}'", c);
            }
        }
        if floor == -1 && position_santa_enters_basement.is_none() {
            position_santa_enters_basement = Some(index as u32 + 1);
        }
    }

    return (floor, position_santa_enters_basement);
}

fn main() {
    let puzzle_input = "";

    let (floor, position_santa_enters_basement) = parse_floors(puzzle_input);

    println!(
        "Final floor: {}\nPosition santa goes to basement: {:?}",
        floor, position_santa_enters_basement
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_floors() {
        assert_eq!(parse_floors("(())").0, 0);
        assert_eq!(parse_floors("()()").0, 0);
        assert_eq!(parse_floors("(((").0, 3);
        assert_eq!(parse_floors("(()(()(").0, 3);
        assert_eq!(parse_floors("))(((((").0, 3);
        assert_eq!(parse_floors("())").0, -1);
        assert_eq!(parse_floors("))(").0, -1);
        assert_eq!(parse_floors(")))").0, -3);
        assert_eq!(parse_floors(")())())").0, -3);
        assert_eq!(parse_floors(")()a)(e))").0, -3);
    }

    #[test]
    fn test_basement_entry() {
        assert_eq!(parse_floors(")").1, Some(1));
        assert_eq!(parse_floors("()())").1, Some(5));
        assert_eq!(parse_floors("(((").1, None);
    }
}
