use serde::{Serialize, Deserialize};
use ndarray as na;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandPacket {
    pub commands: Vec<CommandType>,
    pub values: Vec<na::Array1<f64>>,
}

impl CommandPacket {
    pub fn new() -> Self{
        CommandPacket {
            commands: Vec::<CommandType>::new(),
            values: Vec::<na::Array1<f64>>::new()
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandType {
    // System Commands
    SetSysMode,

    // Control Commands
    SetControlState,

    // Autopilot Commands
    SetAPState
}
