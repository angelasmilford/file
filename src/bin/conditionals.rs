pub fn main() {
    //if...else statements
    if 4 < 11{ //if is used to check if a condition is true
        println!("4 is less than 11 \n"); //block of code that runs if the condition is true
    }

    //testing variables
    let s = 2;
    let j = 6;

    if s < j {
        println!("s is less than j \n");
    }


    //if...else
    let batman = "Bruce Wayne";
    if batman == "Bruce Wayne" {
        println!("Batman is Bruce Wayne");
    } else { //else is used to check if the first condition is false
        println!("Batman is not Bruce Wayne");
    }
    println!("");


    //else if
    let superman = "Clark Kent";
    if superman == "Clark Kent" {
        println!("Superman is Clark Kent");
    } else if superman == "Kal-El" { //checks multiple conditions
        println!("Superman is Kal-El");
    } else {
        println!("Superman is not Clark Kent nor Kal-El");
    }
    println!("");


    /* if as an expression
    In Rust, if can also be used as an expression, meaning you can 
    assign the result of an if statement to a variable or
    return a value.
    */
    let pacific_islander = "Samoan";
    let greeting = if pacific_islander == "Samoan" {
        "Talofa lava!" //if the condition is true, return this value
    } else if pacific_islander == "Fijian" {
        "Bula vinaka!" //if the condition is true, return this value
    } else {
        "Warm Pacific Greetings"
    };
    println!("Greeting: {} \n", greeting); //prints the value returned by the if expression


    //Simplified Syntax
    let dc_superhero = "Wonder Woman";
    let hero = if dc_superhero == "Wonder Woman" {"Thank you for your service"} else {"Someone call the Justice League!"};
    println!("Hero: {} \n", hero); //prints the value assigned to the variable hero
    // This is a simplified syntax for if expressions, where the if statement is used to assign a value to a variable



    //Match
    let justice_league_member = 2;

    /*
    match is used to compare a value against a series of patterns
    match is also used instead of writing multiple if...else statements
    It is similar to switch statements in other languages, but more powerful
    */

    match justice_league_member { 
        1 => println!("Superman"), //branch = value => result
        2 => println!("Batman"),
        3 => println!("Wonder Woman"),
        4 => println!("Aquaman"),
        5 => println!("The Flash"),
        6 => println!("Green Lantern"),
        7 => println!("Shazam"),
        8 => println!("Green Arrow"),
        9 => println!("Red Tornado"),
        10 => println!("Martian Manhunter"),
        11 => println!("Hawkgirl"),
        12 => println!("Hawkman"),
        13 => println!("Black Canary"),
        14 => println!("Doctor Fate"),
        15 => println!("Plastic Man"),
        16 => println!("Atom"),
        17 => println!("Captain Atom"),
        18 => println!("Atom"),
        _ => println!("Unknown member"), 
        /*
        _ is used to specify some code to run if there's no match
        _ is basically a default case in a switch statement
        */
    }
    println!("");


    //Mutliple Matches
    let angela = 2;

    //multple values can match using the OR operator (|)

    match angela {
        1 | 3 => println!("Angela is an odd number baby."), 
        2 | 4 => println!("Angela is an even number baby"), 
    }
    println!("");


    //match with a Return Value
    let month = 1;

    let birth_month = match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Invalid Month", //catch-all pattern
    };

    println!("My birth month is {}", birth_month); //prints the value returned by the match expression
}