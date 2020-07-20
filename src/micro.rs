use std::convert::From;

#[derive(Debug, Clone, Copy)]
pub struct Integer {
  value: i16
}

impl Integer { 
  pub fn new(value: i16) -> Self {
    Integer::from(value)
  }
  
  fn add(&self, other: Integer) -> Self {
    Integer::new(self.value + other.value)
  }
  
  fn subtract(&self, other: Integer) -> Self {
    Integer::new(self.value - other.value)  
  }
  
  fn multiply(&self, other: Integer) -> Self {
    Integer::new(self.value * other.value)  
  }
  
  fn dgt(&self, other: Integer) -> Self {
      let mut value = self.value;
      for _ in 0..other.value { value /= 10; }

      Integer::new(value % 10)
  }

  fn dst(&self, _digit_index: Integer, _value: Integer) -> Self {
      // TODO
      // First, wipe the digit
      // Next, shift the new digit over and add it in
      
      Integer::new(0)
  }      
}

impl std::convert::From<i16> for Integer {
  fn from(value: i16) -> Self {
    match value {
      i16::MIN ..= -999 => Integer { value: -999 },
      999 ..= i16::MAX => Integer { value: 999 },
      -998 ..= 998 => Integer { value }
    }
  }
}

#[derive(Clone)]
pub enum PinRegister {
  P0,
  P1,
  X0,
  X1
}

pub enum CoreRegister {
  Acc,
  Dat,
  Null
}

pub struct Label <'a> {
  value: &'a str
}

pub enum Register {
  PinRegister(PinRegister),
  CoreRegister(CoreRegister)
}

pub enum RegisterInteger {
  Register(Register),
  Integer(Integer)
}

pub enum Instruction<'a> {
  // Basic Instructions
  Nop,
  Mov(RegisterInteger, Register),
  Jmp(Label<'a>),
  Slp(RegisterInteger),
  Slx(PinRegister),

  // Arithmetic Instructions
  Add(RegisterInteger),
  Sub(RegisterInteger),
  Mul(RegisterInteger),
  Not,
  Dgt(RegisterInteger),
  Dst(RegisterInteger, RegisterInteger),

  // Test Instructions
  Teq(RegisterInteger, RegisterInteger),
  Tgt(RegisterInteger, RegisterInteger),
  Tlt(RegisterInteger, RegisterInteger),
  Tcp(RegisterInteger, RegisterInteger)
}

enum Conditional {
  Regular,
  Plus,
  Minus
}

struct InstructionWithContext<'a, 'b> {
  label: &'a str,
  conditional: Conditional,
  instruction: Instruction<'b>
}

pub enum BlockedState {
  NotBlocked,
  BlockedOnInput(PinRegister),
  BlockedOnTime(Integer)
}

#[derive(Debug)]
pub struct Processor {
  program_counter: usize,
  acc: Integer,
  dat: Integer,
  plus_operations_allowed: bool,
  minus_operations_allowed: bool
}

impl Processor {
  pub fn new() -> Self {
      Processor {
          program_counter: 0,
          acc: Integer::new(0),
          dat: Integer::new(0),
          plus_operations_allowed: true,
          minus_operations_allowed: true
      }
  }

  fn read_core_register(&self, register: &CoreRegister) -> Integer {
      match register {
          CoreRegister::Acc => self.acc,
          CoreRegister::Dat => self.dat,
          CoreRegister::Null => Integer::new(0)
      }
  }

  fn write_core_register(&mut self, register: &CoreRegister, value: Integer) {
      match register {
          CoreRegister::Acc => self.acc = value,
          CoreRegister::Dat => self.dat = value,
          _ => {}
      }
  }

  fn read_register(&self, register: &Register) -> Integer {
      match register {
          Register::CoreRegister(c) => self.read_core_register(c),
          Register::PinRegister(_p) => Integer::new(0) // TODO
      }
  }

  fn write_register(&mut self, register: &Register, value: Integer) {
      match register {
          Register::CoreRegister(c) => self.write_core_register(c, value),
          Register::PinRegister(_p) => () // TODO
      }
  }

