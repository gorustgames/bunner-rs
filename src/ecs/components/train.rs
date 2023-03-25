use crate::ecs::components::MovementDirection;
use bevy::prelude::*;
use bevy::sprite::Anchor;

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
                // TODO: randomize train color!
                texture: asset_server.load(if direction == MovementDirection::LEFT {
                    "images/train00.png"
                } else {
                    "images/train01.png"
                }),
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
}
