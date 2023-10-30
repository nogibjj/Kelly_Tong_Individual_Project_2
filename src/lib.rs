use csv::ReaderBuilder;
use reqwest;
use reqwest::blocking::Client;
use rusqlite::{params, Connection};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{prelude::*, BufReader};

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

#[derive(Debug)]
pub struct Auto {   
    _mpg: f64,
    _cylinders: i32,
    _displacement: f64,
    _horsepower: f64,
    _weight: f64,
    _acceleration: f64,
    _year: i32,
    _origin: i32,
    _name: String,
}

// Function to extract data from a URL and save to a file
pub fn extract(url: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let mut file = File::create(file_path)?;
    file.write_all(&content)?;
    Ok(file_path.to_string())
}

// Function to load CSV data into SQLite
pub fn transform_load(dataset: &str) -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::open("AutoDB.db")?;

    conn.execute("DROP TABLE IF EXISTS AutoDB", params![])?;

    let create_table_query = "
        CREATE TABLE AutoDB (
            MPG REAL,
            Cylinders INTEGER,
            Displacement REAL,
            Horsepower REAL,
            Weight REAL,
            Acceleration REAL,
            Year INTEGER,
            Origin INTEGER,
            Name Text
        )
    ";
    conn.execute(create_table_query, params![])?;

    let file = File::open(dataset)?;
    let reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(reader);

    for result in rdr.records() {
        let record = result?;
        conn.execute(
            "INSERT INTO AutoDB (MPG, Cylinders, Displacement, Horsepower, Weight, Acceleration, Year, Origin, Name) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                record.get(0).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(1).unwrap_or("0").parse::<i32>().unwrap_or(0),
                record.get(2).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(3).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(4).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(5).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(6).unwrap_or("0").parse::<i32>().unwrap_or(0),
                record.get(7).unwrap_or("0").parse::<i32>().unwrap_or(0),
                record.get(8).unwrap_or("")
            ],
        )?;
    }

    Ok("Auto.db".to_string())
}

pub fn query(query_string: &str) -> Result<String, rusqlite::Error> {
    let conn = Connection::open("Auto.db")?;

    if query_string
        .trim_start()
        .to_uppercase()
        .starts_with("SELECT")
    {
        let mut stmt = conn.prepare(query_string)?;
        let Auto_iter = stmt.query_map([], |row| {
            Ok(Auto {         
                _mpg: row.get(0)?,
                _cylinders: row.get(1)?,
                _displacement: row.get(2)?,
                _horsepower: row.get(3)?,
                _weight: row.get(4)?,
                _acceleration: row.get(5)?,
                _year: row.get(6)?,
                _origin: row.get(7)?,
                _name: row.get(8)?,
            })
        })?;

        for result in Auto_iter {
            match result {
                Ok(Auto) => {
                    println!(
                        "Results: MPG = {}, Cylinders = {}, Displacement = {}, Horsepower = {}, Weight = {}, Acceleration = {}, Year = {}, Origin = {}, Name = {}",  
                        Auto._mpg, Auto._cylinders, Auto._displacement, Auto._horsepower, Auto._weight, Auto._acceleration, Auto._year, Auto._origin, Auto._name
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        let _num_affected_rows = conn.execute_batch(query_string)?;
    }
    log_query(query_string, LOG_FILE);
    Ok("Query executed successfully".to_string())
}
