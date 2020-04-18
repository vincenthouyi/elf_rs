use core::fmt::{Formatter, Debug};

use num_traits::PrimInt;

use crate::elf::ElfGen;

const SHT_LOOS         : u32 = 0x60000000;
const SHT_HIOS         : u32 = 0x6fffffff;
const SHT_LOPROC       : u32 = 0x70000000;
const SHT_HIPROC       : u32 = 0x7fffffff;
const SHT_LOUSER       : u32 = 0x80000000;
const SHT_HIUSER       : u32 = 0xffffffff;
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
    OsSpecific(u32),
    ProcessorSpecific(u32),
    ApplicationSpecific(u32),
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
            x @ SHT_LOOS ..= SHT_HIOS => SectionType::OsSpecific(x),
            x @ SHT_LOPROC ..= SHT_HIPROC => SectionType::ProcessorSpecific(x),
            x @ SHT_LOUSER ..= SHT_HIUSER => SectionType::ApplicationSpecific(x),
            n => SectionType::Unknown(n)
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SectionHeaderGen<T: PrimInt> {
    sh_name: u32,
    sh_type: u32,
    sh_flags: T,
    sh_addr: T,
    sh_offset: T,
    sh_size: T,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: T,
    sh_entsize: T,
}

impl<T: PrimInt> SectionHeaderGen<T> {
    pub fn name_off(&self) -> u32 {
        self.sh_name
    }

    pub fn sh_type(&self) -> SectionType {
        self.sh_type.into()
    }

    pub fn flags(&self) -> SectionHeaderFlags {
        SectionHeaderFlags::from_bits_truncate(self.sh_flags.to_u64().unwrap())
    }

    pub fn addr(&self) -> T {
        self.sh_addr
    }

    pub fn offset(&self) -> T {
        self.sh_offset
    }

    pub fn size(&self) -> T {
        self.sh_size
    }

    pub fn link(&self) -> u32 {
        self.sh_link
    }

    pub fn info(&self) -> u32 {
        self.sh_info
    }

    pub fn addralign(&self) -> T {
        self.sh_addralign
    }

    pub fn entsize(&self) -> T {
        self.sh_entsize
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

pub struct SectionHeader<'a, T: PrimInt> {
    elf: &'a ElfGen<'a, T>,
    pub sh: &'a SectionHeaderGen<T>
}

impl<'a, T: PrimInt> SectionHeader<'a, T> {
    pub fn segment(&'a self) -> &'a [u8] {
        let seg_off = self.sh.offset().to_usize().unwrap();
        let seg_filesz = self.sh.size().to_usize().unwrap();
        &self.elf.as_bytes()[seg_off .. seg_off+seg_filesz]
    }

    pub fn section_name(&'a self) -> &'a str {
        let name_off = self.sh.name_off() as usize;
        let shstr = self.elf.shstr_section();
        let name_len = shstr[name_off..].iter()
                                        .position(|&x| x == b'\0')
                                        .unwrap();
        core::str::from_utf8(&shstr[name_off .. name_off + name_len]).unwrap()
    }
}

impl<'a, T: PrimInt + Debug> Debug for SectionHeader<'a, T>{
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
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
pub struct SectionHeaderIter<'a, T: PrimInt> {
    elf: &'a ElfGen<'a, T>,
    sh: &'a[SectionHeaderGen<T>],
    idx: u16,
}

impl<'a, T: PrimInt> SectionHeaderIter<'a, T> {
    pub fn new(elf: &'a ElfGen<T>) -> Self {
        Self {
            elf: elf,
            sh: elf.section_headers(),
            idx: 0,
        }
    }
}

impl <'a, T: PrimInt> core::iter::Iterator for SectionHeaderIter<'a, T> {
    type Item = SectionHeader<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(SectionHeader {
            elf: self.elf,
            sh: self.sh.get(self.idx as usize)?
        });
        self.idx += 1;
        ret
    }
}