  fn read_register_integer(&self, register: &RegisterInteger) -> Integer {
      match register {
          RegisterInteger::Integer(i) => *i,
          RegisterInteger::Register(r) => self.read_register(r)
      }
  }

  fn run_until_blocked(&mut self, program: &Vec<InstructionWithContext>) {
    loop {
      // Fetch the next instruction and increment the PC, possibly wrapping
      let instruction = program.get(self.program_counter).unwrap();
      self.program_counter += 1;
      
      if self.program_counter == program.len() {
        self.program_counter = 0;
      }

      // Return early if the operation is conditional and disallowed
      match instruction.conditional {
          Conditional::Plus => if !self.plus_operations_allowed { continue },
          Conditional::Minus => if !self.minus_operations_allowed { continue },
          _ => {}
      };

      // Run the instruction
      match self.run_instruction(&instruction.instruction) {
        BlockedState::NotBlocked => continue,
        BlockedState::BlockedOnTime(t) => return,
        BlockedState::BlockedOnInput(t) => return
      }
    };
  }

  pub fn run_instruction(&mut self, instruction: &Instruction) -> BlockedState {
      match instruction {
          Instruction::Nop => BlockedState::NotBlocked,

          Instruction::Mov(value, destination) => {
              let integer_value = self.read_register_integer(value);
              self.write_register(destination, integer_value);

              BlockedState::NotBlocked
          },

          Instruction::Jmp(label) => {
              // TODO: Pass in an index map
              BlockedState::NotBlocked
          },


          Instruction::Slp(value) => {
              let time = self.read_register_integer(value);
              BlockedState::BlockedOnTime(time)
          },

          Instruction::Slx(pin) => BlockedState::BlockedOnInput(pin.clone()),

          // Arithmetic Instructions
          Instruction::Add(value) => {
              let integer_value = self.read_register_integer(value);
              self.acc = self.acc.add(integer_value);
              
              BlockedState::NotBlocked
          },

          Instruction::Sub(value) => {
              let integer_value = self.read_register_integer(value);
              self.acc = self.acc.subtract(integer_value);

              BlockedState::NotBlocked
          },

          Instruction::Mul(value) => {
              let integer_value = self.read_register_integer(value);
              self.acc = self.acc.multiply(integer_value);

              BlockedState::NotBlocked
          },

          Instruction::Not => {
              self.acc = match self.acc.value {
                  0 => Integer::new(100),
                  _ => Integer::new(0)
              };

              BlockedState::NotBlocked
          },

          Instruction::Dgt(value) => {
              let integer_value = self.read_register_integer(value);
              self.acc = self.acc.dgt(integer_value);

              BlockedState::NotBlocked
          },

          Instruction::Dst(digit_index, value) => {
              let digit_index_integer = self.read_register_integer(digit_index);
              let value_integer = self.read_register_integer(value);
              self.acc = self.acc.dst(digit_index_integer, value_integer);

              BlockedState::NotBlocked
          },

          // Test Instructions
          Instruction::Teq(a, b) => {
              let a_val = self.read_register_integer(a);
              let b_val = self.read_register_integer(b);

              self.plus_operations_allowed = a_val.value == b_val.value;
              self.minus_operations_allowed = a_val.value != b_val.value;

              BlockedState::NotBlocked
          },

          Instruction::Tgt(a, b) => {
              let a_val = self.read_register_integer(a);
              let b_val = self.read_register_integer(b);

              self.plus_operations_allowed = a_val.value > b_val.value;
              self.minus_operations_allowed = a_val.value <= b_val.value;

              BlockedState::NotBlocked
          },

          Instruction::Tlt(a, b) => {
              let a_val = self.read_register_integer(a);
              let b_val = self.read_register_integer(b);

              self.plus_operations_allowed = a_val.value < b_val.value;
              self.minus_operations_allowed = a_val.value >= b_val.value;

              BlockedState::NotBlocked
          },

          Instruction::Tcp(a, b) => {
              let a_val = self.read_register_integer(a);
              let b_val = self.read_register_integer(b);

              self.plus_operations_allowed = a_val.value > b_val.value;
              self.minus_operations_allowed = b_val.value > a_val.value;

              BlockedState::NotBlocked
          }
      }
  }
}


