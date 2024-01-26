fn main() {
    let var1 = 10;
    let var2 = 19;
    println!("Hello, world! Back to rust again ;) {} {}", var1, var2);
    another();
}

fn another() {
    let unsigned: u8 = 2;

    let signed: i8 = -10;

    let float: f32 = 1.2;

    println!("{} {} {}", signed, unsigned, float);

    // chars
    let letter = "y";
    let emoji = "\u{1F600}"; // :D

    println!("Letter: {} Emoji: {}", letter, emoji);

    let is_true = true;

    println!("isTrue: {}", is_true);


}