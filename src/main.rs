pub mod args;
pub mod file_ops;
pub mod rsa;
use clap::Parser;

fn main() {
    let args = args::InputArgs::parse();
    match args.input {
        args::InputType::Store(args::StoreCommands {
            path,
            password,
            username,
            comments,
        }) => {
            let res = file_ops::store(&path, password, username, comments);
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
                Ok(pass) => {
                    let collected_info: Vec<&str> = pass.split('+').collect();
                    if let Some(fir) = collected_info.first() {
                        match *fir {
                            "a" => println!("password is {}", collected_info[1]),
                            "b" => println!(
                                "password is {} \nusername is {}",
                                collected_info[1], collected_info[2]
                            ),
                            "c" => println!(
                                "password is {} \nleft comments - {}",
                                collected_info[1], collected_info[2]
                            ),
                            "d" => println!(
                                "password is {} \nusername is {} \nleft comments - {}",
                                collected_info[1], collected_info[2], collected_info[3]
                            ),
                            &_ => println!("not found :/"),
                        }
                    }
                }
                Err(e) => println!("Error {:?}", e),
            }
        }
    }
}
