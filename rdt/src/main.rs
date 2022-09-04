use colored::Colorize;
use rdt::add_to_path;
use std::env;
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod rdt;

// C:\Program Files\GitHub CLI\

/* USAGE:
 *h               , help: prints this message.
 *a <path>        , add <path> <pretty name>: adds a directory to the tracking list.
 *s               , show: shows tracked folders.
 *r <pretty name> , remove <pretty name>: removes a directory from the tracking list. Removes all if no name is given.
 *i <pretty name> , install <pretty name>: installs a tracked folder. Installs all if no name is given.
 *u <prety name>  , uninstall <pretty name>: uninstalls a tracked folder. Uninstalls all if no name is given.
 *v               , version: prints the version.
*/

fn main() {
    // Check if config file exists and create it if it doesn't
    match rdt::config_exists() {
        true => (),
        false => match rdt::create_config() {
            Ok(_) => (),
            Err(e) => {
                println!("{}: {}", "Fatal Error".red().bold(), e.to_string().italic());
                std::process::exit(1);
            }
        },
    }

    // Check if no argument is passed
    if env::args().len() == 1 {
        rdt::printhelp();
        std::process::exit(0);
    }

    // Get the first argument
    let arg = env::args().nth(1);
    match arg {
        Some(arg) => {
            match arg.as_str() {
                "h" | "help" => rdt::printhelp(),
                "v" | "version" => println!("{}", VERSION),
                "a" | "add" => {
                    let next_args = env::args().skip(2).collect::<Vec<String>>();
                    // Check if the remianing number of arguments can be divided by 3
                    if next_args.len() % 2 == 0  && next_args.len() != 0 {
                        // Get the remaining arguments
                        // Add the directories
                        next_args.iter().step_by(2).for_each(|path| {
                            let name = match next_args
                                .get(next_args.iter().position(|x| x == path).unwrap() + 1){
                                    Some(name) => name,
                                    None => {
                                        println!("{}: {}", "Fatal Error".red().bold(), "Invalid number of arguments.".italic());
                                        std::process::exit(1);
                                    }
                                };
                            match rdt::add_directory(rdt::Directory {
                                path: path.to_string(),
                                pretty_name: name.to_string(),
                            }) {
                                Ok(_) => {
                                    println!(
                                        "{} {} [{}]",
                                        "Added".green(),
                                        path.bold(),
                                        name.dimmed().italic()
                                    );
                                }
                                Err(e) => {
                                    println!(
                                        "{}: {}",
                                        "Error".red().bold(),
                                        e.to_string().italic()
                                    );
                                }
                            }
                        });
                    } else {
                        println!("{}", "Invalid number of arguments.".red());
                        println!("{}", "Please check the help message.".bold());
                        std::process::exit(1);
                    }
                }
                "s" | "show" => match rdt::get_directories() {
                    Ok(directories) => {
                        if directories.len() == 0 {
                            println!("{}", "No directories are being tracked.".red());
                        } else {
                            println!("{}:", "Tracked Directories".bold());
                            directories.iter().for_each(|directory| {
                                println!(
                                    "{} {} {}",
                                    "Name:".green(),
                                    directory.pretty_name.bold(),
                                    directory.path.italic().dimmed(),
                                );
                            });
                        }
                    }
                    Err(e) => {
                        println!("{}: {}", "Error".red().bold(), e.to_string().italic());
                    }
                },
                "r" | "remove" => {
                    // Colect the remaining arguments
                    let args = env::args().skip(2).collect::<Vec<String>>();
                    // Pass the remaining arguments to the remove function
                    match rdt::remove_directory(args) {
                        Ok(_) => {
                            println!("{}", "Removed the desired directories.".green());
                        },
                        Err(e) => {
                            println!("{}: {}", "Error".red().bold(), e.to_string().italic());
                        }
                    }
                },
                "i" | "install" => {
                    let args = env::args().skip(2).collect::<Vec<String>>();
                    match add_to_path(args){
                        Ok(_) => {
                            println!("{}", "Installed the desired directories.".green());
                        },
                        Err(e) => {
                            println!("{}: {}", "Error".red().bold(), e.to_string().italic());
                        }
                    }
                },
                "u" | "uninstall" => {
                    let args = env::args().skip(2).collect::<Vec<String>>();
                    match rdt::remove_from_path(args){
                        Ok(_) => {
                            println!("{}", "Uninstalled the desired directories.".green());
                        },
                        Err(e) => {
                            println!("{}: {}", "Error".red().bold(), e.to_string().italic());
                        }
                    }
                },
                _ => {
                    println!("{}", "Invalid argument.".red());
                    rdt::printhelp();
                }
            }
        }
        None => {
            rdt::printhelp();
            std::process::exit(0);
        }
    }
}
