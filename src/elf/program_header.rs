use super::ElfFile;
use crate::ProgramHeaderRaw;
use core::fmt;
use core::ops;

pub struct ProgramHeaderEntry<'a> {
    elf_file: &'a dyn ElfFile,
    inner: &'a dyn ProgramHeaderRaw,
}

impl<'a> ProgramHeaderEntry<'a> {
    pub fn new(elf_file: &'a dyn ElfFile, inner: &'a dyn ProgramHeaderRaw) -> Self {
        Self { elf_file, inner }
    }

    pub fn content(&self) -> Option<&'a [u8]> {
        let offset = self.inner.offset() as usize;
        let size = self.inner.filesz() as usize;
        let top = offset.saturating_add(size);
        self.elf_file.content().get(offset..top)
    }
}

impl<'a> ops::Deref for ProgramHeaderEntry<'a> {
    type Target = dyn ProgramHeaderRaw + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a> fmt::Debug for ProgramHeaderEntry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Program Header")
            .field("type", &self.ph_type())
            .field("flags", &self.flags())
            .field("offset", &self.offset())
            .field("vaddr", &self.vaddr())
            .field("paddr", &self.paddr())
            .field("filesize", &self.filesz())
            .field("memsize", &self.memsz())
            .field("alignment", &self.align())
            .finish()
    }
}

pub struct ProgramHeaderIter<'a> {
    elf_file: &'a dyn ElfFile,
    index: usize,
}

impl<'a> ProgramHeaderIter<'a> {
    pub fn new(elf_file: &'a dyn ElfFile) -> Self {
        Self { elf_file, index: 0 }
    }
}

impl<'a> Iterator for ProgramHeaderIter<'a> {
    type Item = ProgramHeaderEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elf_file.program_header_nth(self.index).map(|e| {
            self.index += 1;
            e
        })
    }
}
