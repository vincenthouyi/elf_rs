use super::{
    ElfFile, ElfHeader, ElfType, ProgramHeaderEntry, ProgramHeaderIter, SectionHeaderEntry,
    SectionHeaderIter,
};
use crate::elf_header::{ElfClass, ElfHeader32, ElfHeader64, ELF_MAGIC};
use crate::program_header::{ProgramHeader32, ProgramHeader64};
use crate::section_header::{SectionHeader32, SectionHeader64};
use crate::Error;
use core::fmt;
use core::marker::PhantomData;
use core::mem::size_of;
use core::slice::from_raw_parts;

#[derive(Debug)]
pub enum ElfType64 {}

impl ElfType for ElfType64 {
    type ElfHeader = ElfHeader64;
    type ProgramHeader = ProgramHeader64;
    type SectionHeader = SectionHeader64;

    fn elf_class() -> ElfClass {
        ElfClass::Elf64
    }
}

#[derive(Debug)]
pub enum ElfType32 {}

impl ElfType for ElfType32 {
    type ElfHeader = ElfHeader32;
    type ProgramHeader = ProgramHeader32;
    type SectionHeader = SectionHeader32;

    fn elf_class() -> ElfClass {
        ElfClass::Elf32
    }
}

pub struct Elf<'a, ET>(&'a [u8], PhantomData<ET>);

impl<'a, ET: ElfType> Elf<'a, ET> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self(buf, PhantomData)
    }

    pub fn from_bytes(buf: &'a [u8]) -> Result<Self, Error> {
        if !buf.starts_with(&ELF_MAGIC) {
            return Err(Error::InvalidMagic);
        }

        if buf.len() < size_of::<ET::ElfHeader>() {
            return Err(Error::BufferTooShort);
        }

        let elf = Self::new(buf);
        if buf.len() < elf.elf_header().elf_header_size() as usize {
            return Err(Error::BufferTooShort);
        }

        if elf.elf_header().class() != ET::elf_class() {
            return Err(Error::InvalidClass);
        }

        Ok(elf)
    }

    pub fn content(&self) -> &[u8] {
        self.0
    }

    pub fn elf_header_raw(&self) -> &ET::ElfHeader {
        unsafe { &*(self.content().as_ptr() as *const ET::ElfHeader) }
    }

    pub fn program_headers_raw(&self) -> Option<&'a [ET::ProgramHeader]> {
        let ph_off = self.elf_header().program_header_offset() as usize;
        let ph_num = self.elf_header().program_header_entry_num() as usize;
        let ph_top = ph_off.saturating_add(ph_num.saturating_mul(size_of::<ET::ProgramHeader>()));
        self.content()
            .get(ph_off..ph_top)
            .map(|mem| unsafe { from_raw_parts(mem.as_ptr() as *const ET::ProgramHeader, ph_num) })
    }

    pub fn program_header_iter(&self) -> ProgramHeaderIter {
        ProgramHeaderIter::new(self)
    }

    pub fn program_header_nth(&self, index: usize) -> Option<ProgramHeaderEntry> {
        self.program_headers_raw()
            .and_then(|s| s.get(index))
            .map(|ph| ProgramHeaderEntry::new(self, ph))
    }

    pub fn section_headers_raw(&'a self) -> Option<&'a [ET::SectionHeader]> {
        let sh_off = self.elf_header().section_header_offset() as usize;
        let sh_num = self.elf_header().section_header_entry_num() as usize;
        let sh_top = sh_off.saturating_add(sh_num.saturating_mul(size_of::<ET::SectionHeader>()));
        self.content()
            .get(sh_off..sh_top)
            .map(|mem| unsafe { from_raw_parts(mem.as_ptr() as *const ET::SectionHeader, sh_num) })
    }

    pub fn section_header_iter(&self) -> SectionHeaderIter {
        SectionHeaderIter::new(self)
    }

    pub fn section_header_nth(&self, index: usize) -> Option<SectionHeaderEntry> {
        self.section_headers_raw()
            .and_then(|s| s.get(index))
            .map(|sh| SectionHeaderEntry::new(self, sh))
    }
}

impl<'a, ET: ElfType> ElfFile for Elf<'a, ET> {
    fn content(&self) -> &[u8] {
        self.content()
    }

    fn elf_header(&self) -> ElfHeader {
        ElfHeader::new(self, self.elf_header_raw())
    }

    fn program_header_nth(&self, index: usize) -> Option<ProgramHeaderEntry> {
        self.program_header_nth(index)
    }

    fn program_header_iter(&self) -> ProgramHeaderIter {
        self.program_header_iter()
    }

    fn section_header_iter(&self) -> SectionHeaderIter {
        self.section_header_iter()
    }

    fn section_header_nth(&self, index: usize) -> Option<SectionHeaderEntry> {
        self.section_header_nth(index)
    }
}

pub type Elf32<'a> = Elf<'a, ElfType32>;
pub type Elf64<'a> = Elf<'a, ElfType64>;

impl<'a, ET: ElfType> fmt::Debug for Elf<'a, ET> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ELF Buffer")
            .field("Memory Location", &self.content().as_ptr())
            .field("Buffer Size", &self.content().len())
            .finish()
    }
}
