use xml_parser::Tools::decompression;

fn main() {
    println!("XML Parser CLI");
    println!("Use this tool to parse XML files and compressed .rosz files");
    
    // Example usage
    match decompression::decompress_rosz_file("example-data/sample-army.rosz") {
        Ok(files) => {
            println!("Successfully decompressed {} files:", files.len());
            for file in &files {
                println!("  - {}", file.filename);
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
} 