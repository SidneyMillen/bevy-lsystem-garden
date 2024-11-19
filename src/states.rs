use bevy::prelude::*;

use crate::GameState;

fn change_states(state: Res<GameState>){
    if state.is_changed(){
        match state.into_inner(){
            GameState::Default => todo!(),
            GameState::Editor(focused_object) => todo!(),
        };
    }
}
