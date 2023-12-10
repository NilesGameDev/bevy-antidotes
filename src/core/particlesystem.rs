use bevy::prelude::*;
use bevy_hanabi::*;

#[derive(Resource)]
pub struct CellExplodeEffectRes(pub Handle<EffectAsset>);

pub fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.05),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(15.0),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(2.0);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        50,
        Spawner::once(50.0.into(), true),
        // Move the expression module into the asset
        module,
    )
    .with_name("Explosion")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);
    commands.insert_resource(CellExplodeEffectRes(effect_handle));
}
