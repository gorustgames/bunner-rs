use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bunner_rs::ecs::components::background_row::{
    BackgroundRow, GameRowBundle, GrassRow, Row, RowType, WaterRowMarker,
};
use bunner_rs::ecs::components::log::{LogBundle, LogBundleUuid, LogSize};
use bunner_rs::ecs::components::player::{Player, PlayerBundle};
use bunner_rs::ecs::components::MovementDirection;
use bunner_rs::ecs::resources::BackgroundRows;
use bunner_rs::ecs::systems::player_movement;
use bunner_rs::{
    get_random_i32, is_even_number, LOG_BIG_WIDTH, LOG_SMALL_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    SEGMENT_HEIGHT, SEGMENT_WIDTH,
};
use std::boxed::Box;

#[derive(Component)]
struct DebugText;

/// helper  resource used to make system run only once
struct RunOnceRes(bool);

impl RunOnceRes {
    fn did_run(&self) -> bool {
        return self.0;
    }

    fn set_did_run(&mut self) {
        self.0 = true;
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Infinite Bunner".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(game_setup)
        .add_system(put_logs_on_water)
        //.add_system(text_update_system)
        .add_system(player_movement)
        .add_system(active_row)
        .add_system(player_is_standing_on)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            logs_debug.after(TransformSystem::TransformPropagate),
        )
        .insert_resource(BackgroundRows::new())
        .insert_resource(RunOnceRes(false))
        .run();
}

/// modified version of game_setup which will make sure
/// we get some water rows after initial grass rows
pub fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bg_rows: ResMut<BackgroundRows>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // UI camera, needed to display text!
    commands.spawn_bundle(UiCameraBundle::default());

    let offset_from_bottom = 0.;
    let row_count = 20;

    let mut rows: Vec<Box<dyn Row>> = vec![];
    rows.push(Box::new(GrassRow::new_grass_row(0)));

    for i in 0..row_count {
        if i > 0 {
            let mut next_row = rows.get(i as usize - 1).unwrap().next();
            // make sure we get water row after the grass
            if next_row.get_row_type() == RowType::ROAD {
                while next_row.get_row_type() == RowType::ROAD {
                    next_row = rows.get(i as usize - 1).unwrap().next();
                }
            }
            rows.push(next_row);
        }
    }

    rows.reverse();

    for i in 0..row_count {
        let x = -1. * (SCREEN_WIDTH / 2.);
        let y = -1. * (SCREEN_HEIGHT / 2.) + SEGMENT_HEIGHT * (i as f32) + offset_from_bottom;
        let row = rows.pop().unwrap();

        bg_rows.add_row(row.clone_row());
        let new_bundle = GameRowBundle::new(row, x, y, &asset_server, i == row_count - 1);
        new_bundle.spawn_bundle_with_markers(&mut commands);
    }

    // center player in the middle of the screen at the last grass
    //  row of bottom grass section (8 grass rows in total)
    let player_x = 0. - SEGMENT_WIDTH / 2.;
    let player_y = -1. * (SCREEN_HEIGHT / 2.) + 8. * SEGMENT_HEIGHT;
    PlayerBundle::new(player_x, player_y, &asset_server, &mut texture_atlas_assets)
        .spawn_player(&mut commands);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/ALGER.TTF"),
                            font_size: 15.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/ALGER.TTF"),
                            font_size: 15.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/ALGER.TTF"),
                            font_size: 15.0,
                            color: Color::GREEN,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DebugText);
}

fn get_random_log_size() -> LogSize {
    if get_random_i32(1, 2) == 1 {
        LogSize::SMALL
    } else {
        LogSize::BIG
    }
}

