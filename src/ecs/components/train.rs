use crate::ecs::components::{MovementDirection, TrainTimer};
use crate::get_random_i8;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use lazy_static::lazy_static;

struct TrainType {
    pub left_dir_train: String,
    pub right_dir_train: String,
}

lazy_static! {
    static ref ALL_TRAIN_TYPES: [TrainType; 3] = {
        let train1 = TrainType {
            left_dir_train: "images/train00.png".to_owned(),
            right_dir_train: "images/train01.png".to_owned(),
        };

        let train2 = TrainType {
            left_dir_train: "images/train10.png".to_owned(),
            right_dir_train: "images/train11.png".to_owned(),
        };

        let train3 = TrainType {
            left_dir_train: "images/train20.png".to_owned(),
            right_dir_train: "images/train21.png".to_owned(),
        };

        [train1, train2, train3]
    };
}

fn get_random_train(direction: &MovementDirection) -> String {
    let random_train = &ALL_TRAIN_TYPES[get_random_i8(0, 2) as usize];

    if *direction == MovementDirection::LEFT {
        random_train.left_dir_train.to_owned()
    } else {
        random_train.right_dir_train.to_owned()
    }
}

#[derive(Bundle)]
pub struct TrainBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    direction: MovementDirection,
}

impl TrainBundle {
    pub fn new(
        direction: MovementDirection,
        x: f32,
        y: f32,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        TrainBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                texture: asset_server.load(&get_random_train(&direction)),
                transform: Transform::from_xyz(x, y, 1.),
                ..default()
            },
            direction,
        }
    }

    /// spawn train bundle and add it as child to its parent entity (respective rail row)
    pub fn spawn_train(self, commands: &mut Commands, parent_entity: Entity) {
        let train = commands.spawn_bundle(self).id();

        commands.entity(parent_entity).add_child(train);
    }

    /// spawns train bundle with delay
    pub fn spawn_train_with_delay(
        self,
        commands: &mut Commands,
        parent_entity: Entity,
        delay_sec: f32,
    ) {
        let train = commands
            .spawn_bundle(self)
            .insert(TrainTimer::new(delay_sec))
            .id();

        commands.entity(parent_entity).add_child(train);
    }
}
