use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use serde::Deserialize;

/// Number of bytes read at the start of a file.
pub const HEADER_LEN: usize = 32;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid magic.json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid signature {signature:?} for {name}: {reason}")]
    Signature {
        name: String,
        signature: String,
        reason: String,
    },
}

pub type Result<T> = std::result::Result<T, QueryError>;

/// One entry of magic.json, compiled once: `None` is a `??` wildcard byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub name: String,
    pub pattern: Vec<Option<u8>>,
}

impl Signature {
    /// Parses `"4d 41 4d 04"` or `"?? ?? ?? ?? 53 43 43 41"`.
    pub fn parse(name: &str, signature: &str) -> Result<Self> {
        let error = |reason: &str| QueryError::Signature {
            name: name.to_string(),
            signature: signature.to_string(),
            reason: reason.to_string(),
        };
        let pattern = signature
            .split_whitespace()
            .map(|byte| match byte {
                "??" => Ok(None),
                hex if hex.len() == 2 => u8::from_str_radix(hex, 16)
                    .map(Some)
                    .map_err(|_| error("bytes must be two hex digits or ??")),
                _ => Err(error("bytes must be two hex digits or ??")),
            })
            .collect::<Result<Vec<_>>>()?;
        if pattern.is_empty() || pattern.len() > HEADER_LEN {
            return Err(error("a signature has between 1 and 32 bytes"));
        }
        if pattern.iter().all(Option::is_none) {
            return Err(error("a signature needs at least one fixed byte"));
        }
        Ok(Self {
            name: name.to_string(),
            pattern,
        })
    }

    /// True if `header` starts with this signature.
    pub fn matches(&self, header: &[u8]) -> bool {
        header.len() >= self.pattern.len()
            && self
                .pattern
                .iter()
                .zip(header)
                .all(|(expected, actual)| expected.is_none_or(|b| b == *actual))
    }
}

pub struct ArtifactQuery {
    signatures: Vec<Signature>,
}

impl ArtifactQuery {
    /// Loads the embedded magic.json.
    pub fn new() -> Result<Self> {
        Self::from_json(include_bytes!("../magic.json"))
    }

    /// Loads another magic.json (same format).
    pub fn from_json(json: &[u8]) -> Result<Self> {
        let magic: Magic = serde_json::from_slice(json)?;
        let signatures = magic
            .magics
            .iter()
            .map(|m| Signature::parse(&m.name, &m.signature))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { signatures })
    }

    /// Signatures in magic.json order (the first match wins).
    pub fn signatures(&self) -> &[Signature] {
        &self.signatures
    }

    /// Distinct artifact names, in magic.json order.
    pub fn names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = Vec::new();
        for s in &self.signatures {
            if !names.contains(&s.name.as_str()) {
                names.push(&s.name);
            }
        }
        names
    }

    pub fn from_path(&self, path: &str) -> Result<Option<String>> {
        let mut file = File::open(path)?;
        self.from_reader(&mut file)
    }

    /// Reads up to 32 bytes from the current position, then restores the position.
    /// A file shorter than a signature does not match it (no error).
    pub fn from_reader<R: Read + Seek>(&self, reader: &mut R) -> Result<Option<String>> {
        let start = reader.stream_position()?;
        let mut header = Vec::with_capacity(HEADER_LEN);
        reader
            .by_ref()
            .take(HEADER_LEN as u64)
            .read_to_end(&mut header)?;
        reader.seek(SeekFrom::Start(start))?;
        Ok(self.from_bytes(&header).map(str::to_string))
    }

    /// Name of the first signature matching the start of `header`.
    pub fn from_bytes(&self, header: &[u8]) -> Option<&str> {
        self.signatures
            .iter()
            .find(|s| s.matches(header))
            .map(|s| s.name.as_str())
    }
}

#[derive(Deserialize)]
struct Magic {
    magics: Vec<MagicEntry>,
}

#[derive(Deserialize)]
struct MagicEntry {
    name: String,
    signature: String,
}
