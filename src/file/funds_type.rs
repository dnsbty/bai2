use serde::{Serialize, Serializer};

use crate::file::util::parse_string;

#[derive(Debug)]
pub enum FundsType {
    Unknown,
    ImmediateAvailability,
    OneDayAvailability,
    TwoOrMoreDaysAvailability,
    ValueDated,
    DistributedAvailability(FundsSubType),
}

impl FundsType {
    pub fn parse(value: &str) -> FundsType {
        match parse_string(value).as_str() {
            "0" => FundsType::ImmediateAvailability,
            "1" => FundsType::OneDayAvailability,
            "2" => FundsType::TwoOrMoreDaysAvailability,
            "V" => FundsType::ValueDated,
            "S" => FundsType::DistributedAvailability(FundsSubType::S),
            "D" => FundsType::DistributedAvailability(FundsSubType::D),
            _ => FundsType::Unknown,
        }
    }
}

impl Serialize for FundsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            FundsType::Unknown => serializer.serialize_unit_variant("FundsType", 0, "unknown"),
            FundsType::ImmediateAvailability => {
                serializer.serialize_unit_variant("FundsType", 1, "immediate_availability")
            }
            FundsType::OneDayAvailability => {
                serializer.serialize_unit_variant("FundsType", 2, "one_day_availability")
            }
            FundsType::TwoOrMoreDaysAvailability => {
                serializer.serialize_unit_variant("FundsType", 3, "two_or_more_days_availability")
            }
            FundsType::ValueDated => {
                serializer.serialize_unit_variant("FundsType", 4, "value_dated")
            }
            FundsType::DistributedAvailability(_) => {
                serializer.serialize_unit_variant("FundsType", 4, "distributed_availability")
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum FundsSubType {
    S,
    D,
}
