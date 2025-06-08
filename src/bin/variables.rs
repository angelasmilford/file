pub fn main() {
    let name = "Angela";
    println!("My first name is: {} \n", name); //{} == used as a placeholder to show variable values

    //\n == used to create a new line in the outpu

    let batman = "Bruce Wayne";
    let age = 30;
    println!("Billionaire {} is {} years old. \n", batman, age); //The values passed in the placeholders are used by order

    println!("");


    //Mutable variables
    let mut superman = "Clark Kent";
    println!("Before: {}", superman);
    superman = "Kal-El";
    println!("After: {} \n", superman);


    //Immutable variables
    let flash = "Barry Allen";
    println!("The Flash's real name is {}.", flash);
    let aquaman = "Arthur Curry";
    println!("Aquaman's real name is {}. \n", aquaman);


    //Data Types
    let _my_num = 4;        //integer
    let _my_double = 6.00;  //float
    let _my_letter = 'A';   //character
    let _my_bool = true;    //boolean
    let _my_text = "Joy";   //string
    /*
    Similar to Kotlin, Rust chooses the right type of a variable name 
    by looking at its value 
    */

    //Tells the type a variable is supposed to have
    let _integer: i32 = 6; 
    let _double: f64 = 5.00;
    let _character: char = 'M';
    let _boolean: bool = false;
    let _string: &str = "Hello";
    
    /* Basic data types are divided into different groups
    Numbers
    i32 == whole numbers
    f64 == decimal numbers
    
    Characters
    char == single letters or symbols
    
    Strings
    &str == text, a sequence of characters
    
    Booleans
    bool == true or false values
    */


    /* Numbers
    Number types are divided into two groups: 
    integer types and floating point types
    */

    /* Integers
    i32 stores whole numbers, positive and negative
    */
    let year: i32 = 2025;
    println!("This code was made in the year of {} \n", year);

    /*Floating Point
    f64 stores numbers containing one or more decimals
    */
    let time: f64 = 7.49;
    println!("The time right now is {}am.\n", time);
    println!("");


    /* Characters (char)
    char stores a single character
    */
    let my_grade: char = 'A';
    println!("I want straight {}'s this semester. \n", my_grade);


    /* Strings (&str) 
    &str stores a sequence of characters (text) that must be surrounded by quotes
    */
    let last_name: &str = "Milford";
    println!("Hello, Ms. {}. \n", last_name);


    /* Booleans (bool) 
    bool can only take true or false values
    */
    let am_i_cool: bool = true;
    println!("Is Angela Milford cool? {} \n", am_i_cool);


    /* Combining Data Types
    Data types can be mixed in the same program.
    */
    let full_name = "Angela Milford";
    let age = 21;
    let is_admin = true;
    println!("Full name: {}", full_name);
    println!("Age: {}", age);
    println!("Admin: {}", is_admin);
}