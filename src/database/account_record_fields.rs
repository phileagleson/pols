use std::collections::HashMap;

use lazy_static::lazy_static;

use super::types::{DataType, DatabaseField};

lazy_static! {
    pub static ref ACCOUNT_RECORD_FIELDS: HashMap<&'static str, DatabaseField> =
        load_account_record_fields();
}

fn load_account_record_fields() -> HashMap<&'static str, DatabaseField> {
    let mut account_fields = HashMap::new();

    account_fields.insert(
        "number",
        DatabaseField {
            data_type: DataType::Character,
            field_number: 1,
            mnemonic: "number".to_string(),
            help_file: "00001".to_string(),
            default_control: false,
            description: "This field stores the unique 10-digit account number assigned to the account when it was created.".to_string(),
            details: r#"This field includes the value at the Account Prefix prompt, if any, entered at the time the account was created.

You cannot access this field to revise the account number. To change account numbers, you must use the Change Account Numbers batch program. Once you assign an account number with the Account Creation Wizard, that number remains permanently associated with the account, and you cannot assign the same number to any other account."#.to_string(),
            default_value: "".to_string(),
            length: Some(10),
        },
    );
    account_fields
}
