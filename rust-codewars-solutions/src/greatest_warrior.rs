use std::cmp::Ordering::{Equal, Greater, Less};

pub struct Warrior {
    experience: u16,
    achievements: Vec<&'static str>,
}

impl Warrior {
    pub fn new() -> Self {
        Warrior {
            experience: 100,
            achievements: vec![],
        }
    }

    fn add_experience(&mut self, exp: u16) {
        self.experience = std::cmp::min(self.experience + exp, 10000)
    }

    pub fn level(&self) -> u16 {
        self.experience / 100
    }

    pub fn experience(&self) -> u16 {
        self.experience
    }

    pub fn rank(&self) -> &str {
        match self.level() / 10 {
            0 => "Pushover",
            1 => "Novice",
            2 => "Fighter",
            3 => "Warrior",
            4 => "Veteran",
            5 => "Sage",
            6 => "Elite",
            7 => "Conqueror",
            8 => "Champion",
            9 => "Master",
            _ => "Greatest",
        }
    }

    pub fn achievements(&self) -> &[&'static str] {
        self.achievements.as_slice()
    }

    pub fn training(&mut self, desc: &'static str, exp: u16, min_level: u16) -> &'static str {
        if self.level() >= min_level {
            self.add_experience(exp);
            self.achievements.push(desc);
            desc
        } else {
            "Not strong enough"
        }
    }

    pub fn battle(&mut self, enemy_level: u16) -> &str {
        let warrior_level = self.level();

        if enemy_level < 1 || enemy_level > 100 {
            return "Invalid level";
        }

        if enemy_level >= warrior_level + 5 && enemy_level / 10 > warrior_level / 10 {
            return "You've been defeated";
        }

        match enemy_level.cmp(&warrior_level) {
            Equal => {
                self.add_experience(10);
                "A good fight"
            }
            Less => {
                if warrior_level - enemy_level == 1 {
                    self.add_experience(5);
                    "A good fight"
                } else {
                    "Easy fight"
                }
            }
            Greater => {
                let diff = enemy_level - warrior_level;
                self.add_experience(20 * diff * diff);
                "An intense fight"
            }
        }
    }
}
