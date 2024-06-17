pub trait BlockDisplay {
    fn block_tag(&self) -> String;
    fn entries(&self) -> String;
    fn content(&self) -> String {
        if !self.entries().is_empty() {
            [
                format!("%BLOCK {}", self.block_tag()),
                self.entries(),
                format!("%ENDBLOCK {}", self.block_tag()),
                "\n".to_string(),
            ]
            .join("\n")
        } else {
            [
                format!("%BLOCK {}", self.block_tag()),
                format!("%ENDBLOCK {}", self.block_tag()),
                "\n".to_string(),
            ]
            .join("\n")
        }
    }
}

pub trait FieldDisplay {
    fn field_tag(&self) -> String;
    fn value(&self) -> String;
    fn content(&self) -> String {
        format!("{} : {}\n", self.field_tag(), self.value())
    }
}
