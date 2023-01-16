use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use simula_action::ActionPlugin;
use simula_camera::orbitcam::*;
use simula_viz::{
    grid::{Grid, GridBundle, GridPlugin},
    lines::{LineMesh, LinesMaterial, LinesPlugin},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(LinesPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(AmbientLight {
            color: Color::rgb(0.9, 1.0, 1.0),
            brightness: 0.24,
        })
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut lines_materials: ResMut<Assets<LinesMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    line_mesh: Res<LineMesh>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 5.0, -25.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCamera::default(),
    ));

    commands.spawn(GridBundle {
        grid: Grid {
            size: 10,
            ..default()
        },
        mesh: meshes.add(line_mesh.clone()),
        material: lines_materials.add(LinesMaterial::default()),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("scenes/halo/scene.gltf#Scene0"),
        ..default()
    });

    let ball_texture = asset_server.load("textures/ball/painted_metal_shutter_diff_4k.jpg");
    let ball_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(ball_texture),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 15,
                ..default()
            })),
            material: ball_material.clone(),
            transform: Transform::from_xyz(-6.0, 4.0, 0.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(PointLightBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Y),
                point_light: PointLight {
                    color: Color::rgb(0.9, 1.0, 1.0),
                    intensity: 1000.0,
                    range: 100.0,
                    ..default()
                },
                ..default()
            });
        });
}
