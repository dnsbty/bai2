use self::node::{Node, NodeType};
use std::str::Lines;

use log::debug;

pub mod node;

#[derive(Debug)]
pub struct Scanner<'a> {
    lines: Lines<'a>,
    stack: Vec<Node>,
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a str) -> Scanner<'a> {
        let lines = content.lines();

        Scanner {
            lines,
            stack: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> Result<Node, &'static str> {
        debug!("Scanning file");

        let mut file_header_line;

        // loop until a non-empty line is found
        loop {
            file_header_line = match self.lines.next() {
                Some(line) => line,
                None => {
                    debug!("no lines found in file");
                    return Err("no lines found in file");
                }
            };

            if !file_header_line.is_empty() {
                break;
            }
        }

        // The first line should always be the file header
        if !file_header_line.get(0..2).eq(&Some("01")) {
            debug!("file header not found");
            return Err("file header not found");
        }

        debug!("file header found");
        self.push_node(NodeType::FileHeader, file_header_line.to_string());

        while let Some(line) = self.lines.next() {
            match self.handle_line(line) {
                Ok(_) => (),
                Err(message) => {
                    return Err(message);
                }
            }
        }

        debug!("Done scanning file");

        return Ok(self.stack.pop().unwrap());
    }

    // Private

    fn assert_current_type(&self, node_type: NodeType) -> Result<(), &'static str> {
        match self.current_type() {
            Some(current_type) => {
                if current_type != node_type {
                    return Err("unexpected node type");
                }
            }
            None => return Err("no current node"),
        }
        Ok(())
    }

    fn current_type(&self) -> Option<NodeType> {
        match self.stack.last() {
            None => None,
            Some(node) => Some(node.r#type),
        }
    }

    fn handle_line(&mut self, line: &str) -> Result<(), &'static str> {
        match line.get(0..2) {
            Some("02") => {
                if let Err(_) = self.assert_current_type(NodeType::FileHeader) {
                    return Err("file trailer found without file header");
                }

                debug!("group header found");
                self.push_node(NodeType::GroupHeader, line.to_string());
                Ok(())
            }
            Some("03") => {
                if let Err(_) = self.assert_current_type(NodeType::GroupHeader) {
                    return Err("account identifier found without group header");
                }

                debug!("account identifier found");
                self.push_node(NodeType::AccountIdentifier, line.to_string());
                Ok(())
            }
            Some("16") => {
                match self.current_type() {
                    Some(NodeType::AccountIdentifier) => (),
                    Some(NodeType::TransactionDetail) => self.pop_node(),
                    _ => return Err("transaction detail found without account identifier"),
                }

                debug!("transaction found");
                self.push_node(NodeType::TransactionDetail, line.to_string());
                Ok(())
            }
            Some("49") => {
                match self.current_type() {
                    Some(NodeType::AccountIdentifier) => (),
                    Some(NodeType::TransactionDetail) => self.pop_node(),
                    _ => return Err("account control found without account identifier"),
                }

                debug!("account control found");
                self.put_sibling(NodeType::AccountTrailer, line.to_string());
                self.pop_node();
                Ok(())
            }
            Some("88") => {
                debug!("continuation found");
                self.push_continuation(line.to_string());
                Ok(())
            }
            Some("98") => {
                if let Err(_) = self.assert_current_type(NodeType::GroupHeader) {
                    return Err("group trailer found without group header");
                }

                debug!("group trailer found");
                self.put_sibling(NodeType::GroupTrailer, line.to_string());
                self.pop_node();
                Ok(())
            }
            Some("99") => {
                if let Err(_) = self.assert_current_type(NodeType::FileHeader) {
                    return Err("file trailer found without file header");
                }

                debug!("file trailer found");
                self.put_sibling(NodeType::FileTrailer, line.to_string());
                Ok(())
            }
            None => {
                debug!("skipping empty line");
                Ok(())
            }
            Some(record_type) => {
                debug!("skipping unrecognized record type: {}", record_type);
                Ok(())
            }
        }
    }

    fn pop_node(&mut self) {
        let child = self.stack.pop().unwrap();
        let parent = self.stack.last_mut().unwrap();
        parent.push_child(child);
    }

    fn push_continuation(&mut self, line: String) {
        let current_node = self.stack.last_mut().unwrap();
        let continuation = Node {
            children: Vec::new(),
            continuations: Vec::new(),
            line,
            sibling: Box::new(None),
            r#type: NodeType::Continuation,
        };
        current_node.continuations.push(continuation);
    }

    fn push_node(&mut self, node_type: NodeType, line: String) {
        let node = Node {
            children: Vec::new(),
            continuations: Vec::new(),
            line,
            sibling: Box::new(None),
            r#type: node_type,
        };

        self.stack.push(node);
    }

    fn put_sibling(&mut self, node_type: NodeType, line: String) {
        let current_node = self.stack.last_mut().unwrap();

        let sibling = Node {
            children: Vec::new(),
            continuations: Vec::new(),
            line,
            sibling: Box::new(None),
            r#type: node_type,
        };

        current_node.sibling = Box::new(Some(sibling));
    }
}
