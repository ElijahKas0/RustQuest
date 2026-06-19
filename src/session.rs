pub struct Session {
    pub current_dir: String,
    //pub commands_used: u32,
    //pub completed: bool,
}

impl Session {
    pub fn new() -> Self {
        Self {
            current_dir: "/".to_string(),
            //commands_used: 0,
            //completed: false,
        }
    }

    //pub fn increment_commands(&mut self) {
    //    self.commands_used += 1;
    //}

    //pub fn complete(&mut self) {
    //    self.completed = true;
    //}
}
