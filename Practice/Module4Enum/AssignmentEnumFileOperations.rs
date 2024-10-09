use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum FileOperation {
    Create(String),
    Rename(String, String),
    Delete(String),
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn check_file_exists(filename: &str) -> bool{
    Path::new(&filename).exists()
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::Create(filename) => {
            // TODO: Implement file creation logic

            // Check if file already exists
            if check_file_exists(&filename) {
                println!("File '{}' already exists. Do you wish to overwrite it? (y/n): ", filename);
                let overwrite = get_user_input();
                if overwrite.to_lowercase() != "y"{
                    println!("Unable to create new file.");
                    return;
                }
            }

            // Creates new file
            fs::File::create(&filename).unwrap();
            println!("File '{}' created successfully.", filename);
        }

        FileOperation::Rename(old_name, mut new_name) => {
            // TODO: Implement file renaming logic

            // Verifies new filename is unique from original filename
            loop{
                if old_name == new_name{
                    println!("\nYou cannot rename the file with the same name.");
                    println!("Please provide a new file name");
                    new_name = get_user_input();
                }
                else{
                    break;
                }
            }

            // Checks if new filename already exists
            if check_file_exists(&new_name){
                println!("File '{}' already exists. Do you wish to overwrite it? (y/n): ", new_name);
                let proceed = get_user_input();
                if proceed.to_lowercase() != "y"{
                    println!("Unable to rename file.");
                    return;
                }
            }

            // Rename old file 
            fs::rename(&old_name, &new_name).unwrap();
            println!("File renamed from '{}' to '{}' successfully.", old_name, new_name);
        }

        FileOperation::Delete(filename) => {
            // Checks whether file exists
            if !check_file_exists(&filename) {
                println!("File '{}' does not exist.", filename);
                return;
            }

            fs::remove_file(&filename).unwrap();
            println!("File '{}' deleted successfully.", filename);
        }
    }
}

fn main() {
    loop{
        println!("\nChoose an operation:");
        println!("1. Create a new file");
        println!("2. Rename an existing file");
        println!("3. Delete a file");
        println!("q. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        println!();

        match choice.trim() {
            "1" => {
                // TODO: Prompt for new filename and call perform_operation
                println!("Type name of file you want to create:");
                let new_file = get_user_input();
                perform_operation(FileOperation::Create(new_file));
            }

            "2" => {
                // TODO: Prompt for old and new filenames and call perform_operation
                println!("Type the name of the file you want to rename:");
                let old_file = get_user_input();

                // Checks if file exists
                if !check_file_exists(&old_file){
                    println!("File '{}' does not exist. Do you wish to proceed with renaming? (y/n): ", old_file);
                    let proceed = get_user_input();
                    if proceed.to_lowercase() != "y"{
                        println!("Unable to rename file.");
                        return;
                    }
                    // Creates file before renaming
                    perform_operation(FileOperation::Create(old_file.clone()));
                } 
               
                println!("Type the new name of the file:");
                let new_file = get_user_input();  

                perform_operation(FileOperation::Rename(old_file, new_file));
            }

            "3" => {
                println!("Type name of file you want to delete:");
                let delete_file = get_user_input();
                perform_operation(FileOperation::Delete(delete_file));
            }

            "q" | "Q" => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
