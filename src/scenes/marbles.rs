use anyhow::{Result};
use nalgebra_glm as glm;

use crate::{
    game::{
        scene::{Scene},
        lights::{DirectionalLight},
        game_object::{GameObject},
        components::{RotateOverTimeComponent, RenderMarbleComponent, RenderModelComponent}
    },
    shader_input::{standard}
};

pub fn create_scene(scene: &mut Box<Scene>) -> Result<()> {
    // scene.render_camera.transform.pos = glm::vec3(5.0, 5.0, 3.0);
    scene.render_camera.transform.pos = glm::vec3(2.2, 2.2, 2.0);
    // scene.render_camera.transform.pos = glm::vec3(1.0, 1.0, 0.75);
    // scene.render_camera.look_at(*crate::game::transform::ORIGIN);
    scene.render_camera.look_at(glm::vec3(0.0, 0.0, -0.9));
    scene.ambient_light = glm::vec3(4.0, 4.0, 4.0);
    scene.directional_light = Some(DirectionalLight {
        direction: glm::vec3(-1.0, 0.0, -0.3),
        color: glm::vec3(10.0, 10.0, 10.0),
    });

    let mut game_object = Box::new(GameObject::new());
    game_object.add_component(Box::new(RotateOverTimeComponent::new()))?;
    // game_object.add_component(Box::new(RenderModelComponent::<simple::Vertex>::new("resources/models/die/die-with-uvs.obj")?))?;
    // game_object.add_component(Box::new(RenderModelComponent::<simple::Vertex>::new("resources/models/viking-room/viking-room.obj")?))?;
    // game_object.add_component(Box::new(RenderModelComponent::<simple::Vertex>::new("resources/models/coords/coords.obj")?))?;
    // game_object.add_component(Box::new(RenderModelComponent::<simple::Vertex>::new("resources/models/sphere/sphere.obj")?))?;
    // game_object.add_component(Box::new(RenderModelComponent::<standard::Vertex>::new("resources/models/marbles/flat_plane.obj")?))?;
    game_object.add_component(Box::new(RenderModelComponent::<standard::Vertex>::new("resources/models/marbles/bowl.obj")?))?;
    game_object.add_component(Box::new(RenderMarbleComponent::new("resources/models/marbles/marble.obj")?))?;
    scene.add_game_object(game_object)?;

    Ok(())
}
