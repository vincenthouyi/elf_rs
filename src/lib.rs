#![no_std]

#[macro_use] extern crate enum_primitive_derive;
#[macro_use] extern crate bitflags;
extern crate num_traits;

use num_traits::FromPrimitive;
use core::fmt;

mod elf32;
mod elf64;
mod traits;

pub use elf32::*;
pub use elf64::*;
pub use traits::*;

#[allow(unused_variables)]
#[derive(Copy, Clone, Debug)]
pub enum Error {
    BufferTooShort,
    InvalidMagic,
    InvalidClass,
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum ElfAbi {
    SystemV     = 0x00,
    HPUX        = 0x01,
    NetBSD      = 0x02,
    Linux       = 0x03,
    Hurd        = 0x04,
    Solaris     = 0x06,
    AIX         = 0x07,
    IRIX        = 0x08,
    FreeBSD     = 0x09,
    Tru64       = 0x0A,
    NovellModesto = 0x0B,
    OpenBSD     = 0x0C,
    OpenVMS     = 0x0D,
    NonStopKernel = 0x0E,
    AROS        = 0x0F,
    FenixOS     = 0x10,
    CloudABI    = 0x11
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum SectionType {
    SHT_NULL            = 0x0,
    SHT_PROGBITS        = 0x1,
    SHT_SYMTAB          = 0x2,
    SHT_STRTAB          = 0x3,
    SHT_RELA            = 0x4,
    SHT_HASH            = 0x5,
    SHT_DYNAMIC         = 0x6,
    SHT_NOTE            = 0x7,
    SHT_NOBITS          = 0x8,
    SHT_REL             = 0x9,
    SHT_SHLIB           = 0x0A,
    SHT_DYNSYM          = 0x0B,
    SHT_INIT_ARRAY      = 0x0E,
    SHT_FINI_ARRAY      = 0x0F,
    SHT_PREINIT_ARRAY   = 0x10,
    SHT_GROUP           = 0x11,
    SHT_SYMTAB_SHNDX    = 0x12,
    SHT_NUM             = 0x13,
    SHT_LOOS            = 0x60000000,
}
#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum ElfType {
    ET_NONE = 0x00,
    ET_REL = 0x01,
    ET_EXEC = 0x02,
    ET_DYN = 0x03,
    ET_CORE = 0x04,
    ET_LOOS = 0xfe00,
    ET_HIOS = 0xfeff,
    ET_LOPROC = 0xff00,
    ET_HIPROC = 0xffff
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum ElfMachine {
    Unknown     =0x00,
    SPARC       =0x02,
    x86         =0x03,
    MIPS        =0x08,
    PowerPC     =0x14,
    S390        =0x16,
    ARM         =0x28,
    SuperH      =0x2A,
    IA_64       =0x32,
    x86_64      =0x3E,
    AArch64     =0xB7,
    RISC_V      =0xF3,
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum ElfClass {
    Elf32 = 1,
    Elf64 = 2
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum ElfEndian {
    LittleEndian = 1,
    BigEndian = 2,
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, Primitive)]
pub enum ProgramType {
    NULL     = 0x00000000,
    LOAD     = 0x00000001,
    DYNAMIC  = 0x00000002,
    INTERP   = 0x00000003,
    NOTE     = 0x00000004,
    SHLIB    = 0x00000005,
    PHDR     = 0x00000006,
    LOOS     = 0x60000000,
    HIOS     = 0x6FFFFFFF,
    LOPROC   = 0x70000000,
    HIPROC   = 0x7FFFFFFF,
    GNU_STACK = 0x6474e551,
}

pub struct ProgramHeader<'a, E: GenElf> {
    elf: &'a E,
    pub ph: &'a E::ProgramHeaderType
}

impl <'a, E:GenElf> ProgramHeader<'a, E> {
    pub fn segment(&self) -> &[u8] {
        let seg_off = self.ph.offset().into() as usize;
        let seg_filesz = self.ph.filesz().into() as usize;
        &self.elf.as_bytes()[seg_off .. seg_off+seg_filesz]
    }
}

impl<'a, E: GenElf> fmt::Debug for ProgramHeader<'a, E>{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Program Header")
         .field("type", &self.ph.ph_type())
         .field("flags",&self.ph.flags())
         .field("offset", &self.ph.offset())
         .field("vaddr", &self.ph.vaddr())
         .field("paddr", &self.ph.paddr())
         .field("filesize", &self.ph.filesz())
         .field("memsize", &self.ph.memsz())
         .field("alignment", &self.ph.align())
         .finish()
    }
}

#[derive(Debug)]
pub struct ProgramHeaderIter<'a, E: GenElf> {
    elf: &'a E,
    ph: &'a[E::ProgramHeaderType],
    idx: u16,
}

