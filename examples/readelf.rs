extern crate elf_rs;

use std::io::Read;
use std::fs::File;
use std::env;

use elf_rs::*;

fn read_elf(filename: &String) -> Result<(),()> {

    let mut elf_file = File::open(filename)
                            .map_err(|e| {
                                println!("failed to open file {}: {}", filename, e);
                                ()
                             })?;
    let mut elf_buf = Vec::<u8>::new();

    elf_file.read_to_end(&mut elf_buf)
                        .map_err(|e| {
                            println!("failed to read file {}: {}", filename, e);
                            ()
                         })?;

    let elf = Elf::from_bytes(&mut elf_buf)
                  .map_err(|e| {
                      println!("failed to extract elf file {}: {:?}", filename, e);
                      ()
                   })?;

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

    } else if let Elf::Elf32(e) = elf {
        println!("{:?} header: {:?}", e, e.header());

        for p in e.program_header_iter() {
            println!("{:x?}", p);
        }

        for s in e.section_header_iter() {
            println!("{:x?}", s);
        }
    }

    Ok(())
}

fn main() -> Result<(), ()>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Need specify file path!");
        return Err(());
    }

    let filename = &args[1];
    read_elf(&filename)?;

    Ok(())
}
