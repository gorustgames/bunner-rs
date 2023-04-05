use crate::ecs::components::background_row::{
    BackgroundRow, GameRowBundle, GrassRowMarker, RailRowMarker, RoadRowMarker, Row, RowType,
    WaterRowMarker,
};
use crate::ecs::components::bush::{BushBundle, BushHorizontalType, BushVerticalType};
use crate::ecs::components::car::{CarBundle, CarSpeed};
use crate::ecs::components::log::{LogBundle, LogSize};
use crate::ecs::components::train::TrainBundle;
use crate::ecs::components::{
    CarTimer, DelayedCarReadyToBeDisplayedMarker, DelayedTrainReadyToBeDisplayedMarker,
    DespawnEntityTimer, MovementDirection, TrainTimer,
};
use crate::ecs::resources::BackgroundRows;
use crate::{
    get_random_float, get_random_i32, get_random_i8, get_random_row_mask, is_even_number,
    is_odd_number, CAR_SPEED_FROM, CAR_SPEED_TO, CAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    SCROLLING_SPEED_BACKGROUND, SCROLLING_SPEED_LOGS, SCROLLING_SPEED_TRAINS, SEGMENT_HEIGHT,
    SEGMENT_WIDTH, TRAIN_WIDTH,
};
use bevy::prelude::*;

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
    time: Res<Time>,
    mut q: Query<(Entity, &mut Transform, &mut BackgroundRow)>,
    mut bg_rows: ResMut<BackgroundRows>,
) {
    for (entity, mut transform, mut bg_row) in q.iter_mut() {
        transform.translation.y -= SCROLLING_SPEED_BACKGROUND * time.delta_seconds();

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
    const LOG_BIG_WIDTH: i32 = 138;
    const LOG_SMALL_WIDTH: i32 = 84;
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
                        MovementDirection::LEFT,
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
                let mut bush_horizontal_type = BushHorizontalType::LEFTMOST;

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
                        bush_horizontal_type = BushHorizontalType::LEFTMOST;
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
                            -1. * SCREEN_WIDTH / 2. + i as f32 * SEGMENT_WIDTH,
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
