//All the imports
use crate::builtins::cmd_ls;
use anyhow::Error;
use rustyline::completion::Pair;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};
use std::cell::{Cell, RefCell};
use std::env;
use std::io::Write;
use std::path::Path;

#[derive(Helper, Hinter, Highlighter, Validator)]
pub struct HelpTab {
    builtins: Vec<String>,
    last_prefix: RefCell<String>,
    already_tab: Cell<bool>,
}

impl HelpTab {
    pub fn new() -> Self {
        Self {
            builtins: vec![
                "echo".to_string(),
                "exit".to_string(),
                "type".to_string(),
                "history".to_string(),
                "pwd".to_string(),
                "cd".to_string(),
                "ls".to_string(),
            ],
            last_prefix: RefCell::new(String::new()),
            already_tab: Cell::new(false),
        }
    }
}

impl rustyline::completion::Completer for HelpTab {
    type Candidate = rustyline::completion::Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut skp_bltins = false;
        let start: usize;
        let (avant, _) = line.split_at(pos);
        let prefixe: String;
        let nb_match: u64;
        let suggestions: Vec<Pair>;
        let maybe_space_pos = avant.rfind(' ');
        if maybe_space_pos.is_none() {
            start = 0;
            prefixe = avant[0..pos].to_string();
        } else if let Some(space_pos) = maybe_space_pos {
            start = space_pos + 1;
            prefixe = avant[start..pos].to_string();
        } else {
            // not supose to happend
            start = 0;
            prefixe = String::new();
        }

        if line.len() > 1 {
            if &line[0..=1] == "cd" || &line[0..=1] == "ls" {
                skp_bltins = true;
            }
        } else if line.len() == 0 {
            return Ok((start, Vec::new()));
        }
        (nb_match, suggestions) = search_match(skp_bltins, &prefixe, self).unwrap();
        if !self.already_tab.get() || *self.last_prefix.borrow() != prefixe {
            self.last_prefix.replace(prefixe.clone());
            self.already_tab.set(false);
            let exist: bool;
            let every_match_list: Vec<String>;
            match nb_match as i32 {
                0..=1 => {
                    return Ok((start, suggestions));
                }
                _ => {
                    let mut list = Vec::new();
                    for suggestion in &suggestions {
                        list.push(&suggestion.display);
                    }
                    (exist, every_match_list) = match_in_a_vec(list).unwrap();
                    if exist == true {
                        let rempl = Pair {
                            display: every_match_list[0].clone(),
                            replacement: format!("{}", every_match_list[0].clone()),
                        };
                        let mut l_rempl = Vec::new();
                        l_rempl.push(rempl);
                        return Ok((start, l_rempl));
                    } else {
                        self.already_tab.set(true);
                        return Ok((start, Vec::new()));
                    }
                }
            }
        } else {
            self.already_tab.set(false);
            match nb_match as i32 {
                0..=1 => {
                    return Ok((start, suggestions));
                }
                _ => {
                    let mut all_suggestion: String = "".to_string();
                    let mut suggestions_list: Vec<String> = Vec::new();
                    for suggestion in suggestions {
                        suggestions_list.push(suggestion.display);
                    }
                    suggestions_list.sort(); //TODO rendre le tri plus robuste
                    for suggestion in suggestions_list {
                        all_suggestion = all_suggestion + &suggestion + "  ";
                    }
                    println!("\n{}", all_suggestion);
                    std::io::stdout().flush().unwrap();
                    let pref = Pair {
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

pub fn search_match(
    skp_bltins: bool,
    prefixe: &String,
    helper: &HelpTab,
) -> Result<(u64, Vec<Pair>), Error> {
    let mut nb_match: u64 = 0;
    let mut suggestions: Vec<Pair> = Vec::new();
    if !skp_bltins {
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
    }
    match std::env::current_dir() {
        Ok(path) => {
            let file_list = cmd_ls(&path.to_string_lossy().to_string());
            for file in file_list {
                if file.starts_with(prefixe) {
                    nb_match = nb_match + 1;
                    suggestions.push(Pair {
                        display: file.clone(),
                        replacement: format!("{} ", file),
                    });
                }
            }
        }
        Err(_) => (),
    }
    if nb_match != 0 {
        return Ok((nb_match, suggestions));
    }

    let path_value = env::var_os("PATH").unwrap();
    let dirs = env::split_paths(&path_value);

    for p in dirs {
        let dir_path = p; // PathBuf
        let read_dir_iter = match std::fs::read_dir(&dir_path) {
            Ok(iter) => iter,
            Err(_) => continue,
        };

        for entry_result in read_dir_iter {
            let entry = match entry_result {
                Ok(e) => e,
                Err(_) => continue,
            };

            if let Some(os_name) = entry.file_name().to_str() {
                let path = Path::new(os_name);
                if let Some(stem_os) = path.file_stem() {
                    if let Some(stem_str) = stem_os.to_str() {
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

    match nb_match {
        1 => Ok((nb_match, suggestions)),
        0 => Ok((0, Vec::new())),
        _ => Ok((nb_match, suggestions)),
    }
}

pub fn match_in_a_vec(list: Vec<&String>) -> Result<(bool, Vec<String>), Error> {
    let mut every_match_list = Vec::new();
    for trame in &list {
        let common_prefix = list.iter().all(|test| test.starts_with(trame.as_str()));
        if common_prefix {
            every_match_list.push(trame.to_string());
        }
    }
    let exist = !every_match_list.is_empty();
    return Ok((exist, every_match_list));
}
