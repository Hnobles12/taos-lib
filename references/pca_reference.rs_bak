use pca9685::*;
use std::time::{Instant,Duration};
use std::thread::sleep_ms;

fn from_deg(angle: f32) -> f32{
    if angle >= 180.0 {
        return 2500.0;
    }
    else if angle <= 0.0 {
        return 500.0;
    }
    else{
    ((2500.0-500.0)/180.0)*angle + 500.0
    }
}

fn main() {
	let device = LinuxI2CDevice::new("/dev/i2c-1", 0x40).unwrap();
	let mut pca9685 = PCA9685::new(device, 50).unwrap();
	pca9685.set_frequency(50);
	pca9685.set_all_duty_cycle(0);

	let servo_max = 2000.0;
	let servo_min = 500.0;
	let mut i = servo_min;
	pca9685.set_all_duty_cycle(0);
	sleep_ms(3000);
	pca9685.set_pulse_length(0, from_deg(0.0));
	sleep_ms(100);
	pca9685.set_pulse_length(0, from_deg(45.0));
	sleep_ms(100);
	pca9685.set_pulse_length(0, from_deg(90.0));
	sleep_ms(100);
	pca9685.set_pulse_length(0, from_deg(135.0));
	sleep_ms(100);
	pca9685.set_pulse_length(0, from_deg(180.0));
	sleep_ms(100);
	pca9685.set_pulse_length(0, from_deg(0.0));
	sleep_ms(100);


    /*
	// Arm
	pca9685.set_all_pulse_length(1100.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, servo_min);
	sleep_ms(3000);

	pca9685.set_all_pulse_length(1250.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1200.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1150.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1100.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1050.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1250.0);
	sleep_ms(3000);

*/
	pca9685.set_all_duty_cycle(0);
}
