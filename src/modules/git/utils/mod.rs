#[derive(Default)]
pub struct Branch {
    pub current: String,
}

impl Branch {
    pub fn set(mut self, branch: &str) {
        self.current = branch.to_string()
    }

    // pub fn get(&self) -> String {
    //     self.current
    // }
}
