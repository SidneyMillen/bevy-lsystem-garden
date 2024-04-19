use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use lsystem::LSystem;

use serde::{Deserialize, Serialize};

use lsystem::MapRules;

use crate::fractal_plant::FractalPlant;
use crate::lsys_rendering::GenerateLineList;

#[derive(Component, Debug, Serialize, Deserialize)]
pub(crate) struct LSys {
    pub(crate) name: String,
    pub(crate) rules: LSysRules,
    pub(crate) iterations: usize,
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub(crate) struct LSysDrawer {
    pub(crate) changed: bool,
}

#[derive(Component, Debug, Serialize, Deserialize)]

pub(crate) struct LSysRules {
    pub(crate) axiom: Vec<char>,
    pub(crate) rules: Vec<(char, String)>,
}

#[derive(Debug)]
pub(crate) enum LSystemEvaluationError {
    EvaluationError,
}

impl LSysRules {
    pub fn new(axiom: Vec<char>, rules: Vec<(char, String)>) -> Self {
        Self { axiom, rules }
    }

    pub(crate) fn as_map_rules(&self) -> MapRules<char> {
        let mut map_rules = MapRules::new();
        for (k, v) in &self.rules {
            map_rules.set_str(k.clone(), v.clone().as_str());
        }
        map_rules
    }

    pub fn eval(&self, levels: &usize) -> Result<String, LSystemEvaluationError> {
        let map_rules = self.as_map_rules();
        let mut system = LSystem::new(map_rules, self.axiom.clone());
        let output = system
            .nth(levels.clone())
            .ok_or(LSystemEvaluationError::EvaluationError)?
            .into_iter()
            .collect();
        Ok(output)
    }
}

impl LSysDrawer {
    pub(crate) fn new() -> Self {
        Self { changed: true }
    }
}
