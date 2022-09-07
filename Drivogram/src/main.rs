mod args;
mod functions;
use args::cli;


use colored::Colorize;
fn main() {
   let data = cli().get_matches();
   match data.subcommand() {
      Some(("source", _sub_matches)) => {
         println!("{} {}",
            "Here is the source of Drivogram:".yellow().blue().bold(), "https://github.com/Axrav/Drivogram".cyan()
         );
      }
      Some(("signup", sub_match)) =>{
         let sign = functions::signup(sub_match);
         match sign {
            Ok(str) => println!("{}", str),
            Err(_) => println!("{}", "An Error Occured, Try Later".red())
         }
      }
      _ => unreachable!(),
   }
      
   
    
}
