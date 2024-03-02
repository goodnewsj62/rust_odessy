struct User{
    active:  bool,
    username:  String,
    email:String,
    sign_in_count:  u64
}

fn main() {
    let user =  User{
        active: true,
        username: String::from("osonwa"),
        email:  String::from("osonwajohn@gmail.com"),
        sign_in_count:  1
    };


    println!("user account info: \n username: {} \n email: {} \n isActive: {} \n signInCount: {}",  user.username, user.email,  user.active, user.sign_in_count);

    let mut user2 =  build_user(String::from("goodnewsj62@gmail.com"), String::from("johnd"));
    user2.username =  String::from("harvard grad");
}


fn build_user(email:String,  username:String) -> User{
    User {
        active:  true,
        username,
        email,
        sign_in_count: 1
    }
}