use bevy::{input::system::exit_on_esc_system, prelude::*};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(210.0)),
                    margin: Rect::all(Val::Auto),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: Color::rgb(0.65, 0.65, 0.65).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Bevy UI Tests",
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            color: Color::rgb(0.15, 0.15, 0.15),
                        },
                        TextAlignment {
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                });
            });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Button",
                            TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                ..Default::default()
                            },
                        ),
                        ..Default::default()
                    });
                });
        });
}

fn btn_system(
    mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), Changed<Interaction>>,
) {
    for (interaction, mut color, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system(btn_system)
        .add_system(exit_on_esc_system)
        .run();
}
