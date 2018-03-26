#[derive(Copy, Clone, PartialEq)]
pub enum Command {
    LOAD(u8),
    DLOAD(isize),
    STORE(u8),
    ADD(u8),
    SUB(u8),
    MULT(u8),
    DIV(u8),
    JUMP(usize),
    JGE(usize),
    JGT(usize),
    JLE(usize),
    JLT(usize),
    JEQ(usize),
    JNE(usize),
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
    data_registers: [Register; 15],
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

    fn get_value(&self) -> isize {
        self.value
    }

}


impl RegisterMachine{

    pub fn new() -> RegisterMachine {
        RegisterMachine {
            accumulator: Register{
                value: 0,
            },
            instuction_counter: Register {
                value: 1,
            },
            data_registers: [Register{value: 0,}; 15],
        }
    }

    pub fn run(&mut self, command: Command) {
        match command{
            Command::LOAD(i)   => self.load(i),
            Command::DLOAD(i)  => self.d_load(i),
            Command::STORE(i)  => self.store(i),
            Command::ADD(i)    => self.add(i),
            Command::SUB(i)    => self.sub(i),
            Command::MULT(i)   => self.mult(i),
            Command::DIV(i)    => self.div(i),
            Command::JUMP(i)   => self.jump(i),
            Command::JGE(i)    => self.jge(i),
            Command::JGT(i)    => self.jgt(i),
            Command::JLE(i)    => self.jle(i),
            Command::JLT(i)    => self.jlt(i),
            Command::JEQ(i)    => self.jeq(i),
            Command::JNE(i)    => self.jne(i),
            Command::END       => self.end(),
        }
    }

    pub fn get_instruction_counter(&self) -> usize {
        self.instuction_counter.value as usize
    }

    pub fn get_accumulator(&self) -> isize {
        self.accumulator.value
    }

    pub fn get_registers(&self) -> [Register; 15] {
        self.data_registers
    }

    pub fn get_registers_to_string(&self) -> [String; 17] {
        let mut stry = String::new();
        let mut ret_str: [String; 17] = [" ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()];
        stry.push_str("Accumulator: ");
        stry.push_str(&self.get_accumulator().to_string());
        ret_str[0] = stry;
        stry = String::new();
        stry.push_str("Instruction counter: ");
        stry.push_str(&self.get_instruction_counter().to_string());
        ret_str[1] = stry;
        let registers = self.get_registers();
        let mut i = 0;
        for register in registers.iter() {
            stry = String::new();
            stry.push_str("R");
            stry.push_str(&i.to_string());
            stry.push_str(": ");
            stry.push_str(&register.get_value().to_string());
            ret_str[2 + i] = stry;
            i += 1;
        }
        ret_str
    }

    fn load(&mut self, n: u8) {
        if n > 15 {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.value = self.data_registers[n as usize].value;
    }

    fn d_load(&mut self, n: isize) {
        self.instuction_counter.add(1);
        self.accumulator.value = n;
    }

    fn store(&mut self, n: u8) {
        if n > 15 {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.data_registers[n as usize].value = self.accumulator.value;
    }

    fn add(&mut self, n: u8) {
        if n > 15 {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.add(self.data_registers[n as usize].value);
    }

    fn sub(&mut self, n: u8) {
        if n > 15 {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.sub(self.data_registers[n as usize].value);
    }

    fn mult(&mut self, n: u8) {
        if n > 15 {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.mult(self.data_registers[n as usize].value);
    }

    fn div(&mut self, n: u8) {
        if n > 15 {
            panic!(format!("Register {} does not exist.", n));
        }
        self.instuction_counter.add(1);
        self.accumulator.div(self.data_registers[n as usize].value);
    }

    fn jump(&mut self, n: usize) {
        self.instuction_counter.value = n as isize;
    }

    fn jge(&mut self, n: usize) {
        if self.accumulator.value >= 0{
            self.instuction_counter.value = n as isize;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jgt(&mut self, n: usize) {
        if self.accumulator.value > 0{
            self.instuction_counter.value = n as isize;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jle(&mut self, n: usize) {
        if self.accumulator.value <= 0{
            self.instuction_counter.value = n as isize;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jlt(&mut self, n: usize) {
        if self.accumulator.value < 0{
            self.instuction_counter.value = n as isize;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jeq(&mut self, n: usize) {
        if self.accumulator.value == 0{
            self.instuction_counter.value = n as isize;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn jne(&mut self, n: usize) {
        if self.accumulator.value != 0{
            self.instuction_counter.value = n as isize;
        }else{
            self.instuction_counter.add(1);
        }
    }

    fn end(&mut self) {
        self.accumulator.value = 0;
        self.instuction_counter.value = 1;
        for i in 0..self.data_registers.len(){
            self.data_registers[i].value = 0;
        }
    }

}
