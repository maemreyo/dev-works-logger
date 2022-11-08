use crate::modules::git::Commit;

pub fn map_changed_file(changed_file: u64) -> String {
    match changed_file {
        0..=5 => "low".to_owned(),
        6..=20 => "medium".to_owned(),
        _ => "hard".to_owned(),
    }
}
