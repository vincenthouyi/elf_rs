use core::fmt;
use core::marker::PhantomData;

use num_traits::PrimInt;

use crate::section_header::{SectionHeaderGen, SectionHeader, SectionHeaderIter};
use crate::program_header::{ProgramHeaderGen, ProgramHeaderIter};
use crate::elf_header::ElfHeaderGen;

pub struct ElfGen<'a, T>(&'a [u8], PhantomData<T>);

impl<'a, T: PrimInt> ElfGen<'a, T> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self(buf, PhantomData)
    }

    pub fn as_bytes(&self) -> &[u8] { self.0 }

    pub fn header(&self) -> &ElfHeaderGen<T> {
        unsafe {
            &*(self.as_bytes().as_ptr() as *const ElfHeaderGen<T>)
        }
    }

    pub fn program_headers(&self) -> &[ProgramHeaderGen<T>] {
        use core::slice::from_raw_parts;

        let ph_off = self.header().program_header_offset().to_usize().unwrap();
        let ph_num = self.header().program_header_entry_num() as usize;
        unsafe {
            let ph_ptr = self.as_bytes().as_ptr().add(ph_off);
            from_raw_parts(ph_ptr as *const ProgramHeaderGen<T>,
                           ph_num)
        }
    }

    pub fn program_header_iter(&'a self) -> ProgramHeaderIter<'a, T> {
        ProgramHeaderIter::new(self)
    }

    pub fn section_headers(&self) -> &[SectionHeaderGen<T>] {
        use core::slice::from_raw_parts;

        let sh_off = self.header().section_header_offset().to_usize().unwrap();
        let sh_num = self.header().section_header_entry_num() as usize;
        unsafe {
            let sh_ptr = self.as_bytes().as_ptr().add(sh_off);
            from_raw_parts(sh_ptr as *const SectionHeaderGen<T>,
                           sh_num)
        }
    }

    pub fn section_header_iter(&self) -> SectionHeaderIter<T> {
        SectionHeaderIter::new(self)
    }

    pub fn shstr_section(&self) -> &[u8] {
        let sh =  &self.section_headers()[self.header().shstr_index() as usize];
        let seg_off = sh.offset().to_usize().unwrap();
        let seg_filesz = sh.size().to_usize().unwrap();
        &self.as_bytes()[seg_off .. seg_off+seg_filesz]
    }

    pub fn lookup_section(&self, name: &str) -> Option<SectionHeader<T>> {
        self.section_header_iter()
            .find(|s| s.section_name() == name)
    }
}

impl<'a, T> fmt::Debug for ElfGen<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Elf File")
         .field("Memory Location", &self.0.as_ptr())
         .finish()
    }
}

