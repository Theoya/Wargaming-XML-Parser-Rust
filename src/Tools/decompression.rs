use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;
use zip::ZipArchive;
use anyhow::{Result, Context};

#[derive(Debug)]
pub struct DecompressedFile {
    pub filename: String,
    pub content: String,
}

pub fn decompress_zip_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<DecompressedFile>> {
    let file = File::open(&file_path)
        .with_context(|| format!("Failed to open file: {:?}", file_path.as_ref()))?;
    
    let mut archive = ZipArchive::new(BufReader::new(file))
        .with_context(|| "Failed to read ZIP archive")?;
    
    let mut decompressed_files = Vec::new();
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .with_context(|| format!("Failed to access file at index {}", i))?;
        
        let filename = file.name().to_string();
        
        // Skip directories
        if file.name().ends_with('/') {
            continue;
        }
        
        // Read file content
        let mut content = String::new();
        file.read_to_string(&mut content)
            .with_context(|| format!("Failed to read file content: {}", filename))?;
        
        decompressed_files.push(DecompressedFile {
            filename,
            content,
        });
    }
    
    Ok(decompressed_files)
}

pub fn decompress_rosz_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<DecompressedFile>> {
    // .rosz files are just ZIP files with a different extension
    decompress_zip_file(file_path)
}

pub fn find_xml_files(decompressed_files: &[DecompressedFile]) -> Vec<&DecompressedFile> {
    decompressed_files
        .iter()
        .filter(|file| {
            file.filename.ends_with(".xml") || 
            file.filename.ends_with(".cat") ||
            file.filename.ends_with(".ros")
        })
        .collect()
} 