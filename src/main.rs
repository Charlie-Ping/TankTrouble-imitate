use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle, a11y::accesskit::Vec2, utils::petgraph::visit::VisitMap, ecs::archetype::ArchetypeRow,
};
use rand::seq::SliceRandom;
use rand::Rng;


const TANK_MOVE_SPEED: f32 = 3.;
const TANK_TURNING_SPEED: f32 = 5.;
const TANK_BULLET_LIMIT: i32 = 5;

const BULLET_SPEED: f32 = 4.;
const BULLET_LIFECYCLE: f32 = 10.;

const BULLET_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const WALL_THICKNESS: f32 = 5.;
const WALL_LENGTH: f32 = 45.;
const WALL_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);
// 随机删除墙体， true为模拟官网效果 false为模拟4399版效果
const WALL_RANDOM_DEL: bool = true;

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
        .add_event::<CollisionEvent>()
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))  
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, )
        .run();
    
}


#[derive(Component)]
struct Name(String);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);


// Wall

#[derive(Component)]
struct Wall;


#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle, // 创建一个拥有默认渲染属性的实体
    collider: Collider,
}

impl WallBundle {
    // toward 0: horizontal  1: vertical
    fn new(x: f32, y: f32, toward: u8) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { 
                    color: WALL_COLOR,
                    ..default()
                },
                transform: Transform { 
                    translation: Vec3 { x, y, z: 0. }, 
                    scale: match toward {
                        // + WALL.THICKNESS/2. 是为了补充直角处的凹陷
                        0 => Vec3 { x: WALL_LENGTH + WALL_THICKNESS, y: WALL_THICKNESS, z: 1. },
                        1 => Vec3 { x: WALL_THICKNESS, y: WALL_LENGTH + WALL_THICKNESS, z: 1. },
                        // unreachable
                        _ => panic!("Invalid value for toward!")
                    },
                    ..default()
                },
                ..default()
            },
            collider: Collider
        }
    }
}

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


// Collider
#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Resource)]
struct CollisionSound;


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
    let mut module = MazeGeneratorDFS::new(
        loop {
            let num = rng.gen_range(3..=11);
            if num % 2 != 0 {
                break num
            }
        }, loop {
            let num = rng.gen_range(3..=11);
            if num % 2 != 0 {
                break num
            }
        }
    );
    
    module.init_maze();
    module.iter(1, 1);
    if WALL_RANDOM_DEL {
        module.rand_del_wall()
    }
    // module.print_maze();
    // 横向墙体 x [2n+1][2n]
    // col: 第...列
    // row: 第...行
    //                         这里取二维向量长度和宽度
    //                         (width * 2 + 1) - 1
    let b_width = module.width * 2 + 1;
    let b_height = module.height * 2 + 1;
    
    for row in (0..b_height+1).step_by(2) {
        for col in (1..b_width).step_by(2) {
            if module.maze[row as usize][col as usize] == 1 {
                let x = ((col-1)/2) as f32 * WALL_LENGTH + WALL_LENGTH /2.;
                let y = (row/2) as f32 * WALL_LENGTH + WALL_LENGTH/2.;
                commands.spawn(WallBundle::new(x-200., y-200., 0));
            }
        }
    }
    // 纵向墙体 y[2n][2n+1]
    for row in (1..b_height).step_by(2) {
        for col in (0..b_width+1).step_by(2) {
            if module.maze[row as usize][col as usize] == 1 {
                let x = (col/2) as f32 * WALL_LENGTH;
                let y = ((row-1)/2) as f32 * WALL_LENGTH + WALL_LENGTH;
                commands.spawn(WallBundle::new(x-200., y-200., 1));
            }
        }
    }
}

struct MazeGeneratorDFS {
    maze: Vec<Vec<u8>>,
    width: u32,
    height: u32,
}



impl MazeGeneratorDFS {
    fn print_maze(&self) {
        for row in &self.maze {
            for ele in row {
                if *ele == 1 {
                    print!("#")
                } else {
                    print!("0")
                }
            }
        println!()
        }
    }

    pub fn new(width: u32, height: u32) -> MazeGeneratorDFS {
        MazeGeneratorDFS{maze: Vec::new(), width, height}
    }

    pub fn init_maze(&mut self) {

        let b_width = self.width * 2 + 1;
        let b_height = self.height * 2 + 1;
        self.maze = vec![vec![1u8; b_width as usize]; b_height as usize];

        for i in (1..=(b_height-1)).step_by(2) {
            if i > b_height-1 {break;}
            for j in (1..=(b_width-1)).step_by(2) {
                if j > b_width-1 { break; }
                self.maze[i as usize][j as usize] = 0u8;
            }
        }
    }

    fn iter (&mut self, start_x:i32, start_y:i32) {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let mut direction: [(i32, i32);4] = [(0, 2), (0, -2), (-2, 0), (2, 0)];
        direction.shuffle(&mut rng);
        for (dx, dy) in direction {
            // thread::sleep(Duration::from_secs(4));
            let x_new = start_x + dx;
            let y_new = start_y + dy;
            // 判断下一个点位是否出界
            if x_new < 0 || x_new as u32 >= self.height * 2 || y_new < 0 || y_new as u32 >= self.width * 2 {
                continue;
            }
            // 判断下一个点位是否没有访问过
            if self.maze[(x_new+1) as usize][y_new as usize] == 1
                && self.maze[(x_new-1) as usize][y_new as usize] == 1
                && self.maze[x_new as usize][(y_new+1) as usize] == 1
                && self.maze[x_new as usize][(y_new-1) as usize] == 1
            {
                self.maze[(x_new - dx/2) as usize][(y_new - dy/2) as usize] = 0;
                self.iter(x_new, y_new)
            }
        }
    }

    fn rand_del_wall(&mut self) {
        let mut rng = rand::thread_rng();

        for i in (1..self.height*2) {
            for j in (1..self.width*2) {

                if self.maze[i as usize][j as usize] == 1 && rng.gen::<f64>() < 0.3 {
                    self.maze[i as usize][j as usize] = 0
                }
            }
        }
    }    
}

