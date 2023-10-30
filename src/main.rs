use invidivual_project_kelly_rust::{extract, query, transform_load};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            let result = extract(
                "https://raw.githubusercontent.com/selva86/datasets/master/Auto.csv",
                "Auto.csv",
            );
            match result {
                Ok(path) => println!("Data extraction completed successfully. Saved to {}", path),
                Err(e) => eprintln!("Error during extraction: {:?}", e),
            }
        }
        "transform_load" => {
            let result = transform_load("Auto.csv");
            match result {
                Ok(path) => println!(
                    "Data transformation and loading completed successfully. DB path: {}",
                    path
                ),
                Err(e) => eprintln!("Error during loading: {:?}", e),
            }
        }
        "query" => {
            if args.len() < 3 {
                println!("Please provide a SQL query string");
                return;
            }
            let query_string = &args[2];
            query(query_string).unwrap();
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
        }
    }
}
