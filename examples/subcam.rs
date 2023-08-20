use std::f32::consts::TAU;

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{BevyDefault, ImageSampler, TextureFormatPixelInfo, Volume},
        view::RenderLayers,
    },
    window::close_on_esc,
};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_esc, input))
        .run()
}

#[derive(Component)]
enum Controls {
    Esdf,
    Ijkl,
}

fn setup(mut commands: Commands, mut textures: ResMut<Assets<Image>>) {
    let w: u32 = 300;
    let h: u32 = 200;

    let sublayer = RenderLayers::layer(1);

    let img = new_image(w, h);
    let image_handle = textures.add(img);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        sublayer,
    ));

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ALICE_BLUE,
                custom_size: Some(Vec2::new(w as f32, h as f32)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        sublayer,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(w as f32, h as f32)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        sublayer,
        Controls::Esdf,
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_handle.clone(),
            ..default()
        },
        Controls::Ijkl,
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            transform: Transform::from_xyz((w * 2) as f32, 0.0, 0.0),
            ..default()
        },
        Controls::Esdf,
    ));
}

fn input(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut controllables: Query<(&mut Transform, &Controls)>,
) {
    let delta = time.delta_seconds();
    let speed = 400.0;
    let rot = TAU / 2.0;
    let scaling = 4_f32;

    let mut esdf = Transform::IDENTITY;
    if keys.pressed(KeyCode::E) {
        esdf.translation.y += speed * delta;
    }
    if keys.pressed(KeyCode::S) {
        esdf.translation.x -= speed * delta;
    }
    if keys.pressed(KeyCode::D) {
        esdf.translation.y -= speed * delta;
    }
    if keys.pressed(KeyCode::F) {
        esdf.translation.x += speed * delta;
    }
    if keys.pressed(KeyCode::W) {
        esdf.rotation *= Quat::from_rotation_z(rot * delta);
    }
    if keys.pressed(KeyCode::R) {
        esdf.rotation *= Quat::from_rotation_z(-rot * delta);
    }
    if keys.pressed(KeyCode::Q) {
        esdf.scale *= scaling.powf(delta);
    }
    if keys.pressed(KeyCode::A) {
        esdf.scale *= scaling.powf(-delta);
    }

    let mut ijkl = Transform::IDENTITY;
    if keys.pressed(KeyCode::I) {
        ijkl.translation.y += speed * delta;
    }
    if keys.pressed(KeyCode::J) {
        ijkl.translation.x -= speed * delta;
    }
    if keys.pressed(KeyCode::K) {
        ijkl.translation.y -= speed * delta;
    }
    if keys.pressed(KeyCode::L) {
        ijkl.translation.x += speed * delta;
    }
    if keys.pressed(KeyCode::U) {
        ijkl.rotation *= Quat::from_rotation_z(rot * delta);
    }
    if keys.pressed(KeyCode::O) {
        ijkl.rotation *= Quat::from_rotation_z(-rot * delta);
    }
    if keys.pressed(KeyCode::P) {
        ijkl.scale *= scaling.powf(delta);
    }
    if keys.pressed(KeyCode::Semicolon) {
        ijkl.scale *= scaling.powf(-delta);
    }

    controllables.for_each_mut(|(mut transform, controls)| {
        *transform = match controls {
            Controls::Esdf => esdf,
            Controls::Ijkl => ijkl,
        } * *transform;
    });
}

fn new_image(w: u32, h: u32) -> Image {
    // inlined from Image::new
    let size = Extent3d {
        width: w,
        height: h,
        ..default()
    };
    let format = TextureFormat::bevy_default();
    let data = vec![0; w as usize * h as usize * format.pixel_size()];
    debug_assert_eq!(
        size.volume() * format.pixel_size(),
        data.len(),
        "Pixel data, size and format have to match",
    );
    Image {
        data,
        texture_descriptor: TextureDescriptor {
            size,
            format,
            dimension: TextureDimension::D2,
            label: None,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler_descriptor: ImageSampler::Default,
        texture_view_descriptor: None,
    }
}
