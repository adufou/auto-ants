use bevy::prelude::*;

use crate::components::{Name, Person};

pub fn add_people(mut commands: Commands) {
    commands.spawn((
        Person,
        Name {
            value: "Elaina Proctor".to_string(),
        },
    ));
    commands.spawn((
        Person,
        Name {
            value: "Renzo Hume".to_string(),
        },
    ));
    commands.spawn((
        Person,
        Name {
            value: "Zayna Nieves".to_string(),
        },
    ));
}
