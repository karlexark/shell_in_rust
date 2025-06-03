//All the imports
#[allow(unused_imports)]
use std::cell::{Cell, RefCell};
use std::io::{Write};
use std::path::{Path};
use std::process::Command;
use std::{usize, vec};
use anyhow::Error;
use rustyline::completion::Pair;
use rustyline_derive::{Helper, Hinter, Highlighter, Validator};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use std::env;


fn main() {
    // Editor and helper declaration
    let mut editor = Editor::<HelpTab,DefaultHistory>::new().unwrap();
    let helper = HelpTab::new();
    editor.set_helper(Some(helper));
    // recuperation of the path thanks to the key path who is given by the codecrafters tester
    let path_value = env::var_os("PATH").unwrap(); 
    let dirs = env::split_paths(&path_value);
    let mut paths = Vec::new();
    for p in dirs {
        if let Some(s) = p.to_str(){
            paths.push(s.to_string());
        }
        
    }

    loop{   
        // we set the line with a prompt ($ ) its what the user see when he start a line
        // this fonction hold the program until the user finish the line
        let line = editor.readline("$ ");
        // recup the line and make actions 
        match line{
            //if its not an error
            Ok(sline)=> {
                // its not handle yet but we keep in memory the line to get a history
                _= editor.add_history_entry(sline.as_str());
                // trim of the line, remove the $ and space at the end 
                let input = sline.trim();

                if input.is_empty(){ // if there is nothing (the user press enter without writing anything)
                    continue; // we skip all the actions and get back to the top of the loop
                }
                let words: Vec<&str> = input.split_whitespace().collect(); // split the line by space to get the words
                match words.as_slice(){ // reading the line by slice 
                    [""] => continue, // its not suppose to be usefull but still we never know
                    ["exit","0"] => return,  // if its "exit 0" we stop the programme by exiting the main (i prefer only exit but its for codecrafters)
                    ["echo", args @ ..] => cmd_echo(args), // if the first word is echo i inject the rest of the line in the echo function
                    ["type", args @ ..] => cmd_type(args,&paths), // same with type but we also need the path for external commande
                    _ => cmd_ext(&words,&paths), // if its not in the builtin we send the line into a external command function
                }
            }
            // we handle the potentials errors like ctrl-c or ctrl-d 
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof)=> return,
            Err(err)=>{
                eprintln!("Reading error : {}",err);
                return;
            },
            
        }
        
    }
}

// its the echo command function which it call when the user write echo blabla in the terminal
fn cmd_echo(args: &[&str]){
    println!("{}",args.join(" ")); // we print the args wich is all the line but "echo "
}

// its the type command function which is call when the user write type blabla on the terminal
fn cmd_type(args: &[&str],paths: &Vec<String>){
    let args_len = args.len(); // recup of the len of the args

    match args_len{
        0 => return, // if there is nothing (the user only wrote type) we return and on the terminal will print the next line 
        1 =>{ // if there is one arg
            match args[0]{
                "exit" | "echo" | "type" => println!("{} is a shell builtin", args[0]), // fisrt we look into the builtins list

                _ => {// if its not we gonna look into the dir to find if the command exist 
                        let mut found = false;
                        for dir in paths.iter() { // for every dir in the paths
                            let full_path = format!("{}/{}", dir, args[0]); // we create a full path with the idir and the command we are looking for
                            if Path::new(&full_path).is_file() { // if this is a file 
                                found = true; // we found it
                                println!("{} is {}",args[0],full_path); // we print the command and the directory where it is 
                                break;
                            }
                        }
                        if !found { // if after lokking into all the dir we didn't find anything 
                            println!("{}: not found", args[0]); // we print it to the user 
                        }
                }
            }
        },
        _ => { // if there are more than one arg 
            println!("type : too many arguments");
            return;
        },

    }
}

fn cmd_ext(args: &[&str],paths: &Vec<String>){
    let args_len = args.len();
    match args_len{
        0 => return,
        _ => {
            
            let mut found = false;
            for dir in paths.iter() {
                let full_path = format!("{}/{}", dir, args[0]);
                if Path::new(&full_path).is_file() {
                    found = true;
                    run_external(&args[0],&args[1..]);
                    break;
                    //}
                }
            }
            if !found {
                println!("{}: not found", args[0]);
            }

        }
    }
}

