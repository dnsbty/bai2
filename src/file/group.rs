use chrono::NaiveDate;
use serde::Serialize;

use crate::scanner::node::Node;

use super::account::Account;
use super::util::{parse_currency, parse_date, parse_int, parse_string, parse_time};

#[derive(Debug, Serialize)]
pub struct Group {
    accounts: Vec<Account>,
    as_of_date: Option<NaiveDate>,
    as_of_date_modifier: Option<AsOfDateModifier>,
    as_of_time: Option<String>,
    currency_code: String,
    originator: String,
    number_of_accounts: Option<u16>,
    number_of_records: Option<u16>,
    status: GroupStatus,
    total: Option<u64>,
    ultimate_receiver: String,
}

impl Group {
    pub fn from_node(node: &Node) -> Result<Group, &'static str> {
        let header_fields = &node.fields();
        if header_fields.len() < 7 {
            return Err("Invalid group header. Expected 7 fields, but found less.");
        }

        let trailer_fields = node.sibling_fields();
        if trailer_fields.len() < 4 {
            return Err("Invalid group trailer. Expected 4 fields, but found less.");
        }

        let currency_code = parse_currency(header_fields[6], "USD");

        let accounts_result = node
            .children
            .iter()
            .map(|n| Account::from_node(n, &currency_code))
            .collect::<Result<Vec<Account>, &'static str>>();

        match accounts_result {
            Err(e) => Err(e),
            Ok(accounts) => Ok(Group {
                accounts,
                as_of_date: parse_date(header_fields[4]),
                as_of_date_modifier: AsOfDateModifier::parse(header_fields[7]),
                as_of_time: parse_time(header_fields[5]),
                currency_code,
                number_of_accounts: parse_int(trailer_fields[2]),
                number_of_records: parse_int(trailer_fields[3]),
                originator: parse_string(header_fields[2]),
                status: GroupStatus::parse(header_fields[3]),
                total: parse_int(trailer_fields[1]),
                ultimate_receiver: parse_string(header_fields[1]),
            }),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AsOfDateModifier {
    InterimPreviousDayData,
    FinalPreviousDayData,
    InterimSameDayData,
    FinalSameDayData,
}

impl AsOfDateModifier {
    fn parse(value: &str) -> Option<AsOfDateModifier> {
        match parse_string(value).as_str() {
            "1" => Some(AsOfDateModifier::InterimPreviousDayData),
            "2" => Some(AsOfDateModifier::FinalPreviousDayData),
            "3" => Some(AsOfDateModifier::InterimSameDayData),
            "4" => Some(AsOfDateModifier::FinalSameDayData),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupStatus {
    Update,
    Deletion,
    Correction,
    TestOnly,
    Unknown(String),
}

impl GroupStatus {
    fn parse(value: &str) -> GroupStatus {
        match parse_string(value).as_str() {
            "1" => GroupStatus::Update,
            "2" => GroupStatus::Deletion,
            "3" => GroupStatus::Correction,
            "4" => GroupStatus::TestOnly,
            code => GroupStatus::Unknown(code.to_string()),
        }
    }
}
