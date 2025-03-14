use card::Card;
mod card;
mod phase_manager;

fn main() {
    let my_card = Card::new("Black Lotus");
    println!("{:?}", my_card);
}
