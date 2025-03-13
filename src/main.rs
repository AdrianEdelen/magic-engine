use card::Card;
mod card;

fn main() {
    let my_card = Card::new("Black Lotus");
    println!("{:?}", my_card);
}
