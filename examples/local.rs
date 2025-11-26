use coat_check::Closet;

fn main() {
    let closet = Closet::default();

    let ticket_1 = closet.store("hello".to_string());
    let ticket_2 = closet.store(42);

    let retrieved_1: String = closet.retrieve(ticket_1);
    let retrieved_2: i32 = closet.retrieve(ticket_2);
    println!("Retrieved 1: {}", retrieved_1);
    println!("Retrieved 2: {}", retrieved_2);
}
