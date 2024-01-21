use crate::elf::common::{self, ei};
use std::error;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug)]
pub enum Error {
    Ident(String),
    Header(String),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(err) => {
                write!(f, "malformed ident: {}", err)
            }
            Self::Header(err) => {
                write!(f, "malformed header: {}", err)
            }
        }
    }
}

#[derive(Debug)]
pub struct Ident<'a> {
    ident: &'a [u8; common::NIDENT],
}

#[derive(Debug)]
pub struct RawIdent<'a>(pub &'a [u8; common::NIDENT]);

impl Deref for RawIdent<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> TryFrom<RawIdent<'a>> for Ident<'a> {
    type Error = self::Error;

    // TODO: Improve error messages
    fn try_from(raw: RawIdent<'a>) -> Result<Self, Self::Error> {
        if u32::from_le_bytes(raw[ei::MAG].try_into().unwrap()) != 0x464C457F {
            return Err(Error::Ident("Invalid magic bits".into()));
        }

        let class = raw[ei::CLASS];
        if class == 0 || class > 0x02 {
            return Err(Error::Ident("Invalid class".into()));
        }

        let endianness = raw[ei::DATA];
        if endianness == 0 || endianness > 0x02 {
            return Err(Error::Ident("Invalid endianness".into()));
        }

        let version = raw[ei::VERSION];
        if version != 0x01 {
            return Err(Error::Ident("Invalid version".into()));
        }

        let abi = raw[ei::OSABI];
        if abi > 0x12 {
            return Err(Error::Ident("Invalid abi".into()));
        }

        // TODO: validate ABI version

        Ok(Self { ident: raw.0 })
    }
}

impl Deref for Ident<'_> {
    type Target = [u8; common::NIDENT];

    fn deref(&self) -> &Self::Target {
        self.ident
    }
}

// TODO: Refator this
impl Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "     |  Header   |\n")?;
        write!(
            f,
            "0x00 | {:02X}{:02X} {:02X}{:02X} |   magic: .ELF\n",
            self[0x00], self[0x01], self[0x02], self[0x03]
        )?;

        write!(
            f,
            "{2:#04X} |        {1:02X} |   class: {0}-bits\n",
            self.arch(),
            self.class(),
            ei::CLASS
        )?;

        write!(
            f,
            "{2:#04X} |        {1:02X} |    data: {0}\n",
            common::ei_data_name(self.endianness()),
            self.endianness(),
            ei::DATA,
        )?;

        write!(
            f,
            "{1:#04X} |        {0:02X} | version: {0}\n",
            self.version(),
            ei::VERSION,
        )?;

        write!(
            f,
            "{2:#04X} |        {1:02X} |     abi: {0}\n",
            common::ei_abi_name(self.abi()),
            self.abi(),
            ei::OSABI,
        )?;

        write!(
            f,
            "{1:#04X} |        {0:02X} | abi ver: {0}",
            self.abi_version(),
            ei::ABIVERSION,
        )
    }
}

impl Ident<'_> {
    pub fn arch(&self) -> usize {
        match self[ei::CLASS] {
            0x01 => 0x34,
            0x02 => 0x40,
            _ => unreachable!(),
        }
    }

    pub fn class(&self) -> u8 {
        self[ei::CLASS]
    }

    pub fn endianness(&self) -> u8 {
        self[ei::DATA]
    }

    pub fn version(&self) -> u8 {
        self[ei::VERSION]
    }

    pub fn abi(&self) -> u8 {
        self[ei::OSABI]
    }

    pub fn abi_version(&self) -> u8 {
        self[ei::ABIVERSION]
    }
}

#[derive(Debug)]
pub struct Header<'a> {
    header: *const u8,
    ident: Ident<'a>,
}

impl<'a> From<(&[u8], Ident<'a>)> for Header<'a> {
    fn from((header, ident): (&[u8], Ident<'a>)) -> Self {
        Self {
            header: header.as_ptr(),
            ident,
        }
    }
}

impl Deref for Header<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.header, self.ident.arch()) }
    }
}

impl Display for Header<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "type={:#04X}\n", self.r#type())?;
        write!(f, "machine={:#04X}\n", self.machine())?;
        write!(f, "machine={}\n", common::machine_name(self.machine()))?;
        write!(f, "version={}\n", self.version())?;
        write!(f, "entry={:#08X}\n", self.entry())?;
        write!(f, "phoff={:#08X}\n", self.phoff())?;
        write!(f, "shoff={:#08X}\n", self.shoff())?;
        write!(f, "flags={:}\n", self.flags())?;
        write!(f, "ehsize={:}\n", self.ehsize())?;
        write!(f, "phentzise={:}\n", self.phentzise())?;
        write!(f, "phnum={:}\n", self.phnum())?;
        write!(f, "shentsize={:}\n", self.shentsize())?;
        write!(f, "shnum={:}\n", self.shnum())?;
        write!(f, "shstrndx={:}\n", self.shstrndx())
    }
}

