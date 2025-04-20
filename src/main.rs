
// findo "/domain" /hugo/Escritorio/archivos -json
// findo "/domain" /hugo/Escritorio/archivos -csv
// findo "/domain" /hugo/Escritorio/archivos -yaml

use std::collections::HashMap;
use std::{env, str};
use serde_json;
use serde_yaml;
use std::process::Command;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Variable declaration 

    if args.len() < 4 {
        eprintln!("Error: Missing arguments");
        eprintln!("Usage: {} <string_to_find> <path> <file_format>", args[0]);
        std::process::exit(1);
    }

    let string_to_find = &args[1];
    let path = &args[2];
    let file_format = &args[3];

    let mut path_file: HashMap<String, Vec<String>> = HashMap::new();

    for s in greprl(string_to_find, path) {
        
        let path = Path::new(&s);
        
        if let Some(result) = path.parent() {
            // Processing the base directory
            let base_dir = result.to_string_lossy().to_string();

            if let Some (result) = path.file_name() {
                // Processing the name of the file 
                let file_name = result.to_string_lossy().to_string();

                // Add the both variables into the hashmap. 
                if path_file.contains_key(&base_dir) {
                    let mut file_name_vector = path_file.get(&base_dir).unwrap().clone();
                    file_name_vector.push(file_name);
                    path_file.insert(base_dir, file_name_vector);

                }
                else {
                    path_file.insert(base_dir, [file_name].to_vec());
                }
            }

        }
    }

    match file_format.as_str() {
        "-json" => {
            let j = serde_json::to_string_pretty(&path_file).unwrap();
            println! ("{j}");
            
        },
        "-yaml" => {
            let j = serde_yaml::to_string(&path_file).unwrap();
            println! ("{j}");
        },
        "-csv" => {
            let j = to_csv_vertical(path_file);
            println! ("{j}");
        },
        "-csv-horizontal" => {
            let j = to_csv_horizontal(path_file);
            println! ("{j}");
        },
        "-csv-vertical" => {
            let j = to_csv_vertical(path_file);
            println! ("{j}");
        },
        _=> {
            panic! ("Unrecognized format (choose -json, -yaml, -csv)");
        }
    }

    fn greprl (string_to_find: &str, path: &str) -> Vec<String> {

        // Will contain the output of the grep -rl command
        let mut result: Vec<String> = Vec::new();

        // Execution of the command
        let output = if cfg!(target_os = "windows") {
            panic!("This program is intended to be ron on linux only, with the use of the grep -rf command. Bye!");
        } else {
            Command::new("grep")
                .arg("-rl")
                .arg(string_to_find) 
                .arg(path)
                .output()
                .expect("failed to execute process")
        };

        // Convert the output into utf8 compatible string
        let s = match str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        // Save the ocurrences into the result array
        for ocurrence in s.split("\n") {
            result.push(ocurrence.to_string());
        }

        // Return
        result
    }

    fn to_csv_horizontal (path_file: HashMap<String, Vec<String>>) -> String {
        let mut result: String = String::new();

        for (path, files) in &path_file {
            result.push_str(&path);
            result.push_str(", ");
            result.push_str(&files.join(", "));
            result.push_str("\n");
        }

        result
    }

    fn to_csv_vertical  (path_file: HashMap<String, Vec<String>>) -> String { 
        let mut result: String = String::new();

        // Determine the number of rows and columns the table will have.

        let columns = path_file.len();
        let mut rows =0;
        let mut rows_aux =0;

        for (_path, files) in &path_file {
            for _file in files  {
                rows_aux+=1;
            }

            if rows_aux>rows {
                rows = rows_aux;
            };
            rows_aux =0;   
        }

        // Enter the values into a two-dimensional array of strings, for further processing.

        let mut matrix: Vec<Vec<String>> = vec![vec![String::from(""); rows+1]; columns];
        let mut i =0;
        let mut j =0;

        for (path, files) in &path_file {
            matrix [i][j] = path.to_string();
            j+=1;

            for file in files {
                matrix [i][j] = file.to_string();
                j+=1;
            }

            j=0;
            i+=1;
        } 

        // Process the two-dimensional matrix so that rows are inverted instead of columns.
        
        for j in 0..rows+1 {
            for i in 0..columns{
                result.push_str(&matrix [i][j]);
                if i!=columns-1 {
                    result.push_str(", ");
                }
            }
            result.push_str("\n");
        
        }

        result
    }

}