use coat_check::public_traits::{Coat, Ticket};

fn main() {
    let closet = coat_check::Closet::default();

    let ticket = closet.store("hello".to_string());

    let retrieved: String = closet.retrieve(ticket);
}
