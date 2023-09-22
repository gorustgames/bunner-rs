use crate::ecs::components::background_row::{
    BackgroundRow, GameRowBundle, GrassRow, GrassRowMarker, RailRowMarker, RoadRowMarker, Row,
    RowType, WaterRowMarker,
};
use crate::ecs::components::background_row_border::{GameRowBorder, GameRowBorderBundle};
use crate::ecs::components::bush::{BushBundle, BushHorizontalType, BushVerticalType};
use crate::ecs::components::car::{CarBundle, CarSpeed};
use crate::ecs::components::debug_text::{DebugText, DebugTextMarker};
use crate::ecs::components::log::{LogBundle, LogSize};
use crate::ecs::components::player::{
    AnimationTimer, Player, PlayerBundle, PlayerDirection, PlayerDirectionIndex,
};
use crate::ecs::components::train::TrainBundle;
use crate::ecs::components::{
    ButtonExitMarker, ButtonPlayMarker, CarTimer, DelayedCarReadyToBeDisplayedMarker,
    DelayedTrainReadyToBeDisplayedMarker, DespawnEntityTimer, MovementDirection, TrainTimer,
};
use crate::ecs::resources::{
    BackgroundRows, BackgroundScrollingEnabled, CollisionType, MenuData,
    PlayerMovementBlockedDirection, PlayerPosition,
};
use crate::{
    get_random_float, get_random_i32, get_random_i8, get_random_row_mask, is_even_number,
    is_odd_number, player_col_to_coords, player_x_to_player_col, AppState, CAR_HEIGHT,
    CAR_SPEED_FROM, CAR_SPEED_TO, CAR_WIDTH, HOVERED_BUTTON, LOG_BIG_WIDTH, LOG_SMALL_WIDTH,
    NORMAL_BUTTON, PRESSED_BUTTON, SCREEN_HEIGHT, SCREEN_WIDTH, SCROLLING_SPEED_BACKGROUND,
    SCROLLING_SPEED_LOGS, SCROLLING_SPEED_PLAYER, SCROLLING_SPEED_TRAINS, SEGMENT_HEIGHT,
    SEGMENT_WIDTH, TRAIN_HEIGHT, TRAIN_WIDTH, Z_GAMEOVER, Z_GRID,
};
use bevy::app::AppExit;
use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

/// this system takes care of entities scheduled for delayed despawning
pub fn delayed_despawn_recursive(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnEntityTimer)>,
) {
    for (entity, mut de_timer) in &mut query.iter_mut() {
        if de_timer.timer.tick(time.delta()).just_finished() {
            // despawn entity wrapped by timer together with all child entities
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// this system checks if TrainTimer associated with TrainBundle
/// has elapsed. If so it will add special marker component to train
/// which will cause train to come into screen.
pub fn delayed_spawn_train(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TrainTimer)>,
) {
    for (entity, mut se_timer) in query.iter_mut() {
        if se_timer.timer.tick(time.delta()).just_finished() {
            commands
                .entity(entity)
                .insert(DelayedTrainReadyToBeDisplayedMarker);
        }
    }
}

/// this system checks if CarTimer associated with CarBundle
/// has elapsed. If so it will add special marker component to car
/// which will cause car to come into screen.
pub fn delayed_spawn_car(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CarTimer)>,
) {
    for (entity, mut se_timer) in query.iter_mut() {
        if se_timer.timer.tick(time.delta()).just_finished() {
            commands
                .entity(entity)
                .insert(DelayedCarReadyToBeDisplayedMarker);
        }
    }
}

/// generate_hedge is helper function used from background_scrolling
fn generate_hedge(next_bg_row: &mut Box<dyn Row>, bg_rows: &ResMut<BackgroundRows>) {
    // for added rows of grass type we might potentially want to add bushes
    if next_bg_row.get_row_type() == RowType::GRASS {
        // generate bushes only for certain grass rows (7,14) randomly (50:50)
        let is_mask_eligible =
            get_random_float() < 0.5 && next_bg_row.get_index() > 7 && next_bg_row.get_index() < 14;

        if let Some(previous_row) = bg_rows.last_row() {
            if let Some(row_mask) = previous_row.get_row_mask() {
                if let Some(row_data) = previous_row.get_row_data() {
                    if let Ok(row_with_top_bushes) = row_data.downcast::<bool>() {
                        if !*row_with_top_bushes {
                            // create top hedge row only if previous row is bottom row
                            // if it is top row do not create anything, we want have gap
                            // between two hedges
                            next_bg_row.set_row_mask(row_mask);
                            next_bg_row.set_row_data(Box::new(true)); // this is top hedge row
                        }
                    }
                }
            } else {
                if is_mask_eligible {
                    next_bg_row.set_row_mask(get_random_row_mask());
                    next_bg_row.set_row_data(Box::new(false)); // this is bottom hedge row
                }
            }
        }
    }
}

/// this system takes care of scrolling of generated background landscape
pub fn background_scrolling(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scrolling_enabled: Res<BackgroundScrollingEnabled>,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Transform, &mut BackgroundRow)>,
    mut bg_rows: ResMut<BackgroundRows>,
) {
    if !scrolling_enabled.enabled {
        return;
    }

    for (entity, mut transform, mut bg_row) in q.iter_mut() {
        transform.translation.y -= SCROLLING_SPEED_BACKGROUND * time.delta_seconds();
        bg_rows.set_row_y_by_row_uuid(&bg_row.row.get_row_uuid(), transform.translation.y);

        // if current top row's top Y coord is already below top of the screen (i.e. there is blank space) -> create new top row
        if bg_row.is_top_row && transform.translation.y < SCREEN_HEIGHT / 2. - SEGMENT_HEIGHT {
            bg_row.is_top_row = false; // make current top row as non-top since we are going to create new top level block

            // create new row and position it at the top of current top row
            let x = -1. * (SCREEN_WIDTH / 2.);
            let y = transform.translation.y + SEGMENT_HEIGHT;

            let mut next_bg_row = bg_row.row.next();

            generate_hedge(&mut next_bg_row, &bg_rows);

            bg_rows.add_row(next_bg_row.clone_row());
            let new_bundle = GameRowBundle::new(next_bg_row, x, y, &asset_server, true);
            new_bundle.spawn_bundle_with_markers(&mut commands);
        }

        // remove entity which has scrolled down bellow screen bottom and is not visible any more
        let y_bellow_bottom = -1. * (SCREEN_HEIGHT / 2.) - SEGMENT_HEIGHT;
        if transform.translation.y < y_bellow_bottom {
            // remove background row entity and its children (i.e. logs, trains, cars)
            //println!("despawning {:?} {:?}", entity, bg_row);

            // do not remove immediately...
            //commands.entity(entity).despawn_recursive();

            // ...instead delay the despawning!!!
            // let _empty_entity = commands.spawn().id();
            commands.entity(entity).insert(DespawnEntityTimer::new(5.));
        }
    }
}

