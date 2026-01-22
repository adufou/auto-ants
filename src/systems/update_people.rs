use crate::components::{Name, Person};
use bevy::prelude::*;

pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.value == "Elaina Proctor" {
            name.value = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}
