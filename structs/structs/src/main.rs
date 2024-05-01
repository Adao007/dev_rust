struct User {
    active:bool,
    username: String, 
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // A Tuple Structs without named fields to create different
struct Point(i32, i32, i32); // types.

fn main() {
    let user1 = User {
        active: true,
        username: String::from("anthonydaoy"),
        email: String::from("anthonydaoy@gmail.com"),
        sign_in_count: 1,
    };

    // The instant of struct User(user1) is not mutable!
    // Lets make a mutable instant!

    let mut MutableUser = User {
        active: true,
        username: String::from("changeme"),
        email: String::from("changeme@changeme.com"),
        sign_in_count: 1,
    };
    
    MutableUser.email = String::from("changedyou@changedyou.com");   
    
    // Lets us create an instant of User, using user1 to shortcut the process. 
    // Only the email variable is changed and the rest are copied from user1
    let user2 = User {
        email: String::from("shortcutinitial"),
        ..user1
    };

    // Using the tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // use a . followed by the index to access an individual value
}
   
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,   // This is using the shorthand init: which is the same as... 
        email,      // username: username, // email:email,
        sign_in_count: 1,
    }
}
