mod section_header;

pub use section_header::{SectionHeader32, SectionHeader64};

const SHT_LOOS: u32 = 0x60000000;
const SHT_HIOS: u32 = 0x6fffffff;
const SHT_LOPROC: u32 = 0x70000000;
const SHT_HIPROC: u32 = 0x7fffffff;
const SHT_LOUSER: u32 = 0x80000000;
const SHT_HIUSER: u32 = 0xffffffff;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SectionType {
    SHT_NULL,          // 0x0,
    SHT_PROGBITS,      // 0x1,
    SHT_SYMTAB,        // 0x2,
    SHT_STRTAB,        // 0x3,
    SHT_RELA,          // 0x4,
    SHT_HASH,          // 0x5,
    SHT_DYNAMIC,       // 0x6,
    SHT_NOTE,          // 0x7,
    SHT_NOBITS,        // 0x8,
    SHT_REL,           // 0x9,
    SHT_SHLIB,         // 0x0A,
    SHT_DYNSYM,        // 0x0B,
    SHT_INIT_ARRAY,    // 0x0E,
    SHT_FINI_ARRAY,    // 0x0F,
    SHT_PREINIT_ARRAY, // 0x10,
    SHT_GROUP,         // 0x11,
    SHT_SYMTAB_SHNDX,  // 0x12,
    SHT_NUM,           // 0x13,
    OsSpecific(u32),
    ProcessorSpecific(u32),
    ApplicationSpecific(u32),
    Unknown(u32),
}

impl From<u32> for SectionType {
    fn from(n: u32) -> Self {
        match n {
            0x0 => SectionType::SHT_NULL,
            0x1 => SectionType::SHT_PROGBITS,
            0x2 => SectionType::SHT_SYMTAB,
            0x3 => SectionType::SHT_STRTAB,
            0x4 => SectionType::SHT_RELA,
            0x5 => SectionType::SHT_HASH,
            0x6 => SectionType::SHT_DYNAMIC,
            0x7 => SectionType::SHT_NOTE,
            0x8 => SectionType::SHT_NOBITS,
            0x9 => SectionType::SHT_REL,
            0x0A => SectionType::SHT_SHLIB,
            0x0B => SectionType::SHT_DYNSYM,
            0x0E => SectionType::SHT_INIT_ARRAY,
            0x0F => SectionType::SHT_FINI_ARRAY,
            0x10 => SectionType::SHT_PREINIT_ARRAY,
            0x11 => SectionType::SHT_GROUP,
            0x12 => SectionType::SHT_SYMTAB_SHNDX,
            0x13 => SectionType::SHT_NUM,
            x @ SHT_LOOS..=SHT_HIOS => SectionType::OsSpecific(x),
            x @ SHT_LOPROC..=SHT_HIPROC => SectionType::ProcessorSpecific(x),
            x @ SHT_LOUSER..=SHT_HIUSER => SectionType::ApplicationSpecific(x),
            n => SectionType::Unknown(n),
        }
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

pub trait SectionHeaderRaw {
    fn name_off(&self) -> u32;

    fn sh_type(&self) -> SectionType;

    fn flags(&self) -> SectionHeaderFlags;

    fn addr(&self) -> u64;

    fn offset(&self) -> u64;

    fn size(&self) -> u64;

    fn link(&self) -> u32;

    fn info(&self) -> u32;

    fn addralign(&self) -> u64;

    fn entsize(&self) -> u64;
}
