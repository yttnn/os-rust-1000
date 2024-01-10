use alloc::boxed::Box;

const PAGE_SIZE: usize = 4096; // byte
const N_PAGE_ENTRY: usize = PAGE_SIZE / 4;

const PAGE_V: u32 = 1 << 0; // 有効化ビット
const PAGE_R: u32 = 1 << 1; // 読み込み可能
const PAGE_W: u32 = 1 << 2; // 書き込み可能
const PAGE_X: u32 = 1 << 3; // 実行可能
const PAGE_U: u32 = 1 << 4; // ユーザーモードでアクセス可能

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

pub fn alloc_pages(n: u32) -> u32 {
  0
}

pub fn map_page(table1: u32, vaddr: u32, paddr: u32, flags: u32) {
  let vnp1 = (vaddr >> 22) & 0x3ff;

  let vpn0 = (vaddr >> 12) & 0x3ff;

}

pub struct PageTableEntry {
  entry: u32,   // 31~20bit: ppn1, 19~10bit: ppn0, 9~0bit: flags
}

impl PageTableEntry {

}

pub struct PageTable {
  entries: [PageTableEntry; N_PAGE_ENTRY],
}

impl PageTable {
  fn map_page(&mut self, vaddr: u32, paddr: u32, flags: u32) {
    let vpn1 = (vaddr >> 22) & 0x3ff; // virtual page number
    if (self.entries[vpn1 as usize].entry & PAGE_V) == 0 { // 2段目がない場合、設定
      let memory = Box::new(Page::new());
      let pt1_paddr = Box::into_raw(memory);
      let p_page_number = pt1_paddr as u32 / PAGE_SIZE as u32;
      self.entries[vpn1 as usize].entry = (p_page_number << 10) | PAGE_V;
    }

    let vpn0 = (vaddr >> 12) & 0x3ff; // virtual page number
    let pt0_paddr = (self.entries[vpn1 as usize].entry >> 10) * (PAGE_SIZE as u32); // 2段目ページテーブルのアドレス(page_number = paddr/PAGE_SIZE)
    self.entries[vpn0 as usize].entry = ()
  }
}