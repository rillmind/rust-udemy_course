#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

#[derive(Debug)]
struct Grapes {
    amount_left: u8,
}

trait Bite {
    fn bite(self: &mut Self);
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left *= 0.8;
    }
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

fn bunny_nibbles<T: Bite>(item: &mut T) {
    for _ in 0..10 {
        item.bite();
    }
}

fn main() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}
