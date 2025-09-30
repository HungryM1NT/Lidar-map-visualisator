use bevy::app::{App, Startup};
use bevy::asset::Assets;
use bevy::color::{Color, LinearRgba};
use bevy::math::{Vec2, Vec3};
use bevy::pbr::AmbientLight;
use bevy::prelude::{
    default, Camera, ClearColorConfig, ColorToComponents, Commands, Cuboid, Deref,
    DetectChangesMut, Handle, Image, Mesh, Mesh3d, Query, Res, ResMut, Resource, Transform, Update,
    Window, With,
};
use bevy::render::camera::{ImageRenderTarget, RenderTarget};
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::{NoFrustumCulling, RenderLayers};
use bevy::window::PrimaryWindow;
use bevy::DefaultPlugins;
use bevy_egui::egui::{epaint, Ui};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiUserTextures};
// use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_voxel_plot::{InstanceData, InstanceMaterialData, VoxelMaterialPlugin};
use hakaton::point_reader::*;
use hakaton::main_file_splitter::*;
use hakaton::util::MyPoint;
use rfd;
use bevy_blendy_cameras::{
    BlendyCamerasPlugin, FlyCameraController, FrameEvent,
    OrbitCameraController, SwitchProjection, SwitchToFlyController,
    SwitchToOrbitController, Viewpoint, ViewpointEvent,
};
use bevy::input::{keyboard::{KeyboardInput, KeyCode}, mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},};

use bevy::core_pipeline::core_3d::Camera3d;
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseButton;
use bevy_blendy_cameras::ActiveCameraData;
use bevy::prelude::Entity;

#[derive(Resource)]
pub struct PointSize(pub f32);


#[derive(Deref, Resource)]
pub struct RenderImage(Handle<Image>);

#[derive(Resource, Default)]
pub struct CameraInputAllowed(pub bool);


#[derive(Resource, Default)]
pub struct PCDFilePath(pub String);


#[derive(Resource)]
pub struct Areas(pub Vec<Vec<MyPoint>>);

#[derive(Resource)]
pub struct PCDFileInfo {
    pub path: String,
    pub areas: Vec<Vec<MyPoint>>,
    pub area_num: u32,
    pub area_len: u32
}

impl Default for PCDFileInfo {
    fn default() -> Self {
        PCDFileInfo {
            path: String::new(),
            areas: vec![],
            area_num: 1,
            area_len: 1
        }
    }
}

fn voxel_plot_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut active_cam: ResMut<ActiveCameraData>,
    windows: Query<&Window, With<PrimaryWindow>>,
    query: Query<Entity, With<Window>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
) {
    println!("213");
    // let (instances, cube_width, cube_height, cube_depth) =
    //     // load_pcd_file("./assets/hak_big/hak_ascii.pcd");
    //     load_pcd_file("./assets/office_ascii.pcd");
    // let instances: Vec<InstanceData> = instances.into_iter().collect();

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

    let empty_instance = InstanceData {
            pos_scale: [0., 0., 0., 0.],
            color: [0., 0., 0., 0.]
        };
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0., 0., 0.))),
        InstanceMaterialData { instances: vec![empty_instance] },
    //     // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
    //     // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
    //     // instanced cubes will be culled.
    //     // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
    //     // instancing, and that is not taken into account with the built-in frustum culling.
    //     // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
    //     // component to avoid incorrect culling.
        NoFrustumCulling,
    ));
    

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.0, // Increase this to wash out shadows
        affects_lightmapped_meshes: false,
    });

    let size = Extent3d {
        width: 1024,
        height: 1024,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    
    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
    egui_user_textures.add_image(image_handle.clone());
    commands.insert_resource(RenderImage(image_handle.clone()));

    // TODO: WTF This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(0);

    // let pan_orbit_id = commands
    //     .spawn((
    //         Camera {
    //             // render before the "main pass" camera
    //             clear_color: ClearColorConfig::Custom(Color::srgba(1.0, 1.0, 1.0, 0.0)),
    //             order: -1,
    //             target: RenderTarget::Image(ImageRenderTarget::from(image_handle.clone())),
    //             ..default()
    //         },
    //         Transform::from_translation(Vec3::new(80000.54, 80000.27, 80010.28908))
    //             .looking_at(Vec3::ZERO, Vec3::Z),
    //         PanOrbitCamera::default(),
    //         first_pass_layer,
    //     ))
    //     .id();
    
    let camera_entity = commands.spawn((
        Camera3d::default() ,
        // Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Transform::from_translation(Vec3::new(0.54, 0.27, 0.28908)),
        OrbitCameraController {
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Left,
            is_enabled: false,
            ..default()
        },
        Camera {
            // render before the "main pass" camera
            clear_color: ClearColorConfig::Custom(Color::srgba(1.0, 1.0, 1.0, 0.0)),
            order: -1,
            target: RenderTarget::Image(ImageRenderTarget::from(image_handle.clone())),
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
            is_enabled: true,
            ..default()
        },
        first_pass_layer
    )).id();

    
    // Set up manual override of PanOrbitCamera. Note that this must run after PanOrbitCameraPlugin
    // is added, otherwise ActiveCameraData will be overwritten.
    // Note: you probably want to update the `viewport_size` and `window_size` whenever they change,
    // I haven't done this here for simplicity.
    let primary_window = windows
        .single()
        .expect("There is only ever one primary window");
    let window_entity = query.single().expect("There is only ever one primary window");
    active_cam.set_if_neq(ActiveCameraData {
        // Set the entity to the entity ID of the camera you want to control. In this case, it's
        // the inner (first pass) cube that is rendered to the texture/image.
        entity: Some(camera_entity),
        // What you set these values to will depend on your use case, but generally you want the
        // viewport size to match the size of the render target (image, viewport), and the window
        // size to match the size of the window that you are interacting with.
        viewport_size: Some(Vec2::new(size.width as f32, size.height as f32)),
        window_size: Some(Vec2::new(primary_window.width(), primary_window.height())),
        // Setting manual to true ensures PanOrbitCameraPlugin will not overwrite this resource
        manual: true,
        window_entity: None
    });

}

