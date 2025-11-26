use coat_check::initialize_static_closet;
use coat_check::TypedTicket;

fn main() {
    initialize_static_closet!();

    struct MyStruct {
        color: String,
    }

    let my_struct = MyStruct {
        color: "blue".to_string(),
    };

    let ticket = store_in_global_closet_typed(my_struct);
    println!("Stored coat with ticket ID: {}", ticket);

    let my_struct = retrieve_from_global_closet_typed(ticket);
    println!("Retrieved coat color: {}", my_struct.color);
}
