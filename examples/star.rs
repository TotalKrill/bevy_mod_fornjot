use bevy::prelude::*;
use bevy_mod_fornjot::*;
mod fj_star;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, rotate);
    app.run()
}

#[derive(Component)]
struct Rotate;
fn rotate(mut query: Query<&mut Transform, With<Rotate>>) {
    for mut t in query.iter_mut() {
        t.rotate_y(0.01);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-13., 10., -13.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let light_t = Transform::from_xyz(0.0, 4.5, 0.);
    commands.spawn((
        PointLightBundle {
            transform: light_t.clone(),
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
        meshes.add(Cuboid::default()),
        materials.add(Color::WHITE),
        // PbrBundle {
        //     mesh: meshes.add(Cuboid::default()),
        //     material: materials.add(Color::WHITE),
        //     transform: light_t,
        //     ..default()
        // },
        // PointLight {
        //     shadows_enabled: true,
        //     ..default()
        // },
    ));
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(13.)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    let mut core = bevy_mod_fornjot::fj_core::Core::new();
    let solid = fj_star::model(4, 4., 2., 1., &mut core);
    let mut mesh = (&solid, Tolerance::from_scalar(0.001).unwrap())
        .triangulate(&mut core)
        .to_bevy_mesh(true);

    let solid = fj_star::model(6, 5., 2., -1., &mut core);
    // let mesh2 = (&solid, Tolerance::from_scalar(0.001).unwrap())
    //     .triangulate(&mut core)
    //     .to_bevy_mesh(true);
    // mesh.merge(mesh2);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0., 2.5, 0.),
            ..default()
        },
        Rotate,
    ));
}