fn set_enable_camera_controls_system(
    cam_input: Res<CameraInputAllowed>,
    mut pan_orbit_query: Query<&mut FlyCameraController>,
) {
    for mut pan_orbit in pan_orbit_query.iter_mut() {
        pan_orbit.is_enabled = cam_input.0;
    }
}


pub fn update_gui(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut InstanceMaterialData, &mut Mesh3d)>,
    cube_preview_image: Res<RenderImage>,
    mut contexts: EguiContexts,
    mut point_size: ResMut<PointSize>,
    mut cam_input: ResMut<CameraInputAllowed>,
    mut pcd_file_info: ResMut<PCDFileInfo>
) {
    let cube_preview_texture_id = contexts.image_id(&cube_preview_image).unwrap();

    let ctx = contexts.ctx_mut();

    let width = 600.0;
    let height = 700.0;

    egui::CentralPanel::default().show(ctx, |ui| {
        if pcd_file_info.path.is_empty() {
            start_menu(
                ui, 
                &mut pcd_file_info,
                &mut meshes,
                &mut query,
            );
        } else {
            show_plot(
                &mut meshes,
                &cube_preview_texture_id,
                width,
                height,
                ui,
                &mut query,
                &mut point_size,
                &mut cam_input,
                &mut pcd_file_info
            )
            }
        });
}

fn start_menu(
    ui: &mut Ui,
    pcd_file_info: &mut ResMut<PCDFileInfo>,
    meshes: &mut ResMut<Assets<Mesh>>,
    query: &mut Query<(&mut InstanceMaterialData, &mut Mesh3d)>,
) {
    ui.label("Выбери нужный вариант");
    ui.horizontal(|ui| {
        if ui.button("Auto").clicked() {
            println!("Too early");
        }
        if ui.button("Editor").clicked() {
            println!("Let's go");
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                let test = Some(path.display().to_string());
                pcd_file_info.path = test.unwrap();
                pcd_file_info.areas = read_and_process_pcd_file(&pcd_file_info.path);
                pcd_file_info.area_len = pcd_file_info.areas.len() as u32;
                    // println!("{:?}", areas.0);
                    // println!("{:?}", test);
                    //     let (instances, cube_width, cube_height, cube_depth) =
                    //     // load_pcd_file("./assets/hak_big/hak_ascii.pcd");
                    //     load_pcd_file(&test.unwrap());
                visualize_area(pcd_file_info, meshes, query);
            }
        }
    });
}

