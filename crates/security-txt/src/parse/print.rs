use std::fmt;

use super::{Contact, ContactKind, SecurityTxt};

impl fmt::Display for SecurityTxt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Project URL: {}", self.project_url)?;

        if !self.contacts.is_empty() {
            writeln!(f, "\nContacts:")?;
            for contact in &self.contacts {
                writeln!(f, "  {}", contact)?;
            }
        }

        writeln!(f, "\nPolicy: {}", self.policy)?;

        if !self.preferred_languages.is_empty() {
            writeln!(f, "\nPreferred Languages:")?;
            for languages in &self.preferred_languages {
                writeln!(f, "  {}", languages)?;
            }
        }

        if let Some(source_code) = &self.source_code {
            writeln!(f, "Source code: {}", source_code)?;
        }
        if let Some(source_release) = &self.source_release {
            writeln!(f, "Source release: {}", source_release)?;
        }
        if let Some(source_revision) = &self.source_revision {
            writeln!(f, "Source revision: {}", source_revision)?;
        }

        if let Some(encryption) = &self.encryption {
            writeln!(f, "\nEncryption:")?;
            writeln!(f, "{}", encryption)?;
        }

        if !self.auditors.is_empty() {
            writeln!(f, "\nAuditors:")?;
            for auditor in &self.auditors {
                writeln!(f, "  {}", auditor)?;
            }
        }

        if let Some(acknowledegments) = &self.acknowledgements {
            writeln!(f, "\nAcknowledgements:")?;
            writeln!(f, "{}", acknowledegments)?;
        }

        if let Some(expiry) = &self.expiry {
            writeln!(f, "Expiry: {}", expiry)?;
        }

        Ok(())
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.value)
    }
}

impl fmt::Display for ContactKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self {
            Self::Discord => "Discord",
            Self::Email => "Email",
            Self::Telegram => "Telegram",
            Self::Twitter => "Twitter",
            Self::Link => "Link",
            Self::Other => "Other",
        };
        f.write_str(kind)
    }
}
