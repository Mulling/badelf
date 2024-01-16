use std::error;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug)]
pub enum Error {
    InvalidMagicBits(),
    InvliadClass(u8),
    InvalidEndianness(u8),
    InvalidVersion(u8),
    InvalidABI(u8),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMagicBits() => {
                write!(f, "invalid magic bits")
            }
            Self::InvliadClass(class) => {
                write!(f, "invalid class: {class:#04X}")
            }
            Self::InvalidEndianness(end) => {
                write!(f, "invalid endianness: {end:#04X}")
            }
            Self::InvalidVersion(version) => {
                write!(f, "invalid version: {version:#04X}")
            }
            Self::InvalidABI(abi) => {
                write!(f, "invalid abi: {abi:#04X}")
            }
        }
    }
}

const NIDENT: usize = 0x10;
const ABIS: [&'static str; 0x12] = [
    "System V",
    "HP-UX",
    "NetBSD",
    "Linux",
    "GNU Hurd",
    "Solaris",
    "AIX (Monterey)",
    "IRIX",
    "FreeBSD",
    "Tru64",
    "Novell Modesto",
    "OpenBSD",
    "OpenVMS",
    "NonStop Kernel",
    "AROS",
    "FenixOS",
    "Nuxi CloudABI",
    "Stratus Technologies OpenVOS",
];

#[derive(Debug)]
pub struct Ident<'a> {
    ident: &'a [u8; NIDENT],
}

#[derive(Debug)]
pub struct RawIdent<'a>(pub &'a [u8; NIDENT]);

impl Deref for RawIdent<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> TryFrom<RawIdent<'a>> for Ident<'a> {
    type Error = Error; // TODO: Error

    fn try_from(raw: RawIdent<'a>) -> Result<Self, Self::Error> {
        if raw[0x00] != 0x7f || raw[0x01] != 0x45 || raw[0x02] != 0x4C || raw[0x03] != 0x046 {
            return Err(Error::InvalidMagicBits());
        }

        if raw[0x04] == 0 || raw[4] > 0x02 {
            return Err(Error::InvliadClass(raw[0x04]));
        }

        if raw[0x05] == 0 || raw[5] > 0x02 {
            return Err(Error::InvalidEndianness(raw[0x05]));
        }

        if raw[0x06] != 0x01 {
            return Err(Error::InvalidVersion(raw[0x06]));
        }

        if raw[0x07] > 0x12 {
            return Err(Error::InvalidABI(raw[0x07]));
        }

        Ok(Self { ident: raw.0 })
    }
}

impl Deref for Ident<'_> {
    type Target = [u8; NIDENT];

    fn deref(&self) -> &Self::Target {
        self.ident
    }
}

impl Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let endian = match self[0x05] {
            0x01 => "little",
            0x02 => "big",
            _ => unreachable!(),
        };

        write!(
            f,
            r#"
ident {{
       magic: {:02X} {:02X} {:02X} {:02X}
       class: {}-bits ({:02X})
        data: {endian}-endian ({:02X})
     version: {}
         abi: {} ({:02X})
 abi version: {:02X}
}}
            "#,
            // magic
            self[0x00],
            self[0x01],
            self[0x02],
            self[0x03],
            // class
            self.class(),
            self[0x04],
            // data
            self[0x05],
            // version
            self[0x06],
            // abi
            ABIS[self[0x07] as usize],
            self[0x07],
            // abi version
            self[0x08]
        )
    }
}

impl Ident<'_> {
    pub fn class(&self) -> u8 {
        match self[0x04] {
            0x01 => 0x34,
            0x02 => 0x40,
            _ => unreachable!(),
        }
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
        unsafe { std::slice::from_raw_parts(self.header, self.ident.class() as usize) }
    }
}

#[cfg(test)]
mod test {
    use super::{Ident, RawIdent, NIDENT};

    #[test]
    fn invalid_ident() {
        let idents: [[u8; NIDENT]; 0x0B] = [
            [126, 69, 76, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 68, 76, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 69, 75, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 69, 76, 69, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 69, 76, 70, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 69, 76, 70, 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 69, 76, 70, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 69, 76, 70, 2, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 69, 76, 70, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 69, 76, 70, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [127, 69, 76, 70, 2, 2, 1, 0x13, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        idents.iter().for_each(|ident| {
            let ident: Result<Ident, _> = RawIdent(&ident).try_into();

            if let Ok(ident) = ident {
                panic!(
                    "expected invalid ident for {:?} but got a valid one",
                    ident.ident
                );
            }
        });
    }
}
