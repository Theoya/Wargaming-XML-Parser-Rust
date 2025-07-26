use crate::Tools::decompression::{decompress_rosz_file, find_xml_files, DecompressedFile};

#[test]
fn test_decompress_rosz_file() {
    // Test with the sample army file
    let result = decompress_rosz_file("example-data/sample-army.rosz");
    
    match result {
        Ok(files) => {
            println!("Successfully decompressed {} files:", files.len());
            for file in &files {
                println!("  - {}", file.filename);
            }
            
            // Find XML files
            let xml_files = find_xml_files(&files);
            println!("Found {} XML files:", xml_files.len());
            for xml_file in &xml_files {
                println!("  - {}", xml_file.filename);
            }
            
            // Basic assertions
            assert!(!files.is_empty(), "Should have decompressed at least one file");
            
            // Check if we found any XML files
            if !xml_files.is_empty() {
                let first_xml = xml_files[0];
                assert!(!first_xml.content.is_empty(), "XML content should not be empty");
                println!("First XML file content preview: {}", 
                    first_xml.content.chars().take(100).collect::<String>());
            }
        }
        Err(e) => {
            panic!("Failed to decompress .rosz file: {:?}", e);
        }
    }
}

#[test]
fn test_find_xml_files() {
    // Create test data
    let test_files = vec![
        DecompressedFile {
            filename: "document.xml".to_string(),
            content: "<root>test</root>".to_string(),
        },
        DecompressedFile {
            filename: "data.json".to_string(),
            content: "{}".to_string(),
        },
        DecompressedFile {
            filename: "catalog.cat".to_string(),
            content: "<catalog>test</catalog>".to_string(),
        },
        DecompressedFile {
            filename: "roster.ros".to_string(),
            content: "<roster>test</roster>".to_string(),
        },
    ];
    
    let xml_files = find_xml_files(&test_files);
    
    assert_eq!(xml_files.len(), 3, "Should find 3 XML-like files");
    assert!(xml_files.iter().any(|f| f.filename == "document.xml"));
    assert!(xml_files.iter().any(|f| f.filename == "catalog.cat"));
    assert!(xml_files.iter().any(|f| f.filename == "roster.ros"));
    assert!(!xml_files.iter().any(|f| f.filename == "data.json"));
}

#[test]
fn test_decompress_nonexistent_file() {
    let result = decompress_rosz_file("nonexistent-file.rosz");
    assert!(result.is_err(), "Should fail when file doesn't exist");
} 