#[derive(Debug, Clone)]
pub struct DatabaseField {
    pub field_number: u32,
    pub mnemonic: &'static str,
    pub data_type: DataType,
    pub help_file: &'static str,
    pub default_control: bool,
    pub default_value: &'static str,
    pub description: &'static str,
    pub details: &'static str,
    pub length: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum DataType {
    Character,
    Code,
    Date,
    Float,
    Money,
    Number,
    Rate,
}

#[derive(Debug)]
pub enum RecordType {
    Account,
}

impl RecordType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RecordType::Account => "account",
        }
    }
}
