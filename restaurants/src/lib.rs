use std::collections::HashMap;



pub fn add(left: usize, right: usize) -> usize {
    left + right
}



mod front_of_house{
    use std::collections::HashMap;

    pub mod hosting {
        pub fn add_to_waitlist(){}
        fn seat_at_table() {}
    }


    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {
            let mut hash =  super::HashMap::new();
            hash.insert(1,2);
        }

        use super::hosting::add_to_waitlist;

        mod feedback{
            fn take_feedback() {
                super::add_to_waitlist();
            }
        }
    }
}


mod back_of_house {
    use crate::front_of_house;

    println!("i am here");



    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer{
        Soup,
        Salad
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            super::add(1, 1);

            Breakfast{
                toast:  String::from(toast),
                seasonal_fruit:  String::from("apple")
            }
        }
    }
}



pub fn eat_at_resturant(){
    front_of_house::hosting::add_to_waitlist();

    let mut meal  =  back_of_house::Breakfast::summer("pie");

    meal.toast =  String::from("pepper soup");

    println!("I'd like some {} toast please",  meal.toast);

    let order1 =  back_of_house::Appetizer::Salad;

    match order1 {
        back_of_house::Appetizer::Salad =>  println!("hey i ordered a salad"),
        _ =>  println!("hey i ordered something else")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
