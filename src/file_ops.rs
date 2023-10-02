use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

use crate::rsa as encryption;

pub type IoResult<T> = Result<T, io::Error>;

pub fn init(path: &str) -> IoResult<()> {
    match env::var("APPDATA") {
        Ok(appdata_dir) => {
            let target_dir = format!("{}/pass_store", appdata_dir);
            let init_path = format!("{}/path", &target_dir);
            let private_key_path = format!("{}/private_key.pem", &target_dir);
            let store_path = format!("{}/store", path);
            let public_key_dir = format!("{}/key", store_path);
            let public_key_path = format!("{}/public_key.pem", public_key_dir);

            // Create the target directory
            fs::create_dir_all(&target_dir)?;

            // Create and write content to the first file
            let mut path_file = File::create(&init_path)?;
            path_file.write(path.as_bytes())?;

            // Create and write content to the second file
            let mut key_file = File::create(&private_key_path)?;

            fs::create_dir_all(&store_path)?;
            fs::create_dir_all(&public_key_dir)?;
            let mut public_key_file = File::create(&public_key_path)?;

            // Create the store directory
            fs::create_dir_all(&store_path)?;
            let private_key_res = encryption::generate_private_public_key_pair();
            match private_key_res {
                Ok((private, public)) => {
                    key_file.write(&private)?;
                    public_key_file.write(&public)?;
                    Ok(())
                }
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed! : {:?}", e),
                )),
            }
        }
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to get AppData directory: {:?}", e),
        )),
    }
}

pub fn get_path() -> IoResult<String> {
    match env::var("APPDATA") {
        Ok(appdata_dir) => {
            let init_path = format!("{}/pass_store/path", appdata_dir);
            let store_path_bytes = fs::read(&init_path)?;
            let store_path_str = String::from_utf8(store_path_bytes).map_err(|err| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to convert: {:?}", err),
                )
            })?;
            let lookup = format!("{}/store", store_path_str);
            let lookup_path = Path::new(&lookup);
            match lookup_path.try_exists() {
                Ok(true) => Ok(lookup),
                Ok(false) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("path does not exists"),
                )),
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to convert: {:?}", e),
                )),
            }
        }
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to get AppData directory: {:?}", e),
        )),
    }
}

pub fn store(
    s: &str,
    pass: String,
    username: Option<String>,
    comments: Option<String>,
) -> IoResult<()> {
    let p = get_path()?;
    let public_key_path = format!("{}/key/public_key.pem", p);
    let store_path = format!("{}/{}", p, s);
    let to_store = match (pass, username, comments) {
        (p, Some(u), Some(c)) => format!("d+{}+{}+{}", p, u, c),
        (p, Some(u), None) => format!("b+{}+{}", p, u),
        (p, None, Some(c)) => format!("c+{}+{}", p, c),
        (p, None, None) => format!("a+{}", p),
    };

    let public_key_bytes = get_key_buffer(&public_key_path)?;
    let encrypted_buffer_res =
        encryption::encrypt(public_key_bytes.as_slice(), to_store.as_bytes());
    match encrypted_buffer_res {
        Ok(buffer) => {
            let mut path_file = File::create(&store_path)?;
            path_file.write(buffer.as_slice())?;
            Ok(())
        }
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed!: {:?}", e),
        )),
    }
}

pub fn get_pass(path: &str) -> IoResult<String> {
    let p = get_path()?;
    let store_path = format!("{}/{}", p, path);
    let private_key_path = get_private_key_path()?;
    let private_key_buffer = get_key_buffer(&private_key_path)?;

    let mut pass_file = File::open(store_path)?;
    let mut encrypted_pass = Vec::new();
    pass_file.read_to_end(&mut encrypted_pass)?;

    let decrypted_buffer_res = encryption::decrypt(&private_key_buffer, &encrypted_pass);
    match decrypted_buffer_res {
        Ok(buffer) => {
            let pass_res = String::from_utf8(buffer);
            match pass_res {
                Ok(pass) => Ok(pass),
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed!: {:?}", e),
                )),
            }
        }
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed!: {:?}", e),
        )),
    }
}

pub fn remove(s: &str) -> IoResult<()> {
    let path = get_path()?;
    let remove_path = format!("{}/{}", path, s);
    let res = fs::remove_file(&remove_path);
    res
}

fn get_key_buffer(path: &str) -> IoResult<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut pem = Vec::new();
    file.read_to_end(&mut pem)?;
    Ok(pem)
}

fn get_private_key_path() -> IoResult<String> {
    match env::var("APPDATA") {
        Ok(appdata_dir) => Ok(format!("{}/pass_store/private_key.pem", appdata_dir)),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to get AppData directory: {:?}", e),
        )),
    }
}
