use std::{path::PathBuf, io::Write};
//use which::{which, Error};

// use toml::{de};
use colored::Colorize;

struct Help{
    short: String,
    long: String,
    arguments: String,
    description: String,
}

pub fn printhelp(){
    // Help list
    let help = vec![
        Help{
            short: "h".to_string(),
            long: "help".to_string(),
            arguments: "".to_string(),
            description: "prints this message.".to_string(),
        },
        Help{
            short: "a".to_string(),
            long: "add".to_string(),
            arguments: "<path> <pretty name>".to_string(),
            description: "adds a directory to the tracking list.".to_string(),
        },
        Help{
            short: "s".to_string(),
            long: "show".to_string(),
            arguments: "".to_string(),
            description: "shows tracked folders.".to_string(),
        },
        Help{
            short: "r".to_string(),
            long: "remove".to_string(),
            arguments: "<pretty name>".to_string(),
            description: "removes a directory from the tracking list. Removes all if no name is given.".to_string(),
        },
        Help{
            short: "i".to_string(),
            long: "install".to_string(),
            arguments: "<pretty name>".to_string(),
            description: "installs a tracked folder. Installs all if no name is given.".to_string(),
        },
        Help{
            short: "u".to_string(),
            long: "uninstall".to_string(),
            arguments: "<prety name>".to_string(),
            description: "uninstalls a tracked folder. Uninstalls all if no name is given.".to_string(),
        },
        Help{
            short: "v".to_string(),
            long: "version".to_string(),
            arguments: "".to_string(),
            description: "prints the version.".to_string(),
        },
    ];
    // Print help message
    println!("{}","USAGE:".bold());
    help.iter()
        .for_each(| help |
            println!("{}, {} {}: {}",
                help.short.bold(),
                help.long.bold().italic(),
                help.arguments.dimmed().italic(),
                help.description
            )
        );
}

// Check if config file exists
pub fn config_exists() -> bool {
    let config_path = PathBuf::from(".rdt");
    config_path.exists()
}

// Create empty config file
pub fn create_config() -> std::io::Result<()> {
    let mut config_file = std::fs::File::create(".rdt")?;
    config_file.write_all(b"")?;
    Ok(())
}

pub struct Directory {
    pub path: String,
    pub pretty_name: String,
}

// Serialize a directory in the config (toml) file
pub fn add_directory(dir: Directory) -> std::io::Result<()> {
    let mut config_file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(".rdt")?;

    let toml = format!("[[directories]]\npath = \"{}\"\npretty_name = \"{}\"\n", dir.path, dir.pretty_name);
    match config_file.write_all(toml.as_bytes()){
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("{}: {}","Error".red(), e);
            return Err(e);
        },
    }
}

// Remove the listed directory from the tracking list, if no name is given, remove all
pub fn remove_directory(name: Vec<String>) -> std::io::Result<()> {
    let config_path = PathBuf::from(".rdt");
    let config = match std::fs::read_to_string(config_path.clone()){
        Ok(config) => config,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to read config file.")),
    };

    let mut config = match toml::from_str(&config){
        Ok(config) => match config {
            toml::Value::Table(config) => config,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse config file.")),
        },
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse config file.")),
    };

    let directories = match config.get_mut("directories"){
        Some(directories) => match directories {
            toml::Value::Array(directories) => directories,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse config file.")),
        },
        None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse config file.")),
    };

    let mut config_file = std::fs::File::create(config_path)?;

    if name.is_empty() {
        config_file.write_all(b"")?;
        return Ok(());
    } else {
        // Iterate over directories and remove the ones with the given name
        let mut i = 0;
        while i < directories.len() {
            let directory = match directories[i].as_table(){
                Some(d) => d,
                None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to serialize config file.")),
            };

            let pretty_name = match directory["pretty_name"].as_str(){
                Some(p) => p,
                None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to serialize config file.")),
            };

            if name.contains(&pretty_name.to_string()) {
                directories.remove(i);
            } else {
                i += 1;
            }
        }
    }

    config_file.write_all(toml::to_string(&config).unwrap().as_bytes())?;

    Ok(())
}

