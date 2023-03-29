use crate::ecs::components::{
    DelayedTrainReadyToBeDisplayedMarker, DelayerTrainTimer, DespawnEntityTimer,
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

pub fn delayed_spawn_train(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DelayerTrainTimer)>,
) {
    for (entity, mut se_timer) in query.iter_mut() {
        if se_timer.timer.tick(time.delta()).just_finished() {
            commands
                .entity(entity)
                .insert(DelayedTrainReadyToBeDisplayedMarker);
        }
    }
}
