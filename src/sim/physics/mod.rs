pub type PhysicsWorld = nphysics3d::world::World<f64>;
use nphysics3d::object::ColliderHandle;
use nphysics3d::object::BodyHandle;

use ncollide3d::bounding_volume::{AABB, BoundingSphere};

/// The state of physics in the simulation
pub struct PhysicsState {
    /// The physics world
    world : PhysicsWorld,
    /// A list of active colliders for spawners.
    /// "Garbage collected" by checking if colliders from other bodies are nearby.
    /// TODO: think of a way to garbage collect colliders for nearby bodies known not to collide
    /// with each other, other than maybe a collision group...
    active : Vec<ColliderHandle>
}

impl PhysicsState {

    /// Create a new physics state, with no active bodies
    pub fn new() -> PhysicsState {
        PhysicsState{ world : PhysicsWorld::new(), active : Vec::new() }
    }

    /// Spawn colliders, given a spawner, for a body within an AABB (if they don't already exist)
    #[allow(dead_code)]
    pub fn spawn_aabb_for<T : BVSpawner>(&mut self,
        aabb : AABB<f64>, body : BodyHandle, spawner : &T) {
        let PhysicsState {
            ref mut world,
            ref mut active
        } = *self;
        spawner.spawn_aabb(aabb, world, body, |handle| {active.push(handle)});
    }

    /// Spawn colliders, given a spawner, for a body within a sphere (if they don't already exist)
    #[allow(dead_code)]
    pub fn spawn_sphere_for<T : BVSpawner>(&mut self,
        sphere : BoundingSphere<f64>, body : BodyHandle, spawner : &T) {
        let PhysicsState {
            ref mut world,
            ref mut active
        } = *self;
        spawner.spawn_sphere(sphere, world, body, |handle| {active.push(handle)});
    }

}

/// An object which contains physics objects to be spawned when an active physics object gets within
/// distance, given by a bounding volume
pub trait BVSpawner {
    /// Spawn colliders for a body within an AABB if they don't already exist
    fn spawn_aabb<F : FnMut(ColliderHandle)>(&self,
        aabb : AABB<f64>, world : &mut PhysicsWorld, body : BodyHandle, desc : F);
    /// Spawn colliders for a body within a sphere if they don't already exist
    fn spawn_sphere<F : FnMut(ColliderHandle)>(&self,
        sphere : BoundingSphere<f64>, world : &mut PhysicsWorld, body : BodyHandle, desc : F);
}
