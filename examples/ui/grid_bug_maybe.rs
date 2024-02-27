//! minimal repro for potential grid layout bug
use bevy::prelude::*;

const DEFAULT_SIDE: f32 = 720.;  // try changing this number too, it's the resulting cell size that's the issue ...
const SIDE: Option<&str> = option_env!("SIDE");
const DEFAULT_SIZE: usize = 13;  // or 26, 31, 51, 57, 58, but other stuff is fine ...
const SIZE: Option<&str> = option_env!("SIZE");

fn side() -> f32 {
    SIDE.and_then(|side| side.parse().ok()).unwrap_or(DEFAULT_SIDE)
}

fn size() -> usize {
    SIZE.and_then(|size| size.parse().ok()).unwrap_or(DEFAULT_SIZE)
}

fn cell_size() -> f32 {
    side() / size() as f32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [1000., 1000.].into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_layout)
        .run();
}

fn spawn_layout(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::px(
                            GridTrackRepetition::AutoFill,
                            cell_size(),
                        ),
                        width: Val::Px(side()),
                        height: Val::Px(side()),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    for i in 0..size() {
                        for j in 0..size() {
                            item_rect(builder, i, j);
                        }
                    }
                });
        });
}

fn item_rect(builder: &mut ChildBuilder, i: usize, j: usize) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(1.)),
                width: Val::Px(cell_size()),
                height: Val::Px(cell_size()),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::RED),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        format!("{},{}", i, j),
                        TextStyle {
                            font_size: 14. * 14. / size() as f32,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}
