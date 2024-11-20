use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{
    panorbit_cam::{PanOrbitCameraBundle, PanOrbitSettings, PanOrbitState},
    player::PlayerCam,
    GameState,
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Editor), setup_editor_state)
            .add_systems(OnExit(GameState::Editor), exit_editor_state);
    }
}

fn setup_editor_state(
    mut q: Query<(Entity, &mut Transform), With<PlayerCam>>,
    mut commands: Commands,
) {
    let (player, mut transform) = q.get_single_mut().unwrap();

    let mut player_e = commands.entity(player);
    player_e.insert((PanOrbitCamera::default()));
    dbg! {"entered editor"};
}
fn exit_editor_state(mut q: Query<(Entity), With<PlayerCam>>, mut commands: Commands) {
    let e = q.get_single().unwrap();
    commands.entity(e).remove::<PanOrbitCamera>();
}
