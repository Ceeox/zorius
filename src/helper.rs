use bson::{doc, Bson, Document};

pub trait NullKeyRemover {
    fn remove_null_keys(self) -> Vec<Document>;
}

impl NullKeyRemover for Document {
    fn remove_null_keys(self) -> Vec<Document> {
        let mut res = vec![];
        for item in self.into_iter() {
            if item.1 != Bson::Null {
                res.push(doc! { "$set": { item.0.as_str(): item.1 }});
            }
        }
        res
    }
}
