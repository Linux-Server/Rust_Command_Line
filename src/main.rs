use std::env;
use std::process;

fn main(){
  let arguments = InputArguments::read_arguments();
  println!("Started to red file data from env argumets: {:?}", arguments);

}
#[allow(dead_code)]
#[derive(Debug)]

struct InputArguments {
  query: String,
  file_path : String
}


// alternate option for a regular funtion
impl InputArguments{
  fn read_arguments()-> Result<InputArguments,String>{
     let arguments: Vec<String> = env::args().collect();
     if arguments.len()<3{
       return Err("Total Input Arguments Errror".to_string())
     }
     let query = arguments[1].clone();
     let file_path = arguments[2].clone();
     Ok(InputArguments{
       query,
       file_path
     })
  }
}


// read arguments in normal functional way
fn read_arguments()-> InputArguments{
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