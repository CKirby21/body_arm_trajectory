use std::io;
#[macro_use]
extern crate assert_float_eq;

// Feet per second squared
const GRAVITY: f64 = 32.17404855643;

struct MaxInfo {angle: f64, max: f64}

impl MaxInfo {
    fn new() -> Self { Self { angle: 0.0, max: 0.0 } }

    fn update(&mut self, potential_max: f64, potential_angle: f64) {
        if potential_max > self.max {
            self.angle = potential_angle;
            self.max = potential_max;
        }
    }
}

fn main() {

    let body_height: f64 = get_float64_input("Height of your body (ft):");
    let arm_length: f64 = get_float64_input("Length of your arm (ft):");
    let arm_velocity: f64 = get_float64_input("Velocity of your arm (mph):");

    let arm_velocity: f64 = mph_to_fps(arm_velocity);
    let arm_veolocity_sq: f64 = f64::powi(arm_velocity, 2);
    
    let mut x_info = MaxInfo::new();
    let mut y_info = MaxInfo::new();

    for i in 0..=90 {
        
        let launch_angle: f64 = f64::from(i);
        let launch_angle_rad = launch_angle.to_radians();
        let launch_angle_cos = launch_angle_rad.cos();
        let launch_angle_tan = launch_angle_rad.tan();
        let launch_angle_cos_sq = f64::powi(launch_angle_cos, 2);

        let arm_height: f64 = launch_angle_cos * arm_length;
        let launch_height: f64 = body_height + arm_height;
        let mut y: f64 = launch_height;
        let mut y_peak: f64 = y;

        // Compute trajectory of current launch angle
        let mut x: f64 = 0.0;
        while y >= 0.0 {
            let denominator: f64 = 2.0 * arm_veolocity_sq * launch_angle_cos_sq;
            let x_sq: f64 = f64::powi(x, 2);
            y = launch_height + ( x * launch_angle_tan ) - GRAVITY * ( x_sq / denominator );
            y_peak = if y > y_peak {y} else {y_peak};

            // Roundoff Error Mitigation
            // 0.015625 was chosen because it can be represented exactly in binary
            x += 0.015625;
        }
        // Walk x back to when y was above 0
        x -= 0.015625;

        let launch_x: f64 = launch_angle_rad.sin() * arm_length;
        let x_final: f64 = x - launch_x;

        // Decide if maxes are updated
        x_info.update(x_final, launch_angle);
        y_info.update(y_peak, launch_angle);
    }

    // Print the stuff
    println!("Max Distance was {:.2}ft at an angle of {} degrees", x_info.max, x_info.angle);
    println!("Max Height was {:.2}ft at an angle of {} degrees", y_info.max, y_info.angle);
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
    const TOLERANCE: f64 = 0.00001;

    #[test]
    fn convert_100_mph_to_fps() {
        assert_float_absolute_eq!(mph_to_fps(100.0), 146.66666, TOLERANCE);
    }

    #[test]
    fn convert_0_mph_to_fps() {
        assert_float_absolute_eq!(mph_to_fps(0.0), 0.0, TOLERANCE);
    }
}