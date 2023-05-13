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
            mnemonic: "number",
            help_file: "00001",
            default_control: false,
            description: "This field stores the unique 10-digit account number assigned to the account when it was created.",
            details: r#"
# NUMBER

This field includes the value at the Account Prefix prompt, if any, entered at the time the account was created.

You cannot access this field to revise the account number. To change account numbers, you must use the Change Account Numbers batch program. Once you assign an account number with the Account Creation Wizard, that number remains permanently associated with the account, and you cannot assign the same number to any other account."#,
            default_value: "",
            length: Some(10),
        },
    );
    account_fields.insert(
        "branch",
        DatabaseField {
            field_number: 7,
            mnemonic: "branch",
            data_type: DataType::Code,
            help_file: "00007",
            default_control: false,
            default_value: "0",
            description: "This field stores the credit union-defined branch number where the account was opened or is currently housed.",
            details: r#"
# BRANCH

### Data Type Descriptions
**0**
    Main branch
**1–9999**
    Credit union defined branch number

A branch can be defined in Institution Branch Address parameters.

If your credit union uses branch accounting, use this field to specify the branch where share transactions should be posted. To translate share transactions to the General Ledger by branch, you must set the **GL Translation Branch Level** parameter in the Miscellaneous Parameters to **(0) Pull GL Branch from Acct Record**.

***Important:*** For NetTeller users, the query 60 processes a branch number value of three characters for check orders. When the branch number is longer than three characters, the JHADRIVER sends a blank value to NetTeller.           
            "#,
            length: Some(9999),
        },
    );
    account_fields
}

/*
[
  {
    "label": "ACCESSTYPE",
    "insertText": "ACCESSTYPE",
    "description": "This field stores a code that indicates what the member can do. There are different access types for Preference Access and Card Access records.",
    "isSnippet": false
  },
  {
    "label": "ACCOUNTNUMBER",
    "insertText": "ACCOUNTNUMBER",
    "description": "This field stores the number of an account the member can access.",
    "isSnippet": false
  },
  {
    "label": "IDTYPE",
    "insertText": "IDTYPE",
    "description": "This field stores a code that specifies what access is allowed to or from any share or loan.",
    "isSnippet": false
  },
  {
    "label": "ID",
    "insertText": "ID",
    "description": "This field stores the ID of the Share or Loan record the member can access.",
    "isSnippet": false
  },
  {
    "label": "EXTLOANACCOUNT",
    "insertText": "EXTLOANACCOUNT",
    "description": "This field stores the number of an external loan account the member can access.",
    "isSnippet": false
  },
  {
    "label": "RECORDCHANGEDATE",
    "insertText": "RECORDCHANGEDATE",
    "description": "This field stores the system date when you create, change, or save the Access record. Sometimes the system updates this field because the record was saved, even though there may not be any actual changes in the record.",
    "isSnippet": false
  },
  {
    "label": "ENABLEWITHDRAWAL",
    "insertText": "ENABLEWITHDRAWAL",
    "description": "This field stores a code that specifies whether withdrawals are allowed.",
    "isSnippet": false
  },
  {
    "label": "ENABLEDEPOSIT",
    "insertText": "ENABLEDEPOSIT",
    "description": "This field stores a code that specifies whether deposits are allowed.",
    "isSnippet": false
  },
  {
    "label": "ENABLEINQUIRY",
    "insertText": "ENABLEINQUIRY",
    "description": "This field stores a code that specifies whether inquiries are allowed.",
    "isSnippet": false
  },
  {
    "label": "ENABLEFM",
    "insertText": "ENABLEFM",
    "description": "This field stores a code that specifies whether inquiries are allowed.",
    "isSnippet": false
  },
  {
    "label": "QUALIFIER",
    "insertText": "QUALIFIER",
    "description": "This field stores a cross-reference to a share or loan in a network processor's database to shares or loans in the system.",
    "isSnippet": false
  },
  {
    "label": "LOCATOR",
    "insertText": "LOCATOR",
    "description": "The system stores a unique number within each member account in this field that can be used as a unique identification of this record.",
    "isSnippet": false
  }
]

*/
