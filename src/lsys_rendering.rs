use bevy::prelude::*;
use bevy::{
    asset::Asset,
    math::Vec3,
    pbr::{Material, MaterialPipeline, MaterialPipelineKey},
    reflect::TypePath,
    render::{
        color::Color,
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};
use serde::{Deserialize, Serialize};

use crate::fractal_plant::FractalPlant;
use crate::fractal_plant::LineList;

pub trait GenerateLineList {
    fn generate_line_list(&self) -> LineList;
}

#[derive(Event, Debug, Clone)]
pub struct LineMeshUpdateEvent(LineMesh);

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone, Serialize, Deserialize)]
pub struct LineMaterial {
    #[uniform(0)]
    color: Color,
}
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct LineMesh {
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) line_list: LineList,
    #[serde(skip_serializing, skip_deserializing)]
    pub mesh_handle: Handle<Mesh>,
    #[serde(skip_serializing, skip_deserializing)]
    pub material_handle: Handle<LineMaterial>,
}
impl Default for LineMesh {
    fn default() -> Self {
        Self {
            line_list: LineList { lines: vec![] },
            mesh_handle: Handle::<Mesh>::default(),
            material_handle: Handle::<LineMaterial>::default(),
        }
    }
}

impl LineMaterial {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

pub fn update_line_mesh_materials() {}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}
