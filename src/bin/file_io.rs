use std::fs::{File, OpenOptions};
/* 
Imports File and OpenOptions from the standard library's file system module for file operations
File is used to create, open, and read/write files
OpenOptions allows for more flexible file opening options, such as appending or creating files
*/

use std::io::{self, Write, Read}; // For input/output operations like reading and writing
/* 
imports input/output (I/O) utilities from the standard library
io is a module for input/output operations
Write and Read traits are required for writing to and reading from files.
*/

// Main function demonstrating file operations
fn main() -> io::Result<()> { //returns an io::Result type, which is a standard way to handle errors in Rust
    // Specify the file path
    let file_path = "text.txt"; // File name

    // Create or overwrite the file and write some lines
    let mut file = File::create(file_path)?; // Creates a new file at file_path or overwrites it if it exists
    //? operator that handles any errors by returning them if they occur
    writeln!(file, "Hello, Rust!")?; //Writes a line to the file
    writeln!(file, "This is written to a file.")?; // Writes another line to the file

    // Open the file to append a line without overwriting
    let mut file = OpenOptions::new()
        .read(true) // Allows reading from the file
        .write(true) // Allows writing to the file
        .append(true) // Ensures content is added instead of overwriting
        .create(true) // Creates the file if it does not exist
        .open(file_path)?; // Opens the file with the specified options
    writeln!(file, "This line was appended.")?;

    // Open the file to read its contents into a string
    let mut file = File::open(file_path)?; // Opens the file for reading
    let mut contents = String::new(); // Initializes an empty string to hold the file contents
    file.read_to_string(&mut contents)?; // Reads the file contents into the string

    // Print the file contents to the console
    println!("--- File Contents ---"); //print a header for clarity
    println!("{}", contents); // Displays the contents of the file

    Ok(()) // Signals that the program executed successfully without any errors.
}