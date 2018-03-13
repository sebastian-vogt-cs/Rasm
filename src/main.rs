mod register_machine;
use register_machine::Command;

fn main() {
    let mut machine = register_machine::new();
    machine.run(Command::DLOAD, 100);
    machine.run(Command::STORE, 1);
    machine.run(Command::DLOAD, 1);
    machine.run(Command::STORE, 2);
    machine.run(Command::DLOAD, 0);
    machine.run(Command::STORE, 3);
    machine.run(Command::LOAD, 3);
    machine.run(Command::ADD, 1);
    machine.run(Command::STORE, 3);
    machine.run(Command::LOAD, 1);
    machine.run(Command::SUB, 2);
    machine.run(Command::STORE, 1);
    machine.run(Command::JGT, 7);
    machine.run(Command::LOAD, 3);
    machine.run(Command::END, 0);
}
