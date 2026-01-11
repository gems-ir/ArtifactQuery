pub mod query;

pub use query::ArtifactQuery;

#[tokio::test]
async fn evtx() {
    let aq = ArtifactQuery::from_path("samples/System.evtx").await.unwrap();
    assert_eq!(Some("evtx".to_string()),aq)
}

#[tokio::test]
async fn lnk() {
    let aq = ArtifactQuery::from_path("samples/File_explorer.lnk").await.unwrap();
    assert_eq!(Some("lnk".to_string()),aq)
}

#[tokio::test]
async fn prefetch() {
    let aq = ArtifactQuery::from_path("samples/MSEDGE.EXE-37D25F9C.pf").await.unwrap();
    assert_eq!(Some("prefetch".to_string()),aq)
}

#[tokio::test]
async fn prefetch_decompressed() {
    let aq = ArtifactQuery::from_path("samples/KAPE.EXE-07476F82-decompressed").await.unwrap();
    assert_eq!(Some("prefetch".to_string()),aq)
}