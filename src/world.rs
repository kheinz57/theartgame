use bevy::prelude::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin{
  fn build(&self, app: & mut App){
    app.add_systems(Startup, (spawn_light,create_board));
  }
}

fn spawn_light(mut commands: Commands){
  commands.insert_resource(AmbientLight {
    color: Color::WHITE,
    brightness: 0.8,
  });
}

fn create_board(
  mut commands:Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // Add meshes and materials
  let mesh = meshes.add(Mesh::from(shape::Plane::from_size(1.0)));
  let white_material = materials.add(Color::rgb(0.7, 0.7, 0.7).into());
  let black_material = materials.add(Color::rgb(0.3, 0.3, 0.3).into());
  let blue_material = materials.add(Color::rgb(0., 0.0, 0.5).into());

  // Spawn many squares
  for i in -2..102 {
      for j in -32..32 {
          commands.spawn(PbrBundle {
              mesh: mesh.clone(),
              // Change material according to position to get alternating pattern
              material: 
              // if (i == 0 && 0 == j){
              //     blue_material.clone()
              // }else 
              if (i + j + 1) % 2 == 0 {
                  white_material.clone()
              } else {
                  black_material.clone()
              },
              transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
              ..Default::default()
          });
      }
  }
}



