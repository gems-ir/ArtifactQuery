pub mod query;

pub use query::{ArtifactQuery, QueryError, Signature, HEADER_LEN};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read, Seek};

    fn padded(prefix: &[u8]) -> Vec<u8> {
        let mut data = prefix.to_vec();
        data.resize(64, 0x11);
        data
    }

    #[test]
    fn every_embedded_artifact_is_recognised() {
        let aq = ArtifactQuery::new().unwrap();
        let samples: &[(&str, &[u8])] = &[
            ("evtx", b"ElfFile\0"),
            ("lnk", b"L\0\0\0"),
            ("prefetch", b"MAM\x04"),
            ("prefetch", b"\x1e\0\0\0SCCA"),
            ("mft", b"FILE"),
            ("mft", b"BAAD"),
            ("registry", b"regf"),
            ("notepad10", b"NP\0\0"),
            ("notepad11", b"NP\0\x01"),
        ];
        for (name, prefix) in samples {
            assert_eq!(aq.from_bytes(&padded(prefix)), Some(*name), "{name}");
        }
        for name in aq.names() {
            assert!(
                samples.iter().any(|(n, _)| *n == name),
                "no test for {name}"
            );
        }
    }

    #[test]
    fn unknown_and_short_inputs() {
        let aq = ArtifactQuery::new().unwrap();
        assert_eq!(aq.from_bytes(&padded(b"%PDF-1.7")), None);
        assert_eq!(aq.from_bytes(b""), None);
        // Shorter than the signature: no match, no error.
        assert_eq!(aq.from_bytes(b"Elf"), None);
        assert_eq!(
            aq.from_reader(&mut Cursor::new(b"reg".to_vec())).unwrap(),
            None
        );
        // Exactly the signature length is enough.
        assert_eq!(aq.from_bytes(b"regf"), Some("registry"));
    }

    #[test]
    fn wildcard_bytes_match_anything() {
        let aq = ArtifactQuery::new().unwrap();
        assert_eq!(
            aq.from_bytes(&padded(b"\x11\x22\x33\x44SCCA")),
            Some("prefetch")
        );
        assert_eq!(aq.from_bytes(&padded(b"\x11\x22\x33\x44SCCB")), None);
    }

    #[test]
    fn reader_position_is_restored() {
        let aq = ArtifactQuery::new().unwrap();
        let mut data = b"xxxx".to_vec();
        data.extend(padded(b"regf"));
        let mut cursor = Cursor::new(data);
        cursor.seek(std::io::SeekFrom::Start(4)).unwrap();
        assert_eq!(
            aq.from_reader(&mut cursor).unwrap().as_deref(),
            Some("registry")
        );
        let mut magic = [0u8; 4];
        cursor.read_exact(&mut magic).unwrap();
        assert_eq!(&magic, b"regf");
    }

    #[test]
    fn invalid_signatures_are_rejected() {
        for bad in ["", "?? ??", "4", "zz", "4d4d", &"00 ".repeat(33)] {
            let json = format!(r#"{{"magics":[{{"name":"x","signature":"{bad}"}}]}}"#);
            assert!(
                ArtifactQuery::from_json(json.as_bytes()).is_err(),
                "{bad:?}"
            );
        }
        let json = br#"{"magics":[{"name":"x","signature":"4D ?? 5a"}]}"#;
        let aq = ArtifactQuery::from_json(json).unwrap();
        assert_eq!(
            aq.signatures()[0].pattern,
            vec![Some(0x4d), None, Some(0x5a)]
        );
    }

    /// Original sample tests; the samples are not versioned (see .gitignore).
    #[test]
    fn local_samples_when_present() {
        let aq = ArtifactQuery::new().unwrap();
        for (path, expected) in [
            ("samples/System.evtx", "evtx"),
            ("samples/File_explorer.lnk", "lnk"),
            ("samples/MSEDGE.EXE-37D25F9C.pf", "prefetch"),
            ("samples/KAPE.EXE-07476F82-decompressed", "prefetch"),
        ] {
            if std::path::Path::new(path).exists() {
                assert_eq!(
                    aq.from_path(path).unwrap().as_deref(),
                    Some(expected),
                    "{path}"
                );
            }
        }
    }
}
