## IDS 706 Individual Project #2: Rust CLI Binary with SQLite
This is a repository for IDS 706 individual project #2 assignment. 

### Purpose
This repository creates a Rust CLI (Command Line Interface) tool that utilizes SQLite database to perform extract, transform_load and query of the Auto dataset. 
A database from online url is extracted into a csv file which is saved directly to the repository. It is then transformed into a SQLite database format and loaded into the SQLite database. Query commands can be performed based on this database format. 

### Github Actions Workflows
[![Rust CI/CD](https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/actions/workflows/rustCI.yml/badge.svg)](https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/actions/workflows/rustCI.yml)

This repository has CI/CD set up in Github Actions Workflows. The workflows include:
- building, formatting, linting, testing, generate and push
- specific query commands: create, delete, read, update
- Archive Binary: Binary is also incorporate into the Github Actions. For downloading the binary, go to Github Actions, click on one of the Workflows, go to the bottom of the page and download the file. 

### Function

**Extraction**

- Extract data from an url to a csv file
- Automatically save and push this csv file in this repository

**Transform_load**
- Transform the csv file to a databse (in this case Auto.csv is transformed into AutoDB.db)
- This ensure the database file is in a format suitable for SQLite database and that it prepares the format for later query command. 

**Query**
- Specific SQL Query can be done to the dataset
- This includes SQL query function like: create-table, create, read, update, delete query commands (specific commands can be found in Makefile or query_log.md)

### Dependencies
Run Cargo Build to download all the dependencies

- reqwest = { version = "^0.11", features = ["blocking"] }: A popular Rust HTTP client library. The "blocking" feature enables synchronous (non-async) network requests, making it easier to use in contexts where asynchronous operations aren't required or desired.

- rusqlite = "^0.29": A Rust binding for the SQLite3 library, allowing for interaction with SQLite databases.

- csv = "^1.0": A crate to read and write CSV (Comma-Separated Values) files, providing utilities to serialize and deserialize CSV data.

- assert_cmd = "^2.0": A crate that aids in writing integration tests for command-line applications. It provides a way to spawn commands and make assertions on their output and exit status.
  
- predicates = "0.9": A library used in conjunction with assert_cmd to define complex assertions on command outputs using predicate logic.
  
### Github Copilot 

**Direct Translation**: You can provide a Python snippet and ask Copilot (or Codex) to translate it to Rust. 

**Code Suggestions**: As you type Rust code, Copilot can suggest completions. This is especially handy for developers new to the Rust language, helping them with Rust's unique syntax and concepts.

**Error Handling**: One of Rust's key features is its robust error-handling through the Result and Option types. Copilot can help generate these patterns, suggesting ways to handle potential errors or unwrap values safely.

**Performance Tips**: Given Rust's focus on performance, Copilot might offer insights or suggestions to improve code efficiency, such as using certain data structures or algorithms.

**Borrow Checker Assistance**: Rust's borrow checker ensures safe memory access but can be challenging for newcomers. Copilot can help by suggesting fixes to borrow checker errors or guiding on lifetimes and borrowing rules.

**Library Usage**: If you're using a particular Rust crate and aren't sure about its functions or usage, Copilot can help by providing examples or correct usages based on the context.

**Integration with Documentation**: Copilot can fetch documentation snippets for Rust's standard library or other popular crates, aiding in quicker referencing without leaving the coding environment.

**Feedback Loop**: As developers interact with Copilot, they can accept, modify, or reject its suggestions. This interaction can be a learning process for both the developer and the model, leading to better suggestions over time.

### Building Process

Rust Building Process Overview

<img width="896" alt="rust_building" src="https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/assets/142815940/01398530-716c-46be-ba1f-fbe94b8df3a8">

`Preparation`
prepare codespace environment 

`Install Dependencies`
run **Cargo build** in codespace to install all the dependencies required for running extract, transform_load and query in this repository

<img width="734" alt="cargo build" src="https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/assets/142815940/e869eca8-23ff-47f8-b6f9-377c69368411">

`Extraction`
extract dataset from an url to a csv file which is automatically saved and pushed into this repository. 

run **make extract** or **cargo run extract** in codespace to function extraction

<img width="508" alt="截屏2023-10-29 23 39 13" src="https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/assets/142815940/69723709-fe8e-4e4d-8c97-1b52a087d7d6">

`Transform_load`
transfrom dataset in the csv file (Auto.csv) to a database (AutoDB.db)

run **make transform_load** or **cargo run transform_load** in codespace to function transform_load

<img width="571" alt="transform_load" src="https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/assets/142815940/36de6cf3-d960-49bd-828c-c7cc36afa3a0">

`Query`
build specific query for the dataset (detailed query messages can be found in Makefile)

run **make query** or **cargo run query** in code space and then run "specific query messages to function specific query
example query command (more can be found in Makefile or query_log.md: 

<img width="499" alt="extract" src="https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/assets/142815940/5ccbce83-b8d3-495b-a9ce-6dd638ff70ef">

<img width="911" alt="query message" src="https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/assets/142815940/90d9ffc5-e93c-4e54-83ff-39dd8bae319e">
