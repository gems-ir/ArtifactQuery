# ArtifactQuery

Determine the type of a forensic artifact from its first bytes (up to 32), using the
signatures in `magic.json` (`??` = any byte, first match wins).

```rust
let query = artifact_query::ArtifactQuery::new()?;
query.from_path("System.evtx")?;          // Some("evtx")
query.from_bytes(b"regf....");            // Some("registry")
```

`from_reader` restores the reader position. Files shorter than a signature simply do not
match it. Library users can skip the CLI dependencies:

```toml
artifact_query = { git = "https://github.com/gems-ir/ArtifactQuery", default-features = false }
```
