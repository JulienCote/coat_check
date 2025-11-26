use coat_check::Closet;

fn main() {
    let closet = Closet::default();

    let ticket_1 = closet.store_typed("hello".to_string());
    let ticket_2 = closet.store_typed(42);
    let ticket_3 = closet.store(uuid::Uuid::new_v4());

    let retrieved_1 = closet.retrieve_typed(ticket_1);
    let retrieved_2 = closet.retrieve_typed(ticket_2);
    let retrieved_3 = closet.retrieve::<uuid::Uuid>(ticket_3);

    println!("Retrieved 1: {}", retrieved_1);
    println!("Retrieved 2: {}", retrieved_2);
    println!("Retrieved 3: {}", retrieved_3);
}
