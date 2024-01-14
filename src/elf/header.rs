use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug)]
pub struct Ident<'a> {
    ident: &'a [u8; 0x10],
}

// impl<'a> TryFrom<&'a [u8; 0x10]> for Ident<'a> {}
impl<'a> From<&'a [u8; 0x10]> for Ident<'a> {
    fn from(ident: &'a [u8; 0x10]) -> Self {
        Self { ident }
    }
}

impl Deref for Ident<'_> {
    type Target = [u8; 0x10];

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

        let abi = match self[0x07] {
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

        write!(
            f,
            r#"
ident {{
       magic: {:02X} {:02X} {:02X} {:02X}
       class: {}-bits ({:02X})
        data: {endian}-endian ({:02X})
     version: {}
         abi: {abi} ({:02X})
 abi version: {:02X}self[0x08])
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
