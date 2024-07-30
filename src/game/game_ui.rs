use super::{ApplyVelocity, Health, Home, Player};
use crate::{assets::GameAssets, AppState};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        // Update
        app.add_systems(
            Update,
            update.after(ApplyVelocity).run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Debug, Component)]
struct HealthBarHome(f32);

#[derive(Debug, Component)]
struct HealthBarPlayer(f32);

fn update(
    time: Res<Time>,
    mut health_bar_player: Query<(&mut Transform, &mut HealthBarPlayer), Without<HealthBarHome>>,
    mut health_bar_home: Query<(&mut Transform, &mut HealthBarHome), Without<HealthBarPlayer>>,
    players: Query<&Health, With<Player>>,
    homes: Query<&Health, With<Home>>,
) {
    let Ok((mut health_bar_player_transform, mut health_bar_player)) =
        health_bar_player.get_single_mut()
    else {
        return;
    };
    let Ok((mut health_bar_home_transform, mut health_bar_home)) = health_bar_home.get_single_mut()
    else {
        return;
    };

    let Ok(player) = players.get_single() else {
        return;
    };
    let home_health_fraction = match homes.get_single() {
        Ok(home) => home.fraction(),
        Err(_) => 0.0,
    };

    health_bar_player.0 = f32::lerp(
        health_bar_player.0,
        player.fraction(),
        1.0 - f32::exp(f32::ln(0.9) * 60.0 * time.delta_seconds()),
    );
    health_bar_player_transform.translation.x = 100.0 - 200.0 * health_bar_player.0 / 2.0;
    health_bar_player_transform.scale.y = health_bar_player.0;

    health_bar_home.0 = f32::lerp(
        health_bar_home.0,
        home_health_fraction,
        1.0 - f32::exp(f32::ln(0.9) * 60.0 * time.delta_seconds()),
    );
    health_bar_home_transform.translation.x = 100.0 - 200.0 * health_bar_home.0 / 2.0;
    health_bar_home_transform.scale.y = health_bar_home.0;
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(612.0, 0.0, 0.0)),
                ..default()
            },
            StateScoped(AppState::Game),
        ))
        .with_children(|builder| {
            // Planet
            builder.spawn(MaterialMesh2dBundle {
                mesh: assets.health_bar_mesh.clone(),
                material: assets.health_bar_material_gray.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 480.0, 0.0))
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            builder.spawn((
                MaterialMesh2dBundle {
                    mesh: assets.health_bar_mesh.clone(),
                    material: assets.health_bar_material_green.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 480.0, 1.0))
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
                HealthBarHome(1.0),
            ));
            builder.spawn(MaterialMesh2dBundle {
                mesh: assets.planet_mesh.clone(),
                material: assets.home_planet_material.clone(),
                transform: Transform::from_translation(Vec3::new(120.0, 480.0, 0.0)),
                ..default()
            });

            // Player
            builder.spawn(MaterialMesh2dBundle {
                mesh: assets.health_bar_mesh.clone(),
                material: assets.health_bar_material_gray.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 450.0, 0.0))
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
            builder.spawn((
                MaterialMesh2dBundle {
                    mesh: assets.health_bar_mesh.clone(),
                    material: assets.health_bar_material_green.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 450.0, 1.0))
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
                HealthBarPlayer(1.0),
            ));
            builder.spawn(MaterialMesh2dBundle {
                mesh: assets.space_ship_mesh.clone(),
                material: assets.player_space_ship_material.clone(),
                transform: Transform::from_translation(Vec3::new(120.0, 450.0, 0.0)),
                ..default()
            });
        });
}

fn cleanup(mut _commands: Commands) {}
