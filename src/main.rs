use bevy::prelude::*;
use tank_trouble_imitate::entity::tank;


mod maze;
mod camera;

const BULLET_SPEED: f32 = 4.;
const BULLET_LIFECYCLE: f32 = 10.;

const BULLET_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);


const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);


fn main() {

    App::new()
        .add_plugins(DefaultPlugins)

        .add_systems(Startup, camera::spawn_camera)

        .add_systems(Startup, maze::spawn_maze)

        .add_systems(Startup, tank::spawn_tank)

        .insert_resource(ClearColor(BACKGROUND_COLOR))

        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))

        .add_systems(Startup, setup)
        .run();
    
}


#[derive(Component)]
struct Name(String);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);


// Bullet
#[derive(Component)]
struct Bullet;

#[derive(Event, Default)]
struct HitEvent;

#[derive(Resource)]
struct HitSound;

#[derive(Event, Default)]
struct ExpiredEvent;

#[derive(Resource)]
struct ExpiredSound(Handle<AudioSource>);


#[derive(Event, Default)]
struct RotatingEvent();

#[derive(Event, Default)]
struct ShootEvent;

#[derive(Resource)]
struct ShootSound(Handle<AudioSource>);


// Scoreboard
#[derive(Component)]
struct Scoreboard;


fn setup(
    commands: Commands,
    meshs: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
){

    
}