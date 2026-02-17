use super::research::ResearchState;
use super::resources::{BaseState, GameState, PlayerState, SquadState};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Scenario {
    pub id: String,
    pub name: String,
    pub description: String,
    pub starting_gold: u32,
    pub starting_level: u32,
    pub starting_characters: Vec<CharacterTemplate>,
}

#[derive(Debug, Clone)]
pub struct CharacterTemplate {
    pub name: String,
    pub race: String,
    pub subrace: String,
    pub starting_squad: u16,
}

pub fn get_all_scenarios() -> Vec<Scenario> {
    vec![
        Scenario {
            id: "lone_wanderer".to_string(),
            name: "Lone Wanderer".to_string(),
            description: "You are a lone wanderer, seeking adventure.".to_string(),
            starting_gold: 500,
            starting_level: 1,
            starting_characters: vec![CharacterTemplate {
                name: "Sera".to_string(),
                race: "Human".to_string(),
                subrace: "Northerner".to_string(),
                starting_squad: 1,
            }],
        },
        Scenario {
            id: "squad_up".to_string(),
            name: "Squad Up".to_string(),
            description: "You are a group of individuals looking to make a name for themselves."
                .to_string(),
            starting_gold: 1000,
            starting_level: 1,
            starting_characters: vec![
                CharacterTemplate {
                    name: "Leader".into(),
                    race: "Human".into(),
                    subrace: "Northerner".into(),
                    starting_squad: 1,
                },
                CharacterTemplate {
                    name: "Warrior".into(),
                    race: "Skeleton".into(),
                    subrace: "Worker".into(),
                    starting_squad: 1,
                },
            ],
        },
    ]
}

pub fn apply_scenario(
    commands: &mut Commands,
    scenario: &Scenario,
    game_state: &mut GameState,
    player_state: &mut PlayerState,
    squad_state: &mut SquadState,
    base_state: &mut BaseState,
    research_state: &mut ResearchState,
) {
    game_state.reset();
    game_state.current_level = scenario.starting_level;

    player_state.gold = scenario.starting_gold;
    player_state.experience = 0;
    player_state.level = scenario.starting_level;

    *squad_state = SquadState::default();
    *base_state = BaseState::default();
    *research_state = ResearchState::default();

    for template in &scenario.starting_characters {
        // Generate a new ID for the character
        let id = format!("char_{}", squad_state.next_id);
        squad_state.next_id += 1;

        // Create the bundle
        let bundle = super::character::CharacterBundle::new(
            id.clone(),
            template.name.clone(),
            template.race.clone(),
            template.subrace.clone(),
            "Home Base".to_string(),
        );

        // Spawn the entity
        let entity = commands.spawn(bundle).id();

        // Register in SquadState
        squad_state.add_character(id.clone(), entity);

        // Assign to squad if needed
        if template.starting_squad > 0 {
            squad_state.add_member_to_squad(&id, template.starting_squad);
        }
    }
}
