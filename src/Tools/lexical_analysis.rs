#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenTag(String),
    CloseTag(String),
    SelfClosingTag(String),
    XmlDeclaration,
    Attribute(String, String),
    Text(String),
    Comment(String),
    EndOfFile,
}

#[derive(Debug)]
pub enum TokenizeError {
    UnexpectedEndOfInput,
    MalformedTag,
    MalformedAttribute,
}

pub fn tokenize(xml_string: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut chars = xml_string.chars().peekable();
    
    while let Some(&ch) = chars.peek() {
        match ch {
            '<' => {
                let tag_tokens = parse_tag_with_attributes(&mut chars)?;
                tokens.extend(tag_tokens);
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next(); // Skip whitespace
            }
            _ => {
                let token = parse_text(&mut chars)?;
                if !token.is_empty() {
                    tokens.push(Token::Text(token));
                }
            }
        }
    }
    
    tokens.push(Token::EndOfFile);
    Ok(tokens)
}

fn parse_tag(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, TokenizeError> {
    chars.next(); // Consume '<'
    
    match chars.peek() {
        Some('/') => parse_close_tag(chars),
        Some('!') => parse_comment(chars),
        Some('?') => parse_xml_declaration(chars),
        Some(_) => parse_open_tag(chars),
        None => Err(TokenizeError::UnexpectedEndOfInput),
    }
}

fn parse_tag_with_attributes(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Vec<Token>, TokenizeError> {
    chars.next(); // Consume '<'
    
    match chars.peek() {
        Some('/') => {
            let token = parse_close_tag(chars)?;
            Ok(vec![token])
        }
        Some('!') => {
            let token = parse_comment(chars)?;
            Ok(vec![token])
        }
        Some('?') => {
            let token = parse_xml_declaration(chars)?;
            Ok(vec![token])
        }
        Some(_) => parse_open_tag_with_attributes(chars),
        None => Err(TokenizeError::UnexpectedEndOfInput),
    }
}

fn parse_open_tag(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, TokenizeError> {
    let mut tag_name = String::new();
    
    // Parse tag name
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
                break;
            }
            '>' => {
                chars.next();
                return Ok(Token::OpenTag(tag_name));
            }
            '/' => {
                chars.next();
                if chars.next() == Some('>') {
                    return Ok(Token::SelfClosingTag(tag_name));
                }
                return Err(TokenizeError::MalformedTag);
            }
            _ => {
                tag_name.push(ch);
                chars.next();
            }
        }
    }
    
    // Parse attributes
    while let Some(&ch) = chars.peek() {
        match ch {
            '>' => {
                chars.next();
                return Ok(Token::OpenTag(tag_name));
            }
            '/' => {
                chars.next();
                if chars.next() == Some('>') {
                    return Ok(Token::SelfClosingTag(tag_name));
                }
                return Err(TokenizeError::MalformedTag);
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            _ => {
                // Parse attribute but don't store it here - it will be handled by the main tokenizer
                parse_attribute(chars)?;
            }
        }
    }
    
    Err(TokenizeError::UnexpectedEndOfInput)
}

fn parse_open_tag_with_attributes(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut tag_name = String::new();
    
    // Parse tag name
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
                break;
            }
            '>' => {
                chars.next();
                tokens.push(Token::OpenTag(tag_name));
                return Ok(tokens);
            }
            '/' => {
                chars.next();
                if chars.next() == Some('>') {
                    tokens.push(Token::SelfClosingTag(tag_name));
                    return Ok(tokens);
                }
                return Err(TokenizeError::MalformedTag);
            }
            _ => {
                tag_name.push(ch);
                chars.next();
            }
        }
    }
    
    // Parse attributes
    while let Some(&ch) = chars.peek() {
        match ch {
            '>' => {
                chars.next();
                tokens.push(Token::OpenTag(tag_name));
                return Ok(tokens);
            }
            '/' => {
                chars.next();
                if chars.next() == Some('>') {
                    tokens.push(Token::SelfClosingTag(tag_name));
                    return Ok(tokens);
                }
                return Err(TokenizeError::MalformedTag);
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            _ => {
                // Parse attribute and add it to tokens
                let attr_token = parse_attribute(chars)?;
                tokens.push(attr_token);
            }
        }
    }
    
    Err(TokenizeError::UnexpectedEndOfInput)
}

fn parse_close_tag(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, TokenizeError> {
    chars.next(); // Consume '/'
    let mut tag_name = String::new();
    
    while let Some(&ch) = chars.peek() {
        match ch {
            '>' => {
                chars.next();
                return Ok(Token::CloseTag(tag_name));
            }
            _ => {
                tag_name.push(ch);
                chars.next();
            }
        }
    }
    
    Err(TokenizeError::UnexpectedEndOfInput)
}

fn parse_comment(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, TokenizeError> {
    chars.next(); // Consume '!'
    
    // Check for <!--
    if chars.next() != Some('-') || chars.next() != Some('-') {
        return Err(TokenizeError::MalformedTag);
    }
    
    let mut comment = String::new();
    let mut prev_chars = [' ', ' '];
    
    while let Some(ch) = chars.next() {
        comment.push(ch);
        prev_chars[0] = prev_chars[1];
        prev_chars[1] = ch;
        
        if prev_chars == ['-', '-'] {
            if chars.next() == Some('>') {
                comment.pop(); // Remove last '-'
                comment.pop(); // Remove second to last '-'
                return Ok(Token::Comment(comment.trim().to_string()));
            }
        }
    }
    
    Err(TokenizeError::UnexpectedEndOfInput)
}

fn parse_xml_declaration(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, TokenizeError> {
    chars.next(); // Consume '?'
    
    // Skip until we find ?>
    while let Some(ch) = chars.next() {
        if ch == '?' {
            if chars.next() == Some('>') {
                return Ok(Token::XmlDeclaration);
            }
        }
    }
    
    Err(TokenizeError::UnexpectedEndOfInput)
}

fn parse_attribute(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, TokenizeError> {
    let mut name = String::new();
    let mut value = String::new();
    
    // Parse attribute name
    while let Some(&ch) = chars.peek() {
        match ch {
            '=' => {
                chars.next();
                break;
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            _ => {
                name.push(ch);
                chars.next();
            }
        }
    }
    
    // Parse attribute value
    let quote_char = chars.next().ok_or(TokenizeError::UnexpectedEndOfInput)?;
    if quote_char != '"' && quote_char != '\'' {
        return Err(TokenizeError::MalformedAttribute);
    }
    
    while let Some(ch) = chars.next() {
        if ch == quote_char {
            return Ok(Token::Attribute(name, value));
        }
        value.push(ch);
    }
    
    Err(TokenizeError::UnexpectedEndOfInput)
}

fn parse_text(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, TokenizeError> {
    let mut text = String::new();
    
    while let Some(&ch) = chars.peek() {
        if ch == '<' {
            break;
        }
        text.push(ch);
        chars.next();
    }
    
    Ok(text.trim().to_string())
}