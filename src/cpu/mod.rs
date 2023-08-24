#[derive(Debug)]
pub struct Cpu {
  /** Accumulator */
  acc: u8,
  /** Program counter */
  pc: u16,
  /** Stack pointer */
  sp: u8,

  /**
   * X: Index register X
   *
   * The X register has one special function. It can be used to get a copy of the stack pointer or change its value.
   */
  x: u8,
  /** Y: Index register Y. Has no special function. */
  y: u8,

  memory: [u8; 0xFFFF],

  /** C: Set when an unsigned addition or subtraction results in an overflow */
  carry: bool,
  /** Z: Set when an operation results in a zero */
  zero: bool,
  /** I: When set, interrupt requests are ignored */
  interrupt_mask: bool,
  /** D: When set, certain instructions operate in decimal rather than binary mode */
  decimal_mode: bool,
  /** B: Set when a BRK instruction is executed */
  break_: bool,
  /** V: Set when a signed addition or subtraction results in an overflow */
  overflow: bool,
  /** N: Set when an operation results in a negative number */
  negative: bool,
}

impl Cpu {
  pub fn new() -> Self {
    Self {acc: 0x0, pc: 0x0, sp: 0x0, x: 0x0, y: 0x0, memory: [0; 0xFFFF], carry: false, zero: false, interrupt_mask: false, decimal_mode: false, break_: false, overflow: false, negative: false}
  }

  pub fn load_program(&mut self, program: &[u8]) {
    for (i, byte) in program.iter().enumerate() {
      self.memory[i] = *byte;
    }
  }

  pub fn pop_pc(&mut self) -> u8 {
    let result = self.memory[self.pc as usize];
    self.pc = (self.pc + 1) % self.memory.len() as u16;
    result
  }

  pub fn x(&self) -> u8 {
    self.x
  }

  pub fn acc(&self) -> u8 {
    self.acc
  }

  /**
   * Returns the number of cycles used by the instruction pointed to by
   * the program counter.
   */
  pub fn execute(&mut self) -> usize {
    let opcode = self.pop_pc();
    match opcode {
      // LDA Immediate
      0x9a => {
        self.acc = self.pop_pc();
        2
      },

      // LDX Immediate
      0xa2 => {
        self.x = self.pop_pc();
        2
      },

      _ => panic!("Unknown opcode: {:x}", opcode),
    }
  }
}