use std::env;
use std::process;

#[allow(dead_code)]
#[derive(Debug)]

pub struct InputArguments {
  pub query: String,
  pub file_path : String
}


// alternate option for a regular funtion
impl InputArguments{
 pub fn read_arguments()-> Result<Self,String>{
     let arguments: Vec<String> = env::args().collect();
     if arguments.len()<3{
       return Err("Total Input Arguments Errror".to_string())
     }
     let query = arguments[1].clone();
     let file_path = arguments[2].clone();
     Ok(Self{
       query,
       file_path
     })
  }

  
}


// read arguments in normal functional way
#[allow(dead_code)]
pub fn read_arguments()-> InputArguments{
  let arguments: Vec<String> = env::args().collect();
  if arguments.len() < 3{
    println!("All argumets are not passed!");
    process::exit(200);
  }
  let query = arguments[1].clone();
  let file_path = arguments[2].clone();
  InputArguments{
    query,
    file_path
  }
  
}