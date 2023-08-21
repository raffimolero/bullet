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
        .insert_resource(Controlled::default())
        .insert_resource(GizmoConfig {
            line_width: 4.0,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                close_on_esc,
                control_selection,
                control_movement,
                aabb_rainbow,
            ),
        )
        .run()
}

fn setup(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    mut controlled: ResMut<Controlled>,
) {
    let w: u32 = 256;
    let h: u32 = 256;

    let (subwindow, sublayer) = spawn_subwindow(w, h, &mut textures, &mut commands);

    let main_cam = commands.spawn(Camera2dBundle::default()).id();

    let rectangle = commands
        .spawn((
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
        ))
        .id();

    controlled.push(subwindow);
    controlled.push(rectangle);
    controlled.push(main_cam);
    controlled.attach_aabb(&mut commands);
}

#[derive(Resource, Default)]
struct Controlled {
    entities: Vec<Entity>,
    selected: usize,
}

impl Controlled {
    fn select<'a>(
        &self,
        controllables: &'a mut Query<&mut Transform>,
    ) -> Option<Mut<'a, Transform>> {
        self.entities
            .get(self.selected)
            .and_then(|entity| controllables.get_mut(*entity).ok())
    }

    fn detach_aabb(&self, commands: &mut Commands) {
        let Some(&entity) = self.entities.get(self.selected) else {
            return;
        };
        commands.entity(entity).remove::<AabbGizmo>();
    }

    fn attach_aabb(&self, commands: &mut Commands) {
        let Some(&entity) = self.entities.get(self.selected) else {
            return;
        };
        commands.entity(entity).insert(AabbGizmo::default());
    }

    fn select_next(&mut self, commands: &mut Commands) {
        self.detach_aabb(commands);
        self.selected += 1;
        if self.selected >= self.entities.len() {
            self.selected = 0;
        }
        self.attach_aabb(commands);
    }

    fn select_prev(&mut self, commands: &mut Commands) {
        self.detach_aabb(commands);
        if self.selected == 0 {
            self.selected = self.entities.len();
        }
        self.selected -= 1;
        self.attach_aabb(commands);
    }

    fn push(&mut self, entity: Entity) {
        self.entities.push(entity)
    }
}

fn aabb_rainbow(time: Res<Time>, mut config: ResMut<GizmoConfig>) {
    match &mut config.aabb.default_color {
        None => {
            *(&mut config.aabb.default_color) = Some(Color::Hsla {
                hue: 0.0,
                saturation: 1.0,
                lightness: 0.5,
                alpha: 0.75,
            })
        }
        Some(Color::Hsla { hue, .. }) => {
            *hue += time.delta_seconds() * 60.0;
            *hue %= 360.0;
        }
        Some(col) => {
            *col = col.as_hsla();
        }
    }
}

fn control_selection(
    keys: Res<Input<KeyCode>>,
    mut controlled: ResMut<Controlled>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::O) {
        controlled.select_next(&mut commands);
    }
    if keys.just_pressed(KeyCode::U) {
        controlled.select_prev(&mut commands);
    }
}

fn control_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    controlled: Res<Controlled>,
    mut controllables: Query<&mut Transform>,
) {
    let delta = time.delta_seconds();
    let speed = 300.0;
    let rot = TAU / 4.0;
    let scaling = 4_f32;

    let mut control_tf = Transform::IDENTITY;
    if keys.pressed(KeyCode::W) {
        control_tf.translation.y += speed * delta;
    }
    if keys.pressed(KeyCode::S) {
        control_tf.translation.y -= speed * delta;
    }
    if keys.pressed(KeyCode::A) {
        control_tf.translation.x -= speed * delta;
    }
    if keys.pressed(KeyCode::D) {
        control_tf.translation.x += speed * delta;
    }
    if keys.pressed(KeyCode::Q) {
        control_tf.rotation *= Quat::from_rotation_z(rot * delta);
    }
    if keys.pressed(KeyCode::E) {
        control_tf.rotation *= Quat::from_rotation_z(-rot * delta);
    }
    if keys.pressed(KeyCode::I) {
        control_tf.scale.y *= scaling.powf(delta);
    }
    if keys.pressed(KeyCode::K) {
        control_tf.scale.y *= scaling.powf(-delta);
    }
    if keys.pressed(KeyCode::L) {
        control_tf.scale.x *= scaling.powf(delta);
    }
    if keys.pressed(KeyCode::J) {
        control_tf.scale.x *= scaling.powf(-delta);
    }

    if let Some(mut entity_tf) = controlled.select(&mut controllables) {
        *entity_tf = *entity_tf * control_tf;
    }
}

fn spawn_subwindow(
    w: u32,
    h: u32,
    textures: &mut ResMut<Assets<Image>>,
    commands: &mut Commands,
) -> (Entity, RenderLayers) {
    // TODO: auto generate this based on available layers
    let sublayer = RenderLayers::layer(1);

    let img = new_image(w, h);
    let image_handle = textures.add(img);

    let container = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ALICE_BLUE,
                custom_size: Some(Vec2::new(w as f32 * 1.25, h as f32 * 1.25)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Camera2dBundle {
                    camera: Camera {
                        target: RenderTarget::Image(image_handle.clone()),
                        order: -1,
                        ..default()
                    },
                    ..default()
                },
                sublayer,
            ));
            parent.spawn(SpriteBundle {
                texture: image_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });
        })
        .id();

    (container, sublayer)
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
