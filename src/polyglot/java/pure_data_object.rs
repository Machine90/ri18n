use crate::polyglot::java::constants::{ENUM_NAME_ISO_639_CODE, JAVA_STRING};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field {
    pub is_public: bool,
    pub r#type: String,
    pub name: String,
    pub base_ident: u8,
}

impl Field {
    pub fn public(r#type: &str, name: &str) -> Self {
        Self {
            is_public: true,
            r#type: r#type.to_string(),
            name: name.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PureDataObject {
    pub name: String,
    pub fields: Vec<Field>,
    pub base_ident: u8,
}

impl PureDataObject {
    pub fn new_support_iso639_po(name: &str, base_ident: u8) -> Self {
        let mut this = Self {
            name: name.to_string(),
            base_ident,
            ..Default::default()
        };
        this.add_pub_field(ENUM_NAME_ISO_639_CODE, "Iso639Code")
            .add_pub_field(JAVA_STRING, "nativeName")
            .add_pub_field(JAVA_STRING, "enName");
        this
    }

    pub fn add_pub_field(&mut self, r#type: &str, name: &str) -> &mut Self {
        let mut field = Field::public(r#type, name);
        field.base_ident = self.base_ident + 1;
        self.fields.push(field);
        self
    }
}
