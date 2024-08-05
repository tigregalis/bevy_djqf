use bevy::prelude::*;
use bevy_djqf::{make_disjoint_markers, Disjoint};

macro_rules! type_template {
    ($Name:ident) => {
        #[derive(Component, Debug, Default)]
        struct $Name;
    };
}
make_disjoint_markers!(
    type_template for
    Player,
    FriendlyPlayer,
    EnemyPlayer,
    NonPlayerCharacter,
    FriendlyAi,
    EnemyAi,
    InanimateObject
);
// The above would be equivalent to something like the below,
// if Rust had enum variant types and if we enforced that entities didn't have
// more than one variant at a time:
// #[derive(Component, Debug, Default)]
// enum GameObject {
//     Player,
//     FriendlyPlayer,
//     EnemyPlayer,
//     NonPlayerCharacter,
//     FriendlyAi,
//     EnemyAi,
//     InanimateObject,
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn)
        .add_systems(Update, movement)
        // Uncomment this line to see the error:
        // .add_systems(Update, alt_movement)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.5, 1.0, 0.6, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Player,
    ));

    for i in 0..=10 {
        let x = -250.0 + (i as f32) / 10.0 * 500.0;
        for j in 0..=10 {
            let y = -250.0 + (j as f32) / 10.0 * 500.0;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                InanimateObject,
            ));
        }
    }
}

// If this system is present, Bevy will refuse to start, due to error [B0001]:
// Query<&bevy_transform::components::transform::Transform, bevy_ecs::query::filter::With<make_disjoint_markers::InanimateObject>>
// in system make_disjoint_markers::alt_movement
// accesses component(s) bevy_transform::components::transform::Transform in a way that
// conflicts with a previous system parameter.
// Consider using `Without<T>` to
// create disjoint Queries or merging conflicting Queries into a `ParamSet`.
// See: https://bevyengine.org/learn/errors/#b0001
#[allow(dead_code, unused_variables, unused_mut)]
fn alt_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Sprite), With<Player>>,
    mut walls: Query<(&Transform, &mut Sprite), With<InanimateObject>>,
) {
}

fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Sprite), <Player as Disjoint>::Only>,
    mut walls: Query<(&Transform, &mut Sprite), <InanimateObject as Disjoint>::Only>,
) {
    let mut direction = Vec2::ZERO;
    for pressed in keyboard.get_pressed() {
        match pressed {
            KeyCode::KeyW => direction += Vec2::new(0.0, 1.0),
            KeyCode::KeyA => direction += Vec2::new(-1.0, 0.0),
            KeyCode::KeyS => direction += Vec2::new(0.0, -1.0),
            KeyCode::KeyD => direction += Vec2::new(1.0, 0.0),
            _ => (),
        }
    }

    const SPEED: f32 = 250.0;

    let dt = time.delta_seconds();

    let velocity = direction.normalize_or_zero() * SPEED * dt;

    let (mut p_transform, p_sprite) = player.single_mut();
    p_transform.translation += velocity.extend(0.0);

    let p_rect = Rect::from_center_size(
        p_transform.translation.truncate(),
        p_sprite.custom_size.unwrap(),
    );
    for (w_transform, mut w_sprite) in walls.iter_mut() {
        let w_rect = Rect::from_center_size(
            w_transform.translation.truncate(),
            w_sprite.custom_size.unwrap(),
        );
        if p_rect.intersect(w_rect).is_empty() {
            w_sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
        } else {
            w_sprite.color = Color::srgba(1.0, 0.0, 0.0, 1.0);
        }
    }
}
