use bevy::prelude::*;
use bevy_reflect::Reflect;
use bevy_inspector_egui::{prelude::*, inspector_options::std_options::NumberDisplay};

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    pub name: String,
    pub race: Race,
    pub discipline: Discipline,
    #[inspector(min = 10.0, max = 90.0, display = NumberDisplay::Slider)]
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
            mv: 32.0,
            dexterity: 7,
            strength: 7,
            toughness: 6,
            perception: 5,
            willpower: 5,
            charisma: 7,
        }
    }
}

#[derive(Debug, Reflect)]
pub enum Race {
    Human,
    Elf,
    Windling,
    Troll,
    Orc,
    Obsidiman,
}

impl Default for Race {
    fn default() -> Self {
        Race::Human
    }
}

#[derive(Debug, Reflect)]
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

impl Default for Discipline {
    fn default() -> Self {
        Discipline::Archer
    }
}