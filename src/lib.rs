use chrono::NaiveDate;
use serde::Serialize;

use crate::file::group::Group;
use crate::file::util::{parse_date, parse_int, parse_string, parse_time};

use crate::scanner::node::Node;
use crate::scanner::Scanner;

mod file;
mod scanner;

#[derive(Debug, Serialize)]
pub struct Bai2File {
    pub creation_date: Option<NaiveDate>,
    pub creation_time: Option<String>,
    pub file_id: String,
    pub groups: Vec<Group>,
    pub number_of_groups: Option<u16>,
    pub number_of_records: Option<u16>,
    pub total: Option<u64>,
    pub receiver: String,
    pub sender: String,
    pub version_number: Option<u8>,
}

impl Bai2File {
    pub fn new(content: String) -> Result<Bai2File, &'static str> {
        let mut scanner = Scanner::new(&content);
        match scanner.scan() {
            Ok(scan_tree) => Bai2File::from_scan(scan_tree),
            Err(e) => Err(e),
        }
    }

    fn from_scan(root_node: Node) -> Result<Bai2File, &'static str> {
        let header_fields = &root_node.fields();
        if header_fields.len() < 9 {
            return Err("Invalid file header. Expected 9 fields, but found less.");
        }

        let trailer_fields = root_node.sibling_fields();
        if trailer_fields.len() < 4 {
            return Err("Invalid file trailer. Expected 4 fields, but found less.");
        }

        let groups_result = root_node
            .children
            .iter()
            .map(Group::from_node)
            .collect::<Result<Vec<Group>, &'static str>>();

        match groups_result {
            Err(e) => Err(e),
            Ok(groups) => Ok(Bai2File {
                creation_date: parse_date(header_fields[3]),
                creation_time: parse_time(header_fields[4]),
                file_id: parse_string(header_fields[5]),
                groups,
                number_of_groups: parse_int(trailer_fields[2]),
                number_of_records: parse_int(trailer_fields[3]),
                total: parse_int(trailer_fields[1]),
                receiver: parse_string(header_fields[2]),
                sender: parse_string(header_fields[1]),
                version_number: parse_int(header_fields[8]),
            }),
        }
    }
}
