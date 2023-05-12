use std::collections::HashMap;

use lazy_static::lazy_static;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation, InsertTextFormat};

pub struct PoweronFunction {
    pub label: &'static str,
    pub insert_text: &'static str,
    pub kind: CompletionItemKind,
    pub description: &'static str,
    pub is_snippet: bool,
}

lazy_static! {
    pub static ref POWERON_FUNCTION_COMPLETIONS: HashMap<&'static str, CompletionItem> =
        load_poweron_functions();
}

fn load_poweron_functions() -> HashMap<&'static str, CompletionItem> {
    let mut poweron_function_completions: HashMap<&'static str, CompletionItem> = HashMap::new();

    poweron_function_completions.insert(
        "ABS",
        CompletionItem {
            label: "ABS".to_string(),
            insert_text: Some("ABS(${1:expression})$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the absolute value of an expression.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "ANYSERVICE",
        CompletionItem {
            label: "ANYSERVICE".to_string(),
            insert_text: Some(
                "ANYSERVICE(${1|SHARE,LOAN,CARD,EXTERNALLOAN|},${2:<1-99>})$0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function evaluates to Boolean TRUE if the particular service code exists on the specified record.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "ANYWARNING",
        CompletionItem {
            label: "ANYWARNING".to_string(),
            insert_text: Some(
                "ANYWARNING(${1|ACCOUNT,SHARE,LOAN,CARD,EXTERNALLOAN|},${2:<1-999>})$0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function evaluates to Boolean TRUE if a particular warning code exists on the specified record.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "CAPITALIZE",
        CompletionItem {
            label: "CAPITALIZE".to_string(),
            insert_text: Some("CAPITALIZE(${1:CharacterExpression})$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function translates the first character of each word to uppercase and translates all other alphabetic characters to lowercase.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "CHARACTERREAD",
        CompletionItem {
            label: "CHARACTERREAD".to_string(),
            insert_text: Some("CHARACTERREAD(\"${1:Prompt}\")$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "Displays a prompt on the user's console and returns the character string entered by the user".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "CHARACTERSEARCH",
        CompletionItem {
            label: "CHARACTERSEARCH".to_string(),
            insert_text: Some(
                "CHARACTERSEARCH(${1:ExpressionToSearchWithin},${2:ExpressionToSearchFor})$0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the position of the first occurrence of one character sequence within another character sequence.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "CHRVALUE",
        CompletionItem {
            label: "CHRVALUE".to_string(),
            insert_text: Some("CHRVALUE(${1:Character})$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the numeric decimal ASCII value of a single character."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "CODEREAD",
        CompletionItem {
            label: "CODEREAD".to_string(),
            insert_text: Some("CODEREAD(\"${1:Prompt}\")$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "Displays the specified prompt on the operator's console, waits for a code response, and returns operator's response as the value of CODEREAD.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "COL",
        CompletionItem {
            label: "COL".to_string(),
            insert_text: Some("COL=${1:X} $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function specifies which column should contain output information."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "COPYAPP",
        CompletionItem {
            label: "COPYAPP".to_string(),
            insert_text: Some(
                "COPYAPP(${1:SourceAppID},${2:DestAcct},${3:DestAppID},${4:MoveFlag},${5:PersonFlag},${6:FinFlag},${7:TrackingFlag},${8:NoteFlag},${9:PreferenceFlag},${10:CBIFlag},${11:ErrorText})$0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function copies or moves fields from an Application record and its child records from one ID on an account to another ID on the same or different account. In addition, you can copy or move associated Credit Report record fields and their child records at the same time an Application is copied or moved.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "CREATEFINANCEFROMCREDREP",
        CompletionItem {
            label: "CREATEFINANCEFROMCREDREP".to_string(),
            insert_text: Some(
                "CREATEFINANCEFROMCREDREP(${1:CheckPrivsFlag},${2:AppID},${3:CreditReportLocator},${4:SkipBlankDescriptionFlag},${5:SkipZeroBalanceFlag},${6:ErrorText})$0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function creates Finance records for a loan application based on trade items in a credit report.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "CTRLCHR",
        CompletionItem {
            label: "CTRLCHR".to_string(),
            insert_text: Some("CTRLCHR(${1:expression})$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function lets you send non-printing control characters to a laser printer to take advantage of special features such as fonts, bold type, and boxes.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DATASIZE",
        CompletionItem {
            label: "DATASIZE".to_string(),
            insert_text: Some("DATASIZE=${1:<1-132>} ${2:expression} $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function populates the data file report generated from a DATAFILE specfile."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DATE",
        CompletionItem {
            label: "DATE".to_string(),
            insert_text: Some("DATE(${1:MonthExpression},${2:DayExpression},${3:YearExpression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function converts a series of three numeric expressions into a single date value".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DATEOFFSET",
        CompletionItem {
            label: "DATEOFFSET".to_string(),
            insert_text: Some("DATEOFFSET(${1:StartDate},${2:MonthCount},${3:DayCount},${4:YearCount})$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns a date value that is offset from the specified date value by the specified number of months, days, and years.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DATEREAD",
        CompletionItem {
            label: "DATEREAD".to_string(),
            insert_text: Some("DATEREAD(\"${1:Prompt}\") $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console and returns the date entered.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DATEVALUE",
        CompletionItem {
            label: "DATEVALUE".to_string(),
            insert_text: Some("DATEVALUE(${1:expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function converts dates stored as character data to the DATE data type."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DAY",
        CompletionItem {
            label: "DAY".to_string(),
            insert_text: Some("DAY(${1:DateExpression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the numerical value for the day in a date field or a date variable.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DAYOFWEEK",
        CompletionItem {
            label: "DAYOFWEEK".to_string(),
            insert_text: Some("DAYOFWEEK(${1:expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns a numeric value from 0-6 representing the day of the week for a date expression.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGPROMPTCHAR",
        CompletionItem {
            label: "DIALOGPROMPTCHAR".to_string(),
            insert_text: Some("DIALOGPROMPTCHAR(\"${1:Prompt}\",${2:MaxLength},${3:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays the text for a character data prompt in an interactive window.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGPROMPTCODE",
        CompletionItem {
            label: "DIALOGPROMPTCODE".to_string(),
            insert_text: Some(
                "DIALOGPROMPTCODE(\"${1:Prompt}\",${2:MaxValue},${3:Default}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays the text for a code data prompt in an interactive window."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGPROMPTCOMBOOPTION",
        CompletionItem {
            label: "DIALOGPROMPTCOMBOOPTION".to_string(),
            insert_text: Some("DIALOGPROMPTCOMBOOPTION(${1:Value},${2:Text}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function adds options to a drop-down list of choices in an interactive window.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGPROMPTCOMBOSTART",
        CompletionItem {
            label: "DIALOGPROMPTCOMBOSTART".to_string(),
            insert_text: Some("DIALOGPROMPTCOMBOSTART(\"${1:Prompt}\",${2:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function indicates the beginning of a drop-down list of options in an interactive window.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGPROMPTMONEY",
        CompletionItem {
            label: "DIALOGPROMPTMONEY".to_string(),
            insert_text: Some("DIALOGPROMPTMONEY(\"${1:Prompt}\",${2:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt for the user to enter a value.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGPROMPTNUMBER",
        CompletionItem {
            label: "DIALOGPROMPTNUMBER".to_string(),
            insert_text: Some("DIALOGPROMPTNUMBER(\"${1:Prompt}\",${2:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a user prompt for a number.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGPROMPTPASSWORD",
        CompletionItem {
            label: "DIALOGPROMPTPASSWORD".to_string(),
            insert_text: Some(
                "DIALOGPROMPTPASSWORD(\"${1:Prompt}\",${2:MaxLength},${3:Default}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt for the user to enter a password.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGPROMPTRATE",
        CompletionItem {
            label: "DIALOGPROMPTRATE".to_string(),
            insert_text: Some("DIALOGPROMPTRATE(\"${1:Prompt}\",${2:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays the text for a rate prompt in an interactive window."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGPROMPTYESNO",
        CompletionItem {
            label: "DIALOGPROMPTYESNO".to_string(),
            insert_text: Some("DIALOGPROMPTYESNO(\"${1:Prompt}\",${2:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: None,
            documentation: Some(Documentation::String(
                "This function displays a drop-down list for a yes or no response in an interactive window.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGSTART",
        CompletionItem {
            label: "DIALOGSTART".to_string(),
            insert_text: Some(
                "DIALOGSTART(\"${1:DialogTitle}\",${2:WHRatio},${3|0,1|}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function initiates a new prompt and response sequence in an interactive window.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGSTARTGROUPBOX",
        CompletionItem {
            label: "DIALOGSTARTGROUPBOX".to_string(),
            insert_text: Some("DIALOGSTARTGROUPBOX(\"${1:Text}\") $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function begins a group of prompts and responses in an interactive window."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGTEXTLISTOPTION",
        CompletionItem {
            label: "DIALOGTEXTLISTOPTION".to_string(),
            insert_text: Some("DIALOGTEXTLISTOPTION(\"${1:Text}\") $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function adds options as lines of data in a read-only list box contained in a pop-up window.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIALOGTEXTLISTSTART",
        CompletionItem {
            label: "DIALOGTEXTLISTSTART".to_string(),
            insert_text: Some("DIALOGTEXTLISTSTART(\"${1:Text}\") $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function indicates the beginning of a list box of read-only data lines contained in a pop-up window.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIM",
        CompletionItem {
            label: "DIM".to_string(),
            insert_text: Some("DIM $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the screen to normal display after characters are displayed with the BRIGHT command.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "DIVPROJECTINIT",
        CompletionItem {
            label: "DIVPROJECTINIT".to_string(),
            insert_text: Some("DIVPROJECTINIT(${1|0,1,2|},${2:ParamDefaultType}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function initializes the setup variables required for performing annual percentage yield (APY) calculations and dividend projections.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "EMAILLINE",
        CompletionItem {
            label: "EMAILLINE".to_string(),
            insert_text: Some("EMAILLINE(${1:EmailLine},${2:ErrorText}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function sends parameters or adds a line of text in an email message."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "EMAILSEND",
        CompletionItem {
            label: "EMAILSEND".to_string(),
            insert_text: Some("EMAILSEND(${1:ErrorText}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function transmits an electronic message from within a specfile if your system is configured for email.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "EMAILSTART",
        CompletionItem {
            label: "EMAILSTART".to_string(),
            insert_text: Some(
                "EMAILSTART(${1:FromAddress},${2:ToAddress},${3:Subject},${4:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function initiates an email message if your system is configured for email."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "ENTERCHARACTER",
        CompletionItem {
            label: "ENTERCHARACTER".to_string(),
            insert_text: Some(
                "ENTERCHARACTER(\"${1:Prompt}\",${2:MaxLength},${3:Default}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console while running an on-demand specfile and waiting for a character response from the operator.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "ENTERCODE",
        CompletionItem {
            label: "ENTERCODE".to_string(),
            insert_text: Some(
                "ENTERCODE(\"${1:Prompt}\",${2:MaxValue},${3:Default}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console and returns the code entered.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "ENTERDATE",
        CompletionItem {
            label: "ENTERDATE".to_string(),
            insert_text: Some("ENTERDATE(\"${1:Prompt}\",${2:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console and returns the date entered.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "ENTERMONEY",
        CompletionItem {
            label: "ENTERMONEY".to_string(),
            insert_text: Some("ENTERMONEY(\"${1:Prompt}\",${2:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console and returns the monetary value entered.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "ENTERNUMBER",
        CompletionItem {
            label: "ENTERNUMBER".to_string(),
            insert_text: Some(
                "ENTERNUMBER(\"${1:Prompt}\",${2:MaxValue},${3:Default}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console and returns the number entered.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "ENTERRATE",
        CompletionItem {
            label: "ENTERRATE".to_string(),
            insert_text: Some("ENTERRATE(\"${1:Prompt}\",${2:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console and returns the rate entered.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "ENTERYESNO",
        CompletionItem {
            label: "ENTERYESNO".to_string(),
            insert_text: Some("ENTERYESNO(\"${1:Prompt}\",${2:Default}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console and returns a Y (yes) or N (no) response.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "EXECUTE",
        CompletionItem {
            label: "EXECUTE".to_string(),
            insert_text: Some(
                "EXECUTE(\"${1:SubroutineSpecfileName}\",${2:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function initiates the running of a subroutine specfile.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "EXP",
        CompletionItem {
            label: "EXP".to_string(),
            insert_text: Some("EXP(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the value of the mathematical constant e raised to a specified power.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILEARCHIVEADD",
        CompletionItem {
            label: "FILEARCHIVEADD".to_string(),
            insert_text: Some(
                "FILEARCHIVEADD(${1:ArchiveType},${2:ArchiveName},${3:FileName},${4:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function adds a file to an archive.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILEARCHIVEEXTRACT",
        CompletionItem {
            label: "FILEARCHIVEEXTRACT".to_string(),
            insert_text: Some(
                "FILEARCHIVEEXTRACT(${1:ArchiveType},${2:ArchiveName},${3:DestinationFileType},${4:DestinationFileName},${5:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function retrieves a file from an archive.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILECLOSE",
        CompletionItem {
            label: "FILECLOSE".to_string(),
            insert_text: Some("FILECLOSE(${1:FileNumber},${2:ErrorText}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function closes file identified in the first argument.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILECREATE",
        CompletionItem {
            label: "FILECREATE".to_string(),
            insert_text: Some(
                "FILECREATE(${1:FileType},${2:FileName},${3:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function creates a new letter file, help file, PowerOn specfile, or edit file. Once the file is created, you must use the FILEOPEN function to access it.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILEDECRYPT",
        CompletionItem {
            label: "FILEDECRYPT".to_string(),
            insert_text: Some(
                "FILEDECRYPT(${1:FileType},${2:FileName},${3:DecryptedFileName},${4:KeyFileName},${5:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function decodes or deciphers files protected by encryption.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILEGETPOS",
        CompletionItem {
            label: "FILEGETPOS".to_string(),
            insert_text: Some(
                "FILEGETPOS(${1:FileNumber},${2:FilePosition},${3:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function retrieves the current byte position in the text file just read from or written to.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILELISTCLOSE",
        CompletionItem {
            label: "FILELISTCLOSE".to_string(),
            insert_text: Some("FILELISTCLOSE(${1:ErrorText}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function closes a previously opened list of files.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILELISTOPEN",
        CompletionItem {
            label: "FILELISTOPEN".to_string(),
            insert_text: Some(
                "FILELISTOPEN(${1:FileType},${2:Template},${3:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function opens a file list you want to read.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILELISTREAD",
        CompletionItem {
            label: "FILELISTREAD".to_string(),
            insert_text: Some("FILELISTREAD(${1:FileName},${2:ErrorText}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function reads the next file name from a file list.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILEOPEN",
        CompletionItem {
            label: "FILEOPEN".to_string(),
            insert_text: Some(
                "FILEOPEN(${1:FileType},${2:FileName},${3:OpenMode},${4:FileNumber},${5:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function opens a file of the specified file type for processing in the selected open mode.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILEREAD",
        CompletionItem {
            label: "FILEREAD".to_string(),
            insert_text: Some(
                "FILEREAD(${1:FileNumber},${2:NumberOfCharacters},${3:CharacterVariable},${4:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function reads an open file and saves the read data in a variable.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILEREADLINE",
        CompletionItem {
            label: "FILEREADLINE".to_string(),
            insert_text: Some(
                "FILEREADLINE(${1:FileNumber},${2:TextLine},${3:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function reads a line of text from a file and then stores that line in the second argument.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILESETPOS",
        CompletionItem {
            label: "FILESETPOS".to_string(),
            insert_text: Some(
                "FILESETPOS(${1:FileNumber},${2:FilePosition},${3:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function sets the current byte position in the text file just read from or written to.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILEWRITE",
        CompletionItem {
            label: "FILEWRITE".to_string(),
            insert_text: Some(
                "FILEWRITE(${1:FileNumber},${2:Text},${3:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function writes the text in the second argument to the file identified in the first argument.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FILEWRITELINE",
        CompletionItem {
            label: "FILEWRITELINE".to_string(),
            insert_text: Some(
                "FILEWRITELINE(${1:FileNumber},${2:CharacterData},${3:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function locates the file number specified and writes a line of text beginning at the current byte position.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FLOAT",
        CompletionItem {
            label: "FLOAT".to_string(),
            insert_text: Some("FLOAT(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function converts a number, money, rate, date, or code value into its equivalent floating point value. It is intended for use on an entire numeric expression. It is useful for assigning a non-floating point value to a float variable. If you use this function on part of a compound expression, it can have unpredictable results.".to_string())),
            ..CompletionItem::default()
        },

    );

    poweron_function_completions.insert(
        "FLOATVALUE",
        CompletionItem {
            label: "FLOATVALUE".to_string(),
            insert_text: Some(
                "FLOATVALUE(${1:CharacterString},${2:FLOATVARIABLE},${3:ErrorPosition}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function converts a character string to a floating point value. It is useful for assigning a non-floating point value to a float variable. If you use this function on part of a compound expression, it can have unpredictable results.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FLOOR",
        CompletionItem {
            label: "FLOOR".to_string(),
            insert_text: Some("FLOOR(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the largest integer less than or equal to the expression."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FORMAT",
        CompletionItem {
            label: "FORMAT".to_string(),
            insert_text: Some(
                "FORMAT(${1:Expression},${2:FormatString}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function formats a numeric expression according to the format string specified in the second argument.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FTPCLOSE",
        CompletionItem {
            label: "FTPCLOSE".to_string(),
            insert_text: Some("FTPCLOSE(${1:Handle},${2:ErrorText}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function closes an FTP session.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FTPCMD",
        CompletionItem {
            label: "FTPCMD".to_string(),
            insert_text: Some(
                "FTPCMD(${1:Handle},${2:Command},${3:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function sends a command directly to the FTP server and receives the response."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FTPGET",
        CompletionItem {
            label: "FTPGET".to_string(),
            insert_text: Some(
                "FTPGET(${1:Handle},${2:SourceFileName},${3:DestFileType},${4:DestFileName},${5:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function retrieves a file from the FTP server.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FTPLOGIN",
        CompletionItem {
            label: "FTPLOGIN".to_string(),
            insert_text: Some(
                "FTPLOGIN(${1:Handle},${2:UserName},${3:Password},${4:ErrorText}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function logs in to an FTP server.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FTPOPEN",
        CompletionItem {
            label: "FTPOPEN".to_string(),
            insert_text: Some(
                "FTPOPEN(${1:ServerName},${2:UserName},${3:Password},${4:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function opens an FTP session.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FTPPUT",
        CompletionItem {
            label: "FTPPUT".to_string(),
            insert_text: Some(
                "FTPPUT(${1:Handle},${2:SourceFileName},${3:DestFileName},${4:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function sends a file to the FTP server.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "FULLYEAR",
        CompletionItem {
            label: "FULLYEAR".to_string(),
            insert_text: Some("FULLYEAR(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns a numerical value (from 1900-2078) equivalent to the four-digit year in a date expression."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETDATACHAR",
        CompletionItem {
            label: "GETDATACHAR".to_string(),
            insert_text: Some(
                "GETDATACHAR(${1:InfoCode},${2:Type1..4}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function instructs PowerOn to retrieve the current value of an accessible character field in the Parameter or Console file. GETDATACHAR and GETDATACHARACTER are equivalent keywords."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETDATADATE",
        CompletionItem {
            label: "GETDATADATE".to_string(),
            insert_text: Some(
                "GETDATADATE(${1:InfoCode},${2:Type1..4}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function retrieves the current value of an accessible date field in the Parameter or Console file."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETDATAMONEY",
        CompletionItem {
            label: "GETDATAMONEY".to_string(),
            insert_text: Some(
                "GETDATAMONEY(${1:InfoCode},${2:Type1..4}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function retrieves the current value of an accessible money field in the Parameter or Console file."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETDATANUMBER",
        CompletionItem {
            label: "GETDATANUMBER".to_string(),
            insert_text: Some(
                "GETDATANUMBER(${1:InfoCode},${2:Type1..4}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function retrieves the current value of an accessible numeric field in the Parameter or Console file."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETDATARATE",
        CompletionItem {
            label: "GETDATARATE".to_string(),
            insert_text: Some(
                "GETDATARATE(${1:InfoCode},${2:Type1..4}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function retrieves the current value of an accessible rate field in the Parameter or Console file.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETFIELDNAME",
        CompletionItem {
            label: "GETFIELDNAME".to_string(),
            insert_text: Some(
                "GETFIELDNAME(${1:RecordNumber},${2:FieldNumber},${3:SubfieldNumber}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("Takes a record number or integer expression and a field mnemonic or character expression, and returns the corresponding field number. Use this function to obtain the field number for a particular field. The field number is required for all the other GETFIELD functions. If any of the passed values are invalid, this function returns 0.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETFIELDNUMBER",
        CompletionItem {
            label: "GETFIELDNUMBER".to_string(),
            insert_text: Some(
                "GETFIELDNUMBER(${1:RecordNumber},${2:FieldMnemonic}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("Takes a record number or integer expression and a field mnemonic or character expression, and returns the corresponding field number. Use this function to obtain the field number for a particular field. The field number is required for all the other GETFIELD functions. If any of the passed values are invalid, this function returns 0.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETFIELDMNEMONIC", 
        CompletionItem {
            label: "GETFIELDMNEMONIC".to_string(),
            insert_text: Some("GETFIELDMNEMONIC(${1:RecordNumber},${2:FieldNumber},${3:SubfieldNumber}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("Takes a record number or integer expression and a field number or integer expression, and returns the corresponding field mnemonic. Use this function to obtain the field mnemonic for a particular field. The field mnemonic is required for all the other GETFIELD functions. If any of the passed values are invalid, this function returns a null string.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETFIELDHELPFILE",
        CompletionItem {
            label: "GETFIELDHELPFILE".to_string(),
            insert_text: Some(
                "GETFIELDHELPFILE(${1:RecordNumber},${2:FieldNumber},${3:SubfieldNumber}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("Takes a record number or integer expression, field number or integer expression, and subfield number or integer expression and returns the corresponding numeric help file number. Pass 0 for the subfield number if not required. If any of the passed values are invalid, this function returns 0.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETFIELDDATATYPE",
        CompletionItem {
            label: "GETFIELDDATATYPE".to_string(),
            insert_text: Some(
                "GETFIELDDATATYPE(${1:RecordNumber},${2:FieldNumber},${3:SubfieldNumber}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("Takes a record number or integer expression, field number or integer expression, and subfield number or integer expression and returns the corresponding numeric data type. Pass 0 for the subfield number if not required. If any of the passed values are invalid, this function returns 0.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "GETFIELDDATAMAX",
        CompletionItem {
            label: "GETFIELDDATAMAX".to_string(),
            insert_text: Some(
                "GETFIELDDATAMAX(${1:RecordNumber},${2:FieldNumber},${3:SubfieldNumber}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("Takes a record number or integer expression, field number or integer expression, and subfield number or integer expression and returns the corresponding numeric maximum value. Pass 0 for the subfield number if not required. If any of the passed values are invalid, this function returns 0.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HEADER",
        CompletionItem {
            label: "HEADER".to_string(),
            insert_text: Some("HEADER=\"${1:Expression}\" $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function defines a single line of column headings and their horizontal placement for PowerOn.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HEADERS",
        CompletionItem {
            label: "HEADERS".to_string(),
            insert_text: Some("HEADERS\n\t$0\nEND".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function marks the beginning of a subsection of output and precedes print statements that create column headings for that subsection.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HOUR",
        CompletionItem {
            label: "HOUR".to_string(),
            insert_text: Some("HOUR(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("This function returns the numerical value (from 00-23) equivalent to the hour stored in HHMM format.".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPBOXDRAW",
        CompletionItem {
            label: "HPBOXDRAW".to_string(),
            insert_text: Some(
                "HPBOXDRAW(${1:X1},${2:Y1},${3:X2},${4:Y2},${5:BoxType},${6:BoxStyle}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function draws a box on the screen.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPDESC",
        CompletionItem {
            label: "HPDESC".to_string(),
            insert_text: Some("HPDESC(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the description of the specified help file number."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPFONT",
        CompletionItem {
            label: "HPFONT".to_string(),
            insert_text: Some("HPFONT(${1:FontNumber},${2:PointSize}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function changes the type font and point size on a laser printer."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPLINEDRAW",
        CompletionItem {
            label: "HPLINEDRAW".to_string(),
            insert_text: Some(
                "HPLINEDRAW(${1:X1},${2:Y1},${3:X2},${4:Y2},${5:Width}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function commands a laser printer to draw a line of specified width between given coordinates."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPLINESPERINCH",
        CompletionItem {
            label: "HPLINESPERINCH".to_string(),
            insert_text: Some("HPLINESPERINCH(${1:LinesPerInch}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function commands a laser printer to change the number of lines of text that can fit into a vertical inch of space on the page."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPRESET",
        CompletionItem {
            label: "HPRESET".to_string(),
            insert_text: Some("HPRESET $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function resets the laser printer to its default settings.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPSETUP",
        CompletionItem {
            label: "HPSETUP".to_string(),
            insert_text: Some(
                "HPSETUP(${1:PageSize},${2:Orientation}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function changes the printer page size (letter or legal) and print orientation (portrait or landscape)."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPUNDERLINE",
        CompletionItem {
            label: "HPUNDERLINE".to_string(),
            insert_text: Some("HPUNDERLINE(${1:Mode}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function controls the underline function of the laser printer.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPXPOS",
        CompletionItem {
            label: "HPXPOS".to_string(),
            insert_text: Some("HPXPOS(${1:Xposition}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function positions a laser printer at a specific horizontal axis coordinate."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HPYPOS",
        CompletionItem {
            label: "HPYPOS".to_string(),
            insert_text: Some("HPYPOS(${1:Yposition}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function positions a laser printer at a specific vertical axis coordinate."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HTMLVIEWDISPLAY",
        CompletionItem {
            label: "HTMLVIEWDISPLAY".to_string(),
            insert_text: Some("HTMLVIEWDISPLAY $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays information in a view window using HTML commands."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HTMLVIEWLINE",
        CompletionItem {
            label: "HTMLVIEWLINE".to_string(),
            insert_text: Some(
                "HTMLVIEWLINE(\"${1:HTMLline}\") $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            documentation: Some(Documentation::String(
                "This function allows you to enter a line of HTML code to create information in a view window."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "HTMLVIEWOPEN",
        CompletionItem {
            label: "HTMLVIEWOPEN".to_string(),
            insert_text: Some("HTMLVIEWOPEN(${1|0|1}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function opens an HTML view window.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "INITCREDITREPORT",
        CompletionItem {
            label: "INITCREDITREPORT".to_string(),
            insert_text: Some(
                "INITCREDITREPORT(${1:SourceType}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function initializes the Credit Retrieval System setup variables that are required for pulling reports from Equifax, Experian, TransUnion, and the ChexSystems suite."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "INT",
        CompletionItem {
            label: "INT".to_string(),
            insert_text: Some("INT(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the integer part of a given number, monetary amount, or floating point expression."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "LENGTH",
        CompletionItem {
            label: "LENGTH".to_string(),
            insert_text: Some("LENGTH(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the number of characters in a character string.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "LOANPROJECTINIT",
        CompletionItem {
            label: "LOANPROJECTINIT".to_string(),
            insert_text: Some(
                "LOANPROJECTINIT(${1:DataSource},${2:ParameterDefaultType}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function initializes the special setup variables required to perform loan projection calculations."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "LOG",
        CompletionItem {
            label: "LOG".to_string(),
            insert_text: Some("LOG(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the natural logarithm of a specified number, code, or floating point value."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "LOWERCASE",
        CompletionItem {
            label: "LOWERCASE".to_string(),
            insert_text: Some("LOWERCASE(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function converts the alphabetic characters of a character expression to lowercase letters."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "MD5HASH",
        CompletionItem {
            label: "MD5HASH".to_string(),
            insert_text: Some("MD5HASH(${1:StringToHash}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function is used with the PASSWORDHASH function to encrypt audio access codes and home banking passwords."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "MINUTE",
        CompletionItem {
            label: "MINUTE".to_string(),
            insert_text: Some("MINUTE(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the minute value (from 0 to 59) of a specified time."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "MOD",
        CompletionItem {
            label: "MOD".to_string(),
            insert_text: Some("MOD(${1:Dividend},${2:Divisor}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns the remainder of a division operation.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "MONEY",
        CompletionItem {
            label: "MONEY".to_string(),
            insert_text: Some("MONEY(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function converts a number, code, float, or rate value into a monetary value that can be assigned to a variable type number or can be printed. It is intended for use on an entire numeric expression. If you use it on part of a compound expression, it can have unpredictable results."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "MONEYREAD",
        CompletionItem {
            label: "MONEYREAD".to_string(),
            insert_text: Some("MONEYREAD(\"${1:Prompt}\")$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console and returns the money response."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "MONTH",
        CompletionItem {
            label: "MONTH".to_string(),
            insert_text: Some("MONTH(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function returns a numeric value from 01 through 12 (January through December)."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "NUMBER",
        CompletionItem {
            label: "NUMBER".to_string(),
            insert_text: Some("NUMBER(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "NUMBERREAD",
        CompletionItem {
            label: "NUMBERREAD".to_string(),
            insert_text: Some("NUMBERREAD(\"${1:Prompt}\")$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a prompt on the user's console and returns the numeric response."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "OUTPUTCLOSE",
        CompletionItem {
            label: "OUTPUTCLOSE".to_string(),
            insert_text: Some("OUTPUTCLOSE(${1:OutputName}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "OUTPUTOPEN",
        CompletionItem {
            label: "OUTPUTOPEN".to_string(),
            insert_text: Some(
                "OUTPUTOPEN(${1:DeviceType},${2:PrinterNumber},${3:Title},${4:ReportCategory},${5:OutputChannel},${6:ErrorText}) $0"
                    .to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function opens the specified output destination, and then all subsequent output is sent to that destination."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "OUTPUTSWITCH",
        CompletionItem {
            label: "OUTPUTSWITCH".to_string(),
            insert_text: Some("OUTPUTSWITCH(${1:OutputChannel},${2:ErrorText})$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function changes all subsequent output to a different output destination."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "PASSWORDHASH",
        CompletionItem {
            label: "PASSWORDHASH".to_string(),
            insert_text: Some("PASSWORDHASH(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function encrypts access codes and home banking passwords.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "POPUPMESSAGE",
        CompletionItem {
            label: "POPUPMESSAGE".to_string(),
            insert_text: Some("POPUPMESSAGE(${1:0},${2:Message})$0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function displays a message in a pop-up window on the user's console."
                    .to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "PRINT",
        CompletionItem {
            label: "PRINT".to_string(),
            insert_text: Some("PRINT(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "PULLCREDITREPORT",
        CompletionItem {
            label: "PULLCREDITREPORT".to_string(),
            insert_text: Some(
                "PULLCREDITREPORT(${1:CreditBureau},${2:CreditReport}) $0".to_string(),
            ),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "This function pulls a credit report from a credit bureau.".to_string(),
            )),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions.insert(
        "PWR",
        CompletionItem {
            label: "PWR".to_string(),
            insert_text: Some("PWR(${1:Expression}) $0".to_string()),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String("".to_string())),
            ..CompletionItem::default()
        },
    );

    poweron_function_completions
}
