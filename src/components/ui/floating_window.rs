use bevy::prelude::*;

/// Root component for the test floating window
#[derive(Component)]
pub struct TestFloatingWindowRoot;

/// Marker component for the close button
#[derive(Component)]
pub struct TestWindowCloseButton;

/// Root component for the human info floating window
#[derive(Component)]
pub struct HumanInfoWindowRoot;

/// Marker component for the human info window close button
#[derive(Component)]
pub struct HumanInfoCloseButton;

/// Marker component for entity ID text
#[derive(Component)]
pub struct HumanInfoEntityId;

/// Marker component for position text
#[derive(Component)]
pub struct HumanInfoPosition;

/// Marker component for velocity text
#[derive(Component)]
pub struct HumanInfoVelocity;

/// Marker component for neighbors count text
#[derive(Component)]
pub struct HumanInfoNeighbors;

/// Marker component for influences text
#[derive(Component)]
pub struct HumanInfoInfluences;
