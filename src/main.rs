use std::env;
use std::process;

fn main(){
  let arguments = read_arguments();
  println!("Started to red file data from env argumets: {:?}", arguments);

}
#[allow(dead_code)]
#[derive(Debug)]

struct InputArguments {
  query: String,
  file_path : String
}

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