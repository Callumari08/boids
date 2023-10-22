use bevy_prototype_lyon::prelude::*;
// use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

use boids::*;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::TEAL))
    .add_plugins((DefaultPlugins, ShapePlugin))
    .add_systems(Startup, init)
    .add_systems(Update, Boid::update)
    .run();
}

const SPEED: f32 = 50.0;
fn init(mut commands: Commands, /*mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>*/) -> ()
{
    commands.spawn(Camera2dBundle::default());

    // What is a triangle?
    let tri = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(11.5),
        ..shapes::RegularPolygon::default()
    };


    for _i in 0..499 
    {
        let transform: Transform = Transform::from_translation(Vec3::new(rand::thread_rng().gen_range(-Boid::BOUNDS.x-1.0..Boid::BOUNDS.x+1.0), 
            rand::thread_rng().gen_range(-Boid::BOUNDS.y+1.0..Boid::BOUNDS.y-1.0), rand::thread_rng().gen_range(-Boid::BOUNDS.z+1.0..Boid::BOUNDS.z-1.0)));

            let mut direction: Vec3;
            // Loop until direction is not zero on all coordinates so it's not still
            loop
            {
                direction = Vec3::new(
                    rand::thread_rng().gen_range(-1..1) as f32,
                    rand::thread_rng().gen_range(-1..1) as f32,
                    rand::thread_rng().gen_range(-1..1) as f32,
                );

                if direction.x != 0.0 || direction.y != 0.0 /*|| direction.z == 0.0*/
                {
                    println!("{}", direction);
                    break;
                }
            }
            commands.spawn((
            ShapeBundle
            {
                path: GeometryBuilder::build_as(&tri),
                transform,
                ..default()
            },
            Fill::color(Color::WHITE),
            Stroke::new(Color::GRAY, 2.0),
    
            Boid::new(Velocity { speed: SPEED, direction }, 5.0),
    
        ));
    }
}

mod boids;