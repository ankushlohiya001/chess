use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .init_gizmo_group::<MyRoundGizmos>()
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        // .add_systems(Update, (draw_example_collection, update_config))
        .run();
}
//
// // We can create our own gizmo config group!
// #[derive(Default, Reflect, GizmoConfigGroup)]
// struct MyRoundGizmos {}

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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
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

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
    // transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(time.delta_seconds() / 2.));
}

// fn draw_example_collection(
//     mut gizmos: Gizmos,
//     mut my_gizmos: Gizmos<MyRoundGizmos>,
//     time: Res<Time>,
// ) {
//     gizmos.cuboid(
//         Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
//         Color::BLACK,
//     );
//     gizmos.rect(
//         Vec3::new(time.elapsed_seconds().cos() * 2.5, 1., 0.),
//         Quat::from_rotation_y(PI / 2.),
//         Vec2::splat(2.),
//         Color::GREEN,
//     );
//
//     my_gizmos.sphere(Vec3::new(1., 0.5, 0.), Quat::IDENTITY, 0.5, Color::RED);
//
//     for y in [0., 0.5, 1.] {
//         gizmos.ray(
//             Vec3::new(1., y, 0.),
//             Vec3::new(-3., (time.elapsed_seconds() * 3.).sin(), 0.),
//             Color::BLUE,
//         );
//     }
//
//     my_gizmos
//         .arc_3d(
//             180.0_f32.to_radians(),
//             0.2,
//             Vec3::ONE,
//             Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
//             Color::ORANGE,
//         )
//         .segments(10);
//
//     // Circles have 32 line-segments by default.
//     my_gizmos.circle(Vec3::ZERO, Direction3d::Y, 3., Color::BLACK);
//     // You may want to increase this for larger circles or spheres.
//     my_gizmos
//         .circle(Vec3::ZERO, Direction3d::Y, 3.1, Color::NAVY)
//         .segments(64);
//     my_gizmos
//         .sphere(Vec3::ZERO, Quat::IDENTITY, 3.2, Color::BLACK)
//         .circle_segments(64);
//
//     gizmos.arrow(Vec3::ZERO, Vec3::ONE * 1.5, Color::YELLOW);
// }

