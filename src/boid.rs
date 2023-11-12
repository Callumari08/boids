pub mod boid
{
    use bevy::{prelude::*, render::render_resource::encase::vector};

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

        pub fn update(time:Res<Time>, mut query: Query<(&mut Boid, &mut Transform /*BoidOptions*/)> )
        {
            for (boid, mut transform) in query.iter_mut()
            {


                transform.translation += boid.velocity.direction * boid.velocity.speed * time.delta_seconds();

                Boid::check_bounds(&mut transform); 
            }
        }

        fn find_visible_boids<'a>() -> Vec<&'a mut Boid>
        {
            return Vec::new();
        }

        fn check_bounds(transform: &mut Transform)
        { 
            // Stupid But Fast
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
