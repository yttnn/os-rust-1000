use alloc::boxed::Box;

pub const PAGE_SIZE: usize = 4096; // byte
const N_PAGE_ENTRY: usize = PAGE_SIZE / 4;

pub const SATP_SV32: u32 = 1 << 31;
pub const PAGE_V: u32 = 1 << 0; // 有効化ビット
pub const PAGE_R: u32 = 1 << 1; // 読み込み可能
pub const PAGE_W: u32 = 1 << 2; // 書き込み可能
pub const PAGE_X: u32 = 1 << 3; // 実行可能
pub const PAGE_U: u32 = 1 << 4; // ユーザーモードでアクセス可能

#[repr(C, align(4096))]
pub struct Page {
  page: [u8; PAGE_SIZE],
}

impl Page {
  pub fn new() -> Self {
    Self {
      page: [0; PAGE_SIZE],
    }
  }
}

pub fn alloc_page() -> *mut Page {
  let memory = Box::new(Page::new());
  Box::into_raw(memory)
}


pub struct PageTableEntry {
  entry: u32,   // 31~20bit: ppn1, 19~10bit: ppn0, 9~0bit: flags
}

impl PageTableEntry {

}

pub struct Paddr(u32);

#[derive(Debug, Clone, Copy)]
pub struct PageTable {
  // entries: [PageTableEntry; N_PAGE_ENTRY],
  pub ptr: *mut u32,
}

impl PageTable {
  pub fn new(ptr: *mut u32) -> Self {
    Self {
      ptr: ptr,
    }
  }

  pub fn map_page(&mut self, vaddr: u32, paddr: u32, flags: u32) {
    // let vpn1 = (vaddr >> 22) & 0x3ff; // virtual page number
    // if (self.entries[vpn1 as usize].entry & PAGE_V) == 0 { // 2段目がない場合、設定
    //   let memory = Box::new(Page::new());
    //   let pt1_paddr = Box::into_raw(memory);
    //   let p_page_number = pt1_paddr as u32 / PAGE_SIZE as u32;
    //   self.entries[vpn1 as usize].entry = (p_page_number << 10) | PAGE_V;
    // }

    // let vpn0 = (vaddr >> 12) & 0x3ff; // virtual page number
    // let pt0_paddr = ((self.entries[vpn1 as usize].entry >> 10) * (PAGE_SIZE as u32)) as*mut u32; // 2段目ページテーブルの物理アドレス(page_number = paddr/PAGE_SIZE)
    // let p_page_number = paddr / PAGE_SIZE as u32;
    // unsafe {
    //   pt0_paddr.offset(vpn0 as isize).write((p_page_number << 10) | flags | PAGE_V);
    // }
   
    let mut table1 = self.ptr;
    let vpn1 = (vaddr >> 22) & 0x3ff;
    unsafe {
      if table1.offset(vpn1 as isize) as u32 & PAGE_V == 0 {
        // let memory = Box::new(Page::new());
        // let pt1_ptr = Box::into_raw(memory);
        let pt1_ptr = alloc_page();
        let p_page_number = pt1_ptr as u32 / PAGE_SIZE as u32;
        table1.offset(vpn1 as isize).write((p_page_number << 10) | PAGE_V);
      }
    }

    let vpn0 = (vaddr >> 12) & 0x3ff;
    let p_page_number = paddr / PAGE_SIZE as u32;
    unsafe {
      let page_number = *(table1.offset(vpn1 as isize));
      let table0 = ((page_number >> 10) * PAGE_SIZE as u32) as *mut u32; // 2段目ページテーブルの物理アドレス(page_number = paddr/PAGE_SIZE)
      table0.offset(vpn0 as isize).write((p_page_number << 10) | flags | PAGE_V);
    }

  }
}