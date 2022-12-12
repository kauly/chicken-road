use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

mod enemy;
mod game_over;
mod in_game;
mod menu;
mod player;

const WIN_WIDTH: f32 = 820.;
const WIN_HEIGHT: f32 = 600.;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 250.;

const SIDE_WALK: f32 = 100.;

const PLAYER_DIM: f32 = 16.;

const COLOR_RED: (f32, f32, f32) = (255. / 255., 89. / 255., 94. / 255.);
const COLOR_YELLOW: (f32, f32, f32) = (255. / 255., 202. / 255., 58. / 255.);
const COLOR_GRAY: (f32, f32, f32) = (141. / 255., 153. / 255., 174. / 255.);

#[derive(Resource)]
pub struct GameAssets {
    player: Handle<TextureAtlas>,
    enemy_red: Handle<Image>,
    enemy_green: Handle<Image>,
    road: Handle<Image>,
    font: Handle<Font>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    InGame,
    GameOver,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "The Chicken Road".to_string(),
                width: WIN_WIDTH,
                height: WIN_HEIGHT + (SIDE_WALK * 2.),
                resizable: false,
                ..default()
            },
            ..default()
        }));

    app.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(menu::MenuPlugin)
        .add_plugin(in_game::InGamePlugin)
        .add_plugin(game_over::GameOverPlugin)
        .add_startup_system(setup_system);

    #[cfg(target_arch = "wasm32")]
    app.add_system(disable_inspector_system);

    app.run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("imgs/chicken_sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 6, 4, None, None);
    let player = texture_atlases.add(texture_atlas);

    commands.insert_resource(GameAssets {
        enemy_green: asset_server.load("imgs/car_green.png"),
        enemy_red: asset_server.load("imgs/car_red.png"),
        road: asset_server.load("imgs/road.png"),
        font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
        player,
    });
}

#[cfg(target_arch = "wasm32")]
fn disable_inspector_system(mut world_inspector_params: ResMut<WorldInspectorParams>) {
    if world_inspector_params.enabled {
        world_inspector_params.enabled = false;
    }
}
