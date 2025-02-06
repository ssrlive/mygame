use bevy::{prelude::*, ui::FocusPolicy};

use crate::{ascii::AsciiSheet, fadeout::create_fadeout, GameState};

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct ButtonActive(bool);

#[derive(Resource, Default)]
struct UiAssets {
    font: Handle<Font>,
    button: Handle<Image>,
    button_pressed: Handle<Image>,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>()
            .add_systems(PreStartup, load_menu_resource)
            .add_systems(OnEnter(GameState::StartMenu), spawn_menu)
            .add_systems(OnExit(GameState::StartMenu), despawn_menu)
            .add_systems(
                Update,
                handle_start_button.run_if(in_state(GameState::StartMenu)),
            );
    }
}

fn despawn_menu(mut commands: Commands, button_query: Query<Entity, With<Button>>) {
    for ent in button_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn handle_start_button(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Children, &mut ButtonActive, &Interaction),
        Changed<Interaction>,
    >,
    mut image_query: Query<&mut ImageNode>,
    ui_assets: Res<UiAssets>,
    ascii: Res<AsciiSheet>,
) {
    for (children, mut active, interaction) in interaction_query.iter_mut() {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();

        match interaction {
            Interaction::Pressed => {
                if active.0 {
                    image.image = ui_assets.button_pressed.clone();
                    create_fadeout(&mut commands, GameState::Overworld, &ascii);
                    active.0 = false;
                }
            }
            Interaction::Hovered | Interaction::None => {
                image.image = ui_assets.button.clone();
            }
        }
    }
}

fn load_menu_resource(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = UiAssets {
        font: assets.load("QuattrocentoSans-Bold.ttf"),
        button: assets.load("button.png"),
        button_pressed: assets.load("button_pressed.png"),
    };
    commands.insert_resource(ui_assets);
}

fn spawn_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn((
            Button,
            Node {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(20.0),
                height: Val::Percent(10.0),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            BackgroundColor(Color::NONE),
            Visibility::default(),
        ))
        .insert(ButtonActive(true))
        .with_children(|parent| {
            parent
                .spawn((
                    ImageNode::from(ui_assets.button.clone()),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .insert(FocusPolicy::Pass)
                .with_children(|parent| {
                    parent.spawn((
                        Text::from("Start Game"),
                        TextFont::from_font(ui_assets.font.clone()).with_font_size(40.0),
                        TextColor::from(Color::srgb(0.9, 0.9, 0.9)),
                        FocusPolicy::Pass,
                    ));
                });
        });
}
