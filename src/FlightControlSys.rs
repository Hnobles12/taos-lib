use serde::{Serialize, Deserialize};

use linux_embedded_hal as hal;
use hal::Delay;
use hal::I2cdev;
use mpu9250::{Mpu9250, ImuMeasurements, MargMeasurements};
// use pca9685;

// #[derive(Serialize, Deserialize)]
pub struct FCS {
    // Powerplant Settings
    pub master_safe: bool,
    pub throttle: f32,

    // Control Surface Deflection Settings
    pub elevator_d: f32,
    pub rudder_d: f32,
    pub r_aileron_d: f32,
    pub l_aileron_d: f32,

    // Control Surface Deflection Bounds
    pub elevator_d_bound: [f32;2],
    pub rudder_d_bound: [f32;2],
    pub r_aileron_d_bound: [f32;2],
    pub l_aileron_d_bound: [f32;2],

    // Flight Control Hardware Controller
    pub fc_hardware: FCSHardware,

    // Telemetry Hardware Controller
    pub telem_hardware: TelemetryHardware,

}

impl Default for FCS {
    fn default() ->Self {

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

            // Flight Control Hardware Controller
            fc_hardware: FCSHardware::default(),

            // Telemetry Hardware
            telem_hardware: TelemetryHardware::default(),

        }
    }
}

impl FCS {

    pub fn init_hardware(&mut self){
        self.fc_hardware.init();
    }

    pub fn shutdown_hardware(&mut self){
        self.fc_hardware.shutdown();
    }


    pub fn update(&mut self) {
        // TODO Implement sending to FCSHardware
        self.fc_hardware.set_elevator_deflection(self.elevator_d);
        self.fc_hardware.set_l_aileron_deflection(self.l_aileron_d);
        self.fc_hardware.set_r_aileron_deflection(self.r_aileron_d);
        self.fc_hardware.set_rudder_deflection(self.rudder_d);
        
        if !self.master_safe {
            self.fc_hardware.set_throttle(self.throttle);
        }
        else {
            self.fc_hardware.set_throttle(0.0);
        }

        self.fc_hardware.update();

    }

    // Powerplant Control Settings
    pub fn set_master_safe(&mut self, setting: bool) {
        self.master_safe = setting;   
    }

