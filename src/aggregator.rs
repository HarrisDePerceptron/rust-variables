pub struct Article {
    pub name: String,
    pub description: String,
    pub body: String,
    pub created_at: String,
}

pub struct Tweet {
    pub body: String,
    pub created_at: String,
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn print(&self) -> () {
        println!("this is a summary trait, written by {}", self.summarize_author());
    }

    fn summarize_author(&self) -> String;
}