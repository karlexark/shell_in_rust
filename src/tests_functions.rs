//All the imports
use crate::builtins::{cmd_cd, cmd_echo, cmd_ext, cmd_history, cmd_ls, cmd_pwd, cmd_type};
use gag::BufferRedirect;
use std::collections::HashMap;
use std::io::Read;
use std::path;

pub fn cmd_echo_test() {
    let mut test_ios: HashMap<String, [&[&str]; 2]> = HashMap::new();

    test_ios.insert("Input".to_string(), [&["test"], &["test", "test"]]);
    test_ios.insert("Output".to_string(), [&["test\n"], &["test test\n"]]);

    let mut capt_out = match BufferRedirect::stdout() {
        Ok(buff_ok) => buff_ok,
        Err(e) => {
            eprintln!("Impossible de déclarer le bufferredirect : {}", e);
            return;
        }
    };

    let mut output = String::new();

    for (i, test) in test_ios.get("Input").unwrap().iter().enumerate() {
        cmd_echo(test);
        match capt_out.read_to_string(&mut output) {
            Ok(_) => (),
            Err(e) => eprintln!("Erreur lors de la lecture du terminal : {}", e),
        };
        let lines: Vec<&str> = output.lines().collect();
        let nb = lines.len();
        let test_output;
        test_output = format!("{}\n", lines[nb - 1]);

        if test_output == test_ios.get("Output").unwrap()[i].join(" ") {
            eprintln!("Test echo n°{} réussis !", i);
        } else {
            eprintln!(
                "Test echo n°{} échoué. \nRésultat attendus : \n[{}]\nRésultat reçu : \n[{}]",
                i,
                test_ios.get("Output").unwrap()[i].join(" "),
                test_output
            )
        };
    }
}

pub fn cmd_type_test(paths: &Vec<String>,existing_file : &str, path_to_the_existing_file: &str, not_existing_file : &str) {
    let mut test_ios: HashMap<String, [&[&str]; 5]> = HashMap::new();
    let exist = [existing_file];
    let not_exist = [not_existing_file];
    let exist_output_sentence = format!("{} is {}{}\n", &existing_file,&path_to_the_existing_file,&existing_file);
    let exist_output = [exist_output_sentence.as_str()];
    let not_exist_output_sentence = format!("{}: not found\n",&not_existing_file);
    let not_exist_output = [not_exist_output_sentence.as_str()];
    test_ios.insert(
        "Input".to_string(),
        [
            &[],
            &["echo"],
            &exist,
            &["t", "t"],
            &not_exist,
        ],
    );
    test_ios.insert(
        "Output".to_string(),
        [
            &["\n"],
            &["echo is a shell builtin\n"],
            &exist_output,
            &["type : too many arguments\n"],
            &not_exist_output,
        ],
    );

    let mut capt_out = match BufferRedirect::stdout() {
        Ok(buff_ok) => buff_ok,
        Err(e) => {
            eprintln!("Impossible de déclarer le bufferredirect : {}", e);
            return;
        }
    };

    let mut output = String::new();
    for (i, test) in test_ios.get("Input").unwrap().iter().enumerate() {
        if i == 0 {
            cmd_type(&[], paths);
        } else {
            cmd_type(test, paths);
        }
        output.clear();
        match capt_out.read_to_string(&mut output) {
            Ok(_) => (),
            Err(e) => eprintln!("Erreur lors de la lecture du terminal : {}", e),
        };

        if output.is_empty() {
            output = "\n".to_string();
        };

        if output == test_ios.get("Output").unwrap()[i].join(" ") {
            eprintln!("Test type n°{} réussis !", i);
        } else {
            eprintln!(
                "Test type n°{} échoué. \nRésultat attendus : \n[{}]\nRésultat reçu : \n[{}]",
                i,
                test_ios.get("Output").unwrap()[i].join(" "),
                output
            )
        };
    }
}
