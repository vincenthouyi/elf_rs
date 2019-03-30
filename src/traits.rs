use crate::*;
use core::fmt;

pub trait GenElf: Sized {
    type Word: Into<u64> + fmt::Debug + fmt::Display + fmt::LowerHex + fmt::UpperHex;
    type ElfHeaderType : GenElfHeader;
    type ProgramHeaderType : GenProgramHeader;
    type SectionHeaderType : GenSectionHeader;

    fn as_bytes(&self) -> &[u8];

    fn header(&self) -> &Self::ElfHeaderType {
        unsafe {
            &*(self.as_bytes() as *const _ as *const Self::ElfHeaderType)
        }
    }

    fn program_headers(&self) -> &[Self::ProgramHeaderType] {
        use core::slice::from_raw_parts;
        let ph_off = self.header().program_header_offset().into();
        let ph_num = self.header().program_header_entry_num() as usize;
        unsafe {
            let ph_ptr = self.as_bytes().as_ptr().add(ph_off as usize);
            from_raw_parts(ph_ptr as *const _ as *const Self::ProgramHeaderType,
                           ph_num)
        }
    }

    fn program_header_iter(&self) -> ProgramHeaderIter<Self> {
        ProgramHeaderIter {
            elf: self,
            ph: self.program_headers(),
            idx: 0
        }
    }

    fn section_headers(&self) -> &[Self::SectionHeaderType] {
        use core::slice::from_raw_parts;
        let sh_off = self.header().section_header_offset().into() as isize;
        let sh_num = self.header().section_header_entry_num() as usize;
        unsafe {
            let sh_ptr = (self.as_bytes() as *const _ as *const u8).offset(sh_off);
            from_raw_parts(sh_ptr as *const _ as *const Self::SectionHeaderType,
                           sh_num)
        }
    }

    fn section_header_iter(&self) -> SectionHeaderIter<Self> {
        SectionHeaderIter {
            elf: self,
            sh: self.section_headers(),
            idx: 0,
        }
    }

    fn shstr_section(&self) -> &[u8] {
        use GenSectionHeader;
        let sh =  &self.section_headers()[self.header().shstr_index() as usize];
        let seg_off = sh.offset().into() as usize;
        let seg_filesz = sh.size().into() as usize;
        &self.as_bytes()[seg_off .. seg_off+seg_filesz]
    }

    fn lookup_section(&self, name: &[u8]) -> Option<SectionHeader<Self>> {
        self.section_header_iter()
            .find(|s| s.section_name().0 == name)
    }
}

pub trait GenElfHeader {
    type Word: Into<u64> + fmt::Debug + fmt::Display + fmt::LowerHex + fmt::UpperHex;

    fn class(&self) -> ElfClass;
    
    fn endianness(&self) -> ElfEndian;

    fn header_version(&self) -> u8;

    fn abi(&self) -> ElfAbi;

    fn abi_version(&self) -> u8;

    fn elftype(&self) -> ElfType;

    fn machine(&self) -> ElfMachine;

    fn elf_version(&self) -> u32;

    fn entry_point(&self) -> Self::Word;

    fn program_header_offset(&self) -> Self::Word;

    fn section_header_offset(&self) -> Self::Word;

    fn flags(&self) -> u32;

    fn elf_header_size(&self) -> u16;

    fn program_header_entry_size(&self) -> u16;

    fn program_header_entry_num(&self) -> u16;

    fn section_header_entry_size(&self) -> u16;

    fn section_header_entry_num(&self) -> u16;

    fn shstr_index(&self) -> u16;

}

pub trait GenProgramHeader {
    type Word: Into<u64> + fmt::Debug + fmt::Display + fmt::LowerHex + fmt::UpperHex;

    fn ph_type(&self) -> ProgramType;

    fn flags(&self) -> u32;

    fn offset(&self) -> Self::Word;

    fn vaddr(&self) -> Self::Word;

    fn paddr(&self) -> Self::Word;

    fn filesz(&self) -> Self::Word;

    fn memsz(&self) -> Self::Word;

    fn align(&self) -> Self::Word;
}

pub trait GenSectionHeader {
    type Word: Into<u64> + fmt::Debug + fmt::Display + fmt::LowerHex + fmt::UpperHex;

    fn name_off(&self) -> u32;

    fn sh_type(&self) -> SectionType;

    fn flags(&self) -> SectionHeaderFlags;

    fn addr(&self) -> Self::Word;

    fn offset(&self) -> Self::Word;

    fn size(&self) -> Self::Word;

    fn link(&self) -> u32;

    fn info(&self) -> u32;

    fn addralign(&self) -> Self::Word;

    fn entsize(&self) -> Self::Word;
}