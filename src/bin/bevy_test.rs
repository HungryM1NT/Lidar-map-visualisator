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
use bevy_voxel_plot::{InstanceData, InstanceMaterialData, VoxelMaterialPlugin};
use bevy::input::{keyboard::{KeyboardInput, KeyCode}, mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},};
use bevy_blendy_cameras::{
    BlendyCamerasPlugin, FlyCameraController, FrameEvent,
    OrbitCameraController, SwitchProjection, SwitchToFlyController,
    SwitchToOrbitController, Viewpoint, ViewpointEvent,
};
use bevy::ecs::prelude::Resource;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventWriter;
use bevy::prelude::*;

use hakaton::point_reader::*;
use hakaton::main_file_splitter::*;

#[derive(Resource)]
struct Scene {
    pub camera_entity: Entity,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VoxelMaterialPlugin, BlendyCamerasPlugin))
        .add_systems(Startup, voxel_plot_setup)
        .add_systems(Update, (cursor, print_keyboard_event_system, mouse_click_system, switch_camera_controler_system))
        .run();
}

fn voxel_plot_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let (instances, cube_width, cube_height, cube_depth) =
        load_pcd_file("./assets/hak_big/hak_ascii.pcd");
        // load_pcd_file("./assets/office_ascii.pcd");

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

    // 103678.125 86112.67188 164.6864319
    let camera_entity = commands.spawn((
        Camera3d::default() ,
        // Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Transform::from_translation(Vec3::new(99523.95, 84802.266, 175.87889)),
        OrbitCameraController {
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Left,
            ..default()
        },
        FlyCameraController {
            key_move_forward:KeyCode::KeyW,
            key_move_backward:KeyCode::KeyS,
            key_move_left:KeyCode::KeyA,
            key_move_right:KeyCode::KeyD,
            key_move_down:KeyCode::KeyQ,
            key_move_up:KeyCode::KeyE,
            button_rotate:MouseButton::Left,
            is_enabled: false,
            ..default()
        },
    )).id();
    commands.insert_resource(Scene {
        camera_entity,
    });
}


fn load_pcd_file(path: &str) -> (Vec<InstanceData>, f32, f32, f32) {
    let areas = read_and_process_pcd_file(path);
    // let areas = get_file_areas(&pcd_data);

    let mut instances = Vec::new();

    // let points = get_area_points(pcd_data, &areas[0]);
    let points = &areas[1];
    println!("{:?}", get_area_center(&points));
    println!("{:?}", points.len());
    // for point in pcd_data.points {
    for point in points {
        let instance = InstanceData {
            pos_scale: [point.x, point.y, point.z, 5.3],
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
    // for keyboard_input in keyboard_inputs.read() {
    //     println!("{:?}", keyboard_input);
    // }
}

fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        // println!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        // println!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        // println!("left mouse just released");
    }
}

fn switch_camera_controler_system(
    mut commands: Commands,
    key_input: Res<ButtonInput<KeyCode>>,
    mut orbit_ev_writer: EventWriter<SwitchToOrbitController>,
    mut fly_ev_writer: EventWriter<SwitchToFlyController>,
    // mut help_text: ResMut<HelpText>,
    scene: Res<Scene>,
) {
    
    if key_input.just_pressed(KeyCode::Tab) {
        fly_ev_writer.write(SwitchToFlyController {
            camera_entity: scene.camera_entity,
        });
    }
    if key_input.just_pressed(KeyCode::CapsLock) {
        orbit_ev_writer.write(SwitchToOrbitController {
            camera_entity: scene.camera_entity,
        });
    }
}

fn cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Ok(windows) = windows.single() else {
        return;
    };
    
    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    // println!("{:?}", ray);

    let point = ray.get_point(12.);
    // println!("{:?}", ray.direction);

    // Draw a circle just above the ground plane at that position.
    // gizmos.circle(
    //     Isometry3d::new(
    //         point + ground.up() * 0.01,
    //         Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
    //     ),
    //     0.2,
    //     Color::WHITE,
    // );
}