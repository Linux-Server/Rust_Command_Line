use std::fmt::*;
fn main() {
    println!("Hello, world!");
    let word = "neyya";
    call_me(&word);
    
    let user = User{
        name:"Sachin".to_string(),
        age: 28
    };
    
    let nora = User::ind_features(&user);
    println!("The nora is {:?}", nora);
}

 // call mee to check the trait bound
fn call_me<T>(data: &T) where T:Debug+Display{
    println!("Call me ....{}", data);
}

trait Character{

// its a default feature 
    fn features(&self)->String{
        format!("He is a good looking person")
    }
// its a common features
    fn ind_features(&self)->String;
}

// struct to store user data
#[allow(dead_code)]
struct User{
    name: String,
    age: u32
}

impl Character for User{
    fn ind_features(&self)->String{
        format!("{} and {} ", self.name, self.features())
    }
}
