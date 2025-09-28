const IDENT: &str = "\t";

#[derive(Default)]
pub(crate) struct CodeLines {
    ident_code: String,
    lines: Vec<String>,
    end_with: &'static str,
}

impl CodeLines {
    pub fn new(ident: u8, end_with: Option<&'static str>) -> Self {
        let ident_code = IDENT.repeat(ident as usize);
        Self {
            ident_code,
            lines: Default::default(),
            end_with: end_with.unwrap_or(";"),
        }
    }

    pub fn next_line_if_true(&mut self, advance: bool) -> &mut Self {
        if advance {
            self.lines.push(format!(""));
        }
        self
    }

    pub fn push<S: ToString>(&mut self, line: S) -> &mut Self {
        self.lines.push(format!(
            "{}{}{}",
            self.ident_code,
            line.to_string(),
            self.end_with
        ));
        self
    }

    pub fn push_without_terminator<S: ToString>(&mut self, line: S) -> &mut Self {
        self.lines
            .push(format!("{}{}", self.ident_code, line.to_string()));
        self
    }

    pub fn extend<S: ToString>(&mut self, codes: S) -> &mut Self {
        self.lines.push(format!("{}", codes.to_string()));
        self
    }

    pub fn generate(&self) -> String {
        self.lines.join("\n")
    }
}
