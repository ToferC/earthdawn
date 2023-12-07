use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub name: String,
    pub mv: f32,
    pub dexterity: i32,
    pub strength: i32,
    pub toughness: i32,
    pub perception: i32,
    pub willpower: i32,
    pub charisma: i32,
}

impl Enemy {
    pub fn default() -> Self {
        Enemy {
            name: "Ghoul".to_owned(),
            mv: 3.0,
            dexterity: 7,
            strength: 7,
            toughness: 6,
            perception: 5,
            willpower: 5,
            charisma: 7,
        }
    }
}