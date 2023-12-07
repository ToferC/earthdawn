use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub race: Race,
    pub discipline: Discipline,
    pub mv: f32,
    pub dexterity: i32,
    pub strength: i32,
    pub toughness: i32,
    pub perception: i32,
    pub willpower: i32,
    pub charisma: i32,
}

impl Player {
    pub fn default() -> Self {
        Player {
            name: "Myhir".to_owned(),
            race: Race::Troll,
            discipline: Discipline::SkyRaider,
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

pub enum Race {
    Human,
    Elf,
    Windling,
    Troll,
    Orc,
    Obsidiman,
}

pub enum Discipline {
    Archer,
    Beastmaster,
    Cavalryman,
    Elementalist,
    Illusionist,
    Nethermancer,
    Scout,
    SkyRaider,
    Swordmaster,
    Thief,
    Troubadour,
    Warrior,
    Weaponsmith,
    Wizard,
}