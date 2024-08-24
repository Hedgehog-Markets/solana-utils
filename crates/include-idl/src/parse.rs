use std::io::{self, Read};

use flate2::bufread::ZlibDecoder;
use goblin::elf::Elf;
use serde_json::Value;
use thiserror::Error;

/// The kind of IDL.
#[derive(Clone, Debug)]
pub enum IdlKind {
    /// Anchor style IDL.
    Anchor,
    /// Kinobi tree IDL.
    Kinobi,
}

impl IdlKind {
    fn section_name(&self) -> &'static str {
        match self {
            IdlKind::Anchor => ".solana.idl",
            IdlKind::Kinobi => ".kinobi.idl",
        }
    }
}

/// This type represents all possible errors that can occur when reading IDL
/// from a program binary.
#[derive(Debug, Error)]
pub enum Error {
    /// Parsing the ELF program binary failed.
    #[error("failed to parse program elf")]
    ParseElf(#[source] goblin::error::Error),
    /// The IDL section could not be found.
    #[error("failed to find '{}' section", .0.section_name())]
    MissingIdlSection(IdlKind),
    /// Decompressing the IDL failed.
    #[error("failed to decompress idl")]
    Decompress(#[source] io::Error),
    /// Parsing the IDL failed.
    #[error("invalid idl json")]
    Json(#[source] serde_json::Error),
}

/// Parses embedded IDL from program binary.
pub fn parse_from_program(program_data: &[u8], kind: IdlKind) -> Result<Value, Error> {
    let elf = Elf::parse(program_data).map_err(Error::ParseElf)?;

    let section_name = kind.section_name();
    let sh = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name) == Some(section_name))
        .ok_or(Error::MissingIdlSection(kind))?;

    // Get offset of section data.
    let offset = sh.sh_offset as usize;

    // Get offset & len of the compressed IDL bytes.
    let data_offset = &program_data[(offset + 4)..(offset + 8)];
    let data_len = &program_data[(offset + 8)..(offset + 16)];

    let data_offset = u32::from_le_bytes(data_offset.try_into().unwrap()) as usize;
    let data_len = u64::from_le_bytes(data_len.try_into().unwrap()) as usize;

    let compressed_data = &program_data[data_offset..(data_offset + data_len)];

    let mut decoder = ZlibDecoder::new(compressed_data);
    let mut data = Vec::new();

    decoder.read_to_end(&mut data).map_err(Error::Decompress)?;

    serde_json::from_slice(&data).map_err(Error::Json)
}