/// adds horizontal lines/borders for each row. used for debugging
/// vertical lines are created only once in game_setup startup system
/// since they are static in nature whereas horizontal lines are scrolling together with
/// respective background rows
pub fn border_adding(mut commands: Commands, mut q: Query<&Transform, Added<BackgroundRow>>) {
    for transform in q.iter_mut() {
        GameRowBorderBundle::new(transform.translation.y).spawn_bundle(&mut commands);
    }
}

/// scrolls horizontal lines/borders for each row with same speed as we are scrolling background
pub fn border_scrolling(
    scrolling_enabled: Res<BackgroundScrollingEnabled>,
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform), With<GameRowBorder>>,
) {
    if !scrolling_enabled.enabled {
        return;
    }
    for (entity, mut transform) in q.iter_mut() {
        transform.translation.y -= SCROLLING_SPEED_BACKGROUND * time.delta_seconds();

        if transform.translation.y < -500. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// we are moving player down with the same speed as background is scrolling
/// this will create illusion of player standing at particular place
pub fn player_scrolling(
    time: Res<Time>,
    mut q: Query<&mut Transform, With<Player>>,
    player_position: ResMut<PlayerPosition>,
    scrolling_enabled: Res<BackgroundScrollingEnabled>,
) {
    if !scrolling_enabled.enabled {
        return;
    }

    for mut transform in q.iter_mut() {
        transform.translation.y -= SCROLLING_SPEED_BACKGROUND * time.delta_seconds();
        match &player_position.collision_type {
            CollisionType::WaterLog(movement_direction) => {
                if *movement_direction == MovementDirection::RIGHT {
                    transform.translation.x += SCROLLING_SPEED_LOGS * time.delta_seconds();
                } else {
                    transform.translation.x -= SCROLLING_SPEED_LOGS * time.delta_seconds();
                }
            }
            _ => {}
        }
    }
}

/// system to stop background scrolling
/// and other debugging goodies
/// intended for debugging only
pub fn debug_system(
    mut scrolling_enabled: ResMut<BackgroundScrollingEnabled>,
    keyboard_input: Res<Input<KeyCode>>,
    player_position: Res<PlayerPosition>,
    bg_rows: Res<BackgroundRows>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if time_now - scrolling_enabled.changed > 1 {
            scrolling_enabled.enabled = !scrolling_enabled.enabled;
            scrolling_enabled.changed = time_now;
        }
    }

    if keyboard_input.pressed(KeyCode::P) {
        println!("player_position: {:?}", player_position);
    }

    if keyboard_input.pressed(KeyCode::D) {
        let player_row = player_position.row_index;
        let _player_col = player_position.col_index as usize;

        println!("player_row: {:?}", player_row);
        let row_above = bg_rows.get_row(player_row as usize + 1);
        let row_row = bg_rows.get_row(player_row as usize);
        let row_below = bg_rows.get_row(player_row as usize - 1);

        if let Some(row_mask) = row_above.unwrap().get_row_mask() {
            println!("row_above: {:?}", row_mask);
        }

        if let Some(row_mask) = row_row.unwrap().get_row_mask() {
            println!("row_row: {:?}", row_mask);
        }

        if let Some(row_mask) = row_below.unwrap().get_row_mask() {
            println!("row_below: {:?}", row_mask);
        }

        println!("all rows:");
        bg_rows.debug_print();
    }
}

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_position: ResMut<PlayerPosition>,
    mut query: Query<
        (
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
            &mut PlayerDirection,
            &mut PlayerDirectionIndex,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut transform, mut sprite, mut timer, mut direction, mut direction_idx)) =
        query.get_single_mut()
    {
        timer.tick(time.delta());

        if keyboard_input.pressed(KeyCode::Up) {
            *direction = PlayerDirection::Up;
            if player_position.movement_blocked_dir == PlayerMovementBlockedDirection::Up {
                return;
            }
            transform.translation.y += SCROLLING_SPEED_PLAYER * time.delta_seconds();
            if transform.translation.y > SCREEN_HEIGHT / 2. - SEGMENT_HEIGHT {
                transform.translation.y = SCREEN_HEIGHT / 2. - SEGMENT_HEIGHT;
            }

            if timer.just_finished() {
                PlayerBundle::change_sprite_icon(&mut direction, &mut direction_idx, &mut sprite);
            }
        }

        if keyboard_input.pressed(KeyCode::Down) {
            *direction = PlayerDirection::Down;
            if player_position.movement_blocked_dir == PlayerMovementBlockedDirection::Down {
                return;
            }
            transform.translation.y -= SCROLLING_SPEED_PLAYER * time.delta_seconds();
            if transform.translation.y < SCREEN_HEIGHT / -2. {
                transform.translation.y = SCREEN_HEIGHT / -2.;
            }

            if timer.just_finished() {
                PlayerBundle::change_sprite_icon(&mut direction, &mut direction_idx, &mut sprite);
            }
        }

        if keyboard_input.pressed(KeyCode::Left) {
            *direction = PlayerDirection::Left;
            if player_position.movement_blocked_dir == PlayerMovementBlockedDirection::Left {
                return;
            }
            transform.translation.x -= SCROLLING_SPEED_PLAYER * time.delta_seconds();
            if transform.translation.x < SCREEN_WIDTH / -2. {
                transform.translation.x = SCREEN_WIDTH / -2.;
            }

            if timer.just_finished() {
                PlayerBundle::change_sprite_icon(&mut direction, &mut direction_idx, &mut sprite);
            }
        }

        if keyboard_input.pressed(KeyCode::Right) {
            *direction = PlayerDirection::Right;
            if player_position.movement_blocked_dir == PlayerMovementBlockedDirection::Right {
                return;
            }
            transform.translation.x += SCROLLING_SPEED_PLAYER * time.delta_seconds();
            if transform.translation.x > SCREEN_WIDTH / 2. - SEGMENT_WIDTH {
                transform.translation.x = SCREEN_WIDTH / 2. - SEGMENT_WIDTH;
            }

            if timer.just_finished() {
                PlayerBundle::change_sprite_icon(&mut direction, &mut direction_idx, &mut sprite);
            }
        }

        player_position.player_x = transform.translation.x;
        player_position.player_y = transform.translation.y;
    }
}

