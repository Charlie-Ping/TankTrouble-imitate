
use bevy::{prelude::*, window::PrimaryWindow, input::keyboard, transform};

pub mod tank {
    const TANK_MOVE_SPEED: f32 = 3.;
    const TANK_TURNING_SPEED: f32 = 5.;
    const TANK_BULLET_LIMIT: i32 = 5;


    fn summon_entity(&mut self, entity: u8, pos: Position) -> Result<(), String> {
        if self.is_position_exceed(pos) {
            return Err("There is over limited".to_string());
        }
        if self.data[pos.y][pos.x] != 0 {
            return Err("There is not empty".to_string());
        }
        self.data[pos.y][pos.x] = entity;

        return Ok(());
    }


    #[derive(Component)]
    pub struct Tank{}

    pub fn spawn_tank(
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        
        asset_server: Res<AssetServer>,
    ) { 
        let window = window_query.get_single().unwrap();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(window.width()/2., window.height()/2., 0.),
                texture: asset_server.load("sprite/tankRed2.png"),
                ..default()
            },
            Tank {},
        ));
    }

    pub fn tank_movement(
        k_input: Res<Input<KeyCode>>,
        mut tank_query: Query<&mut Transform, With<Tank>>,
        time: Res<Time>
    ) {
        // 一种设想
        // 如果需要赋予实体速度, 是否可以bundle一个Comp(Vec3)?  
        if let Ok(mut transform) = tank_query.get_single_mut() {
            let mut direction = Vec3::ZERO;

            if k_input.pressed(KeyCode::Up) {
                
            }

        }

    }
}