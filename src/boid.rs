pub mod boid
{
    use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

    #[derive(Component)]
    pub struct Boid
    {
        pub velocity: Velocity,
        pub view_radius: f32,

        pub has_view: bool,
    }

    pub struct Velocity
    {
        pub speed: f32,
        pub direction: Vec3,
    }

    //TODO: Implement this.
    /*pub struct BoidOptions
    {
    check_bounds: bool, 
    }*/

    impl Boid
    {
        pub const BOUNDS: Vec3 = Vec3::new(650.0, 400.0, 500.0);

        pub fn new(velocity: Velocity, view_radius: f32) -> Self
        {
            Self { velocity, view_radius, has_view: false } 
        }

        /*pub fn start(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>,mut boid: Query<(&mut Boid, &mut Transform)> )
        {
            println!("No Loop");
            for (mut boid, transform) in boid.iter_mut()
            {
                println!("Loop");
                // View Circle/Radius
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: *transform,
                    ..default()
                });
            }
        }*/

        pub fn update(time:Res<Time>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, 
            mut materials: ResMut<Assets<ColorMaterial>>,  
            mut boid: Query<(&mut Boid, &mut Transform /*BoidOptions*/)> )
        {
            for (mut boid, mut transform) in boid.iter_mut()
            {
            if !boid.has_view
            {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::PURPLE)),
                        transform: *transform,
                        ..default()
                    });

                    boid.has_view = true;
            } 

                transform.translation += boid.velocity.direction * boid.velocity.speed * time.delta_seconds();

                Boid::check_bounds(&mut transform);
            }
        }


        fn check_bounds(transform: &mut Transform)
        { 
            match (transform.translation.x, transform.translation.y, transform.translation.z) 
            {
                (x, _y, _z) if x >= Boid::BOUNDS.x => 
                {
                    transform.translation.x = x - 2.0 * Boid::BOUNDS.x;
                }
                (_x, y, _z) if y >= Boid::BOUNDS.y => 
                {
                    transform.translation.y = y - 2.0 * Boid::BOUNDS.y;
                }
                (_x, _y, z) if z >= Boid::BOUNDS.z => 
                {
                    transform.translation.z = z - 2.0 * Boid::BOUNDS.z;
                }
                (x, _y, _z) if x <= -Boid::BOUNDS.x => 
                {
                    transform.translation.x = x + 2.0 * Boid::BOUNDS.x;
                }
                (_x, y, _z) if y <= -Boid::BOUNDS.y => 
                {
                    transform.translation.y = y + 2.0 * Boid::BOUNDS.y;
                }
                (_x, _y, z) if z <= -Boid::BOUNDS.z => 
                {
                    transform.translation.z = z + 2.0 * Boid::BOUNDS.z;
                }
                _ => {}
            }                
        }
    }
}
