use core:: ptr;

use crate::{switch::switch_context, println};

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

static mut PROCESS_POOL: [Process; PROCESS_MAX] = [Process::new(0); PROCESS_MAX];
static mut IDLE_PROCESS: Process = Process::new(0);
static mut CURRENT_PROCESS : *mut Process = ptr::null_mut();

pub unsafe fn init() {
  CURRENT_PROCESS = &mut IDLE_PROCESS as *mut _;
  for i in 0..PROCESS_MAX {
    PROCESS_POOL[i].pid = (i + 1) as u32;
  }
}

pub unsafe fn create_process(pc :u32) -> Result<(), ProcessError> {
  let process = find_unused_process();
  if process.is_none() { return Err(ProcessError::FailedToCreateProcess); }
  let process = process.unwrap();

  process.context.ra = pc;
  let sp = ptr::addr_of!(process.stack);
  process.context.sp = sp as u32;
  process.state = ProcessState::Runnable;
  Ok(())
}

unsafe fn find_unused_process() -> Option<&'static mut Process> {
  for process in &mut PROCESS_POOL {
    if process.state == ProcessState::Unused {
      return Some(process);
    }
  }
  None
}

pub unsafe fn yield_process() {
  let next = find_next_process();
  if next.is_none() { println!("next is none"); return; }
  let next = next.unwrap() as *mut Process;
  // println!("1 {:?}", *next);
  let prev = CURRENT_PROCESS;
  // println!("2 {:?}", *next);
  CURRENT_PROCESS = next;
  // println!("3 {:?}", *next);
  println!("yield {}", (*next).pid);
  println!("yield [0]state {:?}", (*CURRENT_PROCESS).state);

  unsafe {
    // println!("cur: {:?}", *prev);
    // println!("next: {:?}", *next);
    switch_context(&mut (*prev).context, &(*next).context);
  }
}

unsafe fn find_next_process() -> Option<&'static mut Process> {
  // for process in &mut PROCESS_POOL {
  //   println!("pid {}",process.pid);
  //   if (*CURRENT_PROCESS).pid == process.pid { continue; }
  //   if process.state == ProcessState::Runnable && process.pid > 0 {
  //     return Some(process);
  //   }
  // }
  let current_pid = (*CURRENT_PROCESS).pid as usize;
  for i in 0..PROCESS_MAX {
    let process = &mut PROCESS_POOL[(current_pid + i) % PROCESS_MAX];
    println!("pid {}, state {:?}", process.pid, process.state);
    if process.state == ProcessState::Runnable && process.pid > 0 {
      println!("ret some pid {}, state {:?}", process.pid, process.state);
      return Some(process);
    }
  }
  None
}

// impl ProcessManager {
//   pub fn new() -> Self {
//     Self {
//       // !?
//       procs: [
//         Process::new(0),
//         Process::new(1),
//         Process::new(2),
//         Process::new(3),
//         Process::new(4),
//         Process::new(5),
//         Process::new(6),
//         Process::new(7),
//       ],
//       idle_proc: Process::new(-1),
//       current_proc: ptr::null_mut(),
//     }
//   }

//   pub unsafe fn run(&mut self) {
//     CURRENT_PROCESS = &mut self.idle_proc as *mut _;
    
//   }

//   pub fn create_process(&mut self, pc: unsafe fn()) -> Result<&Process, ProcessError> {
//     let process = self.find_unused_process();
//     if process.is_none() { return Err(ProcessError::FailedToCreateProcess); }
//     let process = process.unwrap();

//     // process.context.set(Context::new(pc as u32));
//     process.context.ra = pc as u32;
//     // process.state.set(ProcessState::Runnable);
//     process.state = ProcessState::Runnable;
//     // process.sp.set(0);
//     // process.sp = 0;
    
//     Ok(process)
//   }

//   fn find_unused_process(&mut self) -> Option<&mut Process> {
//     for process in &mut self.procs {
//       if process.state == ProcessState::Unused {
//         return Some(process);
//       }
//     }

//     None
//   }

//   pub unsafe fn yield_process(&mut self) {
//     let mut next = self.find_runnable_process();
//     if next.is_none() { return; }
    
//     let mut next = next.unwrap() as *mut Process;
//     let prev = self.current_proc;
//     self.current_proc = next;
//     unsafe {
//       switch_context(&mut (*prev).context, &(*next).context);
//     }
//   }

//   fn find_runnable_process(&mut self) -> Option<&mut Process> {
//     for process in &mut self.procs {
//       if process.state == ProcessState::Runnable {
//         return Some(process);
//       }
//     }

//     None
//   }

  // unsafe fn proc_a_entry(&mut self) {
  //   println!("starting process A");
  //   loop {
  //     putchar('A');
  //     // switch_context(self.proc_a.context.get_mut(), self.proc_b.context.get_mut());
  //     self.yield_process();

  //     for i in 0..30000000 {
  //       asm!("nop");
  //     }
  //   }
  // }

  // unsafe fn proc_b_entry(&mut self) {
  //   println!("starting process B");
  //   loop {
  //     putchar('B');
  //     // switch_context(self.proc_b.context.get_mut(), self.proc_a.context.get_mut());
  //     self.yield_process();

  //     for i in 0..30000000 {
  //       asm!("nop");
  //     }
  //   }
  // }
// }

#[derive(Debug, Clone, Copy)]
pub struct Process {
  pid: u32,
  state: ProcessState,
  sp: u32,
  context: Context,
  stack: [u8; 8192],
}

impl Process {
  pub const fn new(pid: u32) -> Self {
    Self {
      pid: pid,
      state: ProcessState::Unused,
      sp: 0,
      context: Context::new(),
      stack: [0; 8192],
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
  pub const fn new() -> Self {
    Self {
      ra: 0,
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