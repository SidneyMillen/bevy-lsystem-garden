use bevy::prelude::*;

pub fn add_pot(pos: Vec3) {}

pub fn load_pot(mut commands: Commands, assets: Res<AssetServer>) {
    let pot = assets.load("pot.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: pot,
        ..Default::default()
    });
}
