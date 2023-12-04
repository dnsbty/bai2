use chrono::NaiveDate;
use serde::Serialize;
use std::collections::HashMap;

use super::funds_type::{FundsSubType, FundsType};
use super::transaction_type::TransactionType;
use super::util::{parse_date, parse_int, parse_string, parse_time};

use crate::scanner::node::Node;

#[derive(Debug, Serialize)]
pub struct Transaction {
    amount: Option<u64>,
    availability: HashMap<u16, i64>,
    bank_reference_number: String,
    customer_reference_number: String,
    funds_type: FundsType,
    text: Vec<String>,
    transaction_type: TransactionType,
    value_date: Option<NaiveDate>,
    value_time: Option<String>,
}

impl Transaction {
    pub fn from_node(node: &Node) -> Result<Transaction, &'static str> {
        let fields = node.fields();
        let num_fields = fields.len();

        let transaction_type_code = parse_string(fields[1]);
        let transaction_type = TransactionType::parse(&transaction_type_code);

        let mut next_start_index = 4;
        let mut value_date: Option<NaiveDate> = None;
        let mut value_time: Option<String> = None;
        let mut availability: HashMap<u16, i64> = HashMap::new();
        let funds_type = FundsType::parse(fields.get(3).unwrap_or(&""));

        match funds_type {
            FundsType::ValueDated => {
                value_date = parse_date(fields[next_start_index]);
                value_time = parse_time(fields[next_start_index + 1]);
                next_start_index = next_start_index + 2;
            }
            FundsType::DistributedAvailability(FundsSubType::S) => {
                availability.insert(0, parse_int(fields[next_start_index]).unwrap());
                availability.insert(1, parse_int(fields[next_start_index + 1]).unwrap());
                availability.insert(2, parse_int(fields[next_start_index + 2]).unwrap());
                next_start_index = next_start_index + 3;
            }
            FundsType::DistributedAvailability(FundsSubType::D) => {
                let num_distributions = parse_int(fields[next_start_index]).unwrap_or(0);
                next_start_index = next_start_index + 1;

                for _ in 0..num_distributions {
                    match (
                        parse_int(fields[next_start_index]),
                        parse_int(fields[next_start_index + 1]),
                    ) {
                        (Some(days), Some(amt)) => {
                            availability.insert(days, amt);
                        }
                        _ => {}
                    }

                    next_start_index = next_start_index + 2;
                }
            }
            _ => (),
        }

        let raw_bank_ref = fields.get(next_start_index).unwrap_or(&"");
        let raw_customer_ref = fields.get(next_start_index + 1).unwrap_or(&"");
        next_start_index += 2;

        let num_remaining_fields = num_fields - next_start_index;
        let mut text = Vec::new();

        for i in 0..num_remaining_fields {
            let raw_text = fields.get(next_start_index + i).unwrap_or(&"");
            let parsed_text = parse_string(raw_text);
            text.push(parsed_text);
        }

        Ok(Transaction {
            amount: parse_int(fields[2]),
            availability: HashMap::new(),
            bank_reference_number: parse_string(raw_bank_ref),
            customer_reference_number: parse_string(raw_customer_ref),
            funds_type,
            text,
            transaction_type,
            value_date,
            value_time,
        })
    }
}
