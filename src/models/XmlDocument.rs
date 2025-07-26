use crate::models::XmlElement::XmlElement;

#[derive(Debug)]
pub struct XmlDocument {
    pub root: Option<XmlElement>,
}

impl XmlDocument {
    pub fn get_root_element(&self) -> Option<&XmlElement> {
        self.root.as_ref()
    }
    
    pub fn find_element_by_path(&self, path: &str) -> Option<&XmlElement> {
        let path_parts: Vec<&str> = path.split('/').collect();
        let mut current = self.get_root_element()?;
        
        for part in path_parts {
            if part.is_empty() {
                continue;
            }
            current = current.find_child_by_name(part)?;
        }
        
        Some(current)
    }
}