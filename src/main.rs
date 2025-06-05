//All the imports
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::env;
mod autocompletion;
mod builtins;
use autocompletion::HelpTab;
use builtins::{cmd_cd, cmd_echo, cmd_ext, cmd_history, cmd_ls, cmd_pwd, cmd_type};

fn main() {
    // Editor and helper declaration
    let mut editor = match Editor::<HelpTab, DefaultHistory>::new() {
        Ok(edit) => edit,
        Err(e) => {
            eprintln!("Erreur à l'initialisation de l'Editor : {}", e);
            return;
        }
    };
    let helper = HelpTab::new();
    editor.set_helper(Some(helper));
    _ = editor.set_history_ignore_dups(false);
    let path_value = match env::var_os("PATH") {
        Some(path_val) => path_val,
        None => {
            eprintln!("Variable d'environnement PATH non ou mal définie");
            return;
        }
    };
    let dirs = env::split_paths(&path_value);
    let mut paths = Vec::new();
    for p in dirs {
        if let Some(s) = p.to_str() {
            paths.push(s.to_string());
        }
    }

    loop {
        // we set the line with a prompt ($ ) its what the user see when he start a line
        // this fonction hold the program until the user finish the line
        let line = editor.readline("$ ");
        // recup the line and make actions
        match line {
            //if its not an error
            Ok(sline) => {
                _ = editor.add_history_entry(sline.as_str());
                // trim of the line, remove the $ and space at the end
                let input = sline.trim();

                if input.is_empty() {
                    // if there is nothing (the user press enter without writing anything)
                    continue; // we skip all the actions and get back to the top of the loop
                }
                let words: Vec<&str> = input.split_whitespace().collect(); // split the line by space to get the words
                match words.as_slice() {
                    // reading the line by slice
                    ["exit", args @ ..] => match args {
                        [] => return,
                        ["0"] => return,
                        [x] => println!("{} n'est pas un argument reconnu par exit.", x),
                        _ => println!("Trop d'argument donnés pour l'utilisation d'exit"),
                    },
                    ["echo", args @ ..] => cmd_echo(args), // if the first word is echo i inject the rest of the line in the echo function
                    ["type", args @ ..] => cmd_type(args, &paths), // same with type but we also need the path for external commande
                    ["history", args @ ..] => { //TODO gérer les nombre négatifs pour faire l'inverse (afficher les x premier si -x est écris)
                        //hsitory command handler
                        if args.is_empty() {
                            // if there is no argument that means we wxant to see all the history, so we put a 0 in the function and the function will understand
                            cmd_history(0, &editor);
                        } else if args.len() > 1 {
                            // if there are more than 1 argument its a wrong use of this command so error msg
                            println!("Trop d'arguments donnés pour history, réessayez avec un seul argument.")
                        } else if let Ok(n) = args[0].parse::<usize>() {
                            // if there only one argument (0 and more than one are tested before) and if we can convert this argument in an usize
                            cmd_history(n, &editor); // we can send this n to the function for print n line of the history
                        } else {
                            // else error msg
                            println!("{} n'est pas un argument valide pour history.", args[0])
                        }
                    }
                    ["pwd", args @ ..] => {
                        //pwd command handler
                        if !args.is_empty() {
                            // if there is at least one argument
                            println!("Cette commande ne prend aucun argument en entrée");
                        //error msg
                        } else {
                            // so if there is no argument
                            cmd_pwd(); // execution of the function
                        }
                    }
                    ["cd", args @ ..] => {
                        //cd command handler
                        if args.is_empty() {
                            // if there are no arguments
                            continue; // we go nowhere we do nothing
                        } else if args.len() > 1 {
                            // if there is more than one argument
                            println!("Trop d'arguments donnés pour cd") // error msg
                        } else {
                            cmd_cd(args[0].to_string()); // we send the path as a string in the function
                        }
                    }
                    ["ls", args @ ..] => {
                        let path = if args.is_empty() {
                            match std::env::current_dir() {
                                Ok(p) => p.to_string_lossy().to_string(),
                                Err(e) => {
                                    eprintln!(
                                        "Erreur dans la récupération du répertoire courant : {}",
                                        e
                                    );
                                    continue;
                                }
                            }
                        } else if args.len() == 1 {
                            args[0].to_string()
                        } else {
                            eprintln!("Trop d'arguments donnés pour ls");
                            continue;
                        };
                        for file in cmd_ls(&path) {
                            println!("{}", file);
                        }
                    }
                    _ => cmd_ext(&words, &paths), // if its not in the builtin we send the line into a external command function (if you use autocompletion dont forget to write the extension of the file you wanna execute)
                }
            }
            // we handle the potentials errors like ctrl-c or ctrl-d
            Err(ReadlineError::Interrupted) => continue, // ctrl-c
            Err(ReadlineError::Eof) => return,           // ctrl-d
            Err(err) => {
                eprintln!("Reading error : {}", err);
                return;
            }
        }
    }
}
