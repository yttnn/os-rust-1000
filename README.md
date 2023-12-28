# os-rust-1000
- https://operating-system-in-1000-lines.vercel.app/ja/welcome
- これをRustでやるリポジトリ
- とりあえず完走が目標
- scrapbox: https://scrapbox.io/yttnn/Writing_an_OS_in_1000_lines%E3%82%92Rust%E3%81%A7%E3%82%84%E3%82%8B

## build and run
- build: `cargo build`
- run: `run.sh`

## environment
- `rustc 1.73.0 (cc66ad468 2023-10-03)`
- `QEMU emulator version 8.1.2`

## setup
- Rust
- qemu
  - 私は`~/tools`の下に展開した
  - 必要に応じて`run.sh`を変更する必要あり
- qemu-bios
  - qemuのバージョンに対応したbiosを`run.sh`と同じ階層に
  - 今回は[これ](https://github.com/qemu/qemu/blob/v8.1.2/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin)