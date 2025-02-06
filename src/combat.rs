use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_text, spawn_nine_slice, AsciiSheet, NineSlice, NineSliceIndices},
    fadeout::create_fadeout,
    graphics::{spawn_enemy_sprite, CharacterSheet},
    player::Player,
    GameState, RESOLUTION, TILE_SIZE,
};

#[derive(Component)]
pub struct CombatStats {
    //XXX does this need isize, combat does a subtract but I max it
    pub health: isize,
    pub max_health: isize,
    pub attack: isize,
    pub defense: isize,
}

#[derive(Clone, Copy)]
pub enum EnemyType {
    Bat,
    Ghost,
}

#[derive(Component)]
pub struct Enemy {
    enemy_type: EnemyType,
}

#[derive(Event)]
pub struct FightEvent {
    target: Entity,
    damage_amount: isize,
    next_state: CombatState,
}

pub struct CombatPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, States, Default)]
pub enum CombatState {
    #[default]
    PlayerTurn,
    PlayerAttack,
    EnemyTurn(bool),
    EnemyAttack,
    Reward,
    Exiting,
}

#[derive(Resource)]
pub struct AttackEffects {
    timer: Timer,
    flash_speed: f32,
    screen_shake_amount: f32,
    current_shake: f32,
}

#[derive(Component)]
pub struct CombatText;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FightEvent>()
            .init_state::<CombatState>()
            .insert_resource(AttackEffects {
                timer: Timer::from_seconds(0.7, TimerMode::Repeating),
                flash_speed: 0.1,
                screen_shake_amount: 0.1,
                current_shake: 0.0,
            })
            .insert_resource(CombatMenuSelection {
                selected: CombatMenuOption::Fight,
            })
            .add_systems(
                Update,
                process_enemy_turn.run_if(in_state(CombatState::EnemyTurn(false))),
            )
            .add_systems(
                Update,
                (
                    combat_input,
                    combat_camera,
                    highlight_combat_buttons,
                    combat_damage_calc,
                )
                    .run_if(in_state(GameState::Combat)),
            )
            .add_systems(
                OnEnter(GameState::Combat),
                (
                    set_starting_state,
                    spawn_enemy,
                    spawn_player_health,
                    spawn_combat_menu,
                ),
            )
            .add_systems(
                OnExit(GameState::Combat),
                (despawn_all_combat_text, despawn_enemy, despawn_menu),
            )
            .add_systems(
                Update,
                handle_attack_effects.run_if(in_state(CombatState::PlayerAttack)),
            )
            .add_systems(OnEnter(CombatState::Reward), (give_reward, despawn_enemy))
            .add_systems(
                Update,
                handle_accepting_reward.run_if(in_state(CombatState::Reward)),
            )
            .add_systems(
                Update,
                handle_attack_effects.run_if(in_state(CombatState::EnemyAttack)),
            );
    }
}

fn handle_accepting_reward(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut combat_state: ResMut<NextState<CombatState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        combat_state.set(CombatState::Exiting);
        create_fadeout(&mut commands, GameState::Overworld, &ascii);
    }
}

fn give_reward(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    mut player_query: Query<(&mut Player, &mut CombatStats)>,
    enemy_query: Query<&Enemy>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    keyboard.clear();
    let exp_reward = match enemy_query.single().enemy_type {
        EnemyType::Bat => 10,
        EnemyType::Ghost => 30,
    };
    let reward_text = format!("Earned: {} exp", exp_reward);
    let text = spawn_ascii_text(
        &mut commands,
        &ascii,
        &reward_text,
        Vec3::new(-((reward_text.len() / 2) as f32 * TILE_SIZE), 0.0, 0.0),
    );
    commands.entity(text).insert(CombatText);
    let (mut player, mut stats) = player_query.single_mut();
    if player.give_exp(exp_reward, &mut stats) {
        let level_text = "Level up!";
        let text = spawn_ascii_text(
            &mut commands,
            &ascii,
            level_text,
            Vec3::new(
                -((level_text.len() / 2) as f32 * TILE_SIZE),
                -1.5 * TILE_SIZE,
                0.0,
            ),
        );
        commands.entity(text).insert(CombatText);
    }
}

