mod micro;
use micro::{Processor, Instruction, Integer, RegisterInteger, Register, CoreRegister};

fn main() {
    let mut processor = Processor::new();
    
    // Test an add
    let add_instruction = Instruction::Add(
        RegisterInteger::Integer(Integer::new(10))
    );
    
    processor.run_instruction(&add_instruction);
    print!("{:?}", processor);
    
    // Test a mov
    let move_instruction = Instruction::Mov(
        RegisterInteger::Integer(Integer::new(100)),
        Register::CoreRegister(CoreRegister::Dat)
    );
    
    processor.run_instruction(&move_instruction);
    print!("{:?}", processor);
}
