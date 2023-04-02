use bevy::prelude::*;
use bevy::sprite::Anchor;

/*
Bush sprite naming convention bushXY, e.g. bush00, bush21, etc.

Second digit:
     The second number indicates whether it is bottom (0) or top (1) segment

First digit:
     0 represents a single-tile-width hedge
     1 and 2 represent the left-most or right-most sprites in a multi-tile-width hedge
     3, 4 and 5 all represent middle pieces in hedges which are 3 or more tiles wide
*/

#[derive(Debug, PartialEq, Component, Copy, Clone)]
pub enum BushVerticalType {
    BOTTOM = 0,
    TOP = 1,
}

#[derive(Debug, PartialEq, Component, Copy, Clone)]
pub enum BushHorizontalType {
    SINGLE = 0,
    LEFTMOST = 1,
    RIGHTMOST = 2,
    MIDDLE1 = 3,
    MIDDLE2 = 4,
    MIDDLE3 = 5,
}

#[derive(Bundle)]
pub struct BushBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    bvt: BushVerticalType,
    bht: BushHorizontalType,
}

impl BushBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        x: f32,
        y: f32,
        bvt: BushVerticalType,
        bht: BushHorizontalType,
    ) -> Self {
        BushBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                texture: asset_server.load(&format!("images/bush{}{}.png", bht as u8, bvt as u8)),
                transform: Transform::from_xyz(x, y, 1.),
                ..default()
            },
            bvt,
            bht,
        }
    }
    pub fn spawn_bush(self, commands: &mut Commands, parent_entity: Entity) {
        let train = commands.spawn_bundle(self).id();

        commands.entity(parent_entity).add_child(train);
    }
}
