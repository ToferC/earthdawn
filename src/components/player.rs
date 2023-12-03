use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub race: Race,
    pub discipline: Discipline,
    pub mv: i32,
    pub dexterity: i32,
    pub strength: i32,
    pub toughness: i32,
    pub perception: i32,
    pub willpower: i32,
    pub charisma: i32,
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