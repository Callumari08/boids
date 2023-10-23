pub mod boid_view
{
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct BoidView
    {
        pub transform: Transform,
        pub view_radius: f32,
    }

    impl BoidView
    {
        pub fn new(transform: Transform, view_radius: f32) -> Self
        {
            Self { transform, view_radius }
        }
    }
}