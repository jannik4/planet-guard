use bevy::prelude::*;

pub struct MutePlugin;

impl Plugin for MutePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mute);
    }
}

fn mute(mut global_volume: ResMut<GlobalVolume>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        *global_volume = GlobalVolume::new(if *global_volume.volume < 1.0 {
            1.0
        } else {
            0.0
        });
    }
}
