pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(val:u32) -> u32 {
    val + 2
}


pub fn greeting(text:&str) -> String{
    format!("Hello {}",  text)
}


pub struct Guess {
    value:  i32
}

impl Guess {
    fn new(value:  i32) -> Self {
        if value < 1{
            panic!("value must be greater than one");
        }
        else if value > 100 {
            panic!("value must be less than hundred")
        }

        Guess{
            value: value
        }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    width:u32,
    height:  u32
}

impl Rectangle {
    fn can_hold(&self,other: &Rectangle) -> bool {
        self.height > other.height && self.width >  other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("make this test fail!");
    // }

    #[test]
    fn larger_rect_can_hold() {
        let rect1 =  Rectangle {
            width: 8,
            height:  7
        };
        let rect2 =  Rectangle {
            width: 5,
            height:  4
        };

        assert!(rect1.can_hold(&rect2));
    }
    #[test]
    fn smaller_rect_can_not_hold() {
        let rect1 =  Rectangle {
            width: 2,
            height:  4
        };
        let rect2 =  Rectangle {
            width: 5,
            height:  4
        };

        assert!(!rect1.can_hold(&rect2));
    }

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn text_is_in_value() {
        let text =  String::from("Carol");
        let value =  greeting(text.as_str());

        assert!(value.contains(text.as_str()),  "the returned value: {} does not contain input text", value);
    }


    #[test]
    #[should_panic(expected="less than hundred")]
    fn only_value_greater_then_100( ) {
        Guess::new(101);
    }


}
