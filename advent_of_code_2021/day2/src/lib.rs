struct SubMarine {
    position_x: u32,
    depth: u32
}

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl SubMarine{
    fn new() -> SubMarine {
        SubMarine {
            position_x: 0,
            depth: 0,
        }
    }

    fn move_self(&mut self, command: Command) {
        match command {
            Command::Forward(x) => self.position_x += x,
            Command::Up(y) => self.depth -= y,
            Command::Down(y) => self.depth += y,
            }
        }
}

pub fn part_a(input: &str) -> u32 {
    let mut subby = SubMarine::new();

    for line in input.trim().split('\n') {
        let (direction, magnitude) = line.split_at(line.find(' ').unwrap());
        match direction {
            "forward" => {
                let command = Command::Forward(magnitude.trim().parse::<u32>().unwrap());
                subby.move_self(command);
            }
            "up" => {
                let command = Command::Up(magnitude.trim().parse::<u32>().unwrap());
                subby.move_self(command);
            }
            "down" => {
                let command = Command::Down(magnitude.trim().parse::<u32>().unwrap());
                subby.move_self(command);
            }
            _ => ()
        } 
    }
    subby.position_x * subby.depth
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 150)
    }
}
