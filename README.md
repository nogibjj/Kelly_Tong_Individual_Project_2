## IDS 706 Individual Project #2: Rust CLI Binary with SQLite
This is a repository for IDS 706 individual project #2 assignment. 

### Purpose
This repository compares the running time and efficiency of python and rust. 
This is done by performing the same data processing with python script and rust.
In this case, the average of the chosen columns/variables are calculated. 
Improvement in running time, speed, and resource usage can be analyzed and
evluated through the repository action. 

### Github Workflows
[![Rust CI/CD](https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/actions/workflows/rustCI.yml/badge.svg)](https://github.com/nogibjj/Kelly_Tong_Individual_Project_2/actions/workflows/rustCI.yml)

### Function

*Extraction
Extract data from an url to a csv file
Automatically save and push this csv file in this repository

*Transform_load
Transform the csv file to a databse (in this case Auto.csv is transformed into AutoDB.db)
This ensure the database file is in a format suitable for SQLite database and that it prepares the format for later query command. 

*Query
Specific SQL Query can be done to the dataset
This includes SQL query function like: create-table, create, read, update, delete query commands (specific commands can be found in Makefile or query_log.md)

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
