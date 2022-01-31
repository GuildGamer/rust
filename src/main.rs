fn main() {
    // let a = 10;
    // let b = 15;
    // println!("Hello, World!, {} {}", a, b);

    // NUMBERS
    // let unsigned: u8 = 10;

    // let signed: i8 = -15;

    // let float: f32 = 1.3;
    
    // println!("unsigned: {}, signed: {}, float: {}", unsigned, signed, float)

    // LETTERS AND CHARACTERS
    // let letter = "Happy";
    // let emoji = "\u{1f600}";

    // println!("I am {} {}", letter, emoji);


    // BOOL
    // let is_true: bool = true;

    // println!("is_true: {}", is_true);

    // let arr: [u8; 3] = [1,2,3];
    // let other_arr: [u8; 5] = [100;5];

    // println!("Index 0 is {} and the length of the other array is {}.", arr[0], other_arr.len());

    // println!("{:?}", other_arr);

    // TUPLES
    // let tuple: (u8, bool, f32) = (3, false, 7.4);
    // let tuple2 = (4.2, 17);

    // println!("unsigned: {}, bool: {}, float32: {}", tuple.0, tuple.1, tuple.2);
    // println!("{:?}", tuple2);

    // DESTRUCTURING
    // let(a, b, c) = tuple;
    // println!("unsigned: {}, bool: {}, float32: {}", a, b, c)

    println!("{}", is_even(2));

}
// FUNCTIONS
pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // lack of semicolon is in order to indicate that it is a return value
}