    pub fn set_throttle(&mut self, setting: f32){
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
    pub fn set_elevator_deflection(&mut self, setting: f32){
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

    pub fn set_rudder_deflection(&mut self, setting: f32){
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

    pub fn set_r_aileron_deflection(&mut self, setting: f32){
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

    pub fn set_l_aileron_deflection(&mut self, setting: f32){
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
    pub fn set_rudder_bound(&mut self, low:f32, high:f32){
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

    pub fn get_telemetry(&mut self)->Telemetry{
        self.telem_hardware.get_telemetry()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Servo {
    pub channel: u8,
    pub min_pw:f32,
    pub max_pw:f32,
    pub pos: f32,
    pub pos_pw: f32,
    pub pos_offset: f32,
    pub inverted: bool

}
impl Default for Servo {
    fn default() -> Self {
        Servo {
            channel: 0,
            min_pw: 500.0,
            max_pw: 2500.0,
            pos: 0.0,
            pos_pw: 1500.0,
            pos_offset:0.0,
            inverted: false
        }
    }

}

impl Servo {

    pub fn deg2pw(&self, angle:f32)-> f32{
        angle*(self.max_pw - self.min_pw)/180.0 + (self.max_pw-self.min_pw)/2.0 + self.min_pw
    }

    pub fn set_pos(&mut self, angle:f32) {

        if angle >= 90.0 {
            self.pos = 90.0;
            self.pos_pw = self.deg2pw(90.0);
        }
        else if angle <= -90.0 {
            self.pos = -90.0;
            self.pos_pw = self.deg2pw(-90.0);
        }
        else {
            self.pos = angle;
            self.pos_pw = self.deg2pw(angle);
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
    pub fn set_channel(&mut self, channel: u8){
        self.channel = channel;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Motor {
    pub channel: u8,
    pub min_pw:f32,
    pub max_pw:f32,
    pub throttle: f32,
    pub throttle_pw: f32,
    pub throttle_offset: f32,
    pub inverted: bool

}

impl Default for Motor {

    fn default() -> Self {
        Motor {
            channel: 0,
            min_pw: 500.0, // Unknown
            max_pw: 2500.0,// Unknown
            throttle: 0.0,
            throttle_pw: 0.0,// Unknown
            throttle_offset:0.0,
            inverted: false
        }
    }
}
impl Motor {

    pub fn throttle2pw(&self, setting:f32)-> f32{
        setting*(self.max_pw - self.min_pw)/180.0 + self.min_pw
    }

    pub fn set_throttle(&mut self, setting:f32) {
        // let new_throttle_pw = self.throttle2pw(setting);

        if setting >= 100.0 {
            self.throttle = 100.0;
            self.throttle_pw = self.throttle2pw(100.0);
        }
        else if setting <= 0.0 {
            self.throttle = 0.0;
            self.throttle_pw = self.throttle2pw(0.0);
        }
        else {
            self.throttle = setting;
            self.throttle_pw = self.throttle2pw(setting);
        }
    }

    pub fn set_throttle_offset(&mut self, offset:f32){
        self.throttle_offset = offset;
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


// #[derive(Debug)]
pub struct FCSHardware {
    pub pca: pca9685::PCA9685,
    pub l_elevator_servo: Servo,
    pub r_elevator_servo: Servo,
    pub l_aileron_servo: Servo,
    pub r_aileron_servo: Servo,
    pub rudder_servo: Servo,
    pub throttle_pwm: Motor,
}

impl FCSHardware {
    pub fn default() -> Self {
        let device = pca9685::LinuxI2CDevice::new("/dev/i2c-1", 0x40).unwrap();
        FCSHardware {
            pca: pca9685::PCA9685::new(device, 50).unwrap(),
            l_elevator_servo: Servo{channel: 0, ..Default::default()},
            r_elevator_servo: Servo{channel: 1, ..Default::default()},
            l_aileron_servo: Servo{channel: 2, ..Default::default()},
            r_aileron_servo: Servo{channel: 3, ..Default::default()},
            rudder_servo: Servo{channel: 4, ..Default::default()},
            throttle_pwm: Motor{channel: 5, ..Default::default()},
        }
    }

    pub fn set_elevator_deflection(&mut self, angle:f32){
        self.l_elevator_servo.set_pos(angle);
        self.r_elevator_servo.set_pos(angle);
    }

    pub fn set_l_aileron_deflection(&mut self, angle:f32){
        self.l_aileron_servo.set_pos(angle);
    }

    pub fn set_r_aileron_deflection(&mut self, angle: f32){
        self.r_aileron_servo.set_pos(angle);
    }

    pub fn set_rudder_deflection(&mut self, angle:f32){
        self.rudder_servo.set_pos(angle);
    }

    pub fn set_throttle(&mut self, throttle:f32){
        self.throttle_pwm.set_throttle(throttle);
    }

    ///////////////// TODO: NEEDS ERROR HANDLING HERE!!!!! /////////////////////
    pub fn update(&mut self){
    ///////////////// TODO: NEEDS ERROR HANDLING HERE!!!!! /////////////////////
        self.pca.set_pulse_length(self.l_elevator_servo.channel, self.l_elevator_servo.pos_pw);
        self.pca.set_pulse_length(self.r_elevator_servo.channel, self.r_elevator_servo.pos_pw);

        self.pca.set_pulse_length(self.l_aileron_servo.channel, self.l_aileron_servo.pos_pw);
        self.pca.set_pulse_length(self.r_aileron_servo.channel, self.r_aileron_servo.pos_pw);

        self.pca.set_pulse_length(self.rudder_servo.channel, self.rudder_servo.pos_pw);

        self.pca.set_pulse_length(self.throttle_pwm.channel, self.throttle_pwm.throttle_pw);
    }

    ///////////////// TODO: NEEDS ERROR HANDLING HERE!!!!! /////////////////////
    pub fn init(&mut self){
    ///////////////// TODO: NEEDS ERROR HANDLING HERE!!!!! /////////////////////
        self.pca.set_frequency(50);
    }

    pub fn shutdown(&mut self){
        self.pca.set_all_duty_cycle(0);
    }
}

#[derive(Debug)]
pub struct Telemetry {
    pub accel: [f32;3],
    pub gyro: [f32;3],
    pub mag: [f32;3],
    pub temp:f32,
    // pub heading: f32,
}


pub struct TelemetryHardware {
    pub mpu9265: Mpu9250<mpu9250::I2cDevice<linux_embedded_hal::I2cdev>, mpu9250::Marg >,
    pub accelerometer_bias: [f32; 3],

}

impl Default for TelemetryHardware {
    fn default() -> TelemetryHardware {
        let i2c_device = I2cdev::new("/dev/i2c-1").unwrap();
        TelemetryHardware {
            mpu9265: Mpu9250::marg_default(i2c_device, &mut Delay).unwrap(),
            accelerometer_bias: [0.0,0.0,0.0]
        }
    }
}

impl TelemetryHardware {
    pub fn calibrate_at_rest(&mut self) {
        let bias: [f32;3] = self.mpu9265.calibrate_at_rest(&mut Delay).unwrap();
        self.accelerometer_bias = bias;
    }

    pub fn get_telemetry(&mut self) -> Telemetry {
        let all: MargMeasurements<[f32;3]> = self.mpu9265.all().unwrap();
        let mut accel: [f32;3] = all.accel;

        accel[0] = accel[0] - self.accelerometer_bias[0];
        accel[1] = accel[1] - self.accelerometer_bias[1];
        accel[2] = accel[2] - self.accelerometer_bias[2];

        Telemetry {
            accel: accel,
            gyro: all.gyro,
            mag: all.mag,
            temp: all.temp
        }
    }
}