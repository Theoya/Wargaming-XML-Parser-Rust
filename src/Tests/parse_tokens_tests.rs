use crate::models::XmlDocument::XmlDocument;
use crate::models::XmlElement::XmlElement;
use crate::models::XmlNode::XmlNode;
use crate::Tools::lexical_analysis::tokenize;
use crate::Tools::parse_tokens::parse_tokens;

#[test]
fn test_parse_chaos_thousand_sons_cat() {
    // Read the test XML file
    let xml_content = std::fs::read_to_string("example-data/Test-Chaos-Thousand Sons.cat")
        .expect("Failed to read test file");

    // Step 1: Tokenize the XML
    let tokens = tokenize(&xml_content).expect("Failed to tokenize XML");

    println!("Generated {} tokens", tokens.len());

    // Step 2: Parse tokens into tree structure
    let document = parse_tokens(tokens).expect("Failed to parse tokens");

    // Step 3: Verify the document structure
    let root = document
        .get_root_element()
        .expect("Document should have a root element");

    println!("Root element: {}", root.name);

    // Verify root element properties
    assert_eq!(root.name, "catalogue");
    assert!(root.attributes.contains_key("name"));
    assert_eq!(root.get_attribute("name").unwrap(), "Chaos - Thousand Sons");
    assert!(root.attributes.contains_key("id"));
    assert!(root.attributes.contains_key("revision"));

    // Test finding specific elements
    let publications = root
        .find_child_by_name("publications")
        .expect("Should find publications element");

    let publication = publications
        .find_child_by_name("publication")
        .expect("Should find publication element");

    assert_eq!(
        publication.get_attribute("name").unwrap(),
        "Index - Thousand Sons"
    );

    // Test finding profileTypes
    let profile_types = root
        .find_child_by_name("profileTypes")
        .expect("Should find profileTypes element");

    let ritual_profile = profile_types
        .find_child_by_name("profileType")
        .expect("Should find profileType element");

    assert_eq!(ritual_profile.get_attribute("name").unwrap(), "Rituals");

    // Test finding categoryEntries
    let category_entries = root
        .find_child_by_name("categoryEntries")
        .expect("Should find categoryEntries element");

    // Count category entries
    let mut category_count = 0;
    for child in &category_entries.children {
        if let XmlNode::Element(element) = child {
            if element.name == "categoryEntry" {
                category_count += 1;
            }
        }
    }

    println!("Found {} category entries", category_count);
    assert!(
        category_count > 0,
        "Should have at least one category entry"
    );

    // Test finding a specific category entry
    let ahriman_entry = category_entries
        .children
        .iter()
        .find_map(|child| {
            if let XmlNode::Element(element) = child {
                if element.name == "categoryEntry"
                    && element.get_attribute("name") == Some(&"Ahriman".to_string())
                {
                    Some(element)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("Should find Ahriman category entry");

    assert_eq!(ahriman_entry.get_attribute("name").unwrap(), "Ahriman");
    assert_eq!(ahriman_entry.get_attribute("hidden").unwrap(), "false");

    // Test text content extraction
    let text_content = ahriman_entry.get_text_content();
    assert_eq!(
        text_content, "",
        "Category entry should have no text content"
    );

    // Test finding nested elements with constraints
    let faction_entry = category_entries
        .children
        .iter()
        .find_map(|child| {
            if let XmlNode::Element(element) = child {
                if element.name == "categoryEntry"
                    && element.get_attribute("name")
                        == Some(&"Faction: Scintillating Legions".to_string())
                {
                    Some(element)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("Should find Faction category entry");

    let constraints = faction_entry
        .find_child_by_name("constraints")
        .expect("Should find constraints element");

    let constraint = constraints
        .find_child_by_name("constraint")
        .expect("Should find constraint element");

    assert_eq!(constraint.get_attribute("type").unwrap(), "max");
    assert_eq!(constraint.get_attribute("value").unwrap(), "0");

    println!("✅ All parse_tokens tests passed!");
}

#[test]
fn test_parse_simple_xml() {
    let simple_xml = r#"
        <book id="123" category="fiction">
            <title>Rust Programming</title>
            <author>John Doe</author>
            <chapter>
                <heading>Introduction</heading>
                <text>Welcome to Rust!</text>
            </chapter>
        </book>
    "#;

    // Tokenize
    let tokens = tokenize(simple_xml).expect("Failed to tokenize simple XML");

    // Parse
    let document = parse_tokens(tokens).expect("Failed to parse simple XML");

    let root = document
        .get_root_element()
        .expect("Should have root element");

    assert_eq!(root.name, "book");
    assert_eq!(root.get_attribute("id").unwrap(), "123");
    assert_eq!(root.get_attribute("category").unwrap(), "fiction");

    // Test finding nested elements
    let title = root
        .find_child_by_name("title")
        .expect("Should find title element");

    assert_eq!(title.get_text_content(), "Rust Programming");

    let author = root
        .find_child_by_name("author")
        .expect("Should find author element");

    assert_eq!(author.get_text_content(), "John Doe");

    // Test deeper nesting
    let chapter = root
        .find_child_by_name("chapter")
        .expect("Should find chapter element");

    let heading = chapter
        .find_child_by_name("heading")
        .expect("Should find heading element");

    assert_eq!(heading.get_text_content(), "Introduction");

    let text = chapter
        .find_child_by_name("text")
        .expect("Should find text element");

    assert_eq!(text.get_text_content(), "Welcome to Rust!");
}

#[test]
fn test_parse_self_closing_tags() {
    let xml_with_self_closing = r#"
        <library>
            <book id="1">
                <title>Book 1</title>
                <img src="cover1.jpg"/>
            </book>
            <book id="2">
                <title>Book 2</title>
                <br/>
            </book>
        </library>
    "#;

    let tokens =
        tokenize(xml_with_self_closing).expect("Failed to tokenize XML with self-closing tags");

    let document = parse_tokens(tokens).expect("Failed to parse XML with self-closing tags");

    let root = document
        .get_root_element()
        .expect("Should have root element");

    assert_eq!(root.name, "library");

    // Test that self-closing tags are properly parsed
    let book1 = root
        .find_child_by_name("book")
        .expect("Should find first book");

    let img = book1
        .find_child_by_name("img")
        .expect("Should find img element");

    assert_eq!(img.get_attribute("src").unwrap(), "cover1.jpg");
    assert_eq!(
        img.children.len(),
        0,
        "Self-closing tag should have no children"
    );
}

#[test]
fn test_parse_comments() {
    let xml_with_comments = r#"
        <document>
            <!-- This is a comment -->
            <title>Test Document</title>
            <content>
                Hello World
                <!-- Another comment -->
            </content>
        </document>
    "#;

    let tokens = tokenize(xml_with_comments).expect("Failed to tokenize XML with comments");

    let document = parse_tokens(tokens).expect("Failed to parse XML with comments");

    let root = document
        .get_root_element()
        .expect("Should have root element");

    assert_eq!(root.name, "document");

    // Comments should be preserved as children
    let mut comment_count = 0;
    for child in &root.children {
        if let XmlNode::Comment(_) = child {
            comment_count += 1;
        }
    }

    assert!(comment_count > 0, "Should have at least one comment");
}
