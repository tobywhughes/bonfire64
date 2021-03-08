#[derive(Default, Debug)]
pub struct Coprocessor0Registers {
  index: u32,
  random: u32,
  entry_lo0: u32,
  entry_lo1: u32,
  context: u32,
  page_mask: u32,
  wired: u32,
  bad_virtual_address: u32,
  count: u32,
  entry_hi: u32,
  compare: u32,
  status: u32,
  cause: u32,
  exception_program_counter: u32,
  processor_id: u32,
  config: u32,
  load_linked_address: u32,
  watch_lo: u32,
  watch_hi: u32,
  x_context: u32,
  cache_error: u32,
  tag_lo: u32,
  tag_hi: u32,
  error_exception_program_counter: u32,
}

impl Coprocessor0Registers {
  pub fn simulate_boot(&mut self) {
    self.random = 0x0000_001f;
    self.status = 0x7040_0004;
    self.processor_id = 0x0000_0B000;
    self.config = 0x0006_E463;
  }

  pub fn get(&self, index: u8) -> u32 {
    match index {
      0 => self.index,
      1 => self.random,
      2 => self.entry_lo0,
      3 => self.entry_lo1,
      4 => self.context,
      5 => self.page_mask,
      6 => self.wired,
      8 => self.bad_virtual_address,
      9 => self.count,
      10 => self.entry_hi,
      11 => self.compare,
      12 => self.status,
      13 => self.cause,
      14 => self.exception_program_counter,
      15 => self.processor_id,
      16 => self.config,
      17 => self.load_linked_address,
      18 => self.watch_lo,
      19 => self.watch_hi,
      20 => self.x_context,
      27 => self.cache_error,
      28 => self.tag_lo,
      29 => self.tag_hi,
      30 => self.error_exception_program_counter,
      _ => 0,
    }
  }

  pub fn set(&mut self, index: u8, value: u32) -> () {
    match index {
      0 => self.index = value,
      1 => self.random = value,
      2 => self.entry_lo0 = value,
      3 => self.entry_lo1 = value,
      4 => self.context = value,
      5 => self.page_mask = value,
      6 => self.wired = value,
      8 => self.bad_virtual_address = value,
      9 => self.count = value,
      10 => self.entry_hi = value,
      11 => self.compare = value,
      12 => self.status = value,
      13 => self.cause = value,
      14 => self.exception_program_counter = value,
      15 => self.processor_id = value,
      16 => self.config = value,
      17 => self.load_linked_address = value,
      18 => self.watch_lo = value,
      19 => self.watch_hi = value,
      20 => self.x_context = value,
      27 => self.cache_error = value,
      28 => self.tag_lo = value,
      29 => self.tag_hi = value,
      30 => self.error_exception_program_counter = value,
      _ => (),
    };
  }
}
