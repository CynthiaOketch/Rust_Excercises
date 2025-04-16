#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    // Determine the jacket
    let jacket = match formality_level {
        Some(0) => Jacket::Black,  // If formality_level is 0
        Some(_) => Jacket::White,  // For any value > 0
        None => Jacket::Flowers,   // If formality_level is None
    };

    // Determine the hat
    let hat = match (formality_level, invitation_message) {
        (None, Err(_)) => Hat::Baseball, // If formality_level is None and invitation_message is Err()
        _ if invitation_message.is_ok() => Hat::Fedora,  // If invitation_message is Ok()
        _ => Hat::Snapback,  // Default case
    };

    Outfit { jacket, hat }
}

fn main() {
    println!("My outfit will be: {:?}", choose_outfit(Some(0), Ok("Dear friend, ...")));
    println!("My outfit will be: {:?}", choose_outfit(None, Err("No message")));
    println!("My outfit will be: {:?}", choose_outfit(Some(1), Ok("Dear friend, ...")));
    println!("My outfit will be: {:?}", choose_outfit(None, Ok("Dear friend, ...")));
}
