use std::error;
use std::fmt::Display;
use std::fs;
use std::mem;
use std::path::PathBuf;

pub fn load(file: PathBuf) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let file = fs::read(file)?;

    let ident: &[u8; 0x10] = file[..mem::size_of::<Ident>()].try_into()?;
    let ident: Ident = ident.into();

    println!("ident = {ident}");

    let header: Header = match ident.class() {
        0x34 => {
            let header: &[u8; 0x34] = file[..0x34].try_into()?;
            header.into()
        }
        0x40 => {
            let header: &[u8; 0x40] = file[..0x40].try_into()?;
            header.into()
        }
        _ => unreachable!(),
    };

    println!("header = {header:?}");

    Ok(file)
}

#[derive(Debug)]
struct Ident {
    ident: [u8; 0x10],
}

impl From<&[u8; 0x10]> for Ident {
    fn from(ident: &[u8; 16]) -> Self {
        Self { ident: *ident }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n       magic: {:02X} {:02X} {:02X} {:02X}\n",
            self.ident[0x00], self.ident[0x01], self.ident[0x02], self.ident[0x03]
        )?;
        write!(
            f,
            "       class: {}-bits ({:02X})\n",
            self.class(),
            self.ident[0x04]
        )?;
        let endian = match self.ident[0x05] {
            0x01 => "little",
            0x02 => "big",
            _ => unreachable!(),
        };
        write!(
            f,
            "        data: {endian}-endian ({:02X})\n",
            self.ident[0x05]
        )?;
        write!(f, "     version: {}\n", self.ident[0x06])?;
        let abi = match self.ident[0x07] {
            0x00 => "System V",
            0x01 => "HP-UX",
            0x02 => "NetBSD",
            0x03 => "Linux",
            0x04 => "GNU Hurd",
            0x06 => "Solaris",
            0x07 => "AIX (Monterey)",
            0x08 => "IRIX",
            0x09 => "FreeBSD",
            0x0A => "Tru64",
            0x0B => "Novell Modesto",
            0x0C => "OpenBSD",
            0x0D => "OpenVMS",
            0x0E => "NonStop Kernel",
            0x0F => "AROS",
            0x10 => "FenixOS",
            0x11 => "Nuxi CloudABI",
            0x12 => "Stratus Technologies OpenVOS",
            _ => unreachable!(),
        };
        write!(f, "         abi: {abi} ({:02X})\n", self.ident[0x07])?;
        write!(f, " abi version: {:02X}\n", self.ident[0x08])
    }
}

impl Ident {
    fn class(&self) -> u8 {
        match self.ident[0x04] {
            0x01 => 0x34,
            0x02 => 0x40,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Header {
    header: [u8; 0x40],
}

impl From<&[u8; 0x40]> for Header {
    fn from(header: &[u8; 0x40]) -> Self {
        Self { header: *header }
    }
}

impl From<&[u8; 0x34]> for Header {
    fn from(header: &[u8; 0x34]) -> Self {
        let mut h: [u8; 0x40] = todo!();
        h[..0x34].copy_from_slice(header);

        Self { header: h }
    }
}
