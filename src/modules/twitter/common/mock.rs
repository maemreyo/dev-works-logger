use std::collections::HashMap;

// TODO: Currently, title is fixed, in the future, we would need to improve.
pub fn titles() -> HashMap<String, String> {
    HashMap::from([
        ("low".to_owned(), "A bit lazy today!! 🫠".to_owned()),
        ("medium".to_owned(), "It was the cool day! 😎😎".to_owned()),
        ("hard".to_owned(), "Hard working day! 😎😎".to_owned()),
    ])
}
