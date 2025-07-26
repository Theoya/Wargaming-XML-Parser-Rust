use std::collections::HashMap;
use crate::models::XmlNode::XmlNode;

#[derive(Debug, Clone)]
pub struct XmlElement {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<XmlNode>,
}

impl XmlElement {
    pub fn find_child_by_name(&self, name: &str) -> Option<&XmlElement> {
        for child in &self.children {
            if let XmlNode::Element(element) = child {
                if element.name == name {
                    return Some(element);
                }
            }
        }
        None
    }
    
    pub fn get_text_content(&self) -> String {
        let mut text_parts = Vec::new();
        for child in &self.children {
            if let XmlNode::Text(content) = child {
                text_parts.push(content.clone());
            }
        }
        text_parts.join("")
    }
    
    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }
}