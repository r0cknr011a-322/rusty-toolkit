use core::fmt;

const ELF_MAGICK: u32 = 0x7F454C46;
const ELF_FORMAT_32BIT: u8 = 1;
const ELF_FORMAT_64BIT: u8 = 2;

#[derive(Debug)]
pub enum Error {
    Fatal,
}

#[derive(Debug, Default)]
pub enum ElfFormat {
    #[default]
    Unknown,
    Bit32,
    Bit64,
}

const ELF_FORMAT_OFFSET: usize = 4;
const ELF_ENDIAN_OFFSET: usize = 5;
const ELF_ENDIAN_LITTLE: u8 = 1;
const ELF_ENDIAN_BIG: u8 = 2;

#[derive(Default)]
enum ElfEndian {
    #[default]
    Unknown,
    Little,
    Big,
}

#[derive(Default)]
enum ElfFileType {
    #[default]
    Unknown,
    Relocatable,
    Executable,
    Shared,
    Core,
    Specific,
}

#[derive(Default)]
pub struct ElfId {
    pub magick: u32,
    pub format: ElfFormat,
    pub endian: ElfEndian,
    pub version: u8,
    pub osabi: u8,
    pub abi_version: u8,
}

#[derive(Default)]
pub struct Elf32Header {
    ftype: ElfFileType,
    pub machine: u16,
    is_valid_version: bool,
    entry: u32,
    segmenttbl: u32,
    sectiontbl: u32,
    flags: u32,
    hdrlen: u16,
    segmentlen: u16,
    segmentnr: u16,
    sectionlen: u16,
    sectionnr: u16,
    sectionnametbl: u16,
}

#[derive(Default)]
pub struct Elf64Header {
    ftype: ElfFileType,
    pub machine: u16,
    is_valid_version: bool,
    entry: u64,
    segmenttbl: u64,
    sectiontbl: u64,
    flags: u32,
    hdrlen: u16,
    segmentlen: u16,
    segmentnr: u16,
    sectionlen: u16,
    sectionnr: u16,
    sectionnametbl: u16,
}

pub enum ElfHeader {
    Bit32(Elf32Header),
    Bit64(Elf64Header),
}

impl Default for ElfHeader {
    fn default() -> Self {
        ElfHeader::Bit64(Elf64Header::default())
    }
}

#[derive(Default, Copy, Clone)]
enum ElfSectionType {
    #[default]
    Null,
    Progbits,
    Symbol,
    Str,
    DynSymbol,
    RelocExpl,
    Hash,
    Dynamic,
    Note,
    NoBits,
    RelocImpl,
    Init,
    Fini,
    PreInit,
    Group,
    Unspecified,
    LowOS,
    HighOS,
    LowCpu,
    HighCpu,
}

#[derive(Default, Copy, Clone)]
struct Elf32Section {
    nameidx: u32,
    stype: ElfSectionType,
    flags: u32,
    addr: u32,
    align: u32,
    offset: u32,
    len: u64,
    link: u32,
    info: u32,
    entrylen: u32,
}

#[derive(Default, Copy, Clone)]
struct Elf64Section {
    nameidx: u32,
    stype: ElfSectionType,
    flags: u64,
    addr: u64,
    align: u64,
    offset: u64,
    len: u64,
    link: u32,
    info: u32,
    entrylen: u64,
}

#[derive(Copy, Clone)]
enum ElfSection {
    Bit32(Elf32Section),
    Bit64(Elf64Section),
}

impl Default for ElfSection {
    fn default() -> Self {
        ElfSection::Bit64(Elf64Section::default())
    }
}

#[derive(Default, Copy, Clone)]
enum ElfSegmentType {
    #[default]
    Null,
    Load,
    Dynamic,
    Iterpreter,
    Note,
    Unspecified,
    Segment,
    ThreadLocalStorage,
    LowOS,
    HighOS,
    LowCpu,
    HighCpu,
}

#[derive(Copy, Clone)]
struct Elf32Segment {
    segtype: ElfSegmentType,
    flags: u32,
    offset: u32,
    vaddr: u32,
    paddr: u32,
    filelen: u32,
    memlen: u32,
    align: u32,
}

#[derive(Default, Copy, Clone)]
struct Elf64Segment {
    segtype: ElfSegmentType,
    flags: u32,
    offset: u64,
    vaddr: u64,
    paddr: u64,
    filelen: u64,
    memlen: u64,
    align: u64,
}

#[derive(Copy, Clone)]
enum ElfSegment {
    Bit32(Elf32Segment),
    Bit64(Elf64Segment),
}

impl ElfSegment {
    fn new() -> Self {
        ElfSegment::Bit64(Elf64Segment {
            segtype: ElfSegmentType::Null,
            flags: 0,
            offset: 0,
            vaddr: 0,
            paddr: 0,
            filelen: 0,
            memlen: 0,
            align: 0,
        })
    }
}

impl Default for ElfSegment {
    fn default() -> Self {
        ElfSegment::Bit64(Elf64Segment::default())
    }
}

pub struct ElfParser<L>
where L: fmt::Write {
    logger: L,
    id: ElfId,
    hdr: ElfHeader,
}

impl<L> ElfParser<L>
where L: fmt::Write {
    pub fn new(logger: L) -> Self {
        Self {
            logger: logger,
            id: ElfId::default(),
            hdr: ElfHeader::default(),
        }
    }

    pub fn pull(&mut self, data: &[u8]) -> Result<(), Error> {
        writeln!(self.logger, "trying to pull data from stream");

        self.id.magick = u32::from_be_bytes(data[0..4].try_into().unwrap());

        self.id.format = match data[ELF_FORMAT_OFFSET] {
            ELF_FORMAT_32BIT => ElfFormat::Bit32,
            ELF_FORMAT_64BIT => ElfFormat::Bit64,
            _ => return Err(Error::Fatal),
        };

        self.id.endian = match data[ELF_ENDIAN_OFFSET] {
            ELF_ENDIAN_LITTLE => ElfEndian::Little,
            ELF_ENDIAN_BIG => ElfEndian::Big,
            _ => return Err(Error::Fatal),
        };

        Ok(())
    }

    // fn get_magick(&self) -> u32 {
    //     let slice = &self.id[..];
    //     let (raw, _) = slice.split_at(4);
    //     let magick = <[u8; 4]>::try_from(raw).unwrap();
    //     u32::from_be_bytes(magick)
    // }

    pub fn get_hdr(&self) -> &ElfHeader {
        &self.hdr
    }

    pub fn get_id(&self) -> &ElfId {
        &self.id
    }

    // pub fn check_hdr(&self) -> bool {
    //     let magick = self.get_magick();
    //     magick == ELF_MAGICK
    // }
}
