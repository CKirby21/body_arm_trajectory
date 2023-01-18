use std::io;
#[macro_use]
extern crate assert_float_eq;

// Feet per second squared
const GRAVITY: f64 = 32.17404855643;

// Roundoff Error Mitigation
// 0.015625 was chosen because it can be represented exactly in binary
const X_STEP: f64 = 0.015625;

struct MaxInfo {x_angle: i32, x_max: f64, y_angle: i32, y_max: f64}

impl MaxInfo {
    fn new() -> Self { Self { x_angle: 0, x_max: 0.0, y_angle: 0, y_max: 0.0 } }

    fn update(&mut self, potential_x_max: f64, potential_y_max: f64, potential_angle: i32) {
        if potential_x_max > self.x_max {
            self.x_angle = potential_angle;
            self.x_max = potential_x_max;
        }
        if potential_y_max > self.y_max {
            self.y_angle = potential_angle;
            self.y_max = potential_y_max;
        }
    }
}

fn main() {

    let body_height: f64 = get_float64_input("Height of your body (ft):");
    let arm_length: f64 = get_float64_input("Length of your arm (ft):");
    let arm_velocity_mph: f64 = get_float64_input("Velocity of your arm (mph):");
    
    let max_info = trajectories_at_each_angle(arm_length, body_height, arm_velocity_mph);

    println!("Max Distance was {:.2}ft at an angle of {} degrees", max_info.x_max, max_info.x_angle);
    println!("Max Height was {:.2}ft at an angle of {} degrees", max_info.y_max, max_info.y_angle);
}

// Calculates the trajectory for angles 0 to 90 to determine what angles produce the maximum height and distance
fn trajectories_at_each_angle(arm_length: f64, body_height: f64, arm_velocity_mph: f64) -> MaxInfo {

    let arm_velocity: f64 = mph_to_fps(arm_velocity_mph);
    let arm_veolocity_sq: f64 = f64::powi(arm_velocity, 2);

    let mut max_info = MaxInfo::new();
    for launch_angle in 0..=90 {
    
        let (x_max, y_max) = trajectory(launch_angle, arm_length, body_height, arm_veolocity_sq);
        max_info.update(x_max, y_max, launch_angle);
    }
    max_info
}

fn trajectory(launch_angle: i32, arm_length: f64, body_height: f64, arm_veolocity_sq: f64) -> (f64, f64) {
    let launch_angle_rad = f64::from(launch_angle).to_radians();
    let launch_angle_cos = launch_angle_rad.cos();
    let launch_angle_tan = launch_angle_rad.tan();
    let launch_angle_cos_sq = f64::powi(launch_angle_cos, 2);
    let arm_height: f64 = launch_angle_cos * arm_length;
    let launch_height: f64 = body_height + arm_height;
    let mut y: f64 = launch_height;
    let mut y_max: f64 = y;
    // Compute trajectory of current launch angle
    let mut x: f64 = 0.0;
    while y >= 0.0 {
        let denominator: f64 = 2.0 * arm_veolocity_sq * launch_angle_cos_sq;
        let x_sq: f64 = f64::powi(x, 2);
        y = launch_height + ( x * launch_angle_tan ) - GRAVITY * ( x_sq / denominator );
        y_max = if y > y_max {y} else {y_max};
        x += X_STEP;
    }
    // Walk x back to when y was above 0
    x -= X_STEP;
    let launch_x: f64 = launch_angle_rad.sin() * arm_length;
    let x_max: f64 = x - launch_x;
    (x_max, y_max)
}

fn get_float64_input (message: &str) -> f64 {
    loop {
        println!("{}", message);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input_num: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // go to the next iteration of the loop
        };

        // Handle negative input
        if input_num >= 0.0 {
            return input_num;
        } else {
            println!("Your input must be positive.")
        }
    }
}

fn mph_to_fps (velocity: f64) -> f64 {
    (velocity * 5280.0) / (60.0 * 60.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const CONVERSION_TOLERANCE: f64 = 0.00001;
    const MAXES_TOLERANCE: f64 = 0.1;

    #[test]
    fn convert_100_mph_to_fps() {
        assert_float_absolute_eq!(mph_to_fps(100.0), 146.66666, CONVERSION_TOLERANCE);
    }

    #[test]
    fn convert_0_mph_to_fps() {
        assert_float_absolute_eq!(mph_to_fps(0.0), 0.0, CONVERSION_TOLERANCE);
    }

    #[test]
    fn trajectories_at_each_angle_for_0_arm_length() {
        let max_info = trajectories_at_each_angle(0.0, 212.0, 100.0);
        assert_eq!(max_info.x_angle, 38);
        assert_float_absolute_eq!(max_info.x_max, 854.7, MAXES_TOLERANCE);
        assert_eq!(max_info.y_angle, 89);
        assert_float_absolute_eq!(max_info.y_max, 546.2, MAXES_TOLERANCE);
    }

    #[test]
    fn trajectory_for_45_angle_and_0_arm_length() {
        let (x_max, y_max) = trajectory(45, 0.0, 212.0, 21511.111111111);
        assert_float_absolute_eq!(x_max, 837.8, MAXES_TOLERANCE);
        assert_float_absolute_eq!(y_max, 379.1, MAXES_TOLERANCE);
    }

    #[test]
    fn trajectory_for_0_angle_and_100_arm_length() {
        let (x_max, y_max) = trajectory(0, 100.0, 212.0, 21511.111111111);
        assert_float_absolute_eq!(x_max, 645.9, MAXES_TOLERANCE);
        assert_float_absolute_eq!(y_max, 312.0, MAXES_TOLERANCE);
    }
}