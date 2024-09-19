use direction::Direction;

type Error = Box<dyn core::error::Error>;
type Result<T> = std::result::Result<T, Error>;

mod direction {
    use crate::Error;

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub enum Direction {
        West,
        North,
        East,
        South,
    }

    impl TryFrom<char> for Direction {
        type Error = Error;

        fn try_from(direction: char) -> Result<Self, Self::Error> {
            Ok(match direction {
                'W' => Direction::West,
                'N' => Direction::North,
                'E' => Direction::East,
                'S' => Direction::South,
                _ => Err(format!("Invalid direction {}", direction))?,
            })
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Command {
    SelectPen(u32),
    PenDown,
    PenUp,
    Draw {
        direction: Direction,
        centimeters: u32,
    },
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(line: &str) -> std::result::Result<Self, Self::Error> {
        let mut chars = line.chars();
        let command = chars.next().ok_or("line should not be empty")?;

        let parse_number = || {
            chars.next();
            chars
                .take_while(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<u32>()
        };

        let command = match command {
            'D' => Command::PenDown,
            'U' => Command::PenUp,
            direction @ ('W' | 'N' | 'S' | 'E') => {
                let direction = direction.try_into()?;
                let centimeters = parse_number()?;
                Command::Draw {
                    direction,
                    centimeters,
                }
            }
            'P' => {
                let id = parse_number()?;
                Command::SelectPen(id)
            }
            invalid => Err(format!("Invalid command {invalid}"))?,
        };

        Ok(command)
    }
}

pub fn parse_lines(lines: &str) -> Result<Vec<Command>> {
    lines.lines().map(Command::try_from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stuff() -> Result<()> {
        let program = "P 2  # select pen 2
D    # pen down
W 2  # draw west 2cm
N 1  # then north 1
E 2  # then east 2
S 1  # then back south
U    # pen up";

        assert_eq!(
            parse_lines(program)?,
            [
                Command::SelectPen(2),
                Command::PenDown,
                Command::Draw {
                    direction: Direction::West,
                    centimeters: 2
                },
                Command::Draw {
                    direction: Direction::North,
                    centimeters: 1
                },
                Command::Draw {
                    direction: Direction::East,
                    centimeters: 2
                },
                Command::Draw {
                    direction: Direction::South,
                    centimeters: 1
                },
                Command::PenUp
            ]
        );

        Ok(())
    }
}
