#![no_std]

#[macro_use] extern crate bitflags;

use core::fmt;

mod elf32;
mod elf64;
mod traits;

pub use elf32::*;
pub use elf64::*;
pub use traits::*;

#[allow(unused_variables)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    BufferTooShort,
    InvalidMagic,
    InvalidClass,
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfAbi {
    SystemV     ,   // 0x00,
    HPUX        ,   // 0x01,
    NetBSD      ,   // 0x02,
    Linux       ,   // 0x03,
    Hurd        ,   // 0x04,
    Solaris     ,   // 0x06,
    AIX         ,   // 0x07,
    IRIX        ,   // 0x08,
    FreeBSD     ,   // 0x09,
    Tru64       ,   // 0x0A,
    NovellModesto, // 0x0B,
    OpenBSD     ,   // 0x0C,
    OpenVMS     ,   // 0x0D,
    NonStopKernel, // 0x0E,
    AROS        ,   // 0x0F,
    FenixOS     ,   // 0x10,
    CloudABI    ,   // 0x11,
    Unknown(u8),
}

impl From<u8> for ElfAbi {
    fn from(n: u8) -> Self {
        match n {
            0x00 => ElfAbi::SystemV,
            0x01 => ElfAbi::HPUX,
            0x02 => ElfAbi::NetBSD,
            0x03 => ElfAbi::Linux,
            0x04 => ElfAbi::Hurd,
            0x06 => ElfAbi::Solaris,
            0x07 => ElfAbi::AIX,
            0x08 => ElfAbi::IRIX,
            0x09 => ElfAbi::FreeBSD,
            0x0A => ElfAbi::Tru64,
            0x0B => ElfAbi::NovellModesto,
            0x0C => ElfAbi::OpenBSD,
            0x0D => ElfAbi::OpenVMS,
            0x0E => ElfAbi::NonStopKernel,
            0x0F => ElfAbi::AROS,
            0x10 => ElfAbi::FenixOS,
            0x11 => ElfAbi::CloudABI,
            n => ElfAbi::Unknown(n)
        }
    }
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SectionType {
    SHT_NULL          , // 0x0,
    SHT_PROGBITS      , // 0x1,
    SHT_SYMTAB        , // 0x2,
    SHT_STRTAB        , // 0x3,
    SHT_RELA          , // 0x4,
    SHT_HASH          , // 0x5,
    SHT_DYNAMIC       , // 0x6,
    SHT_NOTE          , // 0x7,
    SHT_NOBITS        , // 0x8,
    SHT_REL           , // 0x9,
    SHT_SHLIB         , // 0x0A,
    SHT_DYNSYM        , // 0x0B,
    SHT_INIT_ARRAY    , // 0x0E,
    SHT_FINI_ARRAY    , // 0x0F,
    SHT_PREINIT_ARRAY , // 0x10,
    SHT_GROUP         , // 0x11,
    SHT_SYMTAB_SHNDX  , // 0x12,
    SHT_NUM           , // 0x13,
    SHT_LOOS          , // 0x60000000,
    Unknown(u32)
}

impl From<u32> for SectionType {
    fn from(n: u32) -> Self {
        match n {
            0x0     => SectionType::SHT_NULL,
            0x1     => SectionType::SHT_PROGBITS,
            0x2     => SectionType::SHT_SYMTAB,
            0x3     => SectionType::SHT_STRTAB,
            0x4     => SectionType::SHT_RELA,
            0x5     => SectionType::SHT_HASH,
            0x6     => SectionType::SHT_DYNAMIC,
            0x7     => SectionType::SHT_NOTE,
            0x8     => SectionType::SHT_NOBITS,
            0x9     => SectionType::SHT_REL,
            0x0A    => SectionType::SHT_SHLIB,
            0x0B    => SectionType::SHT_DYNSYM,
            0x0E    => SectionType::SHT_INIT_ARRAY,
            0x0F    => SectionType::SHT_FINI_ARRAY,
            0x10    => SectionType::SHT_PREINIT_ARRAY,
            0x11    => SectionType::SHT_GROUP,
            0x12    => SectionType::SHT_SYMTAB_SHNDX,
            0x13    => SectionType::SHT_NUM,
            0x60000000 => SectionType::SHT_LOOS,
            n => SectionType::Unknown(n)
        }
    }
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfType {
    ET_NONE   ,// 0x00,
    ET_REL    ,// 0x01,
    ET_EXEC   ,// 0x02,
    ET_DYN    ,// 0x03,
    ET_CORE   ,// 0x04,
    ET_LOOS   ,// 0xfe00,
    ET_HIOS   ,// 0xfeff,
    ET_LOPROC ,// 0xff00,
    ET_HIPROC ,// 0xffff
    Unknown(u16)
}

impl From<u16> for ElfType {
    fn from(n: u16) -> Self {
        match n {
            0x00 => ElfType::ET_NONE,
            0x01 => ElfType::ET_REL,
            0x02 => ElfType::ET_EXEC,
            0x03 => ElfType::ET_DYN,
            0x04 => ElfType::ET_CORE,
            0xfe00 => ElfType::ET_LOOS,
            0xfeff => ElfType::ET_HIOS,
            0xff00 => ElfType::ET_LOPROC,
            0xffff => ElfType::ET_HIPROC,
            n => ElfType::Unknown(n)
        }
    }
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfMachine {
    Unknown ,// 0x00
    SPARC   ,// 0x02
    x86     ,// 0x03
    MIPS    ,// 0x08
    PowerPC ,// 0x14
    S390    ,// 0x16
    ARM     ,// 0x28
    SuperH  ,// 0x2A
    IA_64   ,// 0x32
    x86_64  ,// 0x3E
    AArch64 ,// 0xB7
    RISC_V  ,// 0xF3
    MachineUnknown(u16)
}

impl From<u16> for ElfMachine {
    fn from(n: u16) -> Self {
        match n {
            0x00 => ElfMachine::Unknown,
            0x02 => ElfMachine::SPARC,
            0x03 => ElfMachine::x86,
            0x08 => ElfMachine::MIPS,
            0x14 => ElfMachine::PowerPC,
            0x16 => ElfMachine::S390,
            0x28 => ElfMachine::ARM,
            0x2A => ElfMachine::SuperH,
            0x32 => ElfMachine::IA_64,
            0x3E => ElfMachine::x86_64,
            0xB7 => ElfMachine::AArch64,
            0xF3 => ElfMachine::RISC_V,
            n => ElfMachine::MachineUnknown(n)
        }
    }
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfClass {
    Elf32 , // 1
    Elf64 , // 2
    Unknown(u8)
}

impl From<u8> for ElfClass {
    fn from(n: u8) -> Self {
        match n {
            1 => ElfClass::Elf32,
            2 => ElfClass::Elf64,
            n => ElfClass::Unknown(n)
        }
    }
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfEndian {
    LittleEndian ,// 1,
    BigEndian    ,// 2,
    Unknown(u8),
}

impl From<u8> for ElfEndian {
    fn from(n: u8) -> Self {
        match n {
            1 => ElfEndian::LittleEndian,
            2 => ElfEndian::BigEndian,
            n => ElfEndian::Unknown(n)
        }
    }
}

#[allow(unused_variables, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProgramType {
    NULL      ,// 0x00000000,
    LOAD      ,// 0x00000001,
    DYNAMIC   ,// 0x00000002,
    INTERP    ,// 0x00000003,
    NOTE      ,// 0x00000004,
    SHLIB     ,// 0x00000005,
    PHDR      ,// 0x00000006,
    LOOS      ,// 0x60000000,
    HIOS      ,// 0x6FFFFFFF,
    LOPROC    ,// 0x70000000,
    HIPROC    ,// 0x7FFFFFFF,
    GNU_STACK ,// 0x6474e551,
    Unknown(u32)
}

impl From<u32> for ProgramType {
    fn from(n: u32) -> Self {
        match n {
            0x00000000 => ProgramType::NULL,
            0x00000001 => ProgramType::LOAD,     
            0x00000002 => ProgramType::DYNAMIC,  
            0x00000003 => ProgramType::INTERP,   
            0x00000004 => ProgramType::NOTE,     
            0x00000005 => ProgramType::SHLIB,    
            0x00000006 => ProgramType::PHDR,
            0x60000000 => ProgramType::LOOS,
            0x6FFFFFFF => ProgramType::HIOS,
            0x70000000 => ProgramType::LOPROC,
            0x7FFFFFFF => ProgramType::HIPROC,
            0x6474e551 => ProgramType::GNU_STACK,
            n => ProgramType::Unknown(n)
        }
    }
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
        match tmp_elf.header().class.into() {
            ElfClass::Elf64 => { 
                let elf = Elf64::new(elf_buf);
                if elf_buf.len() < elf.header().elf_header_size() as usize {
                    Err(Error::BufferTooShort)
                } else {
                    Ok(Elf::Elf64(elf)) }
                },
            ElfClass::Elf32 => { 
                let elf = Elf32::new(elf_buf);
                if elf_buf.len() < elf.header().elf_header_size() as usize {
                    Err(Error::BufferTooShort)
                } else {
                    Ok(Elf::Elf32(elf)) }
                },
            ElfClass::Unknown(_) => { Err(Error::InvalidClass) }
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
