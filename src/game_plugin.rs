use bevy::{prelude::*};
use bevy_ecs_tilemap::{prelude::*,helpers::filling};
use rand::prelude::*;
use perlin2d::PerlinNoise2D;

use crate::GameState;
use crate::math::*;
use crate::noise::perlin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Loading)
                .with_system(game_setup)
            )
            ;

    }
}

const TILEMAP_HEIGHT: u32 = 150;
const TILEMAP_WIDTH: u32 = 300;
const TILE_SIZE: f32 = 4.0;

// Lets procedurally draw an island
fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // setup
    let deep_ocean_color = 0;
    let ocean_color = 1;
    let sand_color = 2;
    let forest_color = 3;
    let rock_color = 4;
    let snow_color = 5;
    // Load map
    let texture_handle: Handle<Image> = asset_server.load("textures/tiles.png");
    let tilemap_size = TilemapSize { x: TILEMAP_WIDTH, y: TILEMAP_HEIGHT };
    let mut tile_storage = TileStorage::empty(tilemap_size);
    let tilemap_type = TilemapType::Square {
        diagonal_neighbors: false,
    };
    // Create a tilemap entity a little early
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    let tilemap_entity = commands.spawn().id();

    // Adding color
    filling::fill_tilemap(
        TileTexture(deep_ocean_color), tilemap_size, TilemapId(tilemap_entity),
        &mut commands, &mut tile_storage);

    // ISLAND
    // We first start with a higher sea level
    //let middle_x = TILEMAP_WIDTH / 2;
    //let middle_y = TILEMAP_HEIGHT / 2;
    //let center_distance = 50;
    let mut rng = thread_rng();
    let rdm = rng.gen::<i32>() % 2_i32.pow(16);
    let rdm = 24626;
    let perlin = PerlinNoise2D::new(3, 2.0, 1.0, 1.0, 2.0, (50.0, 50.0), 0.0, rdm);
    //rdm *= 1000000.0;
    //let rdm = rdm as u32;
    for x in 0..TILEMAP_WIDTH {
        for y in 0..TILEMAP_HEIGHT {
            /*
            if is_in_circle(middle_x, middle_y, 25, x, y) {
                let pos = TilePos { x, y };
                commands
                    .entity(tile_storage.get(&pos).unwrap())
                    .insert(TileTexture(ocean_color));
            }
            */
            let pos = TilePos { x, y };
            let mut noise = perlin.get_noise(x as f64, y as f64).floor().abs();
            let noise = noise as f32;
            commands
                .entity(tile_storage.get(&pos).unwrap())
            //    .insert(TileTexture(perlin((x + rdm) as f32 / TILEMAP_WIDTH as f32, (y + rdm) as f32 / TILEMAP_HEIGHT as f32)))
                .insert(TileTexture(noise as u32));
        }
    }

    // This is the size of each individual tiles in pixels.
    let tile_size = TilemapTileSize { x: TILE_SIZE, y: TILE_SIZE };
    let grid_size = TilemapGridSize { x: TILE_SIZE, y: TILE_SIZE };

    // Spawns a tilemap.
    // Once the tile storage is inserted onto the tilemap entity it can no longer be accessed.
    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: tile_storage,
            map_type: tilemap_type,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&tilemap_size, &grid_size, 0.0),
            ..Default::default()
        })
        .insert(LastUpdate(0.0))
        .insert(CurrentColor(1));
}

#[derive(Component)]
struct CurrentColor(u16);

#[derive(Component)]
struct LastUpdate(f64);