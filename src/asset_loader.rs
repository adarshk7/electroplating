use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct AudioAssets {
    pub blip3: Handle<AudioSource>,
    pub blip5: Handle<AudioSource>,
    pub blip8: Handle<AudioSource>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut audio_assets: ResMut<AudioAssets>, asset_server: Res<AssetServer>) {
    *audio_assets = AudioAssets {
        blip3: asset_server.load("sound/blip3.wav"),
        blip5: asset_server.load("sound/blip5.wav"),
        blip8: asset_server.load("sound/blip8.wav"),
    }
}
