use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct PlayerControl;

#[derive(Component)]
pub struct PlayerCamera;

pub fn build_player_entity(commands: &mut Commands) {
    commands.spawn((
        TransformBundle::from_transform(
            Transform::from_xyz(2.0, 0.55, 2.0)
                .looking_to(Vec3::new(1.0, 0.0, 1.0), Vec3::Y)),
        Collider::capsule_y(0.5, 0.25),
        RigidBody::Dynamic,
        Velocity::zero(),
        Ccd::enabled(),
        Sleeping::disabled(),
        LockedAxes::ROTATION_LOCKED,
        PlayerControl,
        ColliderDebugColor(Color::NONE)))
        .with_children(|parent| {
            parent.spawn((Camera3dBundle::default(), PlayerCamera));
        });
}


pub fn update_player(time: Res<Time>,
                 mut player_query: Query<(&PlayerControl, &mut Transform)>,
                 mut camera_query: Query<(&PlayerCamera, &mut Transform), Without<PlayerControl>>,
                 keys: Res<Input<KeyCode>>) {
    let (_, mut transform) = player_query.single_mut();
    let (_, mut cam_transform) = camera_query.single_mut();

    let mut move_change = Vec3::ZERO;
    if keys.pressed(KeyCode::W) {
        move_change += Vec3::NEG_Z;
    }

    if keys.pressed(KeyCode::S) {
        move_change += Vec3::Z;
    }

    if keys.pressed(KeyCode::A) {
        move_change += Vec3::NEG_X;
    }

    if keys.pressed(KeyCode::D) {
        move_change += Vec3::X;
    }

    if keys.pressed(KeyCode::Right) {
        transform.rotate_y(-time.delta_seconds());
    }

    if keys.pressed(KeyCode::Left) {
        transform.rotate_y(time.delta_seconds());
    }

    if keys.pressed(KeyCode::Up) {
        cam_transform.rotate_local_x(time.delta_seconds())
    }

    if keys.pressed(KeyCode::Down) {
        cam_transform.rotate_local_x(-time.delta_seconds())
    }

    let add = transform.rotation.mul_vec3(move_change).normalize_or_zero() * time.delta_seconds() * 2.0;
    transform.translation += add;
}