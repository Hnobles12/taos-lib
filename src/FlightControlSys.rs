use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FCS {
    // Powerplant Settings
    master_safe: bool,
    throttle: f64,

    // Control Surface Deflection Settings
    elevator_d: f64,
    rudder_d: f64,
    r_aileron_d: f64,
    l_aileron_d: f64,

    // Control Surface Deflection Bounds
    elevator_d_bound: [f64;2],
    rudder_d_bound: [f64;2],
    r_aileron_d_bound: [f64;2],
    l_aileron_d_bound: [f64;2],

}


impl FCS {
    pub fn new() -> Self {

        FCS {
            // Powerplant Settings
            master_safe: true,
            throttle: 0.0,

            // Control Surface Deflection Settings
            elevator_d: 0.0,
            rudder_d: 0.0,
            r_aileron_d: 0.0,
            l_aileron_d: 0.0,

            // Control Surface Deflection Bounds
            elevator_d_bound: [-25.0,25.0],
            rudder_d_bound: [-25.0,25.0],
            r_aileron_d_bound: [-25.0,25.0],
            l_aileron_d_bound: [-25.0,25.0],

        }
    }

    pub fn update_controls(&self) {
        println!("Updating Controls: {:?}", self);
    }

    // Powerplant Control Settings
    pub fn set_master_safe(&mut self, setting: bool) {
        self.master_safe = setting;        
    }

    pub fn set_throttle(&mut self, setting: f64){
        if setting <= 0.0 {
            self.throttle = 0.0;
        }
        else if setting >= 100.0 {
            self.throttle = 100.0;
        }
        else {
            self.throttle = setting;
        }
    }


    // Control Surface Settings
    pub fn set_elevator_deflection(&mut self, setting: f64){
        if setting <= self.elevator_d_bound[0] {
            self.elevator_d = self.elevator_d_bound[0];
        }
        else if setting >= self.elevator_d_bound[1]{
            self.elevator_d = self.elevator_d_bound[1];
        }
        else {
            self.elevator_d = setting;
        }
    }

    pub fn set_rudder_deflection(&mut self, setting: f64){
        if setting <= self.rudder_d_bound[0] {
            self.rudder_d = self.rudder_d_bound[0];
        }
        else if setting >= self.rudder_d_bound[1]{
            self.rudder_d = self.rudder_d_bound[1];
        }
        else {
            self.rudder_d = setting;
        }
    }

    pub fn set_r_aileron_deflection(&mut self, setting: f64){
        if setting <= self.r_aileron_d_bound[0] {
            self.r_aileron_d = self.r_aileron_d_bound[0];
        }
        else if setting >= self.r_aileron_d_bound[1]{
            self.r_aileron_d = self.r_aileron_d_bound[1];
        }
        else {
            self.r_aileron_d = setting;
        }
    }

    pub fn set_l_aileron_deflection(&mut self, setting: f64){
        if setting <= self.l_aileron_d_bound[0] {
            self.l_aileron_d = self.l_aileron_d_bound[0];
        }
        else if setting >= self.l_aileron_d_bound[1]{
            self.l_aileron_d = self.l_aileron_d_bound[1];
        }
        else {
            self.l_aileron_d = setting;
        }
    }

    // Control Surface Deflection Bounds
    pub fn set_rudder_bound(&mut self, low:f64, high:f64){
        // let mut bound = [-90.0,90.0];

        // if low <= -90.0 {
        //    bound[0] = -90.0; 
        // }
        // else if  >= 90.0 {
        //     bound[1] = 90.0;
        // }
        // else {

        // }
        // self.rudder_d_bound = bound;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Servo {
    min_pw:f32,
    max_pw:f32,
    pos: f32,
    pos_pw: f32,
    pos_offset: f32,
    inverted: bool

}

impl Servo {
    pub fn default() -> Self {
        Servo {
            min_pw: 500.0,
            max_pw: 2500.0,
            pos: 0.0,
            pos_pw: 1500.0,
            pos_offset:0.0,
            inverted: false
        }
    }

    pub fn deg2pw(&self, angle:f32)-> f32{
        angle*(self.max_pw - self.min_pw)/180.0 + (self.max_pw-self.min_pw)/2.0 + self.min_pw
    }

    pub fn set_pos(&mut self, angle:f32) {
        let new_pos = self.deg2pw(angle);

        if new_pos >= 90.0 {
            self.pos = 90.0;
            self.pos_pw = self.deg2pw(90.0);
        }
        else if new_pos <= -90.0 {
            self.pos = -90.0;
            self.pos_pw = self.deg2pw(-90.0);
        }
        else {
            self.pos = new_pos;
            self.pos_pw = self.deg2pw(new_pos);
        }
    }

    pub fn set_pos_offset(&mut self, offset:f32){
        self.pos_offset = offset;
    }

    pub fn invert(&mut self, inverted: bool){
        self.inverted = inverted;
    }

    pub fn set_min_pw(&mut self, pw: f32) {
        self.min_pw = pw;
    }
    pub fn set_max_pw(&mut self, pw: f32) {
        self.max_pw = pw;
    }
}

// #[derive(Serialize, Deserialize, Debug)]
pub struct FCS_Hardware {
    pca: pca9685::PCA9685,
}