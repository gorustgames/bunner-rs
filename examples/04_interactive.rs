use bevy::prelude::*;
use bunner_rs::ecs::resources::{BackgroundRows, BackgroundScrollingEnabled, PlayerPosition};
use bunner_rs::ecs::systems::*;
use bunner_rs::ecs::systems::{delayed_despawn_recursive, delayed_spawn_train};
use bunner_rs::{SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Infinite Bunner".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(game_setup)
        .add_system(background_scrolling)
        .add_system(player_scrolling)
        .add_system(player_movement)
        .add_system(put_trains_on_rails)
        .add_system(put_logs_on_water)
        .add_system(put_bushes_on_grass)
        .add_system(put_cars_on_roads)
        .add_system(logs_movement)
        .add_system(trains_movement)
        .add_system(cars_movement)
        .add_system(delayed_despawn_recursive)
        .add_system(delayed_spawn_train)
        .add_system(delayed_spawn_car)
        .add_system(active_row)
        .add_system(set_player_row)
        .add_system(active_row_water)
        .add_system(active_row_road)
        .add_system(active_row_rail)
        .add_system(debug_text_update_system)
        .insert_resource(BackgroundRows::new())
        .insert_resource(PlayerPosition::new())
        .insert_resource(BackgroundScrollingEnabled::new())
        .run();
}
