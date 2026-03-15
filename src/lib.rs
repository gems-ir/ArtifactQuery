pub mod query;

pub use query::ArtifactQuery;

#[tokio::test]
async fn evtx() {
    let aq = ArtifactQuery::new().unwrap();
    let res = aq.from_path("samples/System.evtx").unwrap();
    assert_eq!(Some("evtx".to_string()),res)
}

#[tokio::test]
async fn lnk() {
    let aq = ArtifactQuery::new().unwrap();
    let res = aq.from_path("samples/File_explorer.lnk").unwrap();
    assert_eq!(Some("lnk".to_string()),res)
}

#[tokio::test]
async fn prefetch() {
    let aq = ArtifactQuery::new().unwrap();
    let res = aq.from_path("samples/MSEDGE.EXE-37D25F9C.pf").unwrap();
    assert_eq!(Some("prefetch".to_string()),res)
}

#[tokio::test]
async fn prefetch_decompressed() {
    let aq = ArtifactQuery::new().unwrap();
    let res = aq.from_path("samples/KAPE.EXE-07476F82-decompressed").unwrap();
    assert_eq!(Some("prefetch".to_string()),res)
}