use bevy::prelude::*;
use serde::Deserialize;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

// right handed rule with x = thumb. y is up 
// https://bevy-cheatbook.github.io/features/coords.html


pub struct PaintingPlugin;
impl Plugin for PaintingPlugin{
  fn build(&self, app: & mut App){
    app.add_systems(Startup, spawn_painting);
  }
}

#[derive(Deserialize, Debug)]
struct PaintingMetaData{
  width: f32,
  height:f32
}

fn read_metadata<P: AsRef<Path>>(path: P) -> Result<PaintingMetaData, Box<dyn Error>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let u = serde_json::from_reader(reader);
  match u{
    Err(e) => print!("error reading json file {}",e),
    Ok(u) => return Ok(u),
  }
  Ok(PaintingMetaData{width:1.0,height:1.2})
}


fn spawn_painting(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials : ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
)
{
  // Wall
  let painting= PbrBundle{
    mesh: meshes.add(Mesh::from(shape::Box::new(100.0, 5.0,0.2))),
    material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    transform: Transform::from_xyz(48.0, 2.5 ,-0.25),
    ..default()
  };
  commands.spawn(painting);


  // Paintings
  for x in 0..50{
    let filename = format!("assets/th/picture{:03}.json", x);
    println!("read file {}",filename);
    let meta_data:PaintingMetaData; 
    match read_metadata(filename){
      Ok(m) => meta_data = m,
      Err(_) => break,
    }
    println!("metadata:{:?}",meta_data);
    let mut transform = Transform::from_xyz((x as f32)*4.0, 0.8 + 1.0 ,0.0);
    transform.rotate_x(f32::to_radians(90.0));
    let filename = format!("th/picture{:03}.jpg", x);

    let painting= PbrBundle{
      mesh: meshes.add(Mesh::from(shape::Box::new(meta_data.width*5.0, 0.1,meta_data.height*5.0))),
      material: materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load(filename)),
        unlit: true,        
        ..default()
      }),
      transform: transform,
      ..default()
    };
    commands.spawn(painting);
  }
}
