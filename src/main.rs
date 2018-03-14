mod register_machine;
use register_machine::RegisterMachine;
use register_machine::Command;

fn main() {
    let commands = [Command::DLOAD(10), Command::STORE(1), Command::DLOAD(0), Command::STORE(2), Command::DLOAD(1), Command::STORE(3), Command::DLOAD(0),
                    Command::STORE(4), Command::DLOAD(1), Command::STORE(5), Command::SUB(1), Command::JGT(23), Command::LOAD(3), Command::STORE(4),
                    Command::LOAD(2), Command::STORE(3), Command::ADD(4), Command::STORE(2), Command::DLOAD(1), Command::ADD(5), Command::STORE(5),
                    Command::JUMP(11), Command::LOAD(2), Command::END];

    let mut machine = RegisterMachine::new();
    loop{
        let counter = commands[machine.get_instruction_counter() - 1];
        machine.run(counter);
        if counter == Command::END{
            break;
        }
    }

}
