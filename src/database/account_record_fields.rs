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

    account_fields.insert(
        "opendate", 
        DatabaseField {
            data_type: DataType::Date,
            default_control: false,
            default_value: "Date Null",
            field_number: 5,
            help_file: "00005",
            length: None,
            mnemonic: "opendate",
            description: "This field stores the date the account was opened.",
            details: r#"
# Open Date 

Field Number    005
Mnemonic        OPENDATE
Data Type       Date
Source          System-entered
Help File       00005
Default Control No 
Default Value   Date Null 

The system updates this field with the system date when the Account record is created.

Under normal circumstances, you should not revise this field."#,
        });

    account_fields.insert(
        "lastfmdate",
        DatabaseField {
            data_type: DataType::Date,
            default_control: false,
            default_value: "Date Null",
            field_number: 3,
            help_file: "00003",
            length: None,
            mnemonic: "lastfmdate",
            description: "This field stores the date of the last file maintenance or teller transaction affecting this Account record.",
            details: r#"
# Last FM Date 

Field Number    003 
Mnemonic        LASTFMDATE 
Data Type       Date 
Source          System-entered 
Help File       00003 
Default Control No 
Default Value   Date Null 

The system updates this field with the system date whenever you perform manual file maintenance on this Account record using Account File Maintenance or the File Maintenance (FM) teller transaction. 

You cannot perform file maintenance on this field. 

***Important:*** This field in the Account record is not updated when the file maintenance is performed by a batch process."#,
        }
    );

    account_fields.insert(
        "recordchangedate",
        DatabaseField {
            data_type: DataType::Date,
            default_control: false,
            default_value: "Date Null",
            field_number: 57,
            help_file: "00057",
            length: None,
            mnemonic: "recordchangedate",
            description: "This field stores the system date when you create, change, or save the Account record.",
            details: r#"
# Record Change Date 

Field Number    057 
Mnemonic        RECORDCHANGEDATE 
Data Type       Date 
Source          System-entered 
Help File       00057 
Default Control No 
Default Value   Date Null 

***Important:*** The system does not change the ***Record Change Date*** when a calculated field changes.

You cannot perform file maintenance on this field."#,
        }
    );

    account_fields.insert(
        "activitydate",
        DatabaseField {
            data_type: DataType::Date,
            default_control: false,
            default_value: "Date Null",
            field_number: 810,
            help_file: "00090",
            length: None,
            mnemonic: "activitydate",
            description: "This calculated field contains the most recent activity date of all shares and loans in the account.",
            details: r#"
# Activity Date 

Field Number    810 
Mnemonic        ACTIVITYDATE 
Data Type       Date 
Source          System-calculated 
Help File       00090 
Default Control No 
Default Value   Date Null 

If there are no shares or loans in the account, the system sets this field to the ***Open Date*** field value in the Account record. 

If the account has External Loan records and the ***Activity Date Update*** parameter in the External Loan Processing Parameters is set to ***Yes***, the system updates this field in the Account record with the most recent activity date of any external loan. If the parameter is set to ***No***, the system ignores the external loan activity date.

When you update the ***Activity Date*** field in a Share or Loan record, this field in the Account record is automatically updated. If the ***Activity Date Update*** parameter in the External Loan Processing Parameters is set to ***Yes***, this field in the Account record is automatically updated when you update the ***Activity Date*** field in an External Loan record.

When an Account record is first created, the system sets this field to the value in the ***Open Date*** field.

Whenever a teller, ATM network, MemberConnect system, SymConnect client system, or batch program posts a monetary transaction to a Share or Loan record for this account, the system updates this field with the effective date of the previous monetary transaction. However, if you include any of the following types of teller transactions in a transaction string or in batch transactions, the system does not update this field:

  - Fee (FE)
  - Share Dividend (SV)
  - Withholding
  - Interest Refund (IR)
  - Checks Cashing (KC)

The following batch programs do not update this field:

  - Dividend Posting
  - Fee Posting
  - Insurance Posting

***Important:***

  - The system updates this field for dividend disbursements if the ***Dividend Post Code*** field in the Share record is set to ***(1) Check*** or ***(2) Transfer*** and the value in the ***Div Disb Updts Activity Date*** field in the Share record is set to ***(1) Update Activity Date***.
  - The system updates this field for insurance posting transactions if the ***Insurance Updts Activity Date*** parameter in the Miscellaneous Parameters is set to ***Yes***.

You cannot perform file maintenance on this field.
            "#,
        }
    );

    account_fields.insert(
        "prgdrecactivitydt",
        DatabaseField {
            data_type: DataType::Date,
            default_control: false, 
            default_value: "Date Null", 
            field_number: 56,
            help_file: "00056", 
            length: None, 
            mnemonic: "prgdrecactivitydt", 
            description: "This field stores the most recent activity date from shares or loans that are purged.",
            details: r#"
# Purged Rec Activity Dt 

Field Number    056
Mnemonic        PRGDRECACTIVITYDT 
Data Type       Date 
Source          System-entered or User-entered 
Help File       00056 
Default Control No 
Default Value   Date Null 

The system updates this field automatically only if the purged Share or Loan record's ***Activity Date*** field value is greater than or equal to this field. In that case, the purged Share or Loan record's ***Activity Date*** field value is stored in this field. This value is used for complying with state escheatment regulations.

You can also manually update this field, but use caution. Manual file maintenance overwrites the existing date, potentially causing this field to reflect inaccurate data. The system will prevent you from setting this field to a date in the future."#,
        }
    );

    account_fields.insert(
        "corresponddate",
        DatabaseField {
            data_type: DataType::Date,
            default_control: false,
            default_value: "Date Null",
            field_number: 14,
            help_file: "00014",
            length: None,
            mnemonic: "corresponddate",
            description: "This field stores the date the member made the most recent correspondence regarding this account.",
            details: r#"
# Correspondence Date 

Field Number    014 
Mnemonic        CORRESPONDDATE 
Data Type       Date 
Source          User-entered 
Help File       00014 
Default Control No 
Default Value   Date Null 

This field is used for escheatment of accounts. Most states require that funds from accounts inactive for a specified period of time be turned over to the state. The member must then contact the state for the funds, rather than the credit union.

States with this requirement also specify a period after which the credit union must attempt to notify the member that the account is inactive, giving the member a choice of closing the account or keeping it open. If the member chooses to keep the account open, enter the date the member made that decision in this field. The account is reactivated as of that date and can remain open without transactions for the period specified by the state.

***Important:*** If the ***Dormancy Use Correspond Date*** parameter in the Miscellaneous Parameters is set to ***Yes***, the system uses this field along with the ***Activity Date*** field to determine dormancy."#, 
        }
    );

    account_fields.insert(
        "proxydate", 
        DatabaseField {
            data_type: DataType::Date, 
            default_control: true, 
            default_value: "Date Null", 
            field_number: 15, 
            help_file: "00015", 
            length: None, 
            mnemonic: "proxydate", 
            description: "If your credit union allows members to assign their vote to a proxy, this field stores the expiration date or the effective date of the proxy.",
            details: r#"
# Proxy Date 

Field Number    015 
Mnemonic        PROXYDATE 
Data Type       Date 
Source          User-entered 
Help File       00015 
Default Control Yes 
Default Value   Date Null 

***Tip:*** You should always use this field for the same type of date (expiration date or effective date) to avoid confusion. "#,
        }
    );

    account_fields.insert( 
        "closedate", 
        DatabaseField {
            data_type: DataType::Date, 
            default_control: true, 
            default_value: "Date Null", 
            field_number: 6, 
            help_file: "00006", 
            length: None, 
            mnemonic: "closedate", 
            description: "This field stores the date the account was closed.", 
            details: r#"
# Close Date 

Field Number    006 
Mnemonic        CLOSEDATE 
Data Type       Date 
Source          User-entered 
Help File       00006 
Default Control Yes 
Default Value   Date Null 

If there is a date in this field, the system considers the Account record closed on that date and allows no further transactions affecting this Account record. 

Account closing criteria: 

  - If you enter a value in the ***Close Date*** field that is before the value in the ***Activity Date*** field, the system displays the following message: 
    `Cannot be before activity date.` 
  - You cannot close an account with open Share records, open Loan records, or unexpired Account Tracking records that have the ***Tracking Type*** field set to ***(1) FICS Status Inquiry***; additionally, you cannot close an account with Account Tracking records that have the ***Parent Closing Option*** parameter in the Account File Tracking Type Parameters set to ***Yes***.
  - You cannot close an account, either in the Daily Posting batch program or manually in Account Manager, that has open External Loan records (in which the Close Date field is blank).

If you attempt to enter a value in this field for an account that does not meet these criteria, the system displays the following message: 
`This account cannot be closed.`"#,
        }
    );

    account_fields.insert(
        "fmlastpurgedate", 
        DatabaseField {
            data_type: DataType::Date,
            default_control: false,
            default_value: "Date Null",
            field_number: 22,
            help_file: "00022",
            length: None,
            mnemonic: "fmlastpurgedate",
            description: "This field stores the date through which you last purged file maintenance history for the account type with the Purge Transaction Processing batch program.",
            details: r#"
# FMLASTPURGEDATE 

Field Number    022 
Mnemonic        FMLASTPURGEDATE 
Data Type       Date 
Source          System-entered 
Help File       00022 
Default Control No 
Default Value   Date Null 

Whenever you purge file maintenance history for the account type, the system updates this field with a date calculated by subtracting one day from the date you enter at the Save FM History From prompt in the Purge Transaction Processing batch program.

You cannot perform file maintenance on this field."#,
        }
    );

    account_fields.insert(
        "reference",
        DatabaseField {
            data_type: DataType::Character, 
            default_control: true,
            default_value: "",
            field_number: 9,
            help_file: "00009",
            length: Some(20),
            mnemonic: "reference",
            description: "You can use this field to store any account reference information, such as a mother's maiden name.",
            details: r#"
# Reference 

Field Number    009 
Mnemonic        REFERENCE 
Data Type       20 Characters 
Source          User-entered 
Help File       00009 
Default Control Yes 
Default Value   (Blank) 

You can print this field on statements using the Statement Generation batch program. Enter any information you want to use as an account reference. "#,
        }
    );

    account_fields.insert(
        "memberstatus", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: true,
            default_value: "0",
            field_number: 13,
            help_file: "00013", 
            length: Some(2),
            mnemonic: "memberstatus", 
            description: "This field stores the membership status of an individual or organization.", 
            details: r#"
# Membership Status 

Field Number    013 
Mnemonic        MEMBERSTATUS 
Data Type       Code to 2 
Source          User-entered 
Help File       00013 
Default Control Yes 
Default Value   0 

### Data Type Descriptions
***(0) Natural Person***
The individual is a member of the credit union.

***(1) Non-Member***
The individual is not a member of the credit union.
You cannot create shares or loans in the account. You cannot set the value of this field to ***(1) Non-Member*** if there are already any shares or loans on the account.

***(2) Credit Union***
The organization is a credit union.
*For corporate credit unions only:* Set this field to this option only if you have set the ***Scope of Membership*** parameter in the Miscellaneous Parameters to ***Credit Unions Only*** or ***Natural Persons and Credit Unions***. This setting can only be done during account creation. After you create an account, you cannot change this field to or from this option.
For any Name record associated with an account with this option, the value in the ***Last Name*** field appears as "Credit Union Name"."#,
        }
    );

    account_fields.insert(
        "commercialcode", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: true, 
            default_value: "0", 
            field_number: 21, 
            help_file: "00021", 
            length: Some(2), 
            mnemonic: "commercialcode", 
            description: "This field stores a code that indicates whether an account is a consumer account, small business account, or commercial account.", 
            details: r#"
# Commercial Code 

Field Number    021 
Mnemonic        COMMERCIALCODE 
Data Type       Code to 2 
Source          User-entered 
Help File       00021 
Default Control Yes 
Default Value   0 

### Data Type Descriptions
***(0) Consumer***
This account is for an individual member with any joint members.

***(1) Small Business***
This account is for a small business.

***(2) Commercial***
This account is for a larger commercial business.

This field can be used by *Member Business Services* modules.

***Important:*** This field is currently used only for informational purposes, but we recommend that this field be set to identify commercial accounts for future Member Business Services enhancements. "#,
        }
    );

    account_fields.insert(
        "krholdbaseamount", 
        DatabaseField {
            data_type: DataType::Money,
            default_control: true,
            default_value: "0.00",
            field_number: 36,
            help_file: "00036",
            length: None,
            mnemonic: "krholdbaseamount",
            description: "This field stores the check hold base amount for this account.",
            details: r#"
# Check Hold Base Amount

Field Number    036 
Mnemonic        KRHOLDBASEAMOUNT 
Data Type       Money 
Source          User-entered 
Help File       00036 
Default Control Yes 
Default Value   0.00 

If the ***KR Hold Base Option*** parameter in the Regulation CC Parameters is set to a value other than ***(0) Do Not Use KR Hold Base***, the Checks Received (KR)/Checks Cashed (KC) teller transactions use this field in determining if the teller should be prompted to place holds on deposited checks. Shared Branch/Online deposits that use Reg CC hold logic and are identified as a local or non-local hold use this field to determine how much of the deposit to make available immediately. "#,
        }
    );

    account_fields.insert(
        "krtotalamount", 
        DatabaseField {
            data_type: DataType::Money,
            default_control: false,
            default_value: "0.00", 
            field_number: 37, 
            help_file: "00037", 
            length: None, 
            mnemonic: "krtotalamount", 
            description: "This field stores a running total of any non-held portions of check deposits for the date stored in the Check Dep Total Date field.", 
            details: r#"
# Check Dep Total Amount

Field Number    037 
Mnemonic        KRTOTALAMOUNT 
Data Type       Money 
Source          System-entered 
Help File       00037 
Default Control No 
Default Value   0.00 

If the ***KR Hold Base Option*** parameter in the Regulation CC Parameters is set to any value other than ***(0) Do Not Use KR Hold Base***:

  - The system updates this field after posting each Checks Received (KR)/Checks Cashed (KC) transaction if the deposit is not held or there were non-held portions available before the transaction.
  - The system updates this field for Shared Branch/Online transactions when Online parameter is set up to use Reg CC holds, the deposit is identified as a local or non-local hold, and there were non-held portions available before the transaction posted. The system does not update this field for Shared Branch/Online transactions that are identified as immediate funds.

When you void a Checks Received (KR) transaction or a Checks Cashing (KC) transaction and the ***KR Hold Base Option*** parameter in the Regulation CC Parameters is set to any value other than ***(0) Do Not Use KR Hold Base***, the system automatically updates this field.

***Important:*** Whenever this field value changes as a direct result of posting a check, the system does not record the change in FM history. The system also does not record changes to this field in FM history when you void a deposit or payment. The system records changes to this field in the Transaction Log file, and you can recover the previous value using the Recovery Posting batch program. "#,
        }
    );

    account_fields.insert(
        "krtotaldate", 
        DatabaseField {
            data_type: DataType::Date,
            default_control: false,
            default_value: "Date Null",
            field_number: 38, 
            help_file: "00038", 
            length: None, 
            mnemonic: "krtotaldate", 
            description: "This field stores the date of the last check deposit that affected the Account record Check Dep Total Amount field or the Non-Reg CC Check Dep Total Amt field.",
            details: r#"
# Check Dep Total Date 

Field Number    038 
Mnemonic        KRTOTALDATE 
Data Type       Date 
Source          System-entered 
Help File       00038 
Default Control No 
Default Value   Date Null 

When the system processes a transaction that affects either of these two Account record fields, the system checks whether the effective date for this transaction is different from the date in the ***Check Dep Total Date*** field.

  - If the dates are different, the system updates this field with the new date and resets both the ***Check Dep Total Amount*** and the ***Non-Reg CC Check Dep Total*** fields in the Account record to zero before adding the new deposit amount to the appropriate field.
  - If the date is the same, the system adds the new deposit amount to the appropriate field.

***Important:*** Whenever this field value changes as a direct result of posting a check, the system does not record the change in FM history. The system also does not record changes to this field in FM history when you void a deposit or payment. The system records changes to this field in the Transaction Log file, and you can recover the previous value using the Recovery Posting batch program. "#,
        }
    );

    account_fields.insert(
        "nonregcccheckholdbaseamt", 
        DatabaseField {
            data_type: DataType::Money,
            default_control: true,
            default_value: "0.00",
            field_number: 64,
            help_file: "00064", 
            length: None, 
            mnemonic: "nonregcccheckholdbaseamt",
            description: "This field stores the date of the last check deposit that affected the Account record Check Dep Total Amount field or the Non-Reg CC Check Dep Total Amt field.",
            details: r#"
# Non-Reg CC Check Hold Base Amount 

Field Number    064 
Mnemonic        NONREGCCCHECKHOLDBASEAMT 
Data Type       Money 
Source          User-entered
Help File       00064 
Default Control Yes 
Default Value   0.00 

You can set a base amount per deposit for Reg CC deposits using the ***KR Hold Base Option*** parameter of Reg CC Parameters. You can only set a base amount for Non-Reg CC deposits per day.

Similarly, you can set the ***KR Hold Base Amt Option*** parameter in the Reg CC Parameters to control whether the system places a hold on the entire amount of a deposit or only the amount exceeding the base amount. You do not have that option for non-Reg CC deposits; the system always places a hold only on the amount that exceeds the base amount.

Type 0 in this field to make all non-Reg CC check deposits subject to holds. "#,
        }
    );

    account_fields.insert(
        "nonregccchecktotalamt", 
        DatabaseField {
            data_type: DataType::Money,
            default_control: true,
            default_value: "0.00",
            field_number: 65,
            help_file: "00065",
            length: None,
            mnemonic: "nonregccchecktotalamt",
            description: "This field stores a running total of all the non-Reg CC check deposits for the date stored in the Check Dep Total Date field.",
            details: r#"
# Non-Reg CC Check Dep Total Amt 

Field Number    065 
Mnemonic        NONREGCCCHECKTOTALAMT 
Data Type       Money 
Source          User-entered 
Help File       00065 
Default Control Yes 
Default Value   0.00"#,
        }
    );

    account_fields.insert(
        "enablefloat", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: true,
            default_value: "0",
            field_number: 53,
            help_file: "00053", 
            length: Some(1), 
            mnemonic: "enablefloat", 
            description: "This field stores a code that indicates whether to activate credit union floats per account.",
            details: r#"
# Enable Floats 

Field Number    053 
Mnemonic        ENABLEFLOAT 
Data Type       Code to 1 
Source          User-entered 
Help File       00053 
Default Control Yes 
Default Value   0 

When the Member Business Services Account Analysis add-on module is enabled, the system assigns these floats to check transactions affecting accounts of that type. You can change the value in this field only when your credit union purchases the Member Business Services Account Analysis module with float hold functionality.

### Data Type Descriptions
***(0) No***
Disable CU float holds.

***(1) Yes***
Enable CU float holds as defined in Analysis CU Float Parameters.

***Important:*** The default for this field can be assigned at the Account level by account type in Account Defaults in the Default Manager. This assignment provides float information for non-analysis accounts, which can be accessed through a PowerOn specfile. "#,
        }
    );

    account_fields.insert(
        "currentrelationshipcode", 
        DatabaseField {
            data_type: DataType::Code, 
            default_control: false, 
            default_value: "0", 
            field_number: 808, 
            help_file: "000808", 
            length: Some(99), 
            mnemonic: "currentrelationshipcode", 
            description: "This calculated field contains a code that specifies the member's current relationship to your credit union.",
            details: r#"
# Current Relationship Code 

Field Number    808 
Mnemonic        CURRENTRELATIONSHIPCODE 
Data Type       Code to 99 
Source          System-calculated 
Help File       000808 
Default Control No 
Default Value   0 

The system enters a system-calculated code in this field (0–99) that appears in various inquiries and calculations. 

  - The system sets the value in this field to the same as the value in the ***Relationship Override*** field when all the following criteria are met: 
    - There is a value in the ***Rel Override Exp Date*** field. 
    - The system date is on or after the value in the ***Rel Override Eff Date*** field. 
    - The system date is before the value in the ***Rel Override Exp Date*** field.
  - Otherwise, the system sets the value in this field to the same as the value in the ***Relationship Code*** field. 

You cannot perform direct file maintenance of this field. To change the value of this field, you must perform the appropriate file maintenance in the ***Relationship Override, Rel Override Eff Date***, and ***Rel Override Exp Date*** fields. "#,
        }
    );

    account_fields.insert(
        "relationshipcode", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: true,
            default_value: "0",
            field_number: 032,
            help_file: "00032",
            length: Some(99),
            mnemonic: "relationshipcode",
            description: "This field stores a code that specifies the member's base relationship to your credit union.",
            details: r#"
# Relationship Code 

Field Number    032 
Mnemonic        RELATIONSHIPCODE 
Data Type       Code to 99 
Source          System-entered 
Help File       00032 
Default Control Yes 
Default Value   0 

The system enters a code in this field (0–99) that appears in various inquiries and calculations. "#,
        }
    );

    account_fields.insert(
        "relationshipoverride", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: true, 
            default_value: "0", 
            field_number: 033, 
            help_file: "00033", 
            length: Some(99), 
            mnemonic: "relationshipoverride", 
            description: "This field stores a code that specifies a relationship code that overrides the base relationship code until the relationship override expires.",
            details: r#"
# Relationship Override 

Field Number    033 
Mnemonic        RELATIONSHIPOVERRIDE 
Data Type       Code to 99 
Source          User-entered 
Help File       00033 
Default Control Yes 
Default Value   0 

Enter a code in this field (0–99) that appears in various inquiries and calculations. "#,
        }
    );

    account_fields.insert(
        "relationshipoverrideeffdate", 
        DatabaseField {
            data_type: DataType::Date, 
            default_control: true, 
            default_value: "Date Null",
            field_number: 034, 
            help_file: "00034", 
            length: None,
            mnemonic: "relationshipoverrideeffdate",
            description: "This field stores the date that specifies when the Relationship Override field becomes effective.",
            details: r#"
# Relationship Override Eff Date 

Field Number    034 
Mnemonic        RELATIONSHIPOVERRIDEEFFDATE 
Data Type       Date 
Source          User-entered 
Help File       00034 
Default Control Yes 
Default Value   Date Null "#,
        }
    );

    account_fields.insert(
        "relationshipoverrideexpdate", 
        DatabaseField {
            data_type: DataType::Date,
            default_control: true,
            default_value: "Date Null",
            field_number: 035,
            help_file: "00035",
            length: None,
            mnemonic: "relationshipoverrideexpdate",
            description: "This field stores the date that specifies when the Relationship Override field expires.",
            details: r#"
# Relationship Override Exp Date 

Field Number    035 
Mnemonic        RELATIONSHIPOVERRIDEEXPDATE 
Data Type       Date 
Source          User-entered 
Help File       00035 
Default Control Yes 
Default Value   Date Null "#,
        }
    );

    account_fields.insert(
        "headofhousehold",
        DatabaseField {
            data_type: DataType::Code,
            default_control: true,
            default_value: "0",
            field_number: 016,
            help_file: "00016",
            length: Some(1),
            mnemonic: "headofhousehold",
            description: "This field stores a code that specifies whether this account is used as a head of household account to consolidate mailings or target marketing campaigns.",
            details: r#"
# Head of Household 

Field Number    016 
Mnemonic        HEADOFHOUSEHOLD 
Data Type       Code to 1 
Source          User-entered 
Help File       00016 
Default Control Yes 
Default Value   0 

### Data Type Descriptions
***(0) Head of Household***
Use this account as a head of household account.

***(1) Other Family Member***
Do not use this account as a head of household account.

You can set this field automatically using the ***Head of Household Matching*** option of the Miscellaneous Processing batch program. This option matches accounts if the primary members live at the same address and have the same last name. The address and last name must be spelled exactly the same in both primary Name records, or a match does not occur.

In addition, the ***Head of Household*** fields in both Account records must be set to ***(0) Head of Household***. The ***Head of Household Matching*** option ignores Account records if the ***Head of Household*** field is set to ***(1) Other Family Member***. Whenever the program makes a match, the system sets the Head of Household field in both Account records.

You can access the ***Head of Household*** field with PowerOn to target marketing campaigns to heads of household only, avoiding duplicate mailings to the same household. "#,
        }
    );

    account_fields.insert(
        "householdaccount", 
        DatabaseField {
            data_type: DataType::Character,
            default_control: false,
            default_value: "",
            field_number: 025,
            help_file: "00025",
            length: Some(10),
            mnemonic: "householdaccount",
            description: "This field stores the related head of household account, if the value in the Head of Household field is set to (1) Other family member.",
            details: r#"
# Household Account 

Field Number    025 
Mnemonic        HOUSEHOLDACCOUNT 
Data Type       10 Characters 
Source          User-entered or System-entered 
Help File       00025 
Default Control No 
Default Value   (Blank) 

  - Leave this field blank if this is the head of household account. You should also select ***(0) Head of Household*** in the Head of Household field.
  - Enter the account number of the head of household account if this is not the head of household account. You should also select ***(1) Other family member*** in the ***Head of Household*** field.

When the household account number does not correspond to an existing account, the system ignores the household account number and sends mail to the address in the Account record.

Whenever you perform file maintenance on this field, the system automatically creates or deletes the appropriate Household record in the head of household account. Whenever you create or delete a Household record in the head of household account, the system automatically fills in or clears this field in the account referenced by the Household record. You only need to perform file maintenance of one account.
            "#,
        }
    );

    account_fields.insert(
        "householdstatement", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: false, 
            default_value: "0",
            field_number: 026, 
            help_file: "00026", 
            length: Some(1), 
            mnemonic: "householdstatement", 
            description: "This field stores a code that indicates if you want to send multiple statements to one household in a single envelope.",
            details: r#"
# Household Statement 

Field Number    026 
Mnemonic        HOUSEHOLDSTATEMENT 
Data Type       Code to 1 
Source          User-entered 
Help File       00026 
Default Control No 
Default Value   0 

Do not use this field for head of household accounts (the ***Head of Household*** field is set to ***0***). If this is not the head of household account (the ***Head of Household*** field is set to ***1***), choose one of the following:

### Data Type Descriptions
***(0) Do not consolidate statement***
Do not group this statement with the head of household statement for mailing.

***(1) Group with the head of household***
Group this statement with the head of household statement for mailing. "#,
        }
    );

    account_fields.insert(
        "statementmailcode", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: true,
            default_value: "0",
            field_number: 031,
            help_file: "00031",
            length: Some(99),
            mnemonic: "statementmailcode",
            description: "This field stores a code that indicates what mail code to use to for mailing statements.",
            details: r#"
# Statement Mail Code 

Field Number    031 
Mnemonic        STATEMENTMAILCODE 
Data Type       Code to 99 
Source          User-entered 
Help File       00031 
Default Control Yes 
Default Value   0 

### Data Type Descriptions
***(0) Use individual mail codes***
The system honors the credit union-defined mail codes in the Share and Loan records.

***1–99
This credit union-defined mail code in the Account record overrides the statement mail codes in the Share and Loan records.

***Important:*** If custom text descriptions for each mail code in Statement Mail Code Names Parameters have been defined, those text descriptions display in the drop-down menu when revising this field to help you select the correct mail code (as defined by your credit union). The system displays the description and the mail code in the field; however, only the mail code is included on the member's statement.

If you enter a mail code that is defined as No Mail, statements for the entire account are not mailed.

***Tip:*** Statement mail codes indicate if statements should be mailed to the member or, if not, why not. "#,
        }
    );

    account_fields.insert(
        "estmtnotify", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: true,
            default_value: "0",
            field_number: 039,
            help_file: "00039",
            length: Some(1),
            mnemonic: "estmtnotify",
            description: "This field stores a code that specifies whether the member would like to receive e-statements for home banking products.",
            details: r#"
# E-Statement Notify 

Field Number    039 
Mnemonic        ESTMTNOTIFY 
Data Type       Code to 1 
Source          User-entered 
Help File       00039 
Default Control Yes 
Default Value   0 

### Data Type Descriptions
***(0) No E-mail notification***
The user should not receive email notification that an e-statement is available.

***(1) Notify by E-mail***
The user should receive email notification that an e-statement is available. "#,
        }
    );

    account_fields.insert(
        "estmtenable", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: true,
            default_value: "0",
            field_number: 040,
            help_file: "00040",
            length: Some(2),
            mnemonic: "estmtenable",
            description: "This field stores a code that indicates if the member would like to receive e-statements for home banking products.",
            details: r#"
# E-Statement Enable 

Field Number    040 
Mnemonic        ESTMTENABLE 
Data Type       Code to 2    
Source          User-entered 
Help File       00040 
Default Control Yes 
Default Value   0 

### Data Type Descriptions
***(0) E-Statement not enabled***
The member does not want to receive an e-statement.

***(1) Enable E-Statement only***
The member wants to receive an e-statement but not a printed statement.

***(2) Enable both statements***
The member wants to receive both an e-statement and a printed statement. "#,
        }
    );

    account_fields.insert(
        "statereporting", 
        DatabaseField {
            data_type: DataType::Character,
            default_control: true,
            default_value: "",
            field_number: 024,
            help_file: "00024",
            length: Some(2),
            mnemonic: "statereporting",
            description: "This field stores the state code to report dividends to for this member.",
            details: r#"
# State Reporting 

Field Number    024 
Mnemonic        STATEREPORTING 
Data Type       2 Characters 
Source          User-entered 
Help File       00024 
Default Control Yes 
Default Value   (Blank) 

  - Leave the field blank to report dividends to the default state (the state identified in the ***State Reporting*** parameter in the Miscellaneous Parameters).
  - Enter a valid two-character state code to report dividends to a different state.

***Important:*** This field is also used to determine a default state withholding amount when performing a withdrawal from a tax-deferred share. Enter a value here to override the value in the ***State Reporting*** parameter in the Miscellaneous Parameters. The system then calculates and displays a default withholding amount.

  - If you type any of the following state codes, the system automatically fills in the State Withholding Percent value used for the IRS distribution calculation:
    - ***AR*** (Arkansas)
    - ***CA*** (California)
    - ***DC*** (District of Columbia)
    - ***KS*** (Kansas)
    - ***MI*** (Michigan)
    - ***MO*** (Missouri)
    - ***NC*** (North Carolina)
    - ***OK*** (Oklahoma)
    - ***OR*** (Oregon)
    - ***VT*** (Vermont)
  - If you type ME (Maine), the system automatically fills in the amount for the state backup withholding calculation when dividends are posted to shares. "#,
        }
    );

    account_fields.insert(
        "createdbyuser", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: false,
            default_value: "Current user number",
            field_number: 060,
            help_file: "00060",
            length: Some(9999),
            mnemonic: "createdbyuser",
            description: "This field stores the user ID and is populated when the record is created.",
            details: r#"
# Created By User 

Field Number    060 
Mnemonic        CREATEDBYUSER 
Data Type       Code to 9999 
Source          System-entered 
Help File       00060 
Default Control No 
Default Value   Current user number 

### Data Type Descriptions 
***0–9998***
The user ID of the creator of the record.

***9999***
The system is unable to determine the information because it is not available.

Under normal circumstances, you should not revise this field.

***Tip:*** This field is automatically filled with the User ID upon creation of an Account, Loan, Application, or Card record. "#,
        }
    );

    account_fields.insert(
        "createdatbranch", 
        DatabaseField {
            data_type: DataType::Code,
            default_control: false,
            default_value: "Current user branch",
            field_number: 061,
            help_file: "00061",
            length: Some(9999),
            mnemonic: "createdatbranch",
            description: "This field stores the branch ID of the branch where the record is created.",
            details: r#"
# Created At Branch 

Field Number    061 
Mnemonic        CREATEDATBRANCH 
Data Type       Code to 9999 
Source          System-entered 
Help File       00061 
Default Control No 
Default Value   Current user branch 

### Data Type Descriptions 
***0***
The system is unable to determine the information because it is not available.

***1–9999***
The branch ID where the record was created.

Under normal circumstances, you should not revise this field. "#,
        }
    );

    account_fields.insert(
        "crtotalamount", 
        DatabaseField {
            data_type: DataType::Money,
            default_control: false,
            default_value: "0.00",
            field_number: 041,
            help_file: "00041",
            length: None,
            mnemonic: "crtotalamount",
            description: "This field stores the total cash received from a Cash Received (CR) transaction, ATM deposit, or Shared Branch Issuer transactions.",
            details: r#"
# US Cash Rcvd Amount

Field Number    0041 
Mnemonic        CRTOTALAMOUNT 
Data Type       Money 
Source          System-entered 
Help File       00041 
Default Control No 
Default Value   0.00 

After the system posts a Cash Received (CR) transaction, it adds the associated amount to this field. The system checks this field to determine if the amount is greater than or equal to the thresholds for suspicious activity or if it is over the IRS-established limit of $10,000. For a total over that amount, the IRS requires you to prepare a Currency Transaction Report (CTR).

This total also includes qualifying ATM deposits and Shared Branch Issuer transactions. During a Shared Branch deposit transaction or a Shared Branch loan payment, the amount is updated by a corresponding message if funds are marked as cash. The system considers non-envelope cash deposits as qualifying ATM deposits for credit unions using the latest model NCR ATMs.

The system sets the totals for this field to $0.00 if the ***New Banking Day*** prompt is set to ***Yes*** when you run the Daily Posting Batch Program.

***Important:*** Whenever this field value changes as a direct result of posting a Cash Received (CR) teller transaction, the system does not record the change in FM history. The system also does not record changes to this field in FM history when you void a deposit or payment associated with a Cash Received (CR) teller transaction. The system records changes to this field in the transaction log file, and you can recover the previous value using the Recovery Posting batch program. "#,
        }
    );

    account_fields.insert(
        "cdtotalamount", 
        DatabaseField {
            data_type: DataType::Money,
            default_control: false,
            default_value: "0.00",
            field_number: 042,
            help_file: "00042",
            length: None,
            mnemonic: "cdtotalamount",
            description: "This field stores the total cash distributed with a Cash Disbursed (CD) transaction, ATM withdrawals, or Shared Branch Issuer transactions.",
            details: r#"
# US Cash Disb Amount 

Field Number    0042 
Mnemonic        CDTOTALAMOUNT 
Data Type       Money 
Source          System-entered 
Help File       00042 
Default Control No 
Default Value   0.00 

After the system posts a Cash Disbursed (CD) transaction, it adds the associated amount to this field. The system checks this field to determine if the amount is greater than or equal to the thresholds for suspicious activity or past the IRS-established limit of $10,000. For a total beyond that amount, the IRS requires you to prepare a Currency Transaction Report (CTR). For more information on currency reporting, see the IRS site.

This total also includes qualifying ATM withdrawals and Shared Branch Issuer transactions.

The system sets the totals for this field to $0.00 if the ***New Banking Day*** prompt is set to ***Yes*** when you run the Daily Posting Batch Program.

***Important:*** Whenever this field value changes as a direct result of posting a Cash Disbursed (CD) teller transaction, the system does not record the change in FM history. The system records changes to this field in the transaction log file, and you can recover the previous value using the Recovery Posting batch program. "#,
        }
    );

    account_fields
}

/*
*/
