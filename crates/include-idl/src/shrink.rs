use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde_json::Value;
use thiserror::Error;

/// This type represents all possible errors that can occur when compressing IDL
/// for inclusion inside program binary.
#[derive(Debug, Error)]
pub enum Error {
    /// The IDL could not be read from file.
    #[error("failed to read IDL from file '{path}'")]
    Read { path: PathBuf, source: io::Error },
    /// The compressed IDL could not be written to file.
    #[error("failed to write compressed IDL to file '{path}'")]
    Write { path: PathBuf, source: io::Error },
    /// Parsing and serializing the IDL failed.
    #[error("IDL is not valid JSON")]
    Json(#[source] serde_json::Error),
    /// An error compressing the IDL.
    #[error("failed to compress IDL")]
    Compress(#[source] io::Error),
}

/// Writes compressed IDL to the destination path.
///
/// # Example
///
/// Compress IDL in `build.rs` for inclusion in program.
///
/// ```no_run
/// # use std::path::PathBuf;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let out_dir = std::env::var_os("OUT_DIR")
///     .map(PathBuf::from)
///     .expect("missing 'OUT_DIR' environment variable");
///
/// let idl_src = "/path/to/idl.json";
/// let idl_dst = out_dir.join("solana.idl.zip");
///
/// include_idl::compress_idl(idl_src, &idl_dst)?;
/// #
/// # Ok(())
/// # }
/// ```
pub fn compress_idl<S, D>(idl_src: S, idl_dst: D) -> Result<(), Error>
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    fn inner(idl_src: &Path, idl_dst: &Path) -> Result<(), Error> {
        let json = fs::read(idl_src)
            .map_err(|source| Error::Read { path: idl_src.to_path_buf(), source })?;

        // We parse and serialize the IDL to compact the JSON.
        let idl = serde_json::from_slice::<Value>(&json).map_err(Error::Json)?;
        let compact_json = serde_json::to_string(&idl).map_err(Error::Json)?;

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(compact_json.as_bytes()).map_err(Error::Compress)?;
        let compressed_json = encoder.finish().map_err(Error::Compress)?;

        fs::write(idl_dst, &compressed_json)
            .map_err(|source| Error::Write { path: idl_dst.to_path_buf(), source })?;

        Ok(())
    }
    inner(idl_src.as_ref(), idl_dst.as_ref())
}
