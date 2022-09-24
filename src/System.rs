use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Mode {
    Startup,
    Standby,
    InFlight,
    Takeoff,
    Landing,
    Shutdown,
    Failure
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Health {
    Nominal,
    Critical,
    Marginal
}


#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct State {
    mode: Mode,
    pub health: Health,
}

impl State {
    pub fn new() -> Self{
        State {mode: Mode::Startup, health: Health::Nominal}
    }
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn get_mode(&self) -> Mode {
        // match self.mode {
        //     Mode::Startup => Mode::Startup,
        //     Mode::Standby => Mode::Standby,
        //     Mode::InFlight => Mode::InFlight,
        //     Mode::Takeoff => Mode::Takeoff,
        //     Mode::Landing => 
        // }
        self.mode
    }
}
