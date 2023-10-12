#[derive(PartialEq, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub kills: i32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self { name, kills: 0 }
    }

    pub fn killed_another_player(&mut self) {
        self.kills += 1;
    }

    pub fn death_around_the_world(&mut self) {
        if self.kills > 0 {
            self.kills -= 1
        }
    }

    pub fn changed_name(&mut self, name: String) {
        self.name = name;
    }
}
