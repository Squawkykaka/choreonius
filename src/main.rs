use bevy::{color::palettes, prelude::*};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    GameLoad,
    InGame,
}

#[derive(Resource)]
struct GlobalAtlasTextureHandle(Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

#[derive(Component)]
struct PlayerCam;

#[derive(Component)]
struct TableItem {
    score: u32,
}

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
        .add_systems(
            OnEnter(GameState::GameLoad),
            (spawn_table_grid, spawn_table_items, enter_game).chain(),
        )
        .add_systems(Update, wasd_movement.run_if(in_state(GameState::InGame)))
        .run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, PlayerCam));
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalAtlasTextureHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 1, None, None);

    texture_atlas.0 = Some(texture_atlas_layout.add(layout));
    image_handle.0 = Some(asset_server.load("tilesheet.png"));

    next_state.set(GameState::GameLoad);
}

const TABLE_WIDTH: f32 = 1500.0;
const TABLE_HEIGHT: f32 = 1500.0;

fn spawn_table_grid(
    mut commands: Commands,
    texture_atlas: Res<GlobalAtlasTextureHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
) {
    let image_handle = image_handle.0.clone().unwrap();

    commands.spawn(Sprite {
        image: image_handle.clone(),
        custom_size: Some(Vec2::new(TABLE_WIDTH, TABLE_HEIGHT)),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 0,
        }),
        image_mode: SpriteImageMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 10.0,
        },
        ..default()
    });
}

fn spawn_table_items(
    mut commands: Commands,
    texture_atlas: Res<GlobalAtlasTextureHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
) {
    let item_amount = 20;

    let image_handle = image_handle.0.clone().unwrap();

    // Spawn items on tables
    for i in 0..=item_amount {
        let item_size = rand::random_range(50..300) as f32;
        commands
            .spawn((
                Sprite {
                    image: image_handle.clone(),
                    custom_size: Some(Vec2::new(item_size, item_size)),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas.0.clone().unwrap(),
                        index: rand::random_range(1..=5),
                    }),
                    ..default()
                },
                Transform {
                    scale: Vec3::ONE,
                    rotation: Quat::from_rotation_z(0.0),
                    translation: Vec3::new(
                        rand::random_range(-TABLE_WIDTH / 2.0..TABLE_WIDTH / 2.0),
                        rand::random_range(-TABLE_HEIGHT / 2.0..TABLE_HEIGHT / 2.0),
                        0.0,
                    ),
                },
                TableItem { score: i },
                Pickable {
                    is_hoverable: true,
                    ..default()
                },
            ))
            .observe(
                |trigger: Trigger<Pointer<Over>>, mut query: Query<&mut Sprite>| {
                    if let Ok(mut sprite) = query.get_mut(trigger.target()) {
                        sprite.color = Color::from(palettes::tailwind::CYAN_300);
                    }
                },
            )
            .observe(
                |trigger: Trigger<Pointer<Out>>, mut query: Query<&mut Sprite>| {
                    if let Ok(mut sprite) = query.get_mut(trigger.target()) {
                        sprite.color = Color::WHITE;
                    }
                },
            )
            .observe(move_table_item_on_drag)
            .observe(
                |trigger: Trigger<Pointer<DragEnd>>, mut query: Query<&mut Transform>| {
                    if let Ok(mut transform) = query.get_mut(trigger.target()) {
                        transform.translation.z = 0.0;
                    }
                },
            );
    }
}

fn enter_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}

fn wasd_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut playercam: Single<&mut Transform, With<PlayerCam>>,
) {
    let mut velocity = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        velocity.y += 1.0
    };
    if keyboard_input.pressed(KeyCode::KeyS) {
        velocity.y -= 1.0
    };
    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= 1.0
    };
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += 1.0
    };

    playercam.translation += velocity.normalize_or_zero() * 6.0;
}

fn move_table_item_on_drag(
    trigger: Trigger<Pointer<Drag>>,
    mut query: Query<&mut Transform>,
    camera: Single<(&Camera, &GlobalTransform), With<PlayerCam>>,
) {
    if let Ok(mut transform) = query.get_mut(trigger.target()) {
        let (camera, camera_pos) = camera.into_inner();
        if let Ok(mouse_pos) =
            camera.viewport_to_world_2d(camera_pos, trigger.pointer_location.position)
        {
            dbg!(mouse_pos);
            transform.translation = mouse_pos.extend(10.0);
        };
    }
}
