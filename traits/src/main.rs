use std::fmt::Display;

use traits::{Summary,  NewsLetter};

struct PH{
    level:  String,
    description:String
}


impl Display for PH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"level: {} ---- [{}]",self.description,  self.level)
    }
}


mod story_maker {
    pub struct MediaStory<T>{
        title:  String,
        pub content:  T
    }
    
    impl<T: std::fmt::Display> MediaStory<T>{
        pub fn new (type_:  T) -> Self {
            MediaStory{
                title:  String::from("company media!"),
                content: type_
            }
        }

        pub fn get_title (&self) -> String {
            format!("{}", self.title )
        }
    }
}

pub use story_maker::MediaStory;

fn main(){
    let my_newsletter =  NewsLetter {
        author: String::from("Jonho"),
        content:  "you already know whet it is".to_string(),
        title: "a known title".to_string()
    };


    println!("{}",  my_newsletter.summary());

    notify(&my_newsletter);

    let my_story =  MediaStory::new(String::from("hello traits"));

    println!("title: {} \n  content: {}",  my_story.get_title(),  my_story.content);
}


fn notify (item: &impl Summary) {
    println!("notifying you about {}",  item.summary());
}

