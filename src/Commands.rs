use serde::{Serialize, Deserialize};
// use ndarray as na;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandPacket {
    pub commands: Vec<CommandType>,
    pub values: Vec<String>,
}

impl CommandPacket {
    pub fn new() -> Self{
        CommandPacket {
            commands: Vec::<CommandType>::new(),
            values: Vec::<String>::new()
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // pub fn add(&mut self, cmdtype: CommandType, array: na::Array1<f64>) {
    //     self.commands.push(cmdtype);
    //     self.values.push(array);
    // }
    pub fn add<T: Serialize>(&mut self, cmdtype: CommandType, data: T){
        self.commands.push(cmdtype);
        self.values.push(serde_json::to_string(&data).unwrap());
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
