pub trait  Summary {
    fn summary(&self) -> String;
}


pub struct NewsLetter {
    pub title:String,
    pub content:  String,
    pub author:  String
}


impl Summary for NewsLetter{
    fn summary(&self) -> String {
        format!("{} by {}",  self.content, self.author)
    }
}


pub struct Tweet {
    pub username: String,
    pub content:String,
    pub retweeted: bool,
    pub retweet:  bool
}



impl Summary for Tweet{
    fn summary(&self) -> String {
        format!("{} by: {}", self.content ,  self.username )
    }
}


