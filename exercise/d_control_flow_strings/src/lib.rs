pub fn sum() {
    let mut sum = 0;

    for c in 7..24 {
        sum += c;
    }

    println!("The sum is {}", sum);
}

pub fn double() {
    let mut count = 0;
    let mut x = 1;

    while x <= 500 {
        x *= 2;
        count += 1;
    }

    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

pub fn count(arg: String) {
    let mut c = 0;
    loop {
        print!("{} ", arg);
        c += 1;
        if c == 8 {
            break;
        }
    }

    println!();
}
