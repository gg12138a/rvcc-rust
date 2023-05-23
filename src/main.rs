pub mod token;

fn main() {
    let v = token::tokenize("  12 + 34 - 5 + 66");
    v.iter().for_each(|tok| println!("{:?}", tok));
}
