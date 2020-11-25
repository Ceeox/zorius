use bson::Document;

pub trait MongoDbUpdateable {
    fn update(&self) -> Document;
}
