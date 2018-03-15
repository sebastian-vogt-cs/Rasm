pub mod register_machine;
use self::register_machine::RegisterMachine;
use self::register_machine::Command;


pub struct Controller {
    register_machine: RegisterMachine,
    commands: Vec<Command>,
}


impl Controller {

    pub fn new(commands: Vec<Command>) -> Controller {
        Controller{
            register_machine: RegisterMachine::new(),
            commands: commands,
        }
    }

    pub fn new_commands(&mut self, commands: Vec<Command>) {
        self.commands = commands;
    }

    pub fn run(&mut self) -> RegisterMachine {
        let ret: RegisterMachine;
        loop{
            let command_n = self.register_machine.get_instruction_counter() - 1;
            if self.commands[command_n] == Command::END{
                ret = self.register_machine;
                self.register_machine.run(Command::END);
                break;
            }
            self.register_machine.run(self.commands[command_n]);
        }
        ret
    }

    pub fn next_step(&mut self) -> (RegisterMachine, bool) {
        let command_n = self.register_machine.get_instruction_counter() - 1;
        self.register_machine.run(self.commands[command_n]);
        if self.commands[command_n] == Command::END {
            (self.register_machine, true)
        }else {
            (self.register_machine, false)
        }
        
    }

}