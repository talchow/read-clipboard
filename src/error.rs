use std::error::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File {path} error ")]
    File {
        path: String,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Failed to create a new  clipboard context: {source}")]
    ClipboardContextError {
        #[from]
        source: Box<dyn Error + Send + Sync + 'static> ,
    }
    
}
 