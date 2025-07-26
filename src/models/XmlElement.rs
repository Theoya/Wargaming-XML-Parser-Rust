struct XmlElement {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<XmlNode>,
}