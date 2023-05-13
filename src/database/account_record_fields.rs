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
            default_control: false,
            default_value: "",
            field_number: 1,
            help_file: "00001",
            length: Some(10),
            mnemonic: "number",
            description: "This field stores the unique 10-digit account number assigned to the account when it was created.",
            details: r#"
# Account Number

Field Number:     001
Mnemonic:         NUMBER
Data Type:        10 Characters
Source:           System-entered
Help File         00001 
Default Control:  No
Default Value:    <Blank>

This field includes the value at the Account Prefix prompt, if any, entered at the time the account was created.

You cannot access this field to revise the account number. To change account numbers, you must use the Change Account Numbers batch program. Once you assign an account number with the Account Creation Wizard, that number remains permanently associated with the account, and you cannot assign the same number to any other account."#,
        },
    );

    account_fields.insert(
        "branch",
        DatabaseField {
            data_type: DataType::Code,
            default_control: false,
            default_value: "0",
            field_number: 7,
            help_file: "00007",
            length: Some(9999),
            mnemonic: "branch",
            description: "This field stores the credit union-defined branch number where the account was opened or is currently housed.",
            details: r#"
# Branch

Field Number:     007
Mnemonic:         BRANCH
Data Type:        Code to 9999
Source:           User-entered
Help File         00007 
Default Control:  No
Default Value:    0

### Data Type Descriptions
**0**
    Main branch
**1–9999**
    Credit union defined branch number

A branch can be defined in Institution Branch Address parameters.

If your credit union uses branch accounting, use this field to specify the branch where share transactions should be posted. To translate share transactions to the General Ledger by branch, you must set the **GL Translation Branch Level** parameter in the Miscellaneous Parameters to **(0) Pull GL Branch from Acct Record**.

***Important:*** For NetTeller users, the query 60 processes a branch number value of three characters for check orders. When the branch number is longer than three characters, the JHADRIVER sends a blank value to NetTeller.           "#,
        },
    );

    account_fields.insert(
        "type",
        DatabaseField {
            data_type: DataType::Code,
            default_control: false,
            default_value: "0",
            field_number: 2,
            help_file: "00002",
            length: Some(9999),
            mnemonic: "type",
            description:
                "This field stores the credit union-defined code to define the type of account.",
            details: r#"
# Account Type

Field Number:     002
Mnemonic:         TYPE 
Data Type:        Code to 99 or Code to 9999 
Source:           User-entered
Help File         00002
Default Control:  No 
Default Value:    0 

Enter a credit union-defined code (0–9999) to define the type of account. An account type can be defined in Account Type Names parameters.

***Tip:*** If you are currently using a two-digit code and you want to use a four-digit code, contact Symitar Support. "#,
        },
    );

    account_fields.insert(
        "membergroup",
        DatabaseField {
            data_type: DataType::Code,
            default_control: true,
            default_value: "0",
            field_number: 20,
            help_file: "00020",
            length: Some(9999),
            mnemonic: "membergroup",
            description: "This field stores the credit union-defined code to identify the group to which the member belongs.",
            details: r#"
# Member group

Field Number:     020 
Mnemonic:         MEMBERGROUP 
Data Type:        Code to 9999 
Source:           User-entered 
Help File         00020 
Default Control:  Yes 
Default Value:    0 

Enter a credit union-defined code (0–9999) to identify the group to which the member belongs. A member group can be defined in Member Group Descriptions parameters. "#,
        },
    );

    account_fields.insert( 
        "restrict",
        DatabaseField {
            data_type: DataType::Code,
            default_control: true,
            default_value: "0",
            field_number: 8,
            help_file: "00008",
            length: Some(6),
            mnemonic: "restrict",
            description: "This field stores a code that indicates the type of restriction if access is restricted on this account.",
            details: r#"
# Restricted access 

Field Number:     008 
Mnemonic:         RESTRICT 
Data Type:        Code to 6 
Source:           User-entered 
Help File         00008 
Default Control:  Yes 
Default Value:    0 

### Data Type Descriptions

**(0) Normal** 
The account is not restricted. Normal accounts can be accessed for inquiries (including inquiries on the imaging system), file maintenance, or transactions.

**(1) Restricted**
The account is restricted. The user's **Acct Restricted** privileges determine whether the user can access restricted accounts.

**(2) Sensitive**
The account is sensitive. The system automatically records any attempted access to a sensitive account on the File Maintenance Journal. The user's **Acct Sensitive** privileges determine whether the user can access sensitive accounts for inquiries, file maintenance, and transactions. 

**(3) Employee**
The account belongs to a credit union employee. 
  - If an employee account does not appear in a user's Restricted Accounts list, the user's **Employee Acct Other** privileges determine whether the user can access the account for inquiries, file maintenance, and transactions.
  - If an employee account appears in a user's Restricted Accounts list, the user's **Employee Acct Restricted** privileges determine whether the user can access the account for inquiries, file maintenance, and transactions.

**(4) Employee Family**
The account belongs to the family of a credit union employee.
  - If an employee family account does not appear in a user's Restricted Accounts list, the user's **Employee Acct Family** privileges determine whether the user can access the account for inquiries, file maintenance, and transactions.
  - If an employee family account appears in a user's Restricted Accounts list, the user's **Employee Acct Restricted** privileges determine whether the user can access the account for inquiries, file maintenance, and transactions.

**(5) Employee Sensitive**
The account is a sensitive account belonging to a credit union employee. The system automatically records any attempted access to an employee sensitive account on the File Maintenance Journal.
  - If an employee sensitive account does not appear in a user's Restricted Accounts list, the user's **Employee Acct Sensitive** privileges determine whether the user can access the account for inquiries, file maintenance, and transactions.
  - If an employee sensitive account appears in a user's Restricted Accounts list, the user's **Employee Acct Restricted** privileges determine whether the user can access the account for inquiries, file maintenance, and transactions.

**(6) Employee Sensitive Family**
The account is a sensitive account belonging to the family of a credit union employee. The system automatically records any attempted access to an employee sensitive family account on the File Maintenance Journal.
  - If an employee sensitive family account does not appear in a user's Restricted Accounts list, the user's **Employee Acct Sens Family** privileges determine whether the user can access the account for inquiries, file maintenance, and transactions.
  - If an employee sensitive family account appears in a user's Restricted Accounts list, the user's **Employee Acct Restricted** privileges determine whether the user can access the account for inquiries, file maintenance, and transactions.

If an account has a **Restricted Access** code of **1-6**, you can suppress the display of account balances and payroll amounts during transactions and file maintenance by setting the appropriate **Inq** privilege for the user to **No**. For example, if you do not want your tellers to see the balances of other employees' accounts, but want to allow them to view their own account balances, set up the accounts and user privileges as follows:

  - Set the **Restricted Access** field in all employee accounts to **(3) Employee**.
  - Enter each teller's own account number in the Restricted Accounts list in user privileges.
  - Set each user's **Employee Acct Other Inq** privilege to **No**.
  - Set each user's **Employee Acct Restricted Inq** privilege to **Yes**. "#,
        }
    );

    account_fields
}

/*
*/
