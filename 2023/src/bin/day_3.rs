use std::fs;

#[derive(Clone)]
struct Index {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Number {
    value: u32,
    index: Vec<Index>,
}

fn parse_coords(x: usize, y: usize) -> (i32, i32) {
    (i32::try_from(x).unwrap(), i32::try_from(y).unwrap())
}

fn make_index(x: usize, y: usize) -> Index {
    Index {
        x: i32::try_from(x).unwrap(),
        y: i32::try_from(y).unwrap(),
    }
}

fn part_1(input: &String) -> u32 {
    let motor = input.split("\n");
    let mut symbols: Vec<Index> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    motor.enumerate().for_each(|(x, line)| {
        let mut aux_number = String::from("");
        let mut aux_index: Vec<Index> = vec![];

        line.chars().enumerate().for_each(|(y, c)| {
            if c.is_digit(10) {
                aux_number.push(c);
                let (xx, yy) = parse_coords(x, y);

                aux_index.push(Index {
                    x: xx - 1,
                    y: yy - 1,
                });
                aux_index.push(Index { x: xx - 1, y: yy });
                aux_index.push(Index {
                    x: xx - 1,
                    y: yy + 1,
                });
                aux_index.push(Index { x: xx, y: yy - 1 });
                aux_index.push(Index { x: xx, y: yy + 1 });
                aux_index.push(Index {
                    x: xx + 1,
                    y: yy - 1,
                });
                aux_index.push(Index { x: xx + 1, y: yy });
                aux_index.push(Index {
                    x: xx + 1,
                    y: yy + 1,
                });
            } else {
                if aux_number != "" {
                    numbers.push(Number {
                        value: aux_number.parse().unwrap(),
                        index: aux_index.clone(),
                    });
                    aux_number = String::from("");
                    aux_index.clear();
                }

                if c != '.' {
                    symbols.push(make_index(x, y))
                }
            }
        });

        if aux_number != "" {
            numbers.push(Number {
                value: aux_number.parse().unwrap(),
                index: aux_index.clone(),
            });
        }
    });

    numbers
        .iter()
        .map(|n| {
            let is_valid = n.index.iter().any(|index| {
                symbols
                    .iter()
                    .any(|symbol| symbol.x == index.x && symbol.y == index.y)
            });

            match is_valid {
                true => n.value,
                false => 0,
            }
        })
        .sum()
}

fn part_2(input: &String) -> u32 {
    let motor = input.split("\n");
    let mut symbols: Vec<Index> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    motor.enumerate().for_each(|(x, line)| {
        let mut aux_number = String::from("");
        let mut aux_index: Vec<Index> = vec![];

        line.chars().enumerate().for_each(|(y, c)| {
            if c.is_digit(10) {
                aux_number.push(c);
                let (xx, yy) = parse_coords(x, y);

                aux_index.push(Index {
                    x: xx - 1,
                    y: yy - 1,
                });
                aux_index.push(Index { x: xx - 1, y: yy });
                aux_index.push(Index {
                    x: xx - 1,
                    y: yy + 1,
                });
                aux_index.push(Index { x: xx, y: yy - 1 });
                aux_index.push(Index { x: xx, y: yy + 1 });
                aux_index.push(Index {
                    x: xx + 1,
                    y: yy - 1,
                });
                aux_index.push(Index { x: xx + 1, y: yy });
                aux_index.push(Index {
                    x: xx + 1,
                    y: yy + 1,
                });
            } else {
                if aux_number != "" {
                    numbers.push(Number {
                        value: aux_number.parse().unwrap(),
                        index: aux_index.clone(),
                    });
                    aux_number = String::from("");
                    aux_index.clear();
                }

                if c == '*' {
                    symbols.push(make_index(x, y))
                }
            }
        });

        if aux_number != "" {
            numbers.push(Number {
                value: aux_number.parse().unwrap(),
                index: aux_index.clone(),
            });
        }
    });

    symbols
        .iter()
        .map(|symbol| {
            let mut values: Vec<u32> = vec![];

            numbers.iter().for_each(|num| {
                let is_adjacent = num.index.iter().any(|i| symbol.x == i.x && symbol.y == i.y);
                if is_adjacent {
                    values.push(num.value);
                }
            });

            match values.len() {
                2 => values[0] * values[1],
                _ => 0,
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("src/inputs/03").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
