// Metadata for the fields in a result row
pub struct Fieldnames {
    fields: Vec<String>,
}
impl Fieldnames {
    pub fn new(fields: &[&'static str]) -> Fieldnames {
        Fieldnames {
            fields: fields.iter().map(|s| String::from(*s)).collect(),
        }
    }

    pub fn number_of_fields(&self) -> usize {
        self.fields.len()
    }

    pub fn get_fieldname(&self, i: usize) -> Option<&String> {
        self.fields.get(i)
    }
}
