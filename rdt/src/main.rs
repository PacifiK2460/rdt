use std::{path::PathBuf, cmp::Ordering};

use requestty::{Question, DefaultSeparator, prompt, Answer, prompt_one};
use which::{which, Error};

#[derive(Debug, Clone)]
struct Program {
    name: String,
    path: Result<PathBuf, Error>,
}

// C:\Program Files\GitHub CLI\

fn main() {

    /* Programs
        Vscode
        Git
        GitHub CLI
        gcc
    */ 
    let programs = vec![
        Program {
            name: "VSCode".to_string(),
            path: which("code"),
        },
        Program {
            name: "Git".to_string(),
            path: which("git"),
        },
        Program {
            name: "GitHub CLI".to_string(),
            path: which("gh"),
        },
        Program {
            name: "gcc".to_string(),
            path: which("gcc"),
        },
    ];

    // Clone missing programs to a new vector
    let mut missing_programs = programs.clone();
    missing_programs.retain(|program| program.path.is_err());

    if missing_programs.is_empty() {
        // print in bold green
        println!("✅ \x1b[1;32mAll programs are installed\x1b[0m");
        return;
    }

    // Ask user to install missing programs
    let to_install = Question::multi_select("to_install")
        .message("Which programs do you want to install?")
        .choices(
            missing_programs
                .iter()
                .map(|program| program.name.clone())
                .collect::<Vec<String>>(),
        )
        .build();

    prompt_one(to_install).unwrap();

}
