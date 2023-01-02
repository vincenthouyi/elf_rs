use super::ElfFile;
use crate::elf_header::ElfHeaderRaw;
use core::fmt;
use core::ops;

pub struct ElfHeader<'a> {
    _elf_file: &'a dyn ElfFile,
    inner: &'a dyn ElfHeaderRaw,
}

impl<'a> ElfHeader<'a> {
    pub fn new(elf_file: &'a dyn ElfFile, inner: &'a dyn ElfHeaderRaw) -> Self {
        Self {
            _elf_file: elf_file,
            inner,
        }
    }
}

impl<'a> ops::Deref for ElfHeader<'a> {
    type Target = dyn ElfHeaderRaw + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a> fmt::Debug for ElfHeader<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ELF Header")
            .field("Class", &self.class())
            .field("Endianness", &self.endianness())
            .field("ELF Header Version", &self.header_version())
            .field("ABI", &self.abi())
            .field("ABI Version", &self.abi_version())
            .field("File Type", &self.elftype())
            .field("Target Machine", &self.machine())
            .field("ELF Version", &self.elf_version())
            .field("Entry Point", &self.entry_point())
            .field("Program Header Offset", &self.program_header_offset())
            .field("Section Header Offset", &self.section_header_offset())
            .field("Flags", &self.flags())
            .field("ELF Header Size", &self.elf_header_size())
            .field("Program Header Size", &self.program_header_entry_size())
            .field("Program Header Number", &self.program_header_entry_num())
            .field("Section Header Size", &self.section_header_entry_size())
            .field("Section Header Number", &self.section_header_entry_num())
            .field(".shstr Section Index", &self.shstr_index())
            .finish()
    }
}
