use bevy::prelude::*;
use rand::Rng;
use bevy::prelude::Interaction;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Config {
            difficulty: 0,
            difficulty_multiplier: 1.0,
            speed_modifier: 1.0,
            cheats_enabled: false,
        })
        .insert_resource(Player {
            name: "Hero".to_string(),
            health: 100,
            max_health: 100,
            damage: 10,
            potion_size: 25,
            coins: 0,
            score: 0,
        })
        .add_state::<GameState>()
        .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(menu_button_system))
        .add_system_set(SystemSet::on_enter(GameState::Exploring).with_system(exploring))
        .add_system_set(SystemSet::on_update(GameState::Exploring).with_system(display_stats))
        .run();
}


#[derive(Resource)]
struct Player {
    name: String,
    health: i32,
    max_health: i32,
    damage: i32,
    potion_size: i32,
    coins: i32,
    score: i32,
}

#[derive(Resource)]
struct Enemy {
    name: String,
    health: i32,
    max_health: i32,
    damage: i32,
    coins_drop: i32,
}

#[derive(Resource)]
struct Config {
    difficulty: u8,
    difficulty_multiplier: f32,
    speed_modifier: f32,
    cheats_enabled: bool,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    MainMenu,
    Exploring,
    Fighting,
    Shop,
    GameOver,
}

fn setup_main_menu(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(200.0),
            eight: Val::Px(50.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    })
    .with_children(|parent| {
        spawn_menu_button(parent, &assets, "Start Game");
        spawn_menu_button(parent, &assets, "Change Difficulty");
        spawn_menu_button(parent, &assets, "Exit Game");
    });
}

fn spawn_menu_button(parent: &mut ChildBuilder, assets: &Res<AssetServer>, label: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: bevy::prelude::BackgroundColor(Color::rgb(0.25, 0.25, 0.25)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn menu_button_system(
    mut interaction_query: Query<(&Interaction, &Children), Changed<Interaction>>,
    mut text_query: Query<&mut Text>,
    mut Config :ResMut<Config>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, children) in &mut interaction_query{
        if *interaction == Interaction::Pressed {
            let label = &text_query.get_mut(children[0]).unwrap().sections[0].value;
            match label.as_str() {
                "Start Game" => next_state.set(GameState::Exploring),
                "Change Difficulty" => Config.difficulty = (Config.difficulty +1) % 3,
                "Exit Game" => std::process::exit(0),
                _ => ()
            }
        }
    }
}

fn exploring(mut next_state: ResMut<NextState<GameState>>) {
    let event = rand::thread_rng().gen_range(1..=5);
    match event {
        1 => next_state.set(GameState::Fighting),
        2 => println!("Pool event"),
        3 => println!("Chest event"),
        4 => next_state.set(GameState::Shop), 
        5 => println!("Altar Event"),
        _ => {}
    }
}

fn display_stats(player: Res<Player>, mut query: Query<&mut Text>) {
    for mut text in &mut query{
        text.sections[0].value = format!(
            "Name: {}\nHealth: {}/{}\nDamage: {}\nCoins: {}\nScore: {}",
            player.name, 
            player.health, player.max_health, 
            player.damage, 
            player.coins, 
            player.score
        );
    }
}