mod controller;
mod register_machine;
use controller::Controller;
mod parser;


fn main() {

    //you can now parse a String to commands! The example in the comment below makes it the "old fashioned" way
    let commands = parser::parse(
                                    "DLOAD 10 -- some command
                                    STORE 1 -- comands work!
                                    DLOAD 0
                                    STORE 2
                                    DLOAD 1
                                    STORE 3
                                    DLOAD 0
                                    STORE 4
                                    DLOAD 1
                                    STORE 5
                                    SUB 1
                                    JGT 23
                                    LOAD 3
                                    STORE 4
                                    LOAD 2
                                    STORE 3
                                    ADD 4
                                    STORE 2
                                    DLOAD 1
                                    ADD 5
                                    STORE 5
                                    JUMP 11
                                    LOAD 2
                                    END"
                                );

    if commands.0 {
        let mut controller = Controller::new(commands.1);
        let last_state = controller.run();
        println!("\nThis time we let it all run through at once. last state of the accumulator is: {}", last_state.get_accumulator());
    } else {
        println!("Error");
    }

    /*
    //these commands are the programm we want to simulate
    let commands = vec![Command::DLOAD(10), Command::STORE(1), Command::DLOAD(0), Command::STORE(2), Command::DLOAD(1), Command::STORE(3), Command::DLOAD(0),
                    Command::STORE(4), Command::DLOAD(1), Command::STORE(5), Command::SUB(1), Command::JGT(23), Command::LOAD(3), Command::STORE(4),
                    Command::LOAD(2), Command::STORE(3), Command::ADD(4), Command::STORE(2), Command::DLOAD(1), Command::ADD(5), Command::STORE(5),
                    Command::JUMP(11), Command::LOAD(2), Command::END];

    //initiale new controller
    let mut controller = Controller::new(commands);
    
    //use function "next_step" for full controll over the machine (doesn' make sense here but will in the UI)
    loop{
        //the function next_step returns the register_machine in the current state and a boolean to indicate the END command.
        let (new_state, end) = controller.next_step();
        if !end {
            println!("The next state of the accumulator is: {}", new_state.get_accumulator());
        }else {
            println!("We reached the end, all fields of the machine have been retet to: {}", new_state.get_accumulator());
            break;
        }
    }

    //the function "run" runs the whole code at once and returns the end state of the machine
    let last_state = controller.run();
    println!("\nThis time we let it all run through at once. last state of the accumulator is: {}", last_state.get_accumulator());
    */

}
