const IDENT: &str = "\t";

#[derive(Default)]
pub(crate) struct IdentLines {
    ident_code: String,
    lines: Vec<String>,
}

impl IdentLines {
    pub fn new(ident: u8) -> Self {
        let ident_code = IDENT.repeat(ident as usize);
        Self {
            ident_code,
            lines: Default::default(),
        }
    }

    pub fn next_line_if_true(&mut self, advance: bool) {
        if advance {
            self.lines.push(format!(""));
        }
    }

    pub fn push(&mut self, line: String) -> &mut Self {
        self.lines.push(format!("{}{}", self.ident_code, line));
        self
    }

    pub fn generate(&self) -> String {
        self.lines.join("\n")
    }
}
