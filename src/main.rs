use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    GameLoad,
}

#[derive(Resource)]
struct GlobalAtlasTextureHandle(Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

fn main() -> AppExit {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        name: Some("Choreonius".into()),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::linear_rgb(
            45.0 / 255.0,
            41.0 / 255.0,
            45.0 / 255.0,
        )))
        // Custom resources
        .insert_resource(GlobalAtlasTextureHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        // Systems
        .add_systems(OnEnter(GameState::Loading), (spawn_camera, load_assets))
        .add_systems(OnEnter(GameState::GameLoad), spawn_table_grid)
        .run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalAtlasTextureHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None);

    texture_atlas.0 = Some(texture_atlas_layout.add(layout));
    image_handle.0 = Some(asset_server.load("tilesheet.png"));

    next_state.set(GameState::GameLoad);
}

fn spawn_table_grid(
    mut commands: Commands,
    texture_atlas: Res<GlobalAtlasTextureHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
) {
    let table_section_size = 150.0;
    let table_size = (10, 10);

    let image_handle = image_handle.0.clone().unwrap();

    for x in -table_size.0 / 2..=table_size.0 / 2 {
        for y in -table_size.1 / 2..=table_size.1 / 2 {
            commands.spawn((
                Sprite {
                    image: image_handle.clone(),
                    custom_size: Some(Vec2::new(table_section_size, table_section_size)),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas.0.clone().unwrap(),
                        index: 0,
                    }),
                    ..default()
                },
                Transform {
                    scale: Vec3::ONE,
                    rotation: Quat::from_rotation_z(0.0),
                    translation: Vec3::new(
                        x as f32 * table_section_size,
                        y as f32 * table_section_size,
                        0.0,
                    ),
                },
            ));
        }
    }
}
