use super::constants::IppValueTag;



pub struct Attribute {
    pub tag: IppValueTag,
    pub name: &'static str,
    pub value: String,
}
