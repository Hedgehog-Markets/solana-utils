use std::str::{self, FromStr};

mod error;
mod print;

use goblin::elf::Elf;

pub use self::error::Error;

/// Constant for the beginning of the security.txt file.
pub const SECURITY_TXT_BEGIN: &str = "=======BEGIN SECURITY.TXT V1=======\0";
/// Constant for the end of the security.txt file.
pub const SECURITY_TXT_END: &str = "=======END SECURITY.TXT V1=======\0";

#[derive(Debug)]
pub struct SecurityTxt {
    pub name: String,
    pub project_url: String,
    pub contacts: Vec<Contact>,
    pub policy: String,
    pub preferred_languages: Vec<String>,
    pub encryption: Option<String>,
    pub source_code: Option<String>,
    pub source_release: Option<String>,
    pub source_revision: Option<String>,
    pub auditors: Vec<String>,
    pub acknowledgements: Option<String>,
    pub expiry: Option<String>,
}

#[derive(Debug)]
pub struct Contact {
    pub kind: ContactKind,
    pub value: String,
}

#[derive(Debug)]
pub enum ContactKind {
    Email,
    Discord,
    Telegram,
    Twitter,
    Link,
    Other,
}

impl FromStr for Contact {
    type Err = Error;

    fn from_str(contact: &str) -> Result<Self, Self::Err> {
        let (kind, value) = contact
            .split_once(":")
            .ok_or_else(|| Error::InvalidContact { contact: contact.to_owned() })?;

        let kind = match kind.trim() {
            "email" => ContactKind::Email,
            "discord" => ContactKind::Discord,
            "telegram" => ContactKind::Telegram,
            "twitter" => ContactKind::Twitter,
            "link" => ContactKind::Link,
            "other" => ContactKind::Other,
            _ => return Err(Error::InvalidContact { contact: contact.to_owned() }),
        };
        let value = value.trim();

        Ok(Self { kind, value: value.to_owned() })
    }
}

struct Parser<'a> {
    data: &'a [u8],
    iter: memchr::Memchr<'a>,
    offset: usize,
}

impl<'a> Parser<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, iter: memchr::memchr_iter(0, data), offset: 0 }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<(&'a str, &'a str), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.offset;

        if offset == self.data.len() {
            return None;
        }

        let split = self.iter.next()?;
        let field = &self.data[offset..split];

        let field = match str::from_utf8(field) {
            Ok(field) => field,
            Err(source) => {
                return Some(Err(Error::InvalidField { field: field.to_owned(), source }))
            }
        };

        let offset = split + 1;

        let split = match self.iter.next() {
            Some(split) => split,
            None => return Some(Err(Error::MissingValue { field: field.to_owned() })),
        };
        let value = &self.data[offset..split];

        let value = match str::from_utf8(value) {
            Ok(value) => value,
            Err(source) => {
                return Some(Err(Error::InvalidValue {
                    field: field.to_owned(),
                    value: value.to_owned(),
                    source,
                }))
            }
        };

        self.offset = split + 1;

        Some(Ok((field, value)))
    }
}

fn parse(data: &[u8]) -> Result<SecurityTxt, Error> {
    let mut name = None;
    let mut project_url = None;
    let mut policy = None;
    let mut contacts = None;

    let mut preferred_languages = None;
    let mut encryption = None;
    let mut source_code = None;
    let mut source_release = None;
    let mut source_revision = None;
    let mut auditors = None;
    let mut acknowledgements = None;
    let mut expiry = None;

    for entry in Parser::new(data) {
        let (field, value) = entry?;

        let f = match field {
            "name" => &mut name,
            "project_url" => &mut project_url,
            "policy" => &mut policy,
            "contacts" => &mut contacts,
            "preferred_languages" => &mut preferred_languages,
            "encryption" => &mut encryption,
            "source_code" => &mut source_code,
            "source_release" => &mut source_release,
            "source_revision" => &mut source_revision,
            "auditors" => &mut auditors,
            "acknowledgements" => &mut acknowledgements,
            "expiry" => &mut expiry,
            _ => return Err(Error::UnknownField { field: field.to_owned() }),
        };

        if f.is_some() {
            return Err(Error::DuplicateField { field: field.to_owned() });
        }
        *f = Some(value);
    }

    macro_rules! require {
        ($field:ident) => {
            $field.ok_or(Error::RequiredField { field: stringify!($field) })
        };
    }

    let name = require!(name)?;
    let project_url = require!(project_url)?;
    let policy = require!(policy)?;
    let contacts = require!(contacts)?;

    let value_list = |s: &str| s.split(',').map(|s| s.trim().to_owned()).collect::<Vec<_>>();

    let preferred_languages = preferred_languages.map(value_list).unwrap_or_default();
    let auditors = auditors.map(value_list).unwrap_or_default();

    let contacts = contacts.split(',').map(|s| s.trim().parse()).collect::<Result<Vec<_>, _>>()?;

    Ok(SecurityTxt {
        name: name.to_owned(),
        project_url: project_url.to_owned(),
        contacts,
        policy: policy.to_owned(),
        preferred_languages,
        encryption: encryption.map(ToOwned::to_owned),
        source_code: source_code.map(ToOwned::to_owned),
        source_release: source_release.map(ToOwned::to_owned),
        source_revision: source_revision.map(ToOwned::to_owned),
        auditors,
        acknowledgements: acknowledgements.map(ToOwned::to_owned),
        expiry: expiry.map(ToOwned::to_owned),
    })
}

fn find_from_elf(program_data: &[u8]) -> Option<&[u8]> {
    const SECTION_NAME: &str = ".security.txt";

    let elf = Elf::parse(program_data).ok()?;

    let sh = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name) == Some(SECTION_NAME))?;

    // Get offset of section data.
    let offset = sh.sh_offset as usize;

    // Get offset & len of the compressed IDL bytes.
    let data_offset = &program_data[(offset + 4)..(offset + 8)];
    let data_len = &program_data[(offset + 8)..(offset + 16)];

    let data_offset = u32::from_le_bytes(data_offset.try_into().unwrap()) as usize;
    let data_len = u64::from_le_bytes(data_len.try_into().unwrap()) as usize;

    let data = &program_data[data_offset..(data_offset + data_len)];
    let data = data
        .strip_prefix(SECURITY_TXT_BEGIN.as_bytes())?
        .strip_suffix(SECURITY_TXT_END.as_bytes())?;

    Some(data)
}

fn find_from_markers(program_data: &[u8]) -> Result<&[u8], Error> {
    let data = match memchr::memmem::find(program_data, SECURITY_TXT_BEGIN.as_bytes()) {
        Some(start) => &program_data[(start + SECURITY_TXT_BEGIN.len())..],
        None => return Err(Error::StartNotFound),
    };
    let data = match memchr::memmem::find(data, SECURITY_TXT_END.as_bytes()) {
        Some(end) => &data[..end],
        None => return Err(Error::EndNotFound),
    };
    Ok(data)
}

/// Parses embedded security.txt from program binary.
pub fn parse_from_program(program_data: &[u8]) -> Result<SecurityTxt, Error> {
    let data = match find_from_elf(program_data) {
        Some(data) => data,
        None => find_from_markers(program_data)?,
    };
    parse(data)
}
