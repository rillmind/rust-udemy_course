use std::env::args;
use std::process::exit;
use e_ownership_references::*;

fn main() {
    let mut arg: String = args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}
