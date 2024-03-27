use std::f32::consts::PI;

use bevy::{input::keyboard, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .init_gizmo_group::<MyRoundGizmos>()
        .add_systems(Startup, setup)
        .add_systems(Update, transform_cube)
        // .add_systems(Update, (draw_example_collection, update_config))
        .run();
}
//
// // We can create our own gizmo config group!
// #[derive(Default, Reflect, GizmoConfigGroup)]
// struct MyRoundGizmos {}
#[derive(Component)]
struct Docker;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // cube
    commands.spawn((
        Docker,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(2.5, 0.5, 0.0),
            ..default()
        },
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    //
    // // example instructions
    // commands.spawn(
    //     TextBundle::from_section(
    //         "Press 'D' to toggle drawing gizmos on top of everything else in the scene\n\
    //         Press 'P' to toggle perspective for line gizmos\n\
    //         Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
    //         Hold 'Up' or 'Down' to change the line width of round gizmos\n\
    //         Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos\n\
    //         Press 'A' to show all AABB boxes\n\
    //         Press 'K' or 'J' to cycle through primitives rendered with gizmos\n\
    //         Press 'H' or 'L' to decrease/increase the amount of segments in the primitives",
    //         TextStyle {
    //             font_size: 20.,
    //             ..default()
    //         },
    //     )
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(12.0),
    //         left: Val::Px(12.0),
    //         ..default()
    //     }),
    // );
}

fn transform_cube(mut query: Query<&mut Transform, With<Docker>>, time: Res<Time>) {
    // for mut transform in &mut query {
    //     transform.rotate_y(time.delta_seconds() / 2.);
    //     // transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
    // }
    let mut elem = query.single_mut();
    elem.rotate_z(time.delta_seconds());
}

fn rotate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
    }

    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(time.delta_seconds() / -2.),
        );
    }

    if keyboard.pressed(KeyCode::ArrowUp) {
        transform.translate_around(Vec3::ZERO, Quat::from_rotation_z(time.delta_seconds() / 2.));
    }

    if keyboard.pressed(KeyCode::ArrowDown) {
        transform.translate_around(
            Vec3::ZERO,
            Quat::from_rotation_z(time.delta_seconds() / -2.),
        );
    }
    // transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(time.delta_seconds() / 2.));
}