fn run_external(program_name: &str, args:&[&str]) {
    let _cmd = Command::new(program_name)
        .args(args)
        .status();
}
#[derive(Helper,Hinter,Highlighter,Validator)]
pub struct HelpTab{
    builtins: Vec<String>,
    last_prefix : RefCell<String>,
    already_tab : Cell<bool>,
}
impl HelpTab {
    pub fn new() -> Self{
        Self{
            builtins: vec![
                "echo".to_string(),
                "echo_type".to_string(),
                "echo_type_exit".to_string(),
                "echo_type_exit_return".to_string(),
                "exit".to_string(),
                "type".to_string(),
            ],
            last_prefix : RefCell::new(String::new()),
            already_tab : Cell::new(false),

        }
    }
    
}
//TODO comprendre pourquoi l'autocompletion ne fonctionne pas avec les exe externes 
impl rustyline::completion::Completer for HelpTab{
    type Candidate = rustyline::completion::Pair;
    fn complete(
            &self, 
            line: &str,
            pos: usize,
            _ctx: &rustyline::Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {

            let start : usize;
            let (avant,_) = line.split_at(pos);
            let maybe_space_pos= avant.rfind(' ');
            let prefixe : String ;
            let nb_match : u64;
            let suggestions : Vec<Pair>;
            if maybe_space_pos == None{
                start= 0;
                prefixe = avant[0..pos].to_string();
            }else{
                let space_pos = maybe_space_pos.unwrap();
                start = space_pos+1;
                prefixe = avant[start..pos].to_string();
            }
            (nb_match,suggestions) = search_match(&prefixe, self).unwrap();
            print!("{}",suggestions[0].display);
            if !self.already_tab.get() || !(*self.last_prefix.borrow() == prefixe){
                self.last_prefix.replace(prefixe.clone());
                self.already_tab.set(false);
                let exist: bool;
                let every_match_list :Vec<String>;
                match nb_match as i32 {
                    0..=1 => {
                        return Ok((start,suggestions));

                    },
                    _ => {
                        let mut list = Vec::new();
                        for suggestion in &suggestions{
                            list.push(&suggestion.display);
                        }
                        (exist,every_match_list) = match_in_a_vec(list).unwrap();
                        if exist==true{
                            let rempl = Pair{
                                display: every_match_list[0].clone(),
                                replacement: format!("{}",every_match_list[0].clone()),
                            };
                            let mut l_rempl = Vec::new();
                            l_rempl.push(rempl);
                            return Ok((start,l_rempl));
                        }else{
                            self.already_tab.set(true);
                            return Ok((start, Vec::new())) ;
                        }
                            
                    },
                        
                }
                
            }else{
                self.already_tab.set(false);
                match nb_match as i32  {
                    0..=1 => {
                        return Ok((start,suggestions));

                    },
                    _ => {
                        let mut all_suggestion : String ="".to_string();
                        let mut suggestions_list : Vec<String> = Vec::new();
                        for suggestion in suggestions{
                            suggestions_list.push(suggestion.display);
                        }
                        suggestions_list.sort();
                        for suggestion in suggestions_list{
                            all_suggestion = all_suggestion + &suggestion + "  ";
                        }
                        println!("\n{}",all_suggestion);
                        std::io::stdout().flush().unwrap();
                        let pref = Pair{
                            display: prefixe.clone(),
                            replacement: prefixe.clone(),
                        };
                        let mut l_pref = Vec::new();
                        l_pref.push(pref);
                        return Ok((start, l_pref));
                    }
                }
            }
            
    }
}


fn search_match(
    prefixe: &String,
    helper: &HelpTab
) -> Result<(u64, Vec<Pair>), Error> {
    let mut nb_match: u64 = 0;
    let mut suggestions: Vec<Pair> = Vec::new();

    // 1) Cherche dans les builtins
    for builtin in &helper.builtins {
        if builtin.starts_with(prefixe) {
            nb_match += 1;
            suggestions.push(Pair {
                display: builtin.clone(),
                replacement: format!("{} ", builtin),
            });
        }
    }
    if nb_match != 0 {
        return Ok((nb_match, suggestions));
    }

    // 2) Aucun builtin ne matche : on cherche dans les répertoires PATH
    let path_value = env::var_os("PATH").unwrap();
    let dirs = env::split_paths(&path_value);

    for p in dirs {
        // Récupère `p` comme PathBuf. On essaye de convertir en &str pour l'afficher/loguer, mais ce n'est pas nécessaire.
        let dir_path = p; // PathBuf
        // Tenter d'ouvrir ce dossier :
        let read_dir_iter = match std::fs::read_dir(&dir_path) {
            Ok(iter) => iter,
            Err(_) => continue, // dossier introuvable ou inaccessible : on passe au suivant
        };

        // Itérer sur les entrées de ce dossier
        for entry_result in read_dir_iter {
            let entry = match entry_result {
                Ok(e) => e,
                Err(_) => continue, // impossible de lire cette entrée, on l'ignore
            };

            // Récupère le nom de fichier en tant que &str (si possible)
           if let Some(os_name) = entry.file_name().to_str() {
                let path = Path::new(os_name);         
                if let Some(stem_os) = path.file_stem() {
                    if let Some(stem_str) = stem_os.to_str() {
                        print!("{}", stem_str.to_string());
                        // stem_str = "fichier"
                        if stem_str.starts_with(prefixe) {
                            nb_match += 1;
                            suggestions.push(Pair {
                                display: stem_str.to_string(),
                                replacement: format!("{} ", stem_str),
                            });
                        }
                    }
                }
            }
        }
    }

    // 3) Retour selon le nombre de correspondances
    match nb_match {
        1 => Ok((nb_match, suggestions)),
        0 => Ok((0, Vec::new())),
        _ => Ok((nb_match, suggestions)),
    }
}

fn match_in_a_vec(list: Vec<&String>) -> Result<(bool,Vec<String>),Error>{
    let mut every_match_list =Vec::new();
    let mut every_match = true;
    let exist: bool;
    let mut fixed_trame;
    for trame in &list{
        fixed_trame = trame;
        eprintln!("{}", fixed_trame);
        for test in &list{
            if !test.starts_with(&trame.to_string()){
                every_match = false;
                break;
            }
        }
        if every_match == true{
            
            every_match_list.push(fixed_trame.to_string().clone());
        }
    }
    if every_match_list.len()!=0{
        exist = true;
    }else {
        exist = false;
    }

    return Ok((exist,every_match_list));
}