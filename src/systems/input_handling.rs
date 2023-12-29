use bevy::prelude::*;

use crate::resources::HandleInputOnMouse;

pub fn change_state_on_handle_input_on_mouse(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<HandleInputOnMouse>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        state.handle = !state.handle;
    }
}
