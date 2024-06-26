use std::fs::File;
use std::io;
use log::info;
use crate::services::file_api::compressor::Compressor;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

struct ZipCompressor;



impl Compressor for ZipCompressor {
    fn compress(&self, files: &[&str]) -> Vec<io::Result<String>> {
        files.iter().map(|&file_path| {
            info!("Compressing file: {}", file_path);
            let compressed_file_name = format!("{}.zip", file_path);
            let file = match File::create(&compressed_file_name) {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut zip = ZipWriter::new(file);
            let options = FileOptions::default().compression_method(CompressionMethod::Stored);

            let start_file_result = zip.start_file(file_path, options);
            if let Err(e) = start_file_result {
                return Err(e.into());  // Convert ZipError to io::Error if needed
            }

            let mut f = match File::open(file_path) {
                Ok(f) => f,
                Err(e) => return Err(e),
            };

            if let Err(e) = io::copy(&mut f, &mut zip) {
                return Err(e);
            }

            match zip.finish() {
                Ok(_) => Ok(compressed_file_name),
                Err(e) => Err(e.into()),  // Convert ZipError to io::Error if needed
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};
    use tempfile::{NamedTempFile, tempdir};
    use zip::read::ZipArchive;

    #[test]
    fn test_compress_and_uncompress_single_file() {
        let compressor = ZipCompressor;
        let dir = tempdir().unwrap(); // Create a temporary directory

        // Create a temporary file within the temporary directory
        let mut temp_file = NamedTempFile::new_in(dir.path()).unwrap();
        writeln!(temp_file.as_file_mut(), "Test content").unwrap();
        let temp_path = temp_file.path().to_str().unwrap();

        // Compress the file
        let results = compressor.compress(&[temp_path]);
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());

        // Verify the compressed file exists
        let compressed_file_path = results[0].as_ref().unwrap();
        assert!(fs::metadata(compressed_file_path).is_ok());

        // Try to open the compressed file as a zip archive
        let file = File::open(compressed_file_path).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        // Check if the archive has exactly one file, and read its content
        assert_eq!(archive.len(), 1);
        let mut file_in_zip = archive.by_index(0).unwrap();
        let mut contents = String::new();
        file_in_zip.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "Test content\n");

        // Cleanup: The tempdir's Drop trait automatically deletes the directory and its contents
        // No need to explicitly delete the compressed file
    }
}
