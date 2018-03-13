pub enum Command {
    LOAD,
    DLOAD,
    STORE,
    ADD,
    SUB,
    MULT,
    DIV,
    JUMP,
    JGE,
    JGT,
    JLE,
    JLT,
    JEQ,
    JNE,
    END,
}


#[derive(Copy, Clone)]
pub struct Register {
    value: isize,
}


#[derive(Copy, Clone)]
pub struct RegisterMachine {
    accumulator: Register,
    instuction_counter: Register,
    data_registers: [Register; 16],
}

pub fn new() -> RegisterMachine {
    RegisterMachine {
        accumulator: Register{
            value: 0,
        },
        instuction_counter: Register {
            value: 0,
        },
        data_registers: [Register{value: 0,}; 16],
    }
}


impl Register {

    fn add(&mut self, n: isize) {
        self.value += n;
    }

    fn sub(&mut self, n: isize) {
        self.value -= n;
    }

    fn mult(&mut self, n: isize) {
        self.value = self.value * n;
    }

    fn div(&mut self, n: isize) {
        self.value = self.value / n;
    }

}


impl RegisterMachine{

    pub fn run(&mut self, command: Command, n: isize){
        match command{
            Command::LOAD   => self.load(n),
            Command::DLOAD  => self.d_load(n),
            Command::STORE  => self.store(n),
            Command::ADD    => self.add(n),
            Command::SUB    => self.sub(n),
            Command::MULT   => self.mult(n),
            Command::DIV    => self.div(n),
            Command::JUMP   => self.jump(n),
            Command::JGE    => self.jge(n),
            Command::JGT    => self.jgt(n),
            Command::JLE    => self.jle(n),
            Command::JLT    => self.jlt(n),
            Command::JEQ    => self.jeq(n),
            Command::JNE    => self.jne(n),
            Command::END    => self.end(),
        }
    }

    fn load(&mut self, n: isize) {
        if (n > 15) && (n < 0) {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.value = self.data_registers[n as usize].value;
    }

    fn d_load(&mut self, n: isize) {
        self.instuction_counter.add(1);
        self.accumulator.value = n;
    }

    fn store(&mut self, n: isize) {
        if (n > 15) && (n < 0) {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.data_registers[n as usize].value = self.accumulator.value;
    }

    fn add(&mut self, n: isize) {
        if (n > 15) && (n < 0) {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.add(self.data_registers[n as usize].value);
    }

    fn sub(&mut self, n: isize) {
        if (n > 15) && (n < 0) {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.sub(self.data_registers[n as usize].value);
    }

    fn mult(&mut self, n: isize) {
        if (n > 15) && (n < 0) {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.mult(self.data_registers[n as usize].value);
    }

    fn div(&mut self, n: isize) {
        if (n > 15) && (n < 0) {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.div(self.data_registers[n as usize].value);
    }

    fn jump(&mut self, n: isize) {
        self.instuction_counter.value = n;
    }

    fn jge(&mut self, n: isize) {
        if self.accumulator.value >= 0{
            self.instuction_counter.value = n;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jgt(&mut self, n: isize) {
        if self.accumulator.value > 0{
            self.instuction_counter.value = n;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jle(&mut self, n: isize) {
        if self.accumulator.value <= 0{
            self.instuction_counter.value = n;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jlt(&mut self, n: isize) {
        if self.accumulator.value < 0{
            self.instuction_counter.value = n;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jeq(&mut self, n: isize) {
        if self.accumulator.value == 0{
            self.instuction_counter.value = n;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jne(&mut self, n: isize) {
        if self.accumulator.value != 0{
            self.instuction_counter.value = n;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn end(&mut self) {
        println!("accumulator: {}", self.accumulator.value);
        self.accumulator.value = 0;
        self.instuction_counter.value = 0;
        for i in 0..self.data_registers.len(){
            self.data_registers[i].value = 0;
        }
    }

}
