use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Loc {
    x: i32,
    y: i32,
}

fn parse_position_and_presents(instructions: &str, number_of_santas: usize) -> HashMap<Loc, i32> {
    let mut locs = Vec::new();

    for _ in 0..number_of_santas {
        locs.push(Loc { x: 0, y: 0 });
    }

    let mut gift_map: HashMap<Loc, i32> = HashMap::new();

    gift_map.insert(Loc { x: 0, y: 0 }, 1);

    for (index, instruction) in instructions.char_indices() {
        let current_santa_location = &mut locs[index % number_of_santas];
        match instruction {
            '>' => current_santa_location.x += 1,
            'v' => current_santa_location.y += 1,
            '<' => current_santa_location.x -= 1,
            '^' => current_santa_location.y -= 1,
            _ => {
                continue;
            }
        }
        gift_map
            .entry(Loc {
                x: current_santa_location.x,
                y: current_santa_location.y,
            })
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    return gift_map;
}

fn main() {
    let puzzle_input = "";

    let present_hash = parse_position_and_presents(puzzle_input, 2);

    println!(
        "Number of places with more than one present: {}",
        present_hash.iter().count(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_parse_position_and_presents() {
        // > delivers presents to 2 houses: one at the starting location, and one to the east.
        assert_debug_snapshot!(parse_position_and_presents(">", 1));

        // ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
        assert_debug_snapshot!(parse_position_and_presents("^>v<", 1));

        // ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.
        assert_debug_snapshot!(parse_position_and_presents("^v^v^v^v^v", 1));
    }
}
