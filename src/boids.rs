pub use bevy::prelude::*;

#[derive(Component)]
pub struct Boid
{
    pub velocity: Velocity,
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
    pub const BOUNDS: Vec3 = Vec3::new(500.0, 500.0, 500.0);

    pub fn new(velocity: Velocity) -> Self
    {
        Self { velocity} 
    }

    pub fn update(time:Res<Time>, mut boid: Query<(&mut Boid, &mut Transform /*&mut BoidOptions*/)> )
    {
        for (mut boid, mut transform) in boid.iter_mut()
        {
            transform.translation += boid.velocity.direction * boid.velocity.speed * time.delta_seconds();

            Boid::check_bounds(&mut transform);
        }
    }


    fn check_bounds(transform: &mut Transform)
    { 
        /*if transform.translation.x >= Boid::BOUNDS.x || transform.translation.y >= Boid::BOUNDS.y /*|| transform.translation.z >= Boid::BOUNDS.z*/
        {
            println!("Boid out of bounds positive with pos {0} and dir {1}", transform.translation, boid.velocity.direction);
            //transform.translation *= -1.0;
            println!("New Boid Pos: {0} and dir {1}\n", transform.translation, boid.velocity.direction);
        }
        else if transform.translation.x <= -Boid::BOUNDS.x || -transform.translation.y <= -Boid::BOUNDS.y /*|| transform.translation.z <= -Boid::BOUNDS.z*/
        {
            println!("Boid out of bounds negative with pos {0} and dir {1}", transform.translation, boid.velocity.direction);
            //transform.translation *= -1.0;
            println!("New Boid Pos: {0} and dir {1} \n", transform.translation, boid.velocity.direction) ;
        }*/

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