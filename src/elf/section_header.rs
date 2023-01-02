use super::ElfFile;
use crate::section_header::SectionHeaderRaw;
use core::fmt;
use core::ops;

pub struct SectionHeaderEntry<'a> {
    elf_file: &'a dyn ElfFile,
    inner: &'a dyn SectionHeaderRaw,
}

impl<'a> ops::Deref for SectionHeaderEntry<'a> {
    type Target = dyn SectionHeaderRaw + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a> SectionHeaderEntry<'a> {
    pub fn new(elf_file: &'a dyn ElfFile, inner: &'a dyn SectionHeaderRaw) -> Self {
        Self { elf_file, inner }
    }

    pub fn content(&self) -> Option<&'a [u8]> {
        let offset = self.inner.offset() as usize;
        let size = self.inner.size() as usize;
        let top = offset.saturating_add(size);
        self.elf_file.content().get(offset..top)
    }

    pub fn section_name(&self) -> Option<&'a [u8]> {
        let name_off = self.inner.name_off() as usize;
        let shstr_content = self.elf_file.shstr_section()?.content()?;
        shstr_content.get(name_off..)?.split(|&x| x == b'\0').next()
    }
}

impl<'a> fmt::Debug for SectionHeaderEntry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sh_name = self
            .section_name()
            .and_then(|n| core::str::from_utf8(n).ok())
            .unwrap_or("");
        f.debug_struct("Section Header")
            .field("name", &sh_name)
            .field("type", &self.sh_type())
            .field("flags", &self.flags())
            .field("addr", &self.addr())
            .field("offset", &self.offset())
            .field("size", &self.size())
            .field("link", &self.link())
            .field("info", &self.info())
            .field("address alignment", &self.addralign())
            .field("entry size", &self.entsize())
            .finish()
    }
}

pub struct SectionHeaderIter<'a> {
    elf_file: &'a dyn ElfFile,
    index: usize,
}

impl<'a> SectionHeaderIter<'a> {
    pub fn new(elf_file: &'a dyn ElfFile) -> Self {
        Self { elf_file, index: 0 }
    }
}

impl<'a> Iterator for SectionHeaderIter<'a> {
    type Item = SectionHeaderEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elf_file.section_header_nth(self.index).map(|e| {
            self.index += 1;
            e
        })
    }
}
