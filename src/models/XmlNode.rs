use crate::models::XmlElement::XmlElement;

#[derive(Debug, Clone)]
pub enum XmlNode {
    Element(XmlElement),
    Text(String),
    Comment(String),
}
