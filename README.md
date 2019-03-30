elf_rs [![Build Status](https://travis-ci.com/vincenthouyi/elf_rs.svg?token=UBL21ZSzs6EH1xWep8q2&branch=master)](https://travis-ci.com/vincenthouyi/elf_rs)
===
This is a no_std library for ELF file handling.
It supports ELF32 and ELF64 format.

Usage
===
To read an elf file, supply `elf_rs::Elf` with a `&[u8]` memory:
```rust
extern crate elf_rs;

use std::io::Read;
use std::fs::File;
use std::env;

use elf_rs::*;

fn read_elf(filename: &String) {
    let mut elf_file = File::open(filename).unwrap();
    let mut elf_buf = Vec::<u8>::new();
    elf_file.read_to_end(&mut elf_buf).unwrap();

    let elf = Elf::from_bytes(&elf_buf).unwrap();

    if let Elf::Elf64(e) = elf {
        println!("{:?} header: {:?}", e, e.header());

        for p in e.program_header_iter() {
            println!("{:x?}", p);
        }

        for s in e.section_header_iter() {
            println!("{:x?}", s);
        }

        let s = e.lookup_section(b".text");
        println!("s {:?}", s);
    }
}
```
Under example directory there is a demo `readelf` to read an ELF file.
```
$ cargo run --example readelf <path_to_elf_file>
```


License
===
In order to support 996.ICU movement, this project is released under `Anti 996 License`, which require individual and enterprise users of this project strictly comply with local labor and employment laws and regulations.