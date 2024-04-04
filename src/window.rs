use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResized;

pub const WINDOW_WORLD_HEIGHT: f32 = 720.0;
pub const WINDOW_USABLE_WORLD_WIDTH: f32 = 1280.0;
const MIN_WINDOW_WIDTH_TO_HEIGHT: f32 = WINDOW_USABLE_WORLD_WIDTH / WINDOW_WORLD_HEIGHT;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0., 0.2, 0.)))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, guard_resolution);
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scaling_mode: ScalingMode::FixedVertical(WINDOW_WORLD_HEIGHT),
            ..default()
        },
        ..default()
    });
}

fn guard_resolution(
    mut window_query: Query<&mut Window>,
    mut resize_reader: EventReader<WindowResized>,
    mut ui_scale: ResMut<UiScale>,
) {
    let mut last_window_resized_event = None;
    for e in resize_reader.read() {
        last_window_resized_event = Some(e);
    }

    if let Some(e) = last_window_resized_event {
        if e.width / e.height < MIN_WINDOW_WIDTH_TO_HEIGHT {
            let mut window = window_query.get_single_mut().unwrap();
            window
                .resolution
                .set(MIN_WINDOW_WIDTH_TO_HEIGHT * e.height, e.height);
        }
        ui_scale.0 = e.height / WINDOW_WORLD_HEIGHT;
    }
}
