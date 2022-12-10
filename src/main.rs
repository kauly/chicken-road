use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod player;

const WIN_WIDTH: f32 = 820.;
const WIN_HEIGHT: f32 = 600.;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 250.;

const SIDE_WALK: f32 = 100.;

#[derive(Resource)]
pub struct GameTextures {
    // valid indexes 16 - 20
    player: Handle<TextureAtlas>,
    enemy_red: Handle<Image>,
    enemy_green: Handle<Image>,
    road: Handle<Image>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    InGame,
    End,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GameBackground {
    visible: bool,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_state(GameState::InGame)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "The Chicken Road".to_string(),
                width: WIN_WIDTH,
                height: WIN_HEIGHT + (SIDE_WALK * 2.),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("chicken_sheet2.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 6, 4, None, None);
    let player = texture_atlases.add(texture_atlas);

    commands.insert_resource(GameTextures {
        enemy_green: asset_server.load("car_green.png"),
        enemy_red: asset_server.load("car_red.png"),
        road: asset_server.load("road.png"),
        player,
    });

    commands
        .spawn(GameBackground::default())
        .insert(Name::new("GameBackground"));
}

fn spawn_game_background(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut background_query: Query<&mut GameBackground>,
) {
    if let Ok(mut background) = background_query.get_single_mut() {
        if !background.visible {
            background.visible = true;
            commands.spawn(SpriteBundle {
                texture: game_textures.road.clone(),
                ..default()
            });
        }
    }
}