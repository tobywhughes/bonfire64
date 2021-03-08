#[derive(Default, Debug)]
pub struct GeneralPurposeRegisters {
  //Special Purpose
  program_counter: u64,
  hi_lo: u64,
  llb: u64,
  //Misc.
  assembler_temporary: u64,
  return_address: u64,
  //Subroutine return value
  value0: u64,
  value1: u64,
  //Arguments
  argument0: u64,
  argument1: u64,
  argument2: u64,
  argument3: u64,
  //Temporary Registers
  temporary0: u64,
  temporary1: u64,
  temporary2: u64,
  temporary3: u64,
  temporary4: u64,
  temporary5: u64,
  temporary6: u64,
  temporary7: u64,
  temporary8: u64,
  temporary9: u64,
  //Saved Registers
  saved0: u64,
  saved1: u64,
  saved2: u64,
  saved3: u64,
  saved4: u64,
  saved5: u64,
  saved6: u64,
  saved7: u64,
  //Kernel Registers
  kernel0: u64,
  kernel1: u64,
  //Pointers
  global_pointer: u64,
  stack_pointer: u64,
  frame_pointer: u64,
}

impl GeneralPurposeRegisters {
  pub fn simulate_boot(&mut self) {
    self.temporary3 = 0xFFFF_FFFF_A400_0040;
    self.saved4 = 0x0000_0000_0000_0001;
    self.saved6 = 0x0000_0000_0000_003F;
    self.stack_pointer = 0xFFFF_FFFF_A400_1FF0;

    self.program_counter = 0x0000_0000_A400_0040;
  }

  pub fn get(&self, index: u8) -> u64 {
    match index {
      0 => 0,
      1 => self.assembler_temporary,
      2 => self.value0,
      3 => self.value1,
      4 => self.argument0,
      5 => self.argument1,
      6 => self.argument2,
      7 => self.argument3,
      8 => self.temporary0,
      9 => self.temporary1,
      10 => self.temporary2,
      11 => self.temporary3,
      12 => self.temporary4,
      13 => self.temporary5,
      14 => self.temporary6,
      15 => self.temporary7,
      16 => self.saved0,
      17 => self.saved1,
      18 => self.saved2,
      19 => self.saved3,
      20 => self.saved4,
      21 => self.saved5,
      22 => self.saved6,
      23 => self.saved7,
      24 => self.temporary8,
      25 => self.temporary9,
      26 => self.kernel0,
      27 => self.kernel1,
      28 => self.global_pointer,
      29 => self.stack_pointer,
      30 => self.frame_pointer,
      31 => self.return_address,
      _ => 0,
    }
  }

  pub fn set(&mut self, index: u8, value: u64) -> () {
    match index {
      1 => self.assembler_temporary = value,
      2 => self.value0 = value,
      3 => self.value1 = value,
      4 => self.argument0 = value,
      5 => self.argument1 = value,
      6 => self.argument2 = value,
      7 => self.argument3 = value,
      8 => self.temporary0 = value,
      9 => self.temporary1 = value,
      10 => self.temporary2 = value,
      11 => self.temporary3 = value,
      12 => self.temporary4 = value,
      13 => self.temporary5 = value,
      14 => self.temporary6 = value,
      15 => self.temporary7 = value,
      16 => self.saved0 = value,
      17 => self.saved1 = value,
      18 => self.saved2 = value,
      19 => self.saved3 = value,
      20 => self.saved4 = value,
      21 => self.saved5 = value,
      22 => self.saved6 = value,
      23 => self.saved7 = value,
      24 => self.temporary8 = value,
      25 => self.temporary9 = value,
      26 => self.kernel0 = value,
      27 => self.kernel1 = value,
      28 => self.global_pointer = value,
      29 => self.stack_pointer = value,
      30 => self.frame_pointer = value,
      31 => self.return_address = value,
      _ => (),
    };
  }
}
