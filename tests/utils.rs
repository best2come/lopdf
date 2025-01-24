use lopdf::{Document, IncrementalDocument, Result};
use std::fmt::Display;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[allow(dead_code)]
pub fn load_document<P>(path: P) -> Result<Document>
where
    P: AsRef<Path> + Display,
{
    #[cfg(feature = "async")]
    let doc = tokio::runtime::Runtime::new()?
        .block_on(async move { Document::load(&path).await.expect(&*format!("Failed to load {}", path)) });
    let stop = Arc::new(AtomicBool::new(false));
    #[cfg(not(feature = "async"))]
    let doc = Document::load(path, stop)?;

    Ok(doc)
}

#[allow(dead_code)]
pub fn load_incremental_document<P>(path: P) -> Result<IncrementalDocument>
where
    P: AsRef<Path> + Display,
{
    #[cfg(feature = "async")]
    let doc = tokio::runtime::Runtime::new()?.block_on(async move {
        IncrementalDocument::load(&path)
            .await
            .expect(&*format!("Failed to load {}", path))
    });
    let stop = Arc::new(AtomicBool::new(false));
    #[cfg(not(feature = "async"))]
    let doc = IncrementalDocument::load(path, stop)?;

    Ok(doc)
}
