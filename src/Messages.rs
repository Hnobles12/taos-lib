use serde::{Serialize, Deserialize};
use chrono;

// pub trait Serializable {
//     // fn to_json(&self) -> String {
//     //     serde_json::to_string(&self).unwrap()
//     // }
// }




// System Message Section:

#[derive(Serialize, Deserialize, Debug)]
pub enum SysCommand {
    SetMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SysMsg {
    pub command: SysCommand,
    pub data: String // String of JSON for data corresponding with SysCommand.
    
}

// impl Serializable for SysCommand {}
// impl Serializable for SysMsg{}


/////////// Message Section ///////////////////

#[derive(Serialize, Deserialize, Debug)]
pub enum MsgType {
    Sys,
    Control,
    Status,
    Telemetry
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message{
    timestamp: i64,
    msg_type: MsgType,
    content: String
}

impl Message {
    pub fn new(msg_type: MsgType, content: String) -> Message{
        Message{timestamp: chrono::Utc::now().timestamp(),
                msg_type: msg_type,
                content: content
            }
    }
    
}

// impl Serializable for Message{}
// impl Serializable for MsgType{}