mod elf;
pub use elf::{Elf32, Elf64};

mod elf_header;
pub use elf_header::ElfHeader;

mod program_header;
pub use program_header::{ProgramHeaderEntry, ProgramHeaderIter};

mod section_header;
pub use section_header::{SectionHeaderEntry, SectionHeaderIter};

pub trait ElfType {
    type ElfHeader: crate::elf_header::ElfHeaderRaw;
    type ProgramHeader: crate::program_header::ProgramHeaderRaw;
    type SectionHeader: crate::section_header::SectionHeaderRaw;

    fn elf_class() -> crate::elf_header::ElfClass;
}

pub trait ElfFile {
    fn content(&self) -> &[u8];

    fn elf_header(&self) -> ElfHeader;

    fn program_header_nth(&self, index: usize) -> Option<ProgramHeaderEntry>;

    fn program_header_iter(&self) -> ProgramHeaderIter;

    fn section_header_nth(&self, index: usize) -> Option<SectionHeaderEntry>;

    fn section_header_iter(&self) -> SectionHeaderIter;

    fn shstr_section(&self) -> Option<SectionHeaderEntry> {
        let shstr_index = self.elf_header().shstr_index() as usize;
        self.section_header_nth(shstr_index)
    }

    fn lookup_section(&self, name: &[u8]) -> Option<SectionHeaderEntry> {
        self.section_header_iter()
            .find(|s| s.section_name() == Some(name))
    }

    fn entry_point(&self) -> u64 {
        self.elf_header().entry_point()
    }
}
