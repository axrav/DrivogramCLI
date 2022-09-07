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
      },
      Some(("signup", sub_match)) =>{
         let sign = functions::signup(sub_match);
         match sign {
            Ok(str) => println!("{}", str),
            Err(_) => println!("{}", "An Error Occured, Try Later".red())
         }
      },
      Some(("login", sub_match)) =>{
         let login = functions::login_check(sub_match);
         match login {
            Ok(func) => match func {
               Ok(bool) => match bool { 
                  true=> println!("{}", "Logged in Successfully and your Key has been saved!".bright_blue()),
                  false=> println!("{}", "Unable to Login to Drivogram, Check your key and try again!".red().bold())
               },
               Err(_) => {}
                
            }
            Err(_) => println!("{}", "An Error Occured, Try Later".red().bold())

         }
      },
      Some(("myuploads", _)) =>{
               match functions::show_data(){
                  Ok(()) => (),
                  Err(_) => println!("{:#?}", "Unable to Process your request,Try checking your network".bold().red())
               }
      }
     

               
      _ => unreachable!(),
   }
      
   
    
}
