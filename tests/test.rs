use invidivual_project_kelly_rust::{extract, query, transform_load};

#[test]
fn test_extract_and_transform_load() {
    let url = "https://raw.githubusercontent.com/selva86/datasets/master/Auto.csv";
    let file_path = "test_Auto.csv";

    // Extract
    extract(url, file_path).unwrap();
    assert!(std::fs::metadata(file_path).is_ok());

    // Transform and Load
    let result = transform_load(file_path);
    assert_eq!(result.unwrap(), "AutoDB.db");
}

#[test]
fn test_query() {
    // Query the top 5 rows from the CarsDB table
    let result = query("SELECT * FROM AutoDB LIMIT 5;");

    // Check if the query was successful and returns 5 rows
    assert!(result.is_ok());
}
