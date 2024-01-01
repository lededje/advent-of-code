struct Present {
    height: i32,
    width: i32,
    length: i32,
}

impl Present {
    fn parse(input: &str) -> Present {
        let parts: Vec<&str> = input.split('x').collect();

        let height: i32 = parts[0].parse().expect("Not an integer");
        let length: i32 = parts[1].parse().expect("Not an integer");
        let width: i32 = parts[2].parse().expect("Not an integer");

        return Present {
            height,
            width,
            length,
        };
    }
    fn required_paper(&self) -> i32 {
        let lw = &self.length * &self.width;
        let wh = &self.width * &self.height;
        let hl = &self.height * &self.length;

        let smallest_side = lw.min(wh).min(hl);

        return 2 * lw + 2 * wh + 2 * hl + smallest_side;
    }

    fn required_ribbon(&self) -> i32 {
        let bow = &self.height * &self.length * &self.width;

        let two_l_two_w = 2 * &self.length + 2 * &self.width;
        let two_w_two_h = 2 * &self.width + 2 * &self.height;
        let two_h_two_l = 2 * &self.height + 2 * &self.length;

        let smallest_perimeter = two_l_two_w.min(two_w_two_h).min(two_h_two_l);

        return bow + smallest_perimeter;
    }
}

fn main() {
    let puzzle_input = r#""#;

    let puzzle_inputs = puzzle_input.split("\n");

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for input in puzzle_inputs {
        let current_present = Present::parse(input);
        total_paper += current_present.required_paper();
        total_ribbon += current_present.required_ribbon();
    }

    println!(
        "Total paper required {},\nTotal ribbon required {}",
        total_paper, total_ribbon
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present_required_paper() {
        assert_eq!(Present::parse("2x3x4").required_paper(), 58);
        assert_eq!(Present::parse("1x1x10").required_paper(), 43);
    }

    #[test]
    fn test_present_required_ribbon() {
        assert_eq!(Present::parse("2x3x4").required_ribbon(), 34);
        assert_eq!(Present::parse("1x1x10").required_ribbon(), 14);
    }
}