fn despawn_all_combat_text(mut commands: Commands, text_query: Query<Entity, With<CombatText>>) {
    for entity in text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_player_health(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    player_query: Query<(Entity, &CombatStats, &Transform), With<Player>>,
) {
    let (player, stats, transform) = player_query.single();
    let health_text = format!("Health: {}", stats.health);
    let text = spawn_ascii_text(
        &mut commands,
        &ascii,
        &health_text,
        Vec3::new(-RESOLUTION + TILE_SIZE, -1.0 + TILE_SIZE, 0.0) - transform.translation,
    );
    commands.entity(text).insert(CombatText);
    commands.entity(player).add_child(text);
}

fn handle_attack_effects(
    mut attack_fx: ResMut<AttackEffects>,
    time: Res<Time>,
    mut enemy_graphics_query: Query<&mut Visibility, With<Enemy>>,
    state: ResMut<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    attack_fx.timer.tick(time.delta());
    let mut enemy_sprite = enemy_graphics_query.iter_mut().next().unwrap();

    if state.get() == &CombatState::PlayerAttack {
        if attack_fx.timer.elapsed_secs() % attack_fx.flash_speed > attack_fx.flash_speed / 2.0 {
            *enemy_sprite = Visibility::Hidden;
        } else {
            *enemy_sprite = Visibility::Visible;
        }
    } else {
        attack_fx.current_shake = attack_fx.screen_shake_amount
            * f32::sin(attack_fx.timer.fraction() * 2.0 * std::f32::consts::PI);
    }

    if attack_fx.timer.just_finished() {
        *enemy_sprite = Visibility::Visible;
        if state.get() == &CombatState::PlayerAttack {
            next_state.set(CombatState::EnemyTurn(false));
        } else {
            next_state.set(CombatState::PlayerTurn);
        }
    }
}

fn set_starting_state(mut state: ResMut<NextState<CombatState>>) {
    state.set(CombatState::PlayerTurn);
}

const NUM_MENU_OPTIONS: isize = 2;
#[derive(Component, PartialEq, Clone, Copy)]
pub enum CombatMenuOption {
    Fight,
    Run,
}

#[derive(Resource)]
pub struct CombatMenuSelection {
    selected: CombatMenuOption,
}

fn process_enemy_turn(
    mut fight_event: EventWriter<FightEvent>,
    mut combat_state: ResMut<NextState<CombatState>>,
    enemy_query: Query<&CombatStats, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
) {
    let player_ent = player_query.single();
    let enemy_stats = enemy_query.iter().next().unwrap();
    fight_event.send(FightEvent {
        target: player_ent,
        damage_amount: enemy_stats.attack,
        next_state: CombatState::EnemyAttack,
    });
    combat_state.set(CombatState::EnemyTurn(true));
}

fn despawn_menu(mut commands: Commands, button_query: Query<Entity, With<CombatMenuOption>>) {
    for button in button_query.iter() {
        commands.entity(button).despawn_recursive();
    }
}

fn highlight_combat_buttons(
    menu_state: Res<CombatMenuSelection>,
    button_query: Query<(&Children, &CombatMenuOption)>,
    nine_slice_query: Query<&Children, With<NineSlice>>,
    mut sprites_query: Query<&mut Sprite>,
) {
    for (button_children, button_id) in button_query.iter() {
        for button_child in button_children.iter() {
            if let Ok(nine_slice_children) = nine_slice_query.get(*button_child) {
                for nine_slice_child in nine_slice_children.iter() {
                    if let Ok(mut sprite) = sprites_query.get_mut(*nine_slice_child) {
                        if menu_state.selected == *button_id {
                            sprite.color = bevy::color::palettes::css::RED.into();
                        } else {
                            sprite.color = Color::WHITE;
                        }
                    }
                }
            }
        }
    }
}

fn spawn_combat_button(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    indices: &NineSliceIndices,
    translation: Vec3,
    text: &str,
    id: CombatMenuOption,
    size: Vec2,
) -> Entity {
    let fight_nine_slice = spawn_nine_slice(commands, ascii, indices, size.x, size.y);

    let x_offset = (-size.x / 2.0 + 1.5) * TILE_SIZE;
    let fight_text = spawn_ascii_text(commands, ascii, text, Vec3::new(x_offset, 0.0, 0.0));

    commands
        .spawn((
            Transform::from_translation(translation),
            GlobalTransform::default(),
            Name::new("Button"),
            id,
            Visibility::default(),
        ))
        .add_child(fight_text)
        .add_child(fight_nine_slice)
        .id()
}

fn spawn_combat_menu(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    nine_slice_indices: Res<NineSliceIndices>,
) {
    let box_height = 3.0;
    let box_center_y = -1.0 + box_height * TILE_SIZE / 2.0;

    let run_text = "Run";
    let run_width = (run_text.len() + 2) as f32;
    let run_center_x = RESOLUTION - (run_width * TILE_SIZE) / 2.0;

    spawn_combat_button(
        &mut commands,
        &ascii,
        &nine_slice_indices,
        Vec3::new(run_center_x, box_center_y, 100.0),
        run_text,
        CombatMenuOption::Run,
        Vec2::new(run_width, box_height),
    );

    let fight_text = "Fight";
    let fight_width = (fight_text.len() + 2) as f32;
    let fight_center_x = RESOLUTION - (run_width * TILE_SIZE) - (fight_width * TILE_SIZE / 2.0);

    spawn_combat_button(
        &mut commands,
        &ascii,
        &nine_slice_indices,
        Vec3::new(fight_center_x, box_center_y, 100.0),
        fight_text,
        CombatMenuOption::Fight,
        Vec2::new(fight_width, box_height),
    );
}

fn combat_damage_calc(
    mut commands: Commands,
    mut fight_event: EventReader<FightEvent>,
    //Not necssacarily enemy
    mut enemy_query: Query<(&Children, &mut CombatStats)>,
    ascii: Res<AsciiSheet>,
    text_query: Query<&Transform, With<CombatText>>,
    mut combat_state: ResMut<NextState<CombatState>>,
) {
    if let Some(fight_event) = fight_event.read().next() {
        //Get target stats and children
        let (target_children, mut stats) = enemy_query
            .get_mut(fight_event.target)
            .expect("Fighting enemy without stats");

        //Damage calc
        stats.health = std::cmp::max(
            stats.health - (fight_event.damage_amount - stats.defense),
            0,
        );

        //Update health
        for child in target_children.iter() {
            //See if this child is the health text
            if let Ok(transform) = text_query.get(*child) {
                //Delete old text
                commands.entity(*child).despawn_recursive();
                //Create new text
                let new_health = spawn_ascii_text(
                    &mut commands,
                    &ascii,
                    &format!("Health: {}", stats.health as usize),
                    //relative to enemy pos
                    transform.translation,
                );
                commands.entity(new_health).insert(CombatText);
                commands.entity(fight_event.target).add_child(new_health);
            }
        }

        //Kill enemy if dead
        //TODO support multiple enemies
        if stats.health == 0 {
            combat_state.set(CombatState::Reward);
        } else {
            combat_state.set(fight_event.next_state);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn combat_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&CombatStats, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut fight_event: EventWriter<FightEvent>,
    mut menu_state: ResMut<CombatMenuSelection>,
    combat_state: ResMut<State<CombatState>>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    ascii: Res<AsciiSheet>,
) {
    if combat_state.get() != &CombatState::PlayerTurn {
        return;
    }

    let player_combat = player_query.single();

    //TODO handle multiple enemies
    let enemy = enemy_query.single();
    let mut new_selection = menu_state.selected as isize;
    if keyboard.just_pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        new_selection -= 1;
    }
    if keyboard.just_pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        new_selection += 1;
    }
    new_selection = (new_selection + NUM_MENU_OPTIONS) % NUM_MENU_OPTIONS;

    menu_state.selected = match new_selection {
        0 => CombatMenuOption::Fight,
        1 => CombatMenuOption::Run,
        _ => unreachable!("Bad menu selection"),
    };

    if keyboard.just_pressed(KeyCode::Space) {
        match menu_state.selected {
            CombatMenuOption::Fight => {
                fight_event.send(FightEvent {
                    //TODO select enemy and attack type
                    target: enemy,
                    damage_amount: player_combat.attack,
                    next_state: CombatState::PlayerAttack,
                });
            }
            CombatMenuOption::Run => {
                create_fadeout(&mut commands, GameState::Overworld, &ascii);
                next_combat_state.set(CombatState::Exiting);
            }
        }
    }
}

fn combat_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    attack_fx: Res<AttackEffects>,
) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = attack_fx.current_shake;
    camera_transform.translation.y = 0.0;
}

fn spawn_enemy(mut commands: Commands, ascii: Res<AsciiSheet>, characters: Res<CharacterSheet>) {
    let enemy_type = match rand::random::<f32>() {
        x if x < 0.5 => EnemyType::Bat,
        _ => EnemyType::Ghost,
    };
    let stats = match enemy_type {
        EnemyType::Bat => CombatStats {
            health: 3,
            max_health: 3,
            attack: 2,
            defense: 1,
        },
        EnemyType::Ghost => CombatStats {
            health: 5,
            max_health: 5,
            attack: 3,
            defense: 2,
        },
    };
    let health_text = spawn_ascii_text(
        &mut commands,
        &ascii,
        &format!("Health: {}", stats.health as usize),
        //relative to enemy pos
        Vec3::new(-4.5 * TILE_SIZE, 0.5, 100.0),
    );
    commands.entity(health_text).insert(CombatText);
    let sprite = spawn_enemy_sprite(
        &mut commands,
        &characters,
        Vec3::new(0.0, 0.3, 100.0),
        enemy_type,
    );
    commands
        .entity(sprite)
        .insert(Enemy { enemy_type })
        .insert(stats)
        .insert(Name::new("Bat"))
        .add_child(health_text);
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
