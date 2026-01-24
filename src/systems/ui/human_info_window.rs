use crate::components::markers::Human;
use crate::components::movement::{CohesionInfluence, RandomWalkInfluence};
use crate::components::ui::floating_window::{
    HumanInfoCloseButton, HumanInfoEntityId, HumanInfoInfluences, HumanInfoNeighbors,
    HumanInfoPosition, HumanInfoVelocity, HumanInfoWindowRoot,
};
use crate::resources::{SelectedHuman, SpatialGrid};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

/// Validates that the selected human still exists
pub fn validate_selected_human(
    mut selected_human: ResMut<SelectedHuman>,
    humans_query: Query<&Human>,
) {
    if let Some(entity) = selected_human.0 {
        if humans_query.get(entity).is_err() {
            selected_human.0 = None;
        }
    }
}

/// Spawns or despawns the human info window based on selection state
pub fn spawn_human_info_window(
    selected_human: Res<SelectedHuman>,
    window_query: Query<Entity, With<HumanInfoWindowRoot>>,
    mut commands: Commands,
) {
    let window_exists = !window_query.is_empty();

    match (selected_human.0.is_some(), window_exists) {
        (true, false) => {
            // Spawn window when human is selected but window doesn't exist
            commands.spawn((Node::default(), Visibility::default(), HumanInfoWindowRoot));
        }
        (false, true) => {
            // Despawn window when no human is selected but window exists
            for entity in window_query.iter() {
                commands.entity(entity).despawn();
            }
        }
        _ => {}
    }
}

/// Handles the close button interaction
pub fn handle_human_info_close(
    button_query: Query<&Interaction, (Changed<Interaction>, With<HumanInfoCloseButton>)>,
    mut selected_human: ResMut<SelectedHuman>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            selected_human.0 = None;
        }
    }
}

/// Updates the content of the human info window with live data
pub fn update_human_info_content(
    selected_human: Res<SelectedHuman>,
    spatial_grid: Res<SpatialGrid>,
    humans_query: Query<
        (
            &Transform,
            &LinearVelocity,
            &RandomWalkInfluence,
            &CohesionInfluence,
        ),
        With<Human>,
    >,
    mut entity_id_query: Query<
        &mut Text,
        (
            With<HumanInfoEntityId>,
            Without<HumanInfoPosition>,
            Without<HumanInfoVelocity>,
            Without<HumanInfoNeighbors>,
            Without<HumanInfoInfluences>,
        ),
    >,
    mut position_query: Query<
        &mut Text,
        (
            With<HumanInfoPosition>,
            Without<HumanInfoEntityId>,
            Without<HumanInfoVelocity>,
            Without<HumanInfoNeighbors>,
            Without<HumanInfoInfluences>,
        ),
    >,
    mut velocity_query: Query<
        &mut Text,
        (
            With<HumanInfoVelocity>,
            Without<HumanInfoEntityId>,
            Without<HumanInfoPosition>,
            Without<HumanInfoNeighbors>,
            Without<HumanInfoInfluences>,
        ),
    >,
    mut neighbors_query: Query<
        &mut Text,
        (
            With<HumanInfoNeighbors>,
            Without<HumanInfoEntityId>,
            Without<HumanInfoPosition>,
            Without<HumanInfoVelocity>,
            Without<HumanInfoInfluences>,
        ),
    >,
    mut influences_query: Query<
        &mut Text,
        (
            With<HumanInfoInfluences>,
            Without<HumanInfoEntityId>,
            Without<HumanInfoPosition>,
            Without<HumanInfoVelocity>,
            Without<HumanInfoNeighbors>,
        ),
    >,
) {
    // Return early if no human is selected
    let Some(selected_entity) = selected_human.0 else {
        return;
    };

    // Get the selected human's data
    let Ok((transform, linear_velocity, random_walk, cohesion)) = humans_query.get(selected_entity)
    else {
        return;
    };

    let position = transform.translation.truncate();

    // Calculate velocity magnitude and angle
    let velocity_vec = linear_velocity.0;
    let velocity_magnitude = velocity_vec.length();
    let velocity_angle = velocity_vec.y.atan2(velocity_vec.x).to_degrees();

    // Query nearby humans using spatial grid
    let neighbors = spatial_grid.query_neighbors(position, cohesion.perception_radius);
    let nearby_count = neighbors.len().saturating_sub(1); // Exclude self

    // Update entity ID text
    if let Ok(mut text) = entity_id_query.single_mut() {
        **text = format!("Entity: {:?}", selected_entity);
    }

    // Update position text
    if let Ok(mut text) = position_query.single_mut() {
        **text = format!("Position: ({:.1}, {:.1})", position.x, position.y);
    }

    // Update velocity text
    if let Ok(mut text) = velocity_query.single_mut() {
        **text = format!(
            "Velocity: {:.1} px/s @ {:.0}°",
            velocity_magnitude, velocity_angle
        );
    }

    // Update neighbors count
    if let Ok(mut text) = neighbors_query.single_mut() {
        **text = format!("Nearby humans: {}", nearby_count);
    }

    // Update influences text
    if let Ok(mut text) = influences_query.single_mut() {
        **text = format!(
            "Influences:\n  Random Walk: weight {:.2}\n  Cohesion: radius {:.0}px, weight {:.2}",
            random_walk.weight, cohesion.perception_radius, cohesion.weight
        );
    }
}
