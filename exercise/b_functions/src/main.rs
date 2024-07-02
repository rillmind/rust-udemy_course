fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    println!("Area is {}", area_of(width, height));

    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    (x * y) / 2
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    return x * y * z;
}
