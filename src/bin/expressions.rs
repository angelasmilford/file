pub fn main() {
    // Arithmetic Expressions/Operators
    let add = 1 + 1; //Addition
    let sub = 8 - 4; //Subtraction
    let mul = 3 * 2; //Multiplication
    let div = 22 / 2; //Division
    let rem = 44 % 8; //Remainder

    println!("Addition: {}", add);
    println!("Subtraction: {}", sub);
    println!("Multiplication: {}", mul);
    println!("Division: {}", div);
    println!("Remainder: {} \n", rem);

    // Using parentheses to change the order of operations


    // Assignment Expressions/Operators
    let mut i = 11;
    println!("Initial value of a: {}", i);

    i += 2; // i = i + 2
    println!("After += 2, i: {}", i);

    i -= 3; // i = i - 3
    println!("After -= 3, i: {}", i);

    i *= 4; // i = i * 4
    println!("After *= 4, i: {}", i);

    i /= 2; // i = i / 2
    println!("After /= 2, i: {}", i);

    i %= 3; // i = i % 3
    println!("After %= 3, i: {} \n", i);


    // Comparison Expressions/Operators
    let j = 6;
    let i = 11;

    println!("Is j equal to i? {}", j == i);
    println!("Is j not equal to i? {}", j != i);
    println!("Is j greater than i? {}", j > i);
    println!("Is j less than i? {}", j < i);
    println!("Is j greater than or equal to i? {}", j >= i);
    println!("Is j less than or equal to i? {} \n", j <= i);


    // Logical Expressions
    let is_us_citizen = true;
    let is_18_or_older = false;

    println!("Is the person a US citizen and 18 or older? {}", is_us_citizen && is_18_or_older);
    println!("Is the person a US citizen or 18 or older? {}", is_us_citizen || is_18_or_older);
    println!("Can the person vote? {}", is_us_citizen || is_18_or_older);
    println!("Is the person not a US citizen? {}", !is_us_citizen);
    println!("Can this person sponsor immigrants? {} \n", is_18_or_older);


    //Boolean Expressions
    let dc_comics_rule: bool = true; //Explicitly typed boolean variable
    let marvel_comics_rule = false; //Implicitly typed boolean variable
    println!("Does DC Comics rule? {}", dc_comics_rule);
    println!("Does Marvel Comics rule? {}", marvel_comics_rule); //Haha, I'm just kidding, or am I not? ;)
    println!("");

    let age = 21;
    let can_vote = age >= 18; //does not need to have a true or false value, just needs to be a boolean expression
    println!("Is the person 18 years or older? {}", can_vote);
    println!("Can the person vote? {} \n", can_vote);

    let is_college_student = true; 
    if is_college_student { //Boolean expression used in an if statement
        println!("The person is a college student.");
    } else {
        println!("The person is not a college student.");
    }
    println!("");

    //Booleans are the basis for all Rust comparisons and conditions.


    //if expressions
    let hair_type = "curly";
    let _hair_description = if hair_type == "curly" {
        println!("Your hair is curly!");
    } else if hair_type == "straight" {
        println!("You have straight hair!");
    } else {
        println!("You have wavy hair!");
    };
}