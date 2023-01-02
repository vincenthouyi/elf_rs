mod elf_header;

pub use elf_header::{ElfHeader32, ElfHeader64};

pub const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfClass {
    Elf32, // 1
    Elf64, // 2
    Unknown(u8),
}

impl From<u8> for ElfClass {
    fn from(n: u8) -> Self {
        match n {
            1 => ElfClass::Elf32,
            2 => ElfClass::Elf64,
            n => ElfClass::Unknown(n),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfEndian {
    LittleEndian, // 1,
    BigEndian,    // 2,
    Unknown(u8),
}

impl From<u8> for ElfEndian {
    fn from(n: u8) -> Self {
        match n {
            1 => ElfEndian::LittleEndian,
            2 => ElfEndian::BigEndian,
            n => ElfEndian::Unknown(n),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfAbi {
    SystemV,       // 0x00,
    HPUX,          // 0x01,
    NetBSD,        // 0x02,
    Linux,         // 0x03,
    Hurd,          // 0x04,
    Solaris,       // 0x06,
    AIX,           // 0x07,
    IRIX,          // 0x08,
    FreeBSD,       // 0x09,
    Tru64,         // 0x0A,
    NovellModesto, // 0x0B,
    OpenBSD,       // 0x0C,
    OpenVMS,       // 0x0D,
    NonStopKernel, // 0x0E,
    AROS,          // 0x0F,
    FenixOS,       // 0x10,
    CloudABI,      // 0x11,
    Unknown(u8),
}

impl From<u8> for ElfAbi {
    fn from(n: u8) -> Self {
        match n {
            0x00 => ElfAbi::SystemV,
            0x01 => ElfAbi::HPUX,
            0x02 => ElfAbi::NetBSD,
            0x03 => ElfAbi::Linux,
            0x04 => ElfAbi::Hurd,
            0x06 => ElfAbi::Solaris,
            0x07 => ElfAbi::AIX,
            0x08 => ElfAbi::IRIX,
            0x09 => ElfAbi::FreeBSD,
            0x0A => ElfAbi::Tru64,
            0x0B => ElfAbi::NovellModesto,
            0x0C => ElfAbi::OpenBSD,
            0x0D => ElfAbi::OpenVMS,
            0x0E => ElfAbi::NonStopKernel,
            0x0F => ElfAbi::AROS,
            0x10 => ElfAbi::FenixOS,
            0x11 => ElfAbi::CloudABI,
            n => ElfAbi::Unknown(n),
        }
    }
}

const ET_LOOS: u16 = 0xfe00;
const ET_HIOS: u16 = 0xfeff;
const ET_LOPROC: u16 = 0xff00;
const ET_HIPROC: u16 = 0xffff;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfType {
    ET_NONE, // 0x00,
    ET_REL,  // 0x01,
    ET_EXEC, // 0x02,
    ET_DYN,  // 0x03,
    ET_CORE, // 0x04,
    OsSpecific(u16),
    ProcessorSpecific(u16),
    Unknown(u16),
}

impl From<u16> for ElfType {
    fn from(n: u16) -> Self {
        match n {
            0x00 => ElfType::ET_NONE,
            0x01 => ElfType::ET_REL,
            0x02 => ElfType::ET_EXEC,
            0x03 => ElfType::ET_DYN,
            0x04 => ElfType::ET_CORE,
            x @ ET_LOOS..=ET_HIOS => ElfType::OsSpecific(x),
            x @ ET_LOPROC..=ET_HIPROC => ElfType::ProcessorSpecific(x),
            n => ElfType::Unknown(n),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElfMachine {
    Unknown, // 0x00
    SPARC,   // 0x02
    x86,     // 0x03
    MIPS,    // 0x08
    PowerPC, // 0x14
    S390,    // 0x16
    ARM,     // 0x28
    SuperH,  // 0x2A
    IA_64,   // 0x32
    x86_64,  // 0x3E
    AArch64, // 0xB7
    RISC_V,  // 0xF3
    MachineUnknown(u16),
}

impl From<u16> for ElfMachine {
    fn from(n: u16) -> Self {
        match n {
            0x00 => ElfMachine::Unknown,
            0x02 => ElfMachine::SPARC,
            0x03 => ElfMachine::x86,
            0x08 => ElfMachine::MIPS,
            0x14 => ElfMachine::PowerPC,
            0x16 => ElfMachine::S390,
            0x28 => ElfMachine::ARM,
            0x2A => ElfMachine::SuperH,
            0x32 => ElfMachine::IA_64,
            0x3E => ElfMachine::x86_64,
            0xB7 => ElfMachine::AArch64,
            0xF3 => ElfMachine::RISC_V,
            n => ElfMachine::MachineUnknown(n),
        }
    }
}

pub trait ElfHeaderRaw {
    fn class(&self) -> ElfClass;

    fn endianness(&self) -> ElfEndian;

    fn header_version(&self) -> u8;

    fn abi(&self) -> ElfAbi;

    fn abi_version(&self) -> u8;

    fn elftype(&self) -> ElfType;

    fn machine(&self) -> ElfMachine;

    fn elf_version(&self) -> u32;

    fn entry_point(&self) -> u64;

    fn program_header_offset(&self) -> u64;

    fn section_header_offset(&self) -> u64;

    fn flags(&self) -> u32;

    fn elf_header_size(&self) -> u16;

    fn program_header_entry_size(&self) -> u16;

    fn program_header_entry_num(&self) -> u16;

    fn section_header_entry_size(&self) -> u16;

    fn section_header_entry_num(&self) -> u16;

    fn shstr_index(&self) -> u16;
}
