use bevy::color::palettes::basic::{BLUE, PURPLE, RED};
use bevy::prelude::*;
use bevy_mod_osc::osc_receiver::{OscMessageEvent, OscReceiverPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OscReceiverPlugin {
            port: 2020,
            use_thread: true,
            use_ipv6: false,
            debug_print: true,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, osc_event_handler)
        .add_systems(Update, despawn)
        .run();
}

#[derive(Component)]
struct DespawnTimer(Timer);

const TTL: f32 = 0.2;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn osc_event_handler(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<OscMessageEvent>,
) {
    for event in events.read() {
        if event.message.addr == "/dirt/play" {
            if event.message.args.len() < 1 {
                println!("/dirt/play received!");
            } else {
                println!("/dirt/play received! {:?}", event.message.args);
                let sound = event.message.args[11].clone().string().unwrap();
                if sound == "hc" {
                    commands.spawn((
                        Mesh2d(meshes.add(Circle::default())),
                        MeshMaterial2d(materials.add(Color::from(PURPLE))),
                        Transform::from_xyz(-480., 0., 0.).with_scale(Vec3::splat(128.)),
                        DespawnTimer(Timer::from_seconds(TTL, TimerMode::Once)),
                    ));
                } else if sound == "sd" {
                    commands.spawn((
                        Mesh2d(meshes.add(Circle::default())),
                        MeshMaterial2d(materials.add(Color::from(RED))),
                        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(128.)),
                        DespawnTimer(Timer::from_seconds(TTL, TimerMode::Once)),
                    ));
                } else {
                    commands.spawn((
                        Mesh2d(meshes.add(Circle::default())),
                        MeshMaterial2d(materials.add(Color::from(BLUE))),
                        Transform::from_xyz(480., 0., 0.).with_scale(Vec3::splat(128.)),
                        DespawnTimer(Timer::from_seconds(TTL, TimerMode::Once)),
                    ));
                }
                println!("{:?}", sound);
            }
        }
    }
}

fn despawn(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut DespawnTimer)>) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
