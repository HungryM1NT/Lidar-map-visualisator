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
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_voxel_plot::{InstanceData, InstanceMaterialData, VoxelMaterialPlugin};

fn main() {
    App::new()
        // .add_plugins((
        //     DefaultPlugins,
        //     EguiPlugin {
        //         enable_multipass_for_primary_context: false,
        //     },
        //     VoxelMaterialPlugin,
        //     PanOrbitCameraPlugin,
        // ))
        // .insert_resource(OpacityThreshold(0.0)) // Start with no threshold
        // .insert_resource(CameraInputAllowed(false))
        // .add_systems(Startup, voxel_plot_setup)
        // .add_systems(Update, update_gui)
        // .add_systems(Update, set_enable_camera_controls_system)
        .run();
}