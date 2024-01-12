use super::prelude::*;

use bevy::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, control.run_if(in_state(GState::InGame)));
    }
}

#[derive(Component)]
pub struct Enemy;

// fn control(time: Res<Time>, mut player: Query<&mut Vel, With<Player>>, keys: Res<Input<KeyCode>>) {
// }