/// this system move generated logs on the water surface
pub fn logs_movement(
    q_parent: Query<(&Transform, &BackgroundRow, &mut Children)>,
    mut q_child: Query<&mut Transform, Without<BackgroundRow>>,
    time: Res<Time>,
) {
    for (_parent_transform, bg_row, children) in q_parent.iter() {
        if bg_row.is_water_row {
            for &child in children.iter() {
                if let Ok(mut child_transform) = q_child.get_mut(child) {
                    // logs in odd rows flow from right to left
                    // logs in even rows flow from left to right
                    if is_odd_number(bg_row.row.get_index()) {
                        child_transform.translation.x -=
                            SCROLLING_SPEED_LOGS * time.delta_seconds();
                    } else {
                        child_transform.translation.x +=
                            SCROLLING_SPEED_LOGS * time.delta_seconds();
                    }
                }
            }
        }
    }
}

/// this system moves generated trains on rail roads
pub fn trains_movement(
    q_parent: Query<(&BackgroundRow, &mut Children)>,
    mut q_child: Query<
        (&mut Transform, &MovementDirection),
        (
            Without<BackgroundRow>,
            With<DelayedTrainReadyToBeDisplayedMarker>,
        ),
    >,
    time: Res<Time>,
) {
    for (bg_row, children) in q_parent.iter() {
        if bg_row.is_rail_row {
            for &child in children.iter() {
                if let Ok((mut child_transform, movement_direction)) = q_child.get_mut(child) {
                    match movement_direction {
                        MovementDirection::LEFT => {
                            child_transform.translation.x -=
                                SCROLLING_SPEED_TRAINS * time.delta_seconds();
                        }
                        MovementDirection::RIGHT => {
                            child_transform.translation.x +=
                                SCROLLING_SPEED_TRAINS * time.delta_seconds();
                        }
                    }
                }
            }
        }
    }
}

