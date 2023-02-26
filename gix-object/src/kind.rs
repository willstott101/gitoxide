use std::fmt;

use crate::Kind;

/// The Error used in [`Kind::from_bytes()`].
#[derive(Debug, Clone, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Unknown object kind: {kind:?}")]
    InvalidObjectKind { kind: bstr::BString },
}

impl Kind {
    /// Parse a `Kind` from its serialized loose git objects.
    pub fn from_bytes(s: &[u8]) -> Result<Kind, Error> {
        Ok(match s {
            b"tree" => Kind::Tree,
            b"blob" => Kind::Blob,
            b"commit" => Kind::Commit,
            b"tag" => Kind::Tag,
            _ => return Err(Error::InvalidObjectKind { kind: s.into() }),
        })
    }

    /// Return the name of `self` for use in serialized loose git objects.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Kind::Tree => b"tree",
            Kind::Commit => b"commit",
            Kind::Blob => b"blob",
            Kind::Tag => b"tag",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(std::str::from_utf8(self.as_bytes()).expect("Converting Kind name to utf8"))
    }
}