pub fn put_logs_on_water(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &BackgroundRow), Added<WaterRowMarker>>,
) {
    const LOGS_PER_ROW: i32 = 4;
    const LOGS_GAP_FROM: i32 = 20;
    const LOGS_GAP_TO: i32 = 250;

    // child position is relative to parent (i.e. left bottom to parent row is 0,0)!
    let mut x_even_row = 0.;
    let mut x_odd_row = SCREEN_WIDTH / 2. - LOG_SMALL_WIDTH as f32;

    for (entity, bg_row) in q.iter_mut() {
        if bg_row.is_water_row {
            for i in 1..LOGS_PER_ROW + 1 {
                // choose big or small randomly
                let log_size = get_random_log_size();

                if is_even_number(bg_row.row.get_index())
                /* even rows*/
                {
                    // handle logs for even rows. these logs are flowing from left to right
                    // choose negative X offset from previous log randomly so that logs do not overlap
                    // the space between two logs will be within range <LOGS_GAP_FROM, LOGS_GAP_TO>
                    if i > 1 {
                        x_even_row = if log_size == LogSize::BIG {
                            x_even_row
                                - get_random_i32(
                                    LOG_BIG_WIDTH + LOGS_GAP_FROM,
                                    LOG_BIG_WIDTH + LOGS_GAP_TO,
                                ) as f32
                        } else {
                            x_even_row
                                - get_random_i32(
                                    LOG_SMALL_WIDTH + LOGS_GAP_FROM,
                                    LOG_SMALL_WIDTH + LOGS_GAP_TO,
                                ) as f32
                        };
                    }

                    let log_bundle = LogBundle::new(
                        MovementDirection::RIGHT,
                        log_size,
                        x_even_row,
                        0.,
                        &asset_server,
                    );
                    println!("log_bundle {:?}", log_bundle);
                    log_bundle.spawn_log(&mut commands, entity);
                } else
                /* odd rows */
                {
                    // handle logs for odd rows. these logs are flowing from right to left
                    // choose positive X offset from previous log randomly so that logs do not overlap
                    // the space between two logs will be within range <20,200>
                    if i > 1 {
                        x_odd_row = if log_size == LogSize::BIG {
                            x_odd_row
                                + get_random_i32(
                                    LOG_BIG_WIDTH + LOGS_GAP_FROM,
                                    LOG_BIG_WIDTH + LOGS_GAP_TO,
                                ) as f32
                        } else {
                            x_odd_row
                                + get_random_i32(
                                    LOG_SMALL_WIDTH + LOGS_GAP_FROM,
                                    LOG_SMALL_WIDTH + LOGS_GAP_TO,
                                ) as f32
                        };
                    }

                    let log_bundle = LogBundle::new(
                        MovementDirection::LEFT,
                        log_size,
                        x_odd_row,
                        0.,
                        &asset_server,
                    );
                    println!("log_bundle {:?}", log_bundle);
                    log_bundle.spawn_log(&mut commands, entity);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn text_update_system(mut q: Query<&mut Text, With<DebugText>>) {
    for mut text in q.iter_mut() {
        text.sections[1].value = "SomeText".to_string();
    }
}

/// simplified version of player_is_standing_on
/// working with water only
fn player_is_standing_on(
    q_player: Query<&Transform, (With<Player>, Without<BackgroundRow>)>,
    q_background_row: Query<(&Transform, &BackgroundRow, &mut Children)>,
    mut q_debugtxt: Query<&mut Text, With<DebugText>>,
    q_child: Query<
        (&GlobalTransform, &LogSize, &LogBundleUuid),
        (Without<BackgroundRow>, Without<Player>),
    >,
) {
    // first determine which background row player is standing on
    let mut player_x = -1.;
    let mut player_y = -1.;
    for transform in q_player.iter() {
        player_x = transform.translation.x;
        player_y = transform.translation.y;
        break;
    }
    if player_y == -1. {
        println!("unable to find player!!!");
        return;
    }

    for mut text in q_debugtxt.iter_mut() {
        text.sections[0].value = format!("x: {:.2}, y: {:.2}", player_x, player_y);
    }

    let mut standing_on_the_log = false;
    let mut standing_on_the_log_id = "".to_string();
    let mut _row_type_str = "".to_owned();

    'outer: for (transform, bg_row, children) in q_background_row.iter() {
        let bgrow_y_from = transform.translation.y - 20.;
        let bgrow_y_to = transform.translation.y + 40. - 20.;

        if (bgrow_y_from..bgrow_y_to).contains(&player_y) {
            _row_type_str = format!("{:?}", bg_row.row.get_row_type());
            if bg_row.is_water_row {
                for &child in children.iter() {
                    if let Ok((child_global_transform, log_size, uuid)) = q_child.get(child) {
                        let log_size_f32: f32 = log_size.into();
                        let log_x = child_global_transform.translation.x;
                        let log_y = child_global_transform.translation.y;
                        let log_x_plus_width = log_x + log_size_f32;
                        let log_y_plus_height = log_y + 40.;
                        let x_from = log_x - 20.;
                        let x_to = log_x_plus_width - 20.;
                        let y_from = log_y - 20.;
                        let y_to = log_y_plus_height - 20.;

                        if (x_from..x_to).contains(&player_x) && (y_from..y_to).contains(&player_y)
                        {
                            standing_on_the_log = true;
                            standing_on_the_log_id = uuid.get_uuid();
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    if standing_on_the_log {
        for mut text in q_debugtxt.iter_mut() {
            text.sections[1].value = format!("{}", standing_on_the_log_id);
            // text.sections[2].value = row_type_str.to_owned();
        }
    } else {
        for mut text in q_debugtxt.iter_mut() {
            text.sections[1].value = "".to_string();
            // text.sections[2].value = row_type_str.to_owned();
        }
    }
}

fn active_row(
    q_player: Query<&Transform, (With<Player>, Without<BackgroundRow>)>,
    q_bgrow: Query<(&Transform, &BackgroundRow)>,
    mut q_debugtxt: Query<&mut Text, With<DebugText>>,
) {
    // first determine which background row player is standing on
    let mut player_y = -1.;
    for transform in q_player.iter() {
        player_y = transform.translation.y;
        break;
    }
    if player_y == -1. {
        println!("unable to find player!!!");
        return;
    }

    for (transform, bg_row) in q_bgrow.iter() {
        let row_y_from = transform.translation.y - 20.;
        let row_y_to = transform.translation.y + 40. - 20.;

        if (row_y_from..row_y_to).contains(&player_y) {
            for mut text in q_debugtxt.iter_mut() {
                text.sections[2].value = format!("{:?}", bg_row.row.get_row_type());
            }
        }
    }
}

fn logs_debug(
    q_parent: Query<(&Transform, &BackgroundRow, &mut Children)>,
    q_child: Query<
        (
            &Transform,
            &GlobalTransform,
            &LogSize,
            &MovementDirection,
            &LogBundleUuid,
        ),
        (Without<BackgroundRow>, Without<Player>),
    >,
    mut run_once_res: ResMut<RunOnceRes>,
) {
    if run_once_res.did_run() {
        return;
    }
    for (_transform, bg_row, children) in q_parent.iter() {
        if bg_row.is_water_row {
            for &child in children.iter() {
                if let Ok((
                    child_transform,
                    child_global_transform,
                    log_size,
                    log_direction,
                    uuid,
                )) = q_child.get(child)
                {
                    println!(
                        "x: {} y: {} gx: {} gy: {}, size: {:?}, direction: {:?} uuid: {}",
                        child_transform.translation.x,
                        child_transform.translation.y,
                        child_global_transform.translation.x,
                        child_global_transform.translation.y,
                        log_size,
                        log_direction,
                        uuid.get_uuid(),
                    );
                    run_once_res.set_did_run();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bunner_rs::ecs::components::log::LogSize;

    #[test]
    fn test_log_contains_player() {
        // x: 0 y: 0 gx: -240 gy: -40, size: BIG, uuid: f0168890...
        let log_size_f32: f32 = (&LogSize::SMALL).into();
        let log_x = -240.;
        let log_y = -40.;
        let log_x_plus_width = log_x + log_size_f32;
        let log_y_plus_height = log_y + 40.;

        let x_from = log_x - 40.;
        let x_to = log_x_plus_width + 40.;
        let y_from = log_y - 40.;
        let y_to = log_y_plus_height + 40.;

        let player_x = -223.0;
        let player_y = -42.0;

        assert_eq!(
            true,
            (x_from..x_to).contains(&player_x) && (y_from..y_to).contains(&player_y)
        )
    }
}
