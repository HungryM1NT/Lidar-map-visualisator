use bevy::app::{App, Startup, Update};
use bevy::asset::Assets;
use bevy::color::{Color, LinearRgba};
use bevy::core_pipeline::core_3d::Camera3d;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::Res;
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseButton;
use bevy::math::Vec3;
use bevy::pbr::AmbientLight;
use bevy::prelude::{ColorToComponents, Commands, Cuboid, Mesh, Mesh3d, ResMut, Transform};
use bevy::render::view::NoFrustumCulling;
use bevy::DefaultPlugins;
use bevy::utils::default;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_voxel_plot::{InstanceData, InstanceMaterialData, VoxelMaterialPlugin};
pub mod get_points;
use crate::get_points::get_points;
use bevy::input::{keyboard::KeyboardInput, mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},};
use bevy_blendy_cameras::{
    BlendyCamerasPlugin, FlyCameraController, FrameEvent,
    OrbitCameraController, SwitchProjection, SwitchToFlyController,
    SwitchToOrbitController, Viewpoint, ViewpointEvent,
};
use bevy::ecs::prelude::Resource;
use bevy::ecs::entity::Entity;

#[derive(Resource)]
struct Scene {
    pub camera_entity: Entity,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VoxelMaterialPlugin, BlendyCamerasPlugin))
        .add_systems(Startup, voxel_plot_setup)
        .add_systems(Update, (print_keyboard_event_system, mouse_click_system))
        .run();
}

fn voxel_plot_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let (instances, cube_width, cube_height, cube_depth) =
        load_pcd_file("assets/binary.pcd");

    let mut instances: Vec<InstanceData> = instances.into_iter().collect();

    // Sort by opacity (color alpha channel) descending
    // instances.sort_by(|a, b| {
    //     b.color[3]
    //         .partial_cmp(&a.color[3])
    //         .unwrap_or(std::cmp::Ordering::Equal)
    // });

    // Truncate to top 2 million most opaque points, more than that is usually not responsive
    // const MAX_INSTANCES: usize = 2_000_000;
    // if instances.len() > MAX_INSTANCES {
    //     instances.truncate(MAX_INSTANCES);
    // }

    // if instances.len() == MAX_INSTANCES {
    //     let threshold = instances.last().unwrap().color[3];
    //     println!("Auto threshold for opacity was: {}", threshold);
    // }

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(cube_width, cube_height, cube_depth))),
        InstanceMaterialData { instances },
        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
        // instanced cubes will be culled.
        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
        // instancing, and that is not taken into account with the built-in frustum culling.
        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
        // component to avoid incorrect culling.
        NoFrustumCulling,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.0, // Increase this to wash out shadows
        affects_lightmapped_meshes: false,
    });

    // camera
    // commands.spawn((
    //     Transform::from_translation(Vec3::new(1.0, 0.0, 5.0)),
    //     PanOrbitCamera::default(),
    // ));
    let camera_entity = commands.spawn((
        Camera3d::default() ,
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        OrbitCameraController::default(),
        FlyCameraController {
            is_enabled: false,
            ..default()
        },
    )).id();
    commands.insert_resource(Scene {
        camera_entity,
    });
}


fn load_pcd_file(path: &str) -> (Vec<InstanceData>, f32, f32, f32) {
    let point_list = get_points(path).unwrap();
    let mut instances = Vec::new();

    for point in point_list {
        let instance = InstanceData {
            pos_scale: [point.x, point.y, point.z, 3.3],
            color: LinearRgba::from(Color::srgba(1.0, 1.0, 1.0, 1.0)).to_f32_array(), // you can set color later if needed
        };

        instances.push(instance);
    };
    // Choose a reasonable cube size for rendering
    let cube_width = 0.02;
    let cube_height = 0.02;
    let cube_depth = 0.02;

    (instances, cube_width, cube_height, cube_depth)
}

fn print_keyboard_event_system(mut keyboard_inputs: EventReader<KeyboardInput>) {
    for keyboard_input in keyboard_inputs.read() {
        println!("{:?}", keyboard_input);
    }
}

fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        println!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        println!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        println!("left mouse just released");
    }
}