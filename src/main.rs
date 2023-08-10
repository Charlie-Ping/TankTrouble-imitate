use bevy::{
    prelude::*,
    asset::LoadState,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle, a11y::accesskit::Vec2, utils::petgraph::visit::VisitMap, ecs::archetype::ArchetypeRow,
    render::{
        render_resource::{AddressMode, SamplerDescriptor},
        texture::ImageSampler
    }
};

mod maze;


const TANK_MOVE_SPEED: f32 = 3.;
const TANK_TURNING_SPEED: f32 = 5.;
const TANK_BULLET_LIMIT: i32 = 5;

const BULLET_SPEED: f32 = 4.;
const BULLET_LIFECYCLE: f32 = 10.;

const BULLET_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);


const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);


fn add_tank(mut commands: Commands) {
    commands.spawn((Tank, Name("Blue".to_string())));
    commands.spawn((Tank, Name("Red".to_string())));
    commands.spawn((Tank, Name("Green".to_string())));
}

fn main() {


    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<maze::CollisionEvent>()
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))  
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, )
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

// Tank
#[derive(Component)]
struct Tank;

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
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
){
    // camera 
    commands.spawn(Camera2dBundle::default());

    // sound
    let mut rng = rand::thread_rng();

    // gen maze
    let mut module = maze::MazeGeneratorDFS::new(
        maze::rand_odd_between(9..11),
        maze::rand_odd_between(9..11)
    );
    
    module.init_maze()
        .iter(1, 1);
    
    module.build(
        |x: f32, y: f32, toward: u8| {
            if toward == 0 {
                // vertical
                _ = commands.spawn(maze::WallBundle::new(x-200., y-200., 0))
            } else if toward==1 {
                _ = commands.spawn(maze::WallBundle::new(x-200., y-200., 1))
            }
            
        }

    );

    // let tank_texture = asset_server.load("textures/texture.png");
}

