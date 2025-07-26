use crate::models::XmlDocument::XmlDocument;
use crate::models::XmlElement::XmlElement;
use crate::models::XmlNode::XmlNode;
use crate::Tools::lexical_analysis::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    MismatchedTags,
    IncompleteDocument,
    UnexpectedToken(Token),
    EmptyStack,
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<XmlDocument, ParseError> {
    let mut token_iter = tokens.into_iter().peekable();
    let mut stack = Vec::new();
    let mut current_attributes = HashMap::new();
    let mut root_element: Option<XmlElement> = None;

    while let Some(token) = token_iter.next() {
        match token {
            Token::OpenTag(name) => {
                // Create new element and push to stack
                let element = XmlElement {
                    name,
                    attributes: current_attributes.clone(),
                    children: Vec::new(),
                };
                stack.push(element);
                current_attributes.clear();
            }
            Token::CloseTag(name) => {
                // Pop element from stack and add to parent
                if let Some(element) = stack.pop() {
                    if element.name != name {
                        return Err(ParseError::MismatchedTags);
                    }

                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(XmlNode::Element(element));
                    } else {
                        // This is the root element
                        if root_element.is_none() {
                            root_element = Some(element);
                        } else {
                            // Multiple root elements - this is invalid XML
                            return Err(ParseError::MismatchedTags);
                        }
                    }
                } else {
                    return Err(ParseError::EmptyStack);
                }
            }
            Token::SelfClosingTag(name) => {
                // Create self-closing element and add to current parent
                let element = XmlElement {
                    name,
                    attributes: current_attributes.clone(),
                    children: Vec::new(),
                };

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(XmlNode::Element(element));
                } else {
                    // Self-closing root element
                    if root_element.is_none() {
                        root_element = Some(element);
                    } else {
                        return Err(ParseError::MismatchedTags);
                    }
                }
                current_attributes.clear();
            }
            Token::Attribute(name, value) => {
                // Store attribute for the next opening tag
                current_attributes.insert(name, value);
            }
            Token::Text(content) => {
                // Add text as child of current element
                if let Some(element) = stack.last_mut() {
                    element.children.push(XmlNode::Text(content));
                }
            }
            Token::Comment(content) => {
                // Add comment as child of current element
                if let Some(element) = stack.last_mut() {
                    element.children.push(XmlNode::Comment(content));
                }
            }
            Token::XmlDeclaration => {
                // XML declarations are ignored during parsing
                // They don't affect the document structure
            }
            Token::EndOfFile => {
                // Check if we have a complete document
                if stack.is_empty() {
                    return Ok(XmlDocument { root: root_element });
                } else {
                    return Err(ParseError::IncompleteDocument);
                }
            }
        }
    }

    // If we reach here, check if we have a valid document
    if stack.is_empty() {
        Ok(XmlDocument { root: root_element })
    } else {
        Err(ParseError::IncompleteDocument)
    }
}
