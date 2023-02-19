// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// con il rust ho capito che devo leggere le istruzioni se no mi fotto

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // ref è il riferimento a un valore ancora impacchettato
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