/// this system moves generated cars on roads
pub fn cars_movement(
    q_parent: Query<(&BackgroundRow, &mut Children)>,
    mut q_child: Query<
        (&mut Transform, &MovementDirection, &CarSpeed),
        (
            Without<BackgroundRow>,
            With<DelayedCarReadyToBeDisplayedMarker>,
        ),
    >,
    time: Res<Time>,
) {
    for (bg_row, children) in q_parent.iter() {
        if bg_row.is_road_row {
            for &child in children.iter() {
                if let Ok((mut child_transform, movement_direction, car_speed)) =
                    q_child.get_mut(child)
                {
                    match movement_direction {
                        MovementDirection::LEFT => {
                            child_transform.translation.x -=
                                car_speed.value() * time.delta_seconds();
                        }
                        MovementDirection::RIGHT => {
                            child_transform.translation.x +=
                                car_speed.value() * time.delta_seconds();
                        }
                    }
                }
            }
        }
    }
}

fn get_random_log_size() -> LogSize {
    if get_random_i32(1, 2) == 1 {
        LogSize::SMALL
    } else {
        LogSize::BIG
    }
}

/// this system is generating trains on added rail rows
pub fn put_trains_on_rails(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &BackgroundRow), Added<RailRowMarker>>,
) {
    let mut x: f32;

    for (entity, bg_row) in q.iter_mut() {
        if bg_row.is_rail_row {
            // generate 2 to 4 trains per each track
            let mut train_delay = 0.;
            for _ in 0..get_random_i8(2, 4) {
                // randomize train delay. do it incrementally so that we don't have
                // train crash on the track :)
                train_delay = train_delay + get_random_i8(4, 7) as f32;

                // 50:50 chance of train coming from left or right side
                // we are putting train offset 1200 px so that trains does not
                // go into screen immediately after rail row scrolling into screen
                // other approach here would be delay train bundle spawning
                let train_direction;
                if get_random_float() < 0.5 {
                    // // child position is relative to parent (i.e. left bottom to parent row is 0,0)!
                    x = -1. * TRAIN_WIDTH - 100.;
                    train_direction = MovementDirection::RIGHT
                } else {
                    x = SCREEN_WIDTH + 100.;
                    train_direction = MovementDirection::LEFT
                }
                TrainBundle::new(train_direction.clone(), x, 0., &asset_server)
                    //.spawn_train(&mut commands, entity);
                    .spawn_train_with_delay(&mut commands, entity, train_delay);
            }
        }
    }
}

/// returns max speed of car from selected interval lower or equal than max
/// when generating speed for cars on selected row we do following:
/// 1. generate car speed of very first car within <from, to>
/// 2. each subsequent car has 50:50 chance of having same speed as previous car
/// 3. if speed should differ it should be only smaller. this is to ensure we have no collisions
fn get_random_car_speed(max: i32, from: i32, to: i32) -> i32 {
    let mut car_speed: i32 = 0;
    let mut speed_ok = false;
    while !speed_ok {
        car_speed = get_random_i32(from, to);
        if car_speed > 0 && car_speed <= max {
            speed_ok = true;
        }
    }
    car_speed
}

/// this system is generating cars on added road rows
pub fn put_cars_on_roads(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &BackgroundRow), Added<RoadRowMarker>>,
) {
    let mut x: f32;

    for (entity, bg_row) in q.iter_mut() {
        if bg_row.is_road_row {
            let mut previous_car_speed = CAR_SPEED_TO;

            let mut car_delay = 0.;

            // for given row there is 50:50 chance for cars going left or right
            let car_direction = if get_random_float() < 0.5 {
                // // child position is relative to parent (i.e. left bottom to parent row is 0,0)!
                MovementDirection::RIGHT
            } else {
                MovementDirection::LEFT
            };

            // generate 4 to 20 cars per each road row
            for _ in 1..=get_random_i8(4, 20) {
                // randomize car delay
                car_delay = car_delay + get_random_i8(3, 7) as f32;

                // each car can have at most same speed as previous car to prevent clashes!
                let car_speed =
                    get_random_car_speed(previous_car_speed, CAR_SPEED_FROM, CAR_SPEED_TO);
                previous_car_speed = car_speed;

                if car_direction == MovementDirection::RIGHT {
                    // // child position is relative to parent (i.e. left bottom to parent row is 0,0)!
                    x = -1. * CAR_WIDTH - 100.;
                } else {
                    x = SCREEN_WIDTH + 100.;
                }
                CarBundle::new(
                    car_direction.clone(),
                    x,
                    0.,
                    car_speed as f32,
                    &asset_server,
                )
                .spawn_car_with_delay(&mut commands, entity, car_delay);
            }
        }
    }
}

