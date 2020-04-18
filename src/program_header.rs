use core::fmt::{Formatter, Debug};

use num_traits::PrimInt;

use crate::elf::ElfGen;

#[derive(Debug)]
#[repr(C)]
pub struct ProgramHeaderGen<T> {
    p_type: u32,
    p_flags: u32,
    p_offset: T,
    p_vaddr: T,
    p_paddr: T,
    p_filesz: T,
    p_memsz: T,
    p_align: T,
}

const LOOS : u32 = 0x60000000;
const HIOS : u32 = 0x6FFFFFFF;
const LOPROC : u32 = 0x70000000;
const HIPROC : u32 = 0x7FFFFFFF;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProgramType {
    NULL      ,// 0x00000000,
    LOAD      ,// 0x00000001,
    DYNAMIC   ,// 0x00000002,
    INTERP    ,// 0x00000003,
    NOTE      ,// 0x00000004,
    SHLIB     ,// 0x00000005,
    PHDR      ,// 0x00000006,
    GNU_STACK ,// 0x6474e551,
    OsSpecific(u32), // 0x60000000 - 0x6FFFFFFF,
    ProcessorSpecific(u32), // 0x70000000 - 0x7FFFFFFF,

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
            0x6474e551 => ProgramType::GNU_STACK,
            x @ LOOS ..= HIOS => ProgramType::OsSpecific(x),
            x @ LOPROC ..= HIPROC => ProgramType::ProcessorSpecific(x),
            n => ProgramType::Unknown(n)
        }
    }
}

impl<T: PrimInt> ProgramHeaderGen<T> {
    pub fn ph_type(&self) -> ProgramType {
        self.p_type.into()
    }

    pub fn flags(&self) -> u32 {
        self.p_flags
    }

    pub fn offset(&self) -> T {
        self.p_offset
    }

    pub fn vaddr(&self) -> T {
        self.p_vaddr
    }

    pub fn paddr(&self) -> T {
        self.p_paddr
    }

    pub fn filesz(&self) -> T {
        self.p_filesz
    }

    pub fn memsz(&self) -> T {
        self.p_memsz 
    }

    pub fn align(&self) -> T {
        self.p_align
    }
}

#[derive(Debug)]
pub struct ProgramHeaderIter<'a, T> {
    elf: &'a ElfGen<'a, T>,
    ph: &'a[ProgramHeaderGen<T>],
    idx: u16,
}

impl<'a, T: PrimInt> ProgramHeaderIter<'a, T> {
    pub fn new(elf: &'a ElfGen<'a, T>) -> Self {
        Self {
            elf: elf,
            ph: elf.program_headers(),
            idx: 0
        }
    }
}

impl <'a, T> core::iter::Iterator for ProgramHeaderIter<'a, T> {
    type Item = ProgramHeader<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(ProgramHeader {
            elf: self.elf,
            ph: self.ph.get(self.idx as usize)?
        });
        self.idx += 1;
        ret
    }
}

pub struct ProgramHeader<'a, T> {
    elf: &'a ElfGen<'a, T>,
    pub ph: &'a ProgramHeaderGen<T>
}

impl <'a, T: PrimInt> ProgramHeader<'a, T> {
    pub fn segment(&self) -> &[u8] {
        let seg_off = self.ph.offset().to_usize().unwrap();
        let seg_filesz = self.ph.filesz().to_usize().unwrap();
        &self.elf.as_bytes()[seg_off .. seg_off+seg_filesz]
    }
}

impl<'a, T: PrimInt + Debug> Debug for ProgramHeader<'a, T>{
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
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
