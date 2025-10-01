// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'
    //
    fn inspect(argument: &String) {
        if argument.ends_with("s") {
            println!("Argument is plural");
        } else {
            println!("Argument is singular")
        }
    }

    inspect(&arg);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.
    //
    fn change(arg1: &mut String) {
        if !arg1.ends_with("s") {
            arg1.push_str("s");
        }
    }
    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    //
    fn eat(consumes: String) -> bool {
        if consumes.starts_with("b") && consumes.contains("a") {
            true
        } else {
            false
        }
    }

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }


    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    fn bedazzle(s: &mut String) {
        *s = String::from("sparkly");
    }
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}