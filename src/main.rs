#[allow(unused_imports)]
use std::cell::{Cell, RefCell};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::{usize, vec};
use anyhow::Error;
use rustyline::completion::Pair;
use rustyline_derive::{Helper, Hinter, Highlighter, Validator};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
fn main() {
    let mut editor = Editor::<HelpTab,DefaultHistory>::new().unwrap();
    let helper = HelpTab::new();
    editor.set_helper(Some(helper));
    let path_value = std::env::var("PATH").unwrap();
    let paths: Vec<&str> = path_value.split(':').collect();
    loop{   
        // print!("$ ");
        // io::stdout().flush().unwrap();
    
    // Wait for user input
        // let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();
        let line = editor.readline("$ ");
        match line{
            Ok(sline)=> {
                editor.add_history_entry(sline.as_str());
                let input = sline.trim();
                if input.is_empty(){
                    continue;
                }
                let words: Vec<&str> = input.split_whitespace().collect();
                match words.as_slice(){
                    [""] => continue,
                    ["exit","0"] => return, 
                    ["echo", args @ ..] => cmd_echo(args),
                    ["type", args @ ..] => cmd_type(args,&paths),
                    _ => cmd_ext(&words,&paths),
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof)=> return,
            Err(err)=>{
                eprintln!("Reading error : {}",err);
                return;
            },
            
        }
        
    }
}

fn cmd_echo(args: &[&str]){
    println!("{}",args.join(" "));
}

fn cmd_type(args: &[&str],paths: &Vec<&str>){
    let args_len = args.len();

    match args_len{
        0 => return,
        1 =>{
            match args[0]{
                "exit" | "echo" | "type" => println!("{} is a shell builtin", args[0]),

                _ => {
                        let mut found = false;
                        for dir in paths.iter() {
                            let full_path = format!("{}/{}", dir, args[0]);
                            if Path::new(&full_path).is_file() {
                                //let meta = std::fs::metadata(&full_path);
                                //let mode = meta.permissions().mode();
                                //if mode & 0o111 !=0{
                                found = true;
                                println!("{} is {}",args[0],full_path);
                                break;
                                //}
                            }
                        }
                        if !found {
                            println!("{}: not found", args[0]);
                        }
                }
            }
        },
        _ => {
            println!("type : too many arguments");
            return;
        },

    }
}

fn cmd_ext(args: &[&str],paths: &Vec<&str>){
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
                "exit".to_string(),
                "type".to_string(),
            ],
            last_prefix : RefCell::new(String::new()),
            already_tab : Cell::new(false),

        }
    }
    
}

impl rustyline::completion::Completer for HelpTab{
    type Candidate = rustyline::completion::Pair;
    fn complete(
            &self, // FIXME should be `&mut self`
            line: &str,
            pos: usize,
            _ctx: &rustyline::Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {

            let start : usize;
            let (avant,_) = line.split_at(pos);
            let maybe_space_pos= avant.rfind(' ');
            let mut prefixe : String = "".to_string();
            let nb_match : u64;
            let mut suggestions : Vec<Pair> = Vec::new();
            if maybe_space_pos == None{
                start= 0;
                prefixe = avant[0..pos].to_string();
            }else{
                let space_pos = maybe_space_pos.unwrap();
                start = space_pos+1;
                prefixe = avant[start..pos].to_string();
            }
            (nb_match,suggestions) = search_match(&prefixe, self).unwrap();
            if !self.already_tab.get() {
                self.last_prefix.replace(prefixe.clone());
                self.already_tab.set(false);
                match nb_match as i32 {
                    0..=1 => {
                        return Ok((start,suggestions));

                    },
                    _ => {
                        self.already_tab.set(true);
                        return Ok((start, Vec::new()));
                    }
                }
            }else{
                self.already_tab.set(false);
                match nb_match as i32  {
                    0..=1 => {
                        return Ok((start,suggestions));

                    },
                    _ => {
                        let mut all_suggestion : String ="".to_string();
                        for suggestion in suggestions{
                            all_suggestion = all_suggestion + &suggestion.display + "  ";
                        }
                        println!("\n{}",all_suggestion);
                        std::io::stdout().flush().unwrap();
                        let reset = Pair{
                            replacement: "".to_string(),
                            display: "".to_string(),
                        };
                        let mut resets = Vec::new();
                        resets.push(reset);
                        return Ok((start, resets));
                    }
                }
            }
            
    }
}


fn search_match(prefixe: &String,helper : &HelpTab)->Result<(u64, Vec<Pair>), Error>{
    let mut nb_match :u64 = 0;
    let mut suggestions : Vec<Pair> = Vec::new();
    let mut all_suggestions : String = "".to_string();
    if !(prefixe ==""){
        for builtin in &helper.builtins{
            if builtin.starts_with(prefixe) {
                nb_match = nb_match + 1;
            
                all_suggestions = all_suggestions + &builtin.to_string() + "  ";
                let suggestion = Pair{
                    display : builtin.clone(),
                    replacement : format!("{} ", builtin),
                };
                suggestions.push(suggestion);
                    
                
            }
        }
        // let path_value = std::env::var("PATH").unwrap();
        // let paths: Vec<&str> = path_value.split(':').collect();
        
        // for dir in paths.iter() {
        //     let files = std::fs::read_dir(dir).unwrap();
        //     for file_result in files{
        //         let file = match file_result{
        //             Ok(e) => e,
        //             Err(_) => continue,
        //         };
        //         let file_name_os = file.file_name();
        //         if let Some(file_name) = file_name_os.to_str(){
        //             if file_name.starts_with(prefixe) {
        //                 nb_match = nb_match + 1;
        //                 all_suggestions = all_suggestions + &file_name.to_string()+ "  ";
        //                 let suggestion = Pair{
        //                     display : file_name.to_string().clone(),
        //                     replacement : format!("{} ", file_name),
        //                 };
        //                 suggestions.push(suggestion);
        //             }
        //         }      
        //     }
        // }
        match nb_match  {
            1 => return Ok((nb_match,suggestions)),
            0 => return Ok((nb_match, Vec::new())),
            _ =>{
                return Ok((nb_match, suggestions));
            },
        }
    }
    return Ok((0 as u64, Vec::new()));



}