/// puts logs on newly added water row
/// With<Added<WaterRowMarker>>
/// uses bevy change detection to do it only once
/// we are randomizing log size and putting 10 logs in each row
/// with random distance between them from 20 to 200 pixels.
/// 10 random logs should be enough so that there are still some logs
///  on the water while water row is visible (i.e. it does not scroll of vertically) on the screen
pub fn put_logs_on_water(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &BackgroundRow), Added<WaterRowMarker>>,
) {
    const LOGS_PER_ROW: i32 = 10;
    const LOGS_GAP_FROM: i32 = 20;
    const LOGS_GAP_TO: i32 = 250;

    // child position is relative to parent (i.e. left bottom to parent row is 0,0)!
    let mut x_even_row = 0.;
    let mut x_odd_row = SCREEN_WIDTH / 2. - LOG_SMALL_WIDTH as f32;

    for (entity, bg_row) in q.iter_mut() {
        // TODO: replace with bg_row.row.get_row_type() == RowType::XXX
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

                    LogBundle::new(
                        MovementDirection::RIGHT,
                        log_size,
                        x_even_row,
                        0.,
                        &asset_server,
                    )
                    .spawn_log(&mut commands, entity);
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

                    LogBundle::new(
                        MovementDirection::LEFT,
                        log_size,
                        x_odd_row,
                        0.,
                        &asset_server,
                    )
                    .spawn_log(&mut commands, entity);
                }
            }
        }
    }
}

pub fn put_bushes_on_grass(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q: Query<(Entity, &BackgroundRow), Added<GrassRowMarker>>,
) {
    for (entity, bg_row) in q.iter_mut() {
        if bg_row.row.get_row_type() == RowType::GRASS {
            if let Some(mask) = bg_row.row.get_row_mask() {
                let mut bush_vertical_type = BushVerticalType::BOTTOM;
                let mut bush_horizontal_type;

                if let Ok(row_with_top_bushes) =
                    bg_row.row.get_row_data().unwrap().downcast::<bool>()
                {
                    bush_vertical_type = if *row_with_top_bushes == true {
                        BushVerticalType::TOP
                    } else {
                        BushVerticalType::BOTTOM
                    };
                }

                for i in 0..12 {
                    if i == 0 {
                        if mask[i + 1] == true {
                            bush_horizontal_type = BushHorizontalType::SINGLE;
                        } else {
                            bush_horizontal_type = BushHorizontalType::LEFTMOST;
                        }
                    } else if i > 0 && i < 11 {
                        if mask[i - 1] == true && mask[i + 1] == true {
                            bush_horizontal_type = BushHorizontalType::SINGLE;
                        } else if mask[i - 1] == true && mask[i + 1] == false {
                            bush_horizontal_type = BushHorizontalType::LEFTMOST;
                        } else if mask[i - 1] == false && mask[i + 1] == true {
                            bush_horizontal_type = BushHorizontalType::RIGHTMOST;
                        } else {
                            bush_horizontal_type = BushHorizontalType::MIDDLE1;
                        }
                    } else {
                        if mask[i - 1] == true {
                            bush_horizontal_type = BushHorizontalType::LEFTMOST;
                        } else {
                            bush_horizontal_type = BushHorizontalType::MIDDLE1;
                        }
                    }

                    if !mask[i]
                    /* if there should be a bush */
                    {
                        let bush_bundle = BushBundle::new(
                            &asset_server,
                            0. + i as f32 * SEGMENT_WIDTH,
                            0.,
                            bush_vertical_type,
                            bush_horizontal_type,
                        );

                        let bush = commands.spawn_bundle(bush_bundle).id();
                        commands.entity(entity).add_child(bush);
                    }
                }
            }
        }
    }
}

/// helper function used within game_setup (if debugging)
#[allow(dead_code)]
fn draw_line(start_x: f32, start_y: f32, width: f32, height: f32, commands: &mut Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(width, height)),
            ..Default::default()
        },
        transform: Transform::from_xyz(start_x, start_y, Z_GRID),
        ..Default::default()
    });
}