// fn update_config(
//     mut config_store: ResMut<GizmoConfigStore>,
//     keyboard: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     if keyboard.just_pressed(KeyCode::KeyD) {
//         for (_, config, _) in config_store.iter_mut() {
//             config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };
//         }
//     }
//     if keyboard.just_pressed(KeyCode::KeyP) {
//         for (_, config, _) in config_store.iter_mut() {
//             // Toggle line_perspective
//             config.line_perspective ^= true;
//             // Increase the line width when line_perspective is on
//             config.line_width *= if config.line_perspective { 5. } else { 1. / 5. };
//         }
//     }
//
//     let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
//     if keyboard.pressed(KeyCode::ArrowRight) {
//         config.line_width += 5. * time.delta_seconds();
//         config.line_width = config.line_width.clamp(0., 50.);
//     }
//     if keyboard.pressed(KeyCode::ArrowLeft) {
//         config.line_width -= 5. * time.delta_seconds();
//         config.line_width = config.line_width.clamp(0., 50.);
//     }
//     if keyboard.just_pressed(KeyCode::Digit1) {
//         config.enabled ^= true;use std::f32::consts::{PI, TAU};
//
// use bevy::prelude::*;
//
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .init_gizmo_group::<MyRoundGizmos>()
//         .add_systems(Startup, setup)
//         .add_systems(Update, (draw_example_collection, update_config))
//         .run();
// }
//
// // We can create our own gizmo config group!
// #[derive(Default, Re     // if let Ok(pos) = pos.try_into() {
//         //     if self.position == pos || self.can_move(pos) {
//         //         if let Some(ref surrounding_ref) = self.surrounding {
//         //               // if let Ok(pos) = pos.try_into() {
//         //     if self.position == pos || self.can_move(pos) {
//         //         if let Some(ref surrounding_ref) = self.surrounding {
//         //             let mut surrounding = surrounding_ref.borrow_mut();
//         //             let res = surrounding.place_character(self.character, pos);
//         //             game.board = mem::take(surrounding.deref_mut());
//         //             game.state = GameState::PiecePlaced;
//         //             res
//         //         } else {
//         //             Err(GameError::AlonePiece)
//         //         }
//         //     } else {
//         //         Err(GameError::InvalidMove)
//         //     }
//         // } else {
//         //     Err(GameError::InvalidPosition)
//         // }
//    let mut surrounding = surrounding_ref.borrow_mut();
//         //             let res = surrounding.place_character(self.character, pos);
//         //             game.board = mem::take(surrounding.deref_mut());
//         //             game.state = GameState::PiecePlaced;
//         //             res
//         //         } else {
//         //             Err(GameError::AlonePiece)
//         //         }
//         //     } else {
//         //         Err(GameError::InvalidMove)
//         //     }
//         // } else {
//         //     Err(GameError::InvalidPosition)
//         // }
// flect, GizmoConfigGroup)]
// struct MyRoundGizmos {}
//
// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2dBundle::default());
//     // text
//     commands.spawn(TextBundle::from_section(
//         "Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
//         Hold 'Up' or 'Down' to change the line width of round gizmos\n\
//         Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos",
//         TextStyle {
//             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
//             font_size: 24.,
//             color: Color::WHITE,
//         },
//     ));
// }
//
// fn draw_example_collection(
//     mut gizmos: Gizmos,
//     mut my_gizmos: Gizmos<MyRoundGizmos>,
//     time: Res<Time>,
// ) {
//     let sin = time.elapsed_seconds().sin() * 50.;
//     gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
//     gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);
//
//     // Triangle
//     gizmos.linestrip_gradient_2d([
//         (Vec2::Y * 300., Color::BLUE),
//         (Vec2::new(-255., -155.), Color::RED),
//         (Vec2::new(255., -155.), Color::GREEN),
//         (Vec2::Y * 300., Color::BLUE),
//     ]);
//
//     gizmos.rect_2d(
//         Vec2::ZERO,
//         time.elapsed_seconds() / 3.,
//         Vec2::splat(300.),
//         Color::BLACK,
//     );
//
//     // The circles have 32 line-segments by default.
//     my_gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
//     my_gizmos.ellipse_2d(
//         Vec2::ZERO,
//         time.elapsed_seconds() % TAU,
//         Vec2::new(100., 200.),
//         Color::YELLOW_GREEN,
//     );
//     // You may want to increase this for larger circles.
//     my_gizmos
//         .circle_2d(Vec2::ZERO, 300., Color::NAVY)
//         .segments(64);
//
//     // Arcs default amount of segments is linearly interpolated between
//     // 1 and 32, using the arc length as scalar.
//     my_gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);
//
//     gizmos.arrow_2d(
//         Vec2::ZERO,
//         Vec2::from_angle(sin / -10. + PI / 2.) * 50.,
//         Color::YELLOW,
//     );
// }
//
// fn update_config(
//     mut config_store: ResMut<GizmoConfigStore>,
//     keyboard: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
//     if keyboard.pressed(KeyCode::ArrowRight) {
//         config.line_width += 5. * time.delta_seconds();
//         config.line_width = config.line_width.clamp(0., 50.);
//     }
//     if keyboard.pressed(KeyCode::ArrowLeft) {
//         config.line_width -= 5. * time.delta_seconds();
//         config.line_width = config.line_width.clamp(0., 50.);
//     }
//     if keyboard.just_pressed(KeyCode::Digit1) {
//         config.enabled ^= true;
//     }
//
//     let (my_config, _) = config_store.config_mut::<MyRoundGizmos>();
//     if keyboard.pressed(KeyCode::ArrowUp) {
//         my_config.line_width += 5. * time.delta_seconds();
//         my_config.line_width = my_config.line_width.clamp(0., 50.);
//     }
//     if keyboard.pressed(KeyCode::ArrowDown) {
//         my_config.line_width -= 5. * time.delta_seconds();
//         my_config.line_width = my_config.line_width.clamp(0., 50.);
//     }
//     if keyboard.just_pressed(KeyCode::Digit2) {
//         my_config.enabled ^= true;
//     }
// }
//     }
//
//     let (my_config, _) = config_store.config_mut::<MyRoundGizmos>();
//     if keyboard.pressed(KeyCode::ArrowUp) {
//         my_config.line_width += 5. * time.delta_seconds();
//         my_config.line_width = my_config.line_width.clamp(0., 50.);
//     }
//     if keyboard.pressed(KeyCode::ArrowDown) {
//         my_config.line_width -= 5. * time.delta_seconds();
//         my_config.line_width = my_config.line_width.clamp(0., 50.);
//     }
//     if keyboard.just_pressed(KeyCode::Digit2) {
//         my_config.enabled ^= true;
//     }
//
//     if keyboard.just_pressed(KeyCode::KeyA) {
//         // AABB gizmos are normally only drawn on entities with a ShowAabbGizmo component
//         // We can change this behaviour in the configuration of AabbGizmoGroup
//         config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
//     }
// }
