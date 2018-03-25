use register_machine::Command;


pub fn parse(input: &str) -> (bool, Vec<Command>) {

    let mut commands: Vec<Command> = Vec::new();

    for line in input.split("\n") {

        let line_split: Vec<&str> = line.split("--").collect();
        let command = line_split[0];

        let command_split: Vec<&str> = command.split_whitespace().collect();

        match command_split[0] {
            "LOAD" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                if argument <= 15 {
                    commands.push(Command::LOAD(argument as u8));
                } else {
                    return (false, commands);
                }
            },
            "DLOAD" => {
                let mut argument: isize = 0;
                let mut first_one = true;
                let mut negation = false;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as isize;
                    } else if (c == '-') && first_one {
                        first_one = false;
                        negation = true;
                    } else {
                        return (false, commands);
                    }
                    if negation {
                        argument = - argument;
                    }
                }
                commands.push(Command::DLOAD(argument))
            },
            "STORE" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                if argument <= 15 {
                    commands.push(Command::STORE(argument as u8));
                } else {
                    return (false, commands);
                }
            },
            "ADD" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                if argument <= 15 {
                    commands.push(Command::ADD(argument as u8));
                } else {
                    return (false, commands);
                }
            },
            "SUB" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                if argument <= 15 {
                    commands.push(Command::SUB(argument as u8));
                } else {
                    return (false, commands);
                }
            },
            "MULT" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                if argument <= 15 {
                    commands.push(Command::MULT(argument as u8));
                } else {
                    return (false, commands);
                }
            },
            "DIV" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                if argument <= 15 {
                    commands.push(Command::DIV(argument as u8));
                } else {
                    return (false, commands);
                }
            },
            "JUMP" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                commands.push(Command::JUMP(argument));
            },
            "JGE" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                commands.push(Command::JGE(argument));
            },
            "JGT" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                commands.push(Command::JGT(argument));
            },
            "JLE" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                commands.push(Command::JLE(argument));
            },
            "JLT" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                commands.push(Command::JLT(argument));
            },
            "JEQ" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                commands.push(Command::JEQ(argument));
            },
            "JNE" => {
                let mut argument: usize = 0;
                for c in command_split[1].chars() {
                    if c.is_numeric() {
                        argument = argument * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return (false, commands);
                    }
                }
                commands.push(Command::JNE(argument));
            },
            "END" => {
                commands.push(Command::END);
            },
            _     => {
                return (false, commands);
            }
        }
    }

    if commands.len() >= 2 {
        (true, commands)
    } else {
        (false, commands)
    }

}