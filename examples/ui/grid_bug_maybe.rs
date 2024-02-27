//! minimal repro for potential grid layout bug
use bevy::prelude::*;

const SIDE: f32 = 720.0; // try changing this number too, it's the resulting cell size that's the issue ...
const SIZE: usize = 13; // or 26, 31, 51, 57, 58, but other stuff is fine ...
const CELL_SIZE: f32 = SIDE / SIZE as f32;

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
                            CELL_SIZE,
                        ),
                        width: Val::Px(SIDE),
                        height: Val::Px(SIDE),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    for i in 0..SIZE {
                        for j in 0..SIZE {
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
                width: Val::Px(CELL_SIZE),
                height: Val::Px(CELL_SIZE),
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
                            font_size: 14.,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}
