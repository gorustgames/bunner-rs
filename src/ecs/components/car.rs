use crate::ecs::components::{CarTimer, MovementDirection};
use crate::get_random_i8;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use lazy_static::lazy_static;

struct CarType {
    pub left_dir_car: String,
    pub right_dir_car: String,
}

lazy_static! {
    static ref ALL_CAR_TYPES: [CarType; 4] = {
        let car1 = CarType {
            left_dir_car: "images/car00.png".to_owned(),
            right_dir_car: "images/car01.png".to_owned(),
        };

        let car2 = CarType {
            left_dir_car: "images/car10.png".to_owned(),
            right_dir_car: "images/car11.png".to_owned(),
        };

        let car3 = CarType {
            left_dir_car: "images/car20.png".to_owned(),
            right_dir_car: "images/car21.png".to_owned(),
        };

        let car4 = CarType {
            left_dir_car: "images/car30.png".to_owned(),
            right_dir_car: "images/car31.png".to_owned(),
        };

        [car1, car2, car3, car4]
    };
}

fn get_random_car(direction: &MovementDirection) -> String {
    let random_car = &ALL_CAR_TYPES[get_random_i8(0, 3) as usize];

    if *direction == MovementDirection::LEFT {
        random_car.left_dir_car.to_owned()
    } else {
        random_car.right_dir_car.to_owned()
    }
}

#[derive(Component)]
pub struct CarSpeed(f32);

#[derive(Bundle)]
pub struct CarBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    direction: MovementDirection,
    speed: CarSpeed,
}

impl CarBundle {
    pub fn new(
        direction: MovementDirection,
        x: f32,
        y: f32,
        speed: f32,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        CarBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                texture: asset_server.load(&get_random_car(&direction)),
                transform: Transform::from_xyz(x, y, 1.),
                ..default()
            },
            direction,
            speed: CarSpeed(speed),
        }
    }

    /// spawns train bundle with delay
    pub fn spawn_car_with_delay(
        self,
        commands: &mut Commands,
        parent_entity: Entity,
        delay_sec: f32,
    ) {
        let car = commands
            .spawn_bundle(self)
            .insert(CarTimer::new(delay_sec))
            .id();

        commands.entity(parent_entity).add_child(car);
    }
}
