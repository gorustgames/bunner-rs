use bevy::prelude::*;
use bunner_rs::ecs::resources::{BackgroundRows, BackgroundScrollingEnabled, PlayerPosition};
use bunner_rs::ecs::systems::*;
use bunner_rs::{AppState, SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Infinite Bunner".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
        .add_system_set(
            SystemSet::on_update(AppState::Menu)
                .with_system(play_buttton_interactions)
                .with_system(exit_button_interactions),
        )
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(game_setup))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(background_scrolling)
                // uncomment vertical lines in game_setup should they be needed
                // .with_system(border_adding) // for debugging only
                //.with_system(border_scrolling) // for debugging only
                //.with_system(debug_system) // for debugging only
                //.with_system(debug_text_update_system) // for debugging only
                .with_system(player_scrolling)
                .with_system(player_movement)
                .with_system(put_trains_on_rails)
                .with_system(put_logs_on_water)
                .with_system(put_bushes_on_grass)
                .with_system(put_cars_on_roads)
                .with_system(logs_movement)
                .with_system(trains_movement)
                .with_system(cars_movement)
                .with_system(delayed_despawn_recursive)
                .with_system(delayed_spawn_train)
                .with_system(delayed_spawn_car)
                .with_system(detect_bushes)
                .with_system(set_player_row)
                .with_system(set_player_col)
                .with_system(active_row)
                .with_system(active_row_rail_2)
                .with_system(active_row_road)
                .with_system(active_row_water)
                .with_system(player_die_detection),
        )
        .add_system_set(SystemSet::on_enter(AppState::JustDied).with_system(player_just_died_enter))
        .add_system_set(
            SystemSet::on_update(AppState::JustDied)
                .with_system(logs_movement)
                .with_system(trains_movement)
                .with_system(cars_movement)
                .with_system(delayed_spawn_eagle)
                .with_system(eagle_movement),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::JustDiedInWater)
                .with_system(player_just_died_in_water_enter),
        )
        .add_system_set(
            SystemSet::on_update(AppState::JustDiedInWater)
                .with_system(logs_movement)
                .with_system(trains_movement)
                .with_system(cars_movement)
                .with_system(animate_splash)
                .with_system(animate_splash_finish),
        )
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(game_over_enter))
        .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(game_over_update))
        .insert_resource(BackgroundRows::new())
        .insert_resource(PlayerPosition::new())
        .insert_resource(BackgroundScrollingEnabled::new())
        .run();
}