impl <'a, E:GenElf> core::iter::Iterator for ProgramHeaderIter<'a, E> {
    type Item = ProgramHeader<'a, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(ProgramHeader {
            elf: self.elf,
            ph: self.ph.get(self.idx as usize)?
        });
        self.idx += 1;
        ret
    }
}

bitflags! {
    pub struct SectionHeaderFlags: u64 {
        const SHF_WRITE             = 0x1;
        const SHF_ALLOC             = 0x2;
        const SHF_EXECINSTR         = 0x4;
        const SHF_MERGE             = 0x10;
        const SHF_STRINGS           = 0x20;
        const SHF_INFO_LINK         = 0x40;
        const SHF_LINK_ORDER        = 0x80;
        const SHF_OS_NONCONFORMING  = 0x100;
        const SHF_GROUP             = 0x200;
        const SHF_TLS	            = 0x400;
        const SHF_MASKOS            = 0x0ff00000;
        const SHF_MASKPROC          = 0xf0000000;
        const SHF_ORDERED           = 0x40000000;
        const SHF_EXCLUDE           = 0x80000000;
    }
}

pub struct SectionHeader<'a, E: GenElf> {
    elf: &'a E,
    pub sh: &'a E::SectionHeaderType
}

impl<'a, E: GenElf> SectionHeader<'a, E> {
    pub fn segment(&'a self) -> &'a [u8] {
        let seg_off = self.sh.offset().into() as usize;
        let seg_filesz = self.sh.size().into() as usize;
        &self.elf.as_bytes()[seg_off .. seg_off+seg_filesz]
    }

    pub fn section_name(&'a self) -> CChar {
        let name_off = self.sh.name_off() as usize;
        let shstr = self.elf.shstr_section();
        let name_len = shstr[name_off..].iter()
                                        .position(|&x| x == b'\0')
                                        .unwrap();
        CChar(&shstr[name_off .. name_off + name_len])
    }
}

impl<'a, E: GenElf> fmt::Debug for SectionHeader<'a, E>{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Section Header")
         .field("name", &self.section_name())
         .field("type", &self.sh.sh_type())
         .field("flags",&self.sh.flags())
         .field("addr", &self.sh.addr())
         .field("offset", &self.sh.offset())
         .field("size", &self.sh.size())
         .field("link", &self.sh.link())
         .field("info", &self.sh.info())
         .field("address alignment", &self.sh.addralign())
         .field("entry size", &self.sh.entsize())
         .finish()
    }
}

#[derive(Debug)]
pub struct SectionHeaderIter<'a, E: GenElf> {
    elf: &'a E,
    sh: &'a[E::SectionHeaderType],
    idx: u16,
}

impl <'a, E:GenElf> core::iter::Iterator for SectionHeaderIter<'a, E> {
    type Item = SectionHeader<'a, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(SectionHeader {
            elf: self.elf,
            sh: self.sh.get(self.idx as usize)?
        });
        self.idx += 1;
        ret
    }
}

#[derive(Debug)]
pub enum Elf<'a> {
    Elf32(Elf32<'a>),
    Elf64(Elf64<'a>)
}

impl<'a> Elf<'a> {
    pub fn from_bytes(elf_buf: &'a [u8]) -> Result<Self, Error> {
        if elf_buf.len() < 0x14 {
            return Err(Error::BufferTooShort);
        }

        if !elf_buf.starts_with(&[0x7f, 0x45, 0x4C, 0x46]) {
            return Err(Error::InvalidMagic);
        }

        let tmp_elf = Elf32::new(elf_buf);
        match ElfClass::from_u8(tmp_elf.header().class) {
            Some(ElfClass::Elf64) => { 
                let elf = Elf64::new(elf_buf);
                if elf_buf.len() < elf.header().elf_header_size() as usize {
                    Err(Error::BufferTooShort)
                } else {
                    Ok(Elf::Elf64(elf)) }
                },
            Some(ElfClass::Elf32) => { 
                let elf = Elf32::new(elf_buf);
                if elf_buf.len() < elf.header().elf_header_size() as usize {
                    Err(Error::BufferTooShort)
                } else {
                    Ok(Elf::Elf32(elf)) }
                },
            None => { Err(Error::InvalidClass) }
        }
    }
}

pub struct CChar<'a>(&'a [u8]);

impl<'a> fmt::Debug for CChar<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use core::fmt::Write;
        for c in self.0.iter() {
            f.write_char(*c as char)?;
        }

        Ok(())
    }
}