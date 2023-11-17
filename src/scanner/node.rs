#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeType {
    AccountIdentifier,
    AccountTrailer,
    Continuation,
    FileHeader,
    FileTrailer,
    GroupHeader,
    GroupTrailer,
    TransactionDetail,
}

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub continuations: Vec<Node>,
    pub line: String,
    pub sibling: Box<Option<Node>>,
    pub r#type: NodeType,
}

impl Node {
    pub fn fields(&self) -> Vec<&str> {
        let mut fields: Vec<&str> = self.line.split(",").collect();

        for continuation in &self.continuations {
            let continuation_fields = continuation.line.split(",").skip(1);
            for field in continuation_fields {
                fields.push(field);
            }
        }

        fields
    }

    pub fn has_continuations(&self) -> bool {
        !self.continuations.is_empty()
    }

    pub fn push_child(&mut self, node: Node) {
        self.children.push(node);
    }

    pub fn push_continuation(&mut self, node: Node) {
        self.continuations.push(node);
    }

    pub fn sibling_fields(&self) -> Vec<&str> {
        match &*self.sibling {
            Some(sibling) => sibling.fields(),
            None => Vec::new(),
        }
    }
}
