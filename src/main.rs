pub mod args;
pub mod file_ops;
pub mod rsa;
use clap::Parser;

fn main() {
    let args = args::InputArgs::parse();
    match args.input {
        args::InputType::Store(args::StoreCommands { path, password }) => {
            let res = file_ops::store(&path, &password);
            match res {
                Err(e) => println!("failed! {:?}", e),
                Ok(()) => println!("success!"),
            }
        }
        args::InputType::Remove(args::RemoveCommands { path }) => {
            let res = file_ops::remove(&path);
            match res {
                Err(e) => println!("failed! {:?}", e),
                Ok(()) => println!("success!"),
            }
        }
        args::InputType::Init(args::InitCommand { path }) => {
            let res = file_ops::init(&path);
            match res {
                Ok(()) => println!("pass-store initialized!."),
                Err(e) => println!("Error {:?}", e),
            }
        }
        args::InputType::Get(args::GetCommand { path }) => {
            let res = file_ops::get_pass(&path);
            match res {
                Ok(pass) => println!("password is {}", pass),
                Err(e) => println!("Error {:?}", e),
            }
        }
    }
}
