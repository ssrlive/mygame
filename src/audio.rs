use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin, AudioSource};

use crate::combat::{CombatState, FightEvent};
use crate::GameState;

pub struct GameAudioPlugin;

#[derive(Resource, Default)]
pub struct AudioState {
    bgm_handle: Handle<AudioSource>,
    combat_handle: Handle<AudioSource>,
    hit_handle: Handle<AudioSource>,
    reward_handle: Handle<AudioSource>,

    // bgm_channel: AudioChannel<Background>,
    // combat_channel: AudioChannel<Background>,
    // sfx_channel: AudioChannel<Background>,
    volume: f32,
}

// #[derive(Resource)]
// struct Background;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .init_resource::<AudioState>()
            // .add_audio_channel::<Background>()
            .add_systems(PreStartup, load_audio)
            .add_systems(OnEnter(GameState::Combat), start_combat_music)
            .add_systems(OnEnter(GameState::Overworld), resume_bgm_music)
            .add_systems(OnEnter(CombatState::Reward), play_reward_sfx)
            .add_systems(Update, play_hit_sfx)
            .add_systems(Update, volume_control)
            .add_systems(Startup, start_bgm_music);
    }
}
fn play_reward_sfx(audio: Res<Audio>, audio_state: Res<AudioState>) {
    // audio.play_in_channel(audio_state.reward_handle.clone(), &audio_state.sfx_channel);
    audio.play(audio_state.reward_handle.clone());
}

fn play_hit_sfx(
    audio: Res<Audio>,
    audio_state: Res<AudioState>,
    mut fight_event: EventReader<FightEvent>,
) {
    if fight_event.read().count() > 0 {
        // audio.play_in_channel(audio_state.hit_handle.clone(), &audio_state.sfx_channel);
        audio.play(audio_state.hit_handle.clone());
    }
}

fn resume_bgm_music(audio: Res<Audio>, audio_state: Res<AudioState>) {
    // audio.stop_channel(&audio_state.combat_channel);
    // audio.resume_channel(&audio_state.bgm_channel);
    audio.stop();
    audio.play(audio_state.bgm_handle.clone()).looped();
}

fn start_combat_music(audio: Res<Audio>, audio_state: Res<AudioState>) {
    // audio.pause_channel(&audio_state.bgm_channel);
    // audio.play_looped_in_channel(
    //     audio_state.combat_handle.clone(),
    //     &audio_state.combat_channel,
    // );
    audio.stop();
    audio.play(audio_state.combat_handle.clone()).looped();
}

fn volume_control(
    keyboard: Res<ButtonInput<KeyCode>>,
    audio: Res<Audio>,
    mut audio_state: ResMut<AudioState>,
) {
    if keyboard.just_pressed(KeyCode::PageUp) {
        audio_state.volume += 0.10;
    }
    if keyboard.just_pressed(KeyCode::PageDown) {
        audio_state.volume -= 0.10;
    }
    audio_state.volume = audio_state.volume.clamp(0.0, 1.0);
    // audio.set_volume_in_channel(audio_state.volume, &audio_state.bgm_channel);
    audio.set_volume(audio_state.volume as f64);
}

fn start_bgm_music(audio: Res<Audio>, audio_state: Res<AudioState>) {
    audio.play(audio_state.bgm_handle.clone()).looped();
}

fn load_audio(mut commands: Commands, _audio: Res<Audio>, assets: Res<AssetServer>) {
    let bgm_handle = assets.load("bip-bop.ogg");
    let combat_handle = assets.load("ganxta.ogg");
    let hit_handle = assets.load("hit.wav");
    let reward_handle = assets.load("reward.wav");

    // let bgm_channel = AudioChannel::<AudioState>::new("bgm".to_string());
    // let combat_channel = AudioChannel::new("combat".to_string());
    // let sfx_channel = AudioChannel::new("sfx".to_string());
    let volume = 0.5;

    // audio.set_volume_in_channel(volume, &bgm_channel);
    // audio.set_volume_in_channel(volume, &combat_channel);
    // audio.set_volume_in_channel(volume, &sfx_channel);

    commands.insert_resource(AudioState {
        bgm_handle,
        combat_handle,
        hit_handle,
        reward_handle,
        // bgm_channel,
        // combat_channel,
        // sfx_channel,
        volume,
    });
}
