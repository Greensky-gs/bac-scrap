mod files;
mod tools;
mod result;

use crate::files::{Metadata, FilterInput};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn find_value_in_args(arguments: &Vec<String>, value: &String) -> Option<String> {
    let mut i = 0;
    while i < arguments.len() - 1 {
        if *arguments.get(i).unwrap() == format!("--{}", value) {
            let val = arguments.get(i + 1).unwrap();
            if !val.starts_with("--") {
                return Some(val.to_string());
            }
        }

        i+=1;
    }

    return None;
}

fn main() {
    let _ = std::fs::create_dir("data");
    let mut meta = Metadata::new(&"data/errors.txt".to_string(), &"data/results.json".to_string(), &"data/indexes".to_string());

    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--fetch".to_string()) {

        match find_value_in_args(&args, &"input".to_string()) {
            None => {
                println!("\x1b[94mFetch selected\x1b[90m starting...\x1b[0m");
                meta.start_fetch();
            },
            Some(input) => {
                println!("\x1b[94Fetching :\x1b[91m{}\x1b[0m", input);
                meta.fetch_input(&input);
            }
        }

    } else if args.contains(&"--search".to_string()) || args.contains(&"--cli".to_string()) {
        let mut editor = DefaultEditor::new().unwrap();
        let _hsty = editor.load_history("data/history.txt");

        println!("\x1b[4m\x1b[34mQuick reminder :\n\x1b[0m    \x1b[90m--name [nom]\x1b[0m to filter names\n    \x1b[90m--prenom [prenom]\x1b[0m to filter nickname\n    \x1b[90m--admis [bool]\x1b[0m to filter by admission\n\x1b[34mHelpers :\x1b[0m\n    \x1b[90m--exact-nom [bool]\x1b[0m match names exactly\n    \x1b[90m--exact-prenom [bool]\x1b[0m matches prenoms exactly\n    \x1b[90m--exact-all [bool]\x1b[0m matches both exactly (no matter the value given)");

        loop {
            match editor.readline("> ") {
                Err(ReadlineError::Interrupted) => {
                    println!("\x1b[31mCanceled\x1b[0m. Exiting");
                    break;
                },
                Err(ReadlineError::Eof) => {
                    println!("End of file reached");
                }
                Err(err) => {
                    println!("An error occured: {:?}", err);
                },
                Ok(line) => {
                    let _ = editor.add_history_entry(line.as_str());
                    let _ = editor.save_history("data/history.txt");

                    let vals: Vec<String> = line
                        .split(" ")
                        .map(|x| x.to_string())
                        .collect();

                    let exact_all = match find_value_in_args(&vals, &"exact-all".to_string()) {
                        Some(val) => {
                            val == "true".to_string()
                        },
                        None => {
                            false
                        }
                    };
                    let params = FilterInput {
                        nom: find_value_in_args(&vals, &"name".to_string()),
                        prenom: find_value_in_args(&vals, &"prenom".to_string()),
                        admis: match find_value_in_args(&vals, &"admis".to_string()) {
                            Some(val) => {
                                Some(val == "true".to_string())
                            },
                            None => {
                                None
                            }
                        },
                        exact_name: if exact_all {Some(true)} else {match find_value_in_args(&vals, &"exact-nom".to_string()) {
                            None => {
                                None
                            },
                            Some(val) => {
                                Some(val == "true".to_string())
                            }
                        }},
                        exact_prenom: if exact_all {Some(true)} else {match find_value_in_args(&vals, &"exact-prenom".to_string()) {
                            None => {
                                None
                            },
                            Some(val) => {
                                Some(val == "true".to_string())
                            }
                        }}
                    };

                    if params.nom.is_none() && params.prenom.is_none() && params.admis.is_none() {
                        println!("\x1b[31mError\x1b[0m, nothing specified");
                        continue;
                    }

                    let results = meta.filter(params);
                    if results.len() == 0 {
                        println!("No results found");
                    } else {
                        println!("Found \x1b[33m{}\x1b[0m matches", results.len());
                        for r in results {
                            r.display();
                        }
                    }
                }
            }
        }
    } else {
        println!("No action specified, use \x1b[94m--fetch\x1b[0m or \x1b[94m--search\x1b[0m");
    }

}