impl Header<'_> {
    fn r#type(&self) -> u16 {
        if self.ident.endianness() == 0x01 {
            u16::from_le_bytes(self[0x10..=0x11].try_into().unwrap())
        } else {
            u16::from_be_bytes(self[0x10..=0x11].try_into().unwrap())
        }
    }

    fn machine(&self) -> u16 {
        if self.ident.endianness() == 0x01 {
            u16::from_le_bytes(self[0x12..=0x13].try_into().unwrap())
        } else {
            u16::from_be_bytes(self[0x12..=0x13].try_into().unwrap())
        }
    }

    fn version(&self) -> u32 {
        if self.ident.endianness() == 0x01 {
            u32::from_le_bytes(self[0x14..=0x17].try_into().unwrap())
        } else {
            u32::from_be_bytes(self[0x14..=0x17].try_into().unwrap())
        }
    }

    fn entry(&self) -> u64 {
        if self.ident.endianness() == 0x01 {
            if self.ident.class() == 0x01 {
                u64::from_le_bytes(self[0x18..=0x1B].try_into().unwrap())
            } else {
                u64::from_le_bytes(self[0x18..=0x1F].try_into().unwrap())
            }
        } else {
            if self.ident.class() == 0x01 {
                u64::from_le_bytes(self[0x18..=0x1B].try_into().unwrap())
            } else {
                u64::from_be_bytes(self[0x18..=0x1F].try_into().unwrap())
            }
        }
    }

    fn phoff(&self) -> u64 {
        if self.ident.endianness() == 0x01 {
            if self.ident.class() == 0x01 {
                u64::from_le_bytes(self[0x1C..=0x1F].try_into().unwrap())
            } else {
                u64::from_le_bytes(self[0x20..=0x27].try_into().unwrap())
            }
        } else {
            if self.ident.class() == 0x01 {
                u64::from_le_bytes(self[0x1C..=0x1F].try_into().unwrap())
            } else {
                u64::from_be_bytes(self[0x20..=0x27].try_into().unwrap())
            }
        }
    }

    fn shoff(&self) -> u64 {
        if self.ident.endianness() == 0x01 {
            if self.ident.class() == 0x01 {
                u64::from_le_bytes(self[0x20..=0x23].try_into().unwrap())
            } else {
                u64::from_le_bytes(self[0x28..=0x2F].try_into().unwrap())
            }
        } else {
            if self.ident.class() == 0x01 {
                u64::from_le_bytes(self[0x20..=0x23].try_into().unwrap())
            } else {
                u64::from_be_bytes(self[0x28..=0x2F].try_into().unwrap())
            }
        }
    }

    fn flags(&self) -> u32 {
        let start = match self.ident.class() {
            0x01 => 0x24,
            0x02 => 0x30,
            _ => unreachable!(),
        };

        let end = start + 0x04;

        if self.ident.endianness() == 0x01 {
            u32::from_le_bytes(self[start..end].try_into().unwrap())
        } else {
            u32::from_be_bytes(self[start..end].try_into().unwrap())
        }
    }

    fn ehsize(&self) -> u16 {
        let start = match self.ident.class() {
            0x01 => 0x28,
            0x02 => 0x34,
            _ => unreachable!(),
        };

        let end = start + 0x02;

        if self.ident.endianness() == 0x01 {
            u16::from_le_bytes(self[start..end].try_into().unwrap())
        } else {
            u16::from_be_bytes(self[start..end].try_into().unwrap())
        }
    }

    fn phentzise(&self) -> u16 {
        let start = match self.ident.class() {
            0x01 => 0x2A,
            0x02 => 0x36,
            _ => unreachable!(),
        };

        let end = start + 0x02;

        if self.ident.endianness() == 0x01 {
            u16::from_le_bytes(self[start..end].try_into().unwrap())
        } else {
            u16::from_be_bytes(self[start..end].try_into().unwrap())
        }
    }

    fn phnum(&self) -> u16 {
        let start = match self.ident.class() {
            0x01 => 0x2C,
            0x02 => 0x38,
            _ => unreachable!(),
        };

        let end = start + 0x02;

        if self.ident.endianness() == 0x01 {
            u16::from_le_bytes(self[start..end].try_into().unwrap())
        } else {
            u16::from_be_bytes(self[start..end].try_into().unwrap())
        }
    }

    fn shentsize(&self) -> u16 {
        let start = match self.ident.class() {
            0x01 => 0x2E,
            0x02 => 0x3A,
            _ => unreachable!(),
        };

        let end = start + 0x02;

        if self.ident.endianness() == 0x01 {
            u16::from_le_bytes(self[start..end].try_into().unwrap())
        } else {
            u16::from_be_bytes(self[start..end].try_into().unwrap())
        }
    }

    fn shnum(&self) -> u16 {
        let start = match self.ident.class() {
            0x01 => 0x30,
            0x02 => 0x3C,
            _ => unreachable!(),
        };

        let end = start + 0x02;

        if self.ident.endianness() == 0x01 {
            u16::from_le_bytes(self[start..end].try_into().unwrap())
        } else {
            u16::from_be_bytes(self[start..end].try_into().unwrap())
        }
    }

    fn shstrndx(&self) -> u16 {
        let start = match self.ident.class() {
            0x01 => 0x32,
            0x02 => 0x3E,
            _ => unreachable!(),
        };

        let end = start + 0x02;

        if self.ident.endianness() == 0x01 {
            u16::from_le_bytes(self[start..end].try_into().unwrap())
        } else {
            u16::from_be_bytes(self[start..end].try_into().unwrap())
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Ident, RawIdent};

    macro_rules! ident_test {
        ($($test_name:ident: $input:expr,)*) => {$(
            #[test]
            fn $test_name() {
                let ident: Result<Ident, _> = RawIdent(&$input).try_into();

               if let Ok(ident) = ident {
                    panic!(
                        "expected invalid ident for {:?} but got a valid one",
                        ident.ident
                    );
                }
            })*
        }
    }

    ident_test! {
        wrong_magic_bits_1: [0x7E, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_magic_bits_2: [0x7F, 0x44, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_magic_bits_3: [0x7F, 0x45, 0x4B, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_magic_bits_4: [0x7F, 0x45, 0x4C, 0x45, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_class_1:      [0x7F, 0x45, 0x4C, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_class_2:      [0x7F, 0x45, 0x4C, 0x46, 0x03, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_endianess_1:  [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_endianess_2:  [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_version_1:    [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_version_2:    [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x02, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        wrong_abi:          [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x02, 0x01, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}
