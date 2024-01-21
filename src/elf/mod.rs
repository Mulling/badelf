mod header;
mod common;

use crate::elf::header::RawIdent;
use crate::elf::header::{Header, Ident};
use std::error;
use std::fs;
use std::path::PathBuf;

pub fn load(file: PathBuf) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let file = fs::read(file)?;

    let ident: Ident =
        RawIdent(<&[u8] as TryInto<&[u8; 0x10]>>::try_into(&file[..0x10])?).try_into()?;

    println!("{ident}");

    let header: Header = (file[..ident.arch()].as_ref(), ident).into();

    println!("{header}");
    println!("{:?}", header.as_ref());

    Ok(file)
}