// Return a list of tracked directories
pub fn get_directories() -> std::io::Result<Vec<Directory>> {
    let config_path = PathBuf::from(".rdt");
    let config = match std::fs::read_to_string(config_path){
        Ok(config) => config,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to read config file.")),
    };

    let config = match toml::from_str(&config){
        Ok(config) => match config{
            toml::Value::Table(config) => config,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse config file at reading.")),
        },
        Err(e) => {
            println!("{}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse config file."));
        }
    };
    
    let mut list: Vec<Directory> = vec![];

    config.iter().for_each(| (key, value) | {
        if key == "directories" {
            if let toml::Value::Array(directories) = value {
                directories.iter().for_each(| directory | {
                    let directory = match directory.as_table(){
                        Some(d) => d,
                        None => return,
                    };

                    let path = match directory["path"].as_str(){
                        Some(p) => p.to_string(),
                        None => return,
                    };

                    let pretty_name = match directory["pretty_name"].as_str(){
                        Some(p) => p.to_string(),
                        None => return,
                    };

                    list.push(Directory{
                        path,
                        pretty_name,
                    });

                });
            }
        }
    });

    Ok(list)

}

fn add_path(path: PathBuf) {
    let path = PathBuf::from(path);
                // Add the directory to the PATH variable
                match std::env::set_var("PATH", format!("{};{}", match path.to_str(){
                    Some(p) => p,
                    None => return
                }, match std::env::var("PATH"){
                    Ok(p) => p,
                    Err(e) => {
                        println!("{}: {}", "Error".red(), e);
                        return;
                    }
                })){
                    _ => {},
                }
}

// Add directory path all given names, add all if no name is given to the OS user PATH variable
pub fn add_to_path(names: Vec<String>) -> std::io::Result<()> {
    let list = match get_directories(){
        Ok(list) => list,
        Err(e) => return Err(e),
    };

    if names.is_empty() {
        list.iter().for_each(| dir | {
            let path = PathBuf::from(&dir.path);
            // Add the directory to the PATH variable
            // get current program path
            let current = std::env::current_exe().unwrap();
            add_path(current.join(path));
        });
    } else {
        list.iter().for_each(| dir | {
            if names.contains(&dir.pretty_name) {
                let path = PathBuf::from(&dir.path);
                // Add the directory to the PATH variable
                // get current program path
                let current = std::env::current_exe().unwrap();
                add_path(current.join(path));
            }
        });
    }

    Ok(())
}

// Remove directory path all given names, remove all if no name is given from the OS user PATH variable
pub fn remove_from_path(names: Vec<String>) -> std::io::Result<()> {
    let list = match get_directories(){
        Ok(list) => list,
        Err(e) => return Err(e),
    };

    if names.is_empty() {
        list.iter().for_each(| dir | {
            let path = PathBuf::from(&dir.path);
            // Remove the directory from the PATH variable
            match std::env::set_var("PATH", match std::env::var("PATH"){
                Ok(p) => p.replace(format!("{};", match path.to_str(){
                    Some(p) => p,
                    None => return
                }).as_str(), ""),
                Err(e) => {
                    println!("{}: {}", "Error".red(), e);
                    return;
                }
            }){
                _ => {},
            }
        });
    } else {
        list.iter().for_each(| dir | {
            if names.contains(&dir.pretty_name) {
                let path = PathBuf::from(&dir.path);
                // Remove the directory from the PATH variable
                match std::env::set_var("PATH", match std::env::var("PATH"){
                    Ok(p) => p.replace(format!("{};", match path.to_str(){
                        Some(p) => p,
                        None => return
                    }).as_str(), ""),
                    Err(e) => {
                        println!("{}: {}", "Error".red(), e);
                        return;
                    }
                }){
                    _ => {},
                }
            }
        });
    }

    Ok(())
}