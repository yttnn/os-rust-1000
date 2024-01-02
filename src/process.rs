use core::{cell::Cell, arch::asm};

use crate::{println, print::putchar, switch::{self, switch_context}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProcessState {
  Unused,
  Runnable,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProcessError {
  FailedToCreateProcess,
} 

const PROCESS_MAX: usize = 8;

#[derive(Debug)]
pub struct ProcessManager {
  procs: [Process; PROCESS_MAX],
}

impl ProcessManager {
  pub fn new() -> Self {
    Self {
      // !?
      procs: [
        Process::new(0),
        Process::new(1),
        Process::new(2),
        Process::new(3),
        Process::new(4),
        Process::new(5),
        Process::new(6),
        Process::new(7),
      ],
    }
  }

  pub fn run(&self) {
  }

  pub fn create_process(&self, pc: *const ()) -> Result<&Process, ProcessError> {
    let process = self.find_unused_process();
    if process.is_none() { return Err(ProcessError::FailedToCreateProcess); }
    let process: &Process = process.unwrap();

    process.context.set(Context::new(pc as u32));
    process.state.set(ProcessState::Runnable);
    process.sp.set(0);
    
    Ok(process)
  }

  fn find_unused_process(&self) -> Option<&Process> {
    for process in &self.procs {
      if process.state.get() == ProcessState::Unused {
        return Some(&process);
      }
    }

    None
  }

  pub fn yield_process() {

  }

  unsafe fn proc_a_entry(&self) {
    println!("starting process A");
    loop {
      putchar('A');
      // switch_context(self.proc_a.context.get_mut(), self.proc_b.context.get_mut());

      for i in 0..30000000 {
        asm!("nop");
      }
    }
  }

  unsafe fn proc_b_entry(&self) {
    println!("starting process B");
    loop {
      putchar('B');
      // switch_context(self.proc_b.context.get_mut(), self.proc_a.context.get_mut());

      for i in 0..30000000 {
        asm!("nop");
      }
    }
  }


}

#[derive(Debug)]
pub struct Process {
  pid: u32,
  state: Cell<ProcessState>,
  sp: Cell<u32>,
  context: Cell<Context>,
}

impl Process {
  pub fn new(pid: u32) -> Self {
    Self {
      pid: pid,
      state: ProcessState::Unused.into(),
      sp: 0.into(),
      context: Context::new(0).into(),
    }
  }
}

#[derive(Debug,Clone, Copy)]
#[repr(C, packed)]
pub struct Context {
  pub ra: u32,
  pub sp: u32,
  pub s0: u32,
  pub s1: u32,
  pub s2: u32,
  pub s3: u32,
  pub s4: u32,
  pub s5: u32,
  pub s6: u32,
  pub s7: u32,
  pub s8: u32,
  pub s9: u32,
  pub s10: u32,
  pub s11: u32,
}

impl Context {
  pub fn new(ra: u32) -> Self {
    Self {
      ra: ra,
      sp: 0,
      s0: 0,
      s1: 0,
      s2: 0,
      s3: 0,
      s4: 0,
      s5: 0,
      s6: 0,
      s7: 0,
      s8: 0,
      s9: 0,
      s10: 0,
      s11: 0,
    }
  }
}