/// this is main setup system of the game
pub fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bg_rows: ResMut<BackgroundRows>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let offset_from_bottom = 0.;
    let row_count = 20;

    let mut rows: Vec<Box<dyn Row>> = vec![];
    rows.push(Box::new(GrassRow::new_grass_row(0)));

    for i in 0..row_count {
        if i > 0 {
            rows.push(rows.get(i as usize - 1).unwrap().next())
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

    // used for sending debugging information by dedicated debugging systems
    DebugText::new(&asset_server).spawn_debug_text(&mut commands);

    // for debugging only
    /*draw_line(-200., 0., 1.0, 800.0, &mut commands);
    draw_line(-160., 0., 1.0, 800.0, &mut commands);
    draw_line(-120., 0., 1.0, 800.0, &mut commands);
    draw_line(-80., 0., 1.0, 800.0, &mut commands);
    draw_line(-40., 0., 1.0, 800.0, &mut commands);
    draw_line(0., 0., 1.0, 800.0, &mut commands);
    draw_line(40., 0., 1.0, 800.0, &mut commands);
    draw_line(80., 0., 1.0, 800.0, &mut commands);
    draw_line(120., 0., 1.0, 800.0, &mut commands);
    draw_line(160., 0., 1.0, 800.0, &mut commands);
    draw_line(200., 0., 1.0, 800.0, &mut commands);*/
}

/// sets global player position when player is standing on dirt, pavement, or grass row
/// for remaining types (road, rail, water) dedicated systems with collision detection are used.
pub fn active_row(
    q_player: Query<&Transform, (With<Player>, Without<BackgroundRow>)>,
    q_bgrow: Query<(&Transform, &BackgroundRow)>,
    mut player_position: ResMut<PlayerPosition>,
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
            let row_type = bg_row.row.get_row_type();
            if row_type == RowType::GRASS {
                player_position.set_grass();
            }
            if row_type == RowType::PAVEMENT {
                player_position.set_pavement();
            }
            if row_type == RowType::DIRT {
                player_position.set_dirt();
            }
        }
    }
}

pub fn active_row_water(
    q_player: Query<&Transform, (With<Player>, Without<BackgroundRow>)>,
    q_background_row: Query<(&Transform, &BackgroundRow, &mut Children)>,
    q_child: Query<
        (&GlobalTransform, &LogSize, &MovementDirection),
        (Without<BackgroundRow>, Without<Player>),
    >,
    mut player_position: ResMut<PlayerPosition>,
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

    let mut standing_on_the_water = false;
    let mut standing_on_the_log = false;
    let mut log_direction: Option<MovementDirection> = None;

    'outer: for (transform, bg_row, children) in q_background_row.iter() {
        let bgrow_y_from = transform.translation.y - 20.;
        let bgrow_y_to = transform.translation.y + 40. - 20.;

        if (bgrow_y_from..bgrow_y_to).contains(&player_y) {
            if bg_row.is_water_row {
                standing_on_the_water = true;
                for &child in children.iter() {
                    if let Ok((child_global_transform, log_size, movement_direction)) =
                        q_child.get(child)
                    {
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
                            log_direction = Some(movement_direction.clone());
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    if standing_on_the_water && standing_on_the_log {
        player_position.set_water_ok(log_direction.unwrap());
    } else if standing_on_the_water && !standing_on_the_log {
        player_position.set_water_ko();
    } else {
        // send nothing! player is not standing on the water row!
    }
}

pub fn active_row_road(
    q_player: Query<&Transform, (With<Player>, Without<BackgroundRow>)>,
    q_background_row: Query<(&Transform, &BackgroundRow, &mut Children)>,
    q_child: Query<&GlobalTransform, (Without<BackgroundRow>, Without<Player>)>,
    mut player_position: ResMut<PlayerPosition>,
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

    let mut standing_on_the_road = false;
    let mut hit_by_car = false;

    'outer: for (transform, bg_row, children) in q_background_row.iter() {
        let bgrow_y_from = transform.translation.y - 20.;
        let bgrow_y_to = transform.translation.y + 40. - 20.;

        if (bgrow_y_from..bgrow_y_to).contains(&player_y) {
            if bg_row.is_road_row {
                standing_on_the_road = true;
                for &child in children.iter() {
                    if let Ok(child_global_transform) = q_child.get(child) {
                        let car_x = child_global_transform.translation.x;
                        let car_y = child_global_transform.translation.y;
                        let car_x_plus_width = car_x + CAR_WIDTH;
                        let car_y_plus_height = car_y + CAR_HEIGHT;
                        let x_from = car_x - 20.;
                        let x_to = car_x_plus_width - 20.;
                        let y_from = car_y - 20.;
                        let y_to = car_y_plus_height - 20.;

                        if (x_from..x_to).contains(&player_x) && (y_from..y_to).contains(&player_y)
                        {
                            hit_by_car = true;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    if standing_on_the_road && !hit_by_car {
        player_position.set_road_ok();
    } else if standing_on_the_road && hit_by_car {
        player_position.set_road_ko();
    } else {
        // send nothing! player is not standing on the road row!
    }
}

// TODO: while this approach works well with water and road where children have same height as parent row
// with train which we are putting on rail1 only (other 3 rail rows having no children) bunner
// wil get hit by train only when standing on row1 resulting in situation where visually bunner
// is overrun by train but this is not detected since he is standing on rail0 or rail2 (we can ignore rail3 and
// treat it like bunner already did cross the rails).
//
// the solution for that would be always to check rail1 road even if bunner is standing on rail0 or rail2
pub fn active_row_rail(
    q_player: Query<&Transform, (With<Player>, Without<BackgroundRow>)>,
    q_background_row: Query<(&Transform, &BackgroundRow, &mut Children)>,
    q_child: Query<&GlobalTransform, (Without<BackgroundRow>, Without<Player>)>,
    mut player_position: ResMut<PlayerPosition>,
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

    let mut standing_on_the_rail = false;
    let mut hit_by_train = false;

    'outer: for (transform, bg_row, children) in q_background_row.iter() {
        let bgrow_y_from = transform.translation.y - 20.;
        let bgrow_y_to = transform.translation.y + 40. - 20.;

        if (bgrow_y_from..bgrow_y_to).contains(&player_y) {
            if bg_row.is_rail_row {
                standing_on_the_rail = true;
                for &child in children.iter() {
                    if let Ok(child_global_transform) = q_child.get(child) {
                        let train_x = child_global_transform.translation.x;
                        let train_y = child_global_transform.translation.y;
                        let train_x_plus_width = train_x + TRAIN_WIDTH;
                        let train_y_plus_height = train_y + TRAIN_HEIGHT;
                        let x_from = train_x - 20.;
                        let x_to = train_x_plus_width - 20.;
                        let y_from = train_y - 20.;
                        let y_to = train_y_plus_height - 20.;

                        if (x_from..x_to).contains(&player_x) && (y_from..y_to).contains(&player_y)
                        {
                            hit_by_train = true;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    if standing_on_the_rail && !hit_by_train {
        player_position.set_rail_ok();
    } else if standing_on_the_rail && hit_by_train {
        player_position.set_rail_ko();
    } else {
        // send nothing! player is not standing on the rail row!
    }
}

/// determines index of row player is standing on
pub fn set_player_row(
    q_player: Query<&Transform, (With<Player>, Without<BackgroundRow>)>,
    mut player_position: ResMut<PlayerPosition>,
    bg_rows: Res<BackgroundRows>,
) {
    let mut player_y = -1;
    for transform in q_player.iter() {
        player_y = transform.translation.y as i32;
        break;
    }
    if player_y == -1 {
        println!("unable to find player!!!");
        return;
    }
    if let Some(player_row) = bg_rows.get_player_row(player_y as f32) {
        player_position.row_index = player_row;
    } else {
        println!("set_player_row_ng: unable to retrieve player row!!!");
    }
}

pub fn set_player_col(
    q_player: Query<&Transform, (With<Player>, Without<BackgroundRow>)>,
    mut player_position: ResMut<PlayerPosition>,
) {
    let mut player_x = -1;
    for transform in q_player.iter() {
        player_x = transform.translation.x as i32;
        break;
    }
    if player_x == -1 {
        println!("unable to find player!!!");
        return;
    }

    let player_col = player_x_to_player_col(player_x);
    if player_col == -1 {
        println!("active_col_player = -1 for x{:?}", player_x);
        return;
    }

    player_position.col_index = player_col;
}

pub fn detect_bushes(
    mut player_position: ResMut<PlayerPosition>,
    bg_rows: Res<BackgroundRows>,
    query: Query<&PlayerDirection, With<Player>>,
) {
    let player_row = player_position.row_index;
    let player_col = player_position.col_index as usize;

    if player_row == -1 {
        // do not run this system if player scrolls off the screen
        return;
    }

    if player_row == 0 {
        // this would mean accessing element -1 in sliding window array
        // which would result in panic
        return;
    }

    let mut player_direction: &PlayerDirection = &PlayerDirection::Up;

    for direction in query.iter() {
        player_direction = direction;
    }

    let row = match player_direction {
        PlayerDirection::Up => bg_rows.get_row(player_row as usize + 1),
        PlayerDirection::Down => bg_rows.get_row(player_row as usize - 1),
        _ => bg_rows.get_row(player_row as usize),
    };

    if row.is_none() {
        return;
    }
    let row = row.unwrap();

    let mut flg_hit = false;
    if let Some(row_mask) = row.get_row_mask() {
        match player_direction {
            PlayerDirection::Up => {
                if row_mask[player_col] == false
                    && player_position.player_y > bg_rows.get_player_row_to_coords(player_row).0
                {
                    flg_hit = true;
                    player_position.movement_blocked_dir = PlayerMovementBlockedDirection::Up;
                }
            }
            PlayerDirection::Down => {
                if row_mask[player_col] == false
                    && player_position.player_y < bg_rows.get_player_row_to_coords(player_row).1
                {
                    flg_hit = true;
                    player_position.movement_blocked_dir = PlayerMovementBlockedDirection::Down;
                }
            }
            PlayerDirection::Left => {
                if player_col > 0
                    && row_mask[player_col - 1] == false
                    && player_position.player_x < player_col_to_coords(player_col).0
                {
                    flg_hit = true;
                    player_position.movement_blocked_dir = PlayerMovementBlockedDirection::Left;
                }
            }
            PlayerDirection::Right => {
                if player_col < 11
                    && row_mask[player_col + 1] == false
                    && player_position.player_x > player_col_to_coords(player_col).0
                {
                    flg_hit = true;
                    player_position.movement_blocked_dir = PlayerMovementBlockedDirection::Right;
                }
            }
        }
    }
    if !flg_hit {
        player_position.movement_blocked_dir = PlayerMovementBlockedDirection::None;
    }
}

/// responsible for player collision detection (with water, train or car)
/// and transitioning into JustDied state
pub fn player_die(player_position: Res<PlayerPosition>, mut state: ResMut<State<AppState>>) {
    if player_position.collision_type == CollisionType::RoadCar {
        state.set(AppState::JustDied).unwrap();
    }
}

/// called when player transitions into JustDied state
/// will set proper player icon and disable scrolling
pub fn player_die_enter(
    mut scrolling_enabled: ResMut<BackgroundScrollingEnabled>,
    mut query: Query<
        (
            &mut TextureAtlasSprite,
            &mut PlayerDirection,
            &mut PlayerDirectionIndex,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut sprite, mut direction, mut direction_idx)) = query.get_single_mut() {
        PlayerBundle::change_sprite_icon_crushed(&mut direction, &mut direction_idx, &mut sprite);
        scrolling_enabled.enabled = false;
    }
}

pub fn debug_text_update_system(
    mut q: Query<&mut Text, With<DebugTextMarker>>,
    player_position: ResMut<PlayerPosition>,
) {
    for mut text in q.iter_mut() {
        text.sections[0].value = format!(" {:?} ", player_position.row_type);
        text.sections[1].value = format!(" {:?} ", player_position.collision_type);
        text.sections[2].value = format!(" {:?} ", player_position.row_index);
        text.sections[3].value = format!(" {:?} ", player_position.col_index);
        text.sections[4].value = format!(" {:?} ", player_position.movement_blocked_dir);
        text.sections[5].value = format!(
            " X: {:.2}, Y: {:.2} ",
            player_position.player_x, player_position.player_y
        );
    }
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    let button_entity_start = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // image: UiImage::from(asset_server.load("images/start1.png")),
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(ButtonPlayMarker)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/ALGER.TTF"),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();

    let button_entity_exit = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // image: UiImage::from(asset_server.load("images/start0.png")),
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(ButtonExitMarker)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Exit",
                    TextStyle {
                        font: asset_server.load("fonts/ALGER.TTF"),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();

    commands.insert_resource(MenuData {
        button_entity_start,
        button_entity_exit,
    });
}

pub fn play_buttton_interactions(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<ButtonPlayMarker>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn exit_button_interactions(
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<ButtonExitMarker>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                exit.send(AppExit);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands
        .entity(menu_data.button_entity_start)
        .despawn_recursive();
    commands
        .entity(menu_data.button_entity_exit)
        .despawn_recursive();
}

pub fn game_over_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("game_over_enter");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/gameover.png"),
        transform: Transform::from_xyz(0., 0., Z_GAMEOVER),
        ..Default::default()
    });
}

/// game over system
pub fn game_over_update(mut exit: EventWriter<AppExit>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Space) {
        println!("game_over");
        exit.send(AppExit);
    }
}

/// demo setup system for initial experiments with game states
/// originally configured as:
///.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game_demo)
#[allow(dead_code)]
pub fn setup_game_demo(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/car31.png"),
        ..Default::default()
    });
}

/// demo movement system for initial experiments with game states
/// originally configured as:
///.add_system_set(SystemSet::on_update(AppState::InGame).with_system(movement_demo))
#[allow(dead_code)]
pub fn movement_demo(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    const SPEED: f32 = 100.0;
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * SPEED * time.delta_seconds();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::get_random_car_speed;
    use crate::{CAR_SPEED_FROM, CAR_SPEED_TO};

    // cargo test test_get_random_car_speed -- --show-output
    #[test]
    fn test_get_random_car_speed() {
        let mut speed;

        let max_speed = 90;

        for _ in 0..1000 {
            speed = get_random_car_speed(max_speed, CAR_SPEED_FROM, CAR_SPEED_TO);
            println!("generated speed {}", speed);
            assert_eq!(speed <= max_speed, true);
        }
    }
}