fn show_plot(
    meshes: &mut ResMut<Assets<Mesh>>,
    cube_preview_texture_id: &epaint::TextureId,
    width: f32,
    mut height: f32,
    ui: &mut Ui,
    query: &mut Query<(&mut InstanceMaterialData, &mut Mesh3d)>,
    point_size: &mut ResMut<PointSize>,
    cam_input: &mut ResMut<CameraInputAllowed>,
    mut pcd_file_info: &mut ResMut<PCDFileInfo>
) {
    // make space for opacity slider
    height -= 100.0;
    let available_size = egui::vec2(width.min(height), width.min(height));

    // let (instances, cube_width, cube_height, cube_depth) = load_pcd_file("./assets/office_ascii.pcd");
    // let (instances, cube_width, cube_height, cube_depth) = generate_dummy_data();
    // let new_mesh = meshes.add(Cuboid::new(cube_width, cube_height, cube_depth));

    // if pcd_file_path.0.is_empty() {
    //     ui.label("Выбери нужный вариант");
    //     ui.horizontal(|ui| {
    //         if ui.button("Auto").clicked() {
    //             println!("Too early");
    //             println!("{}", pcd_file_path.0);
    //         }
    //         if ui.button("Editor").clicked() {
    //             println!("Let's go");
    //             if let Some(path) = rfd::FileDialog::new().pick_file() {
    //                     let test = Some(path.display().to_string());
    //                     pcd_file_path.0 = test.unwrap();
    //                     // println!("{:?}", test);
    //                     //     let (instances, cube_width, cube_height, cube_depth) =
    //                     //     // load_pcd_file("./assets/hak_big/hak_ascii.pcd");
    //                     //     load_pcd_file(&test.unwrap());
    //                 }
    //         }
    //     });
    // } else {
    // }
    ui.label("text");
    
    ui.vertical(|ui| {
        ui.label("3D Voxel Plot");

        // this is used to only pan / zoom when you are actually clicking inside the texture and not around
        ui.allocate_ui(available_size, |ui| {
            ui.image(egui::load::SizedTexture::new(
                *cube_preview_texture_id,
                available_size,
            ));

            let rect = ui.max_rect();

            let response = ui.interact(
                rect,
                egui::Id::new("sense"),
                egui::Sense::drag() | egui::Sense::hover(),
            );

            if response.dragged() || response.hovered() {
                cam_input.0 = true;
            } else {
                cam_input.0 = false;
            }
        });

        // a simple slider to control the opacity threshold
        // ui.label("Opacity:");
        ui.label(format!("{}/{}", pcd_file_info.area_num, pcd_file_info.area_len));
        ui.horizontal(|ui| {
            if ui.button("<=").clicked() {
                // println!("haha")
                set_new_area(pcd_file_info.area_num - 1, pcd_file_info);
                visualize_area(pcd_file_info, meshes, query);
            }
            if ui.button("=>").clicked() {
                set_new_area(pcd_file_info.area_num + 1, pcd_file_info);
                visualize_area(pcd_file_info, meshes, query);
            }
        });

        if ui
            .add(egui::Slider::new(&mut point_size.0, 0.01..=100.0).text("Point Size"))
            .changed()
        {
            // if let Ok((mut instance_data, mut mesh3d)) = query.single_mut() {
            //     instance_data.instances = instances;
            //     mesh3d.0 = new_mesh;
            //     instance_data
            //         .instances
            //         .retain(|instance| 1 >= 0);
            // }
            println!("123");
        }
        // if let Ok((mut instance_data, mut mesh3d)) = query.single_mut() {
        //         instance_data.instances = instances;
        //         mesh3d.0 = new_mesh;
        //         instance_data
        //             .instances.retain(|instance| 1 > 0);
        //             // .retain(|instance| instance.color[3] >= opacity_threshold.0);
        //     }
    });
}

fn set_new_area(n: u32, mut pcd_file_info: &mut ResMut<PCDFileInfo>) {
    if n == 0 {
        pcd_file_info.area_num = pcd_file_info.area_len;
    } else if n <= pcd_file_info.area_len {
        pcd_file_info.area_num = n;
    } else {
        pcd_file_info.area_num = 1;
    }
}

fn visualize_area(
    mut pcd_file_info: &mut ResMut<PCDFileInfo>,
    meshes: &mut ResMut<Assets<Mesh>>,
    query: &mut Query<(&mut InstanceMaterialData,
        &mut Mesh3d)>) {
    let (instances, cube_width, cube_height, cube_depth) = load_pcd_file(&pcd_file_info.areas[pcd_file_info.area_num as usize - 1]);
    let new_mesh = meshes.add(Cuboid::new(cube_width, cube_height, cube_depth));
    if let Ok((mut instance_data, mut mesh3d)) = query.single_mut() {
        instance_data.instances = instances;
        mesh3d.0 = new_mesh;
        // println!("213qqq")
    }
}

fn load_pcd_file(area: &Vec<MyPoint>) -> (Vec<InstanceData>, f32, f32, f32) {
    // let areas = read_and_process_pcd_file(path);
    // let areas = get_file_areas(&pcd_data);

    let mut instances = Vec::new();

    // let points = get_area_points(pcd_data, &areas[0]);
    // let points = &areas[0];
    println!("{:?}", get_area_center(&area));
    println!("{:?}", &area[0]);
    // println!("{:?}", points.len());
    // for point in pcd_data.points {
    for point in area {
        let instance = InstanceData {
            pos_scale: [point.x, point.y, point.z, 1.3],
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

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin {
                enable_multipass_for_primary_context: false,
            },
            VoxelMaterialPlugin,
            // PanOrbitCameraPlugin,
            BlendyCamerasPlugin,
        ))
        .insert_resource(PointSize(1.0)) // Start with no threshold
        .insert_resource(CameraInputAllowed(false))
        // .insert_resource(PCDFilePath(String::new()))
        // .insert_resource(Areas(Vec::new()))
        .insert_resource(PCDFileInfo::default())
        .add_systems(Startup, voxel_plot_setup)
        .add_systems(Update, update_gui)
        .add_systems(Update, set_enable_camera_controls_system)
        .run();
}