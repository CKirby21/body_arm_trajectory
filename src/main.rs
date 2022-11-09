use std::io;

// Feet per second squared
const GRAVITY: f64 = 32.17404855643;

struct MaxInfo {angle: f64, max: f64}

impl MaxInfo {
    fn new() -> Self { Self { angle: 0.0, max: 0.0 } }
}

fn main() {

    let body_height: f64 = get_float64_input("Height of your body (ft):");
    let arm_length: f64 = get_float64_input("Length of your arm (ft):");
    let arm_velocity: f64 = get_float64_input("Velocity of your arm (mph):");

    let arm_velocity: f64 = mph_to_fps(arm_velocity);
    let arm_veolocity_sq: f64 = squared(arm_velocity);
    
    let mut x_info = MaxInfo::new();
    let mut y_info = MaxInfo::new();

    for i in 0..=90 {
        
        let launch_angle: f64 = f64::from(i);
        let launch_angle_rad = launch_angle.to_radians();
        let launch_angle_cos = launch_angle_rad.cos();
        let launch_angle_tan = launch_angle_rad.tan();
        let launch_angle_cos_sq = squared(launch_angle_cos);

        let arm_height: f64 = launch_angle_cos * arm_length;
        let launch_height: f64 = body_height + arm_height;
        let mut y: f64 = launch_height;
        let mut y_peak: f64 = y;

        // Compute trajectory of current launch angle
        let mut x: f64 = 0.0;
        while y >= 0.0 {
            let denominator: f64 = 2.0 * arm_veolocity_sq * launch_angle_cos_sq;
            y = launch_height + ( x * launch_angle_tan ) - GRAVITY * ( squared(x) / denominator );
            y_peak = if y > y_peak {y} else {y_peak};

            // Roundoff Error Mitigation
            // 0.03125 was chosen because it can be represented exactly in binary 66958.44
            x += 0.03125;
        }
        // Walk x back to when y was above 0
        x -= 0.03125;

        let launch_x: f64 = launch_angle_rad.sin() * arm_length;
        let x_final: f64 = x - launch_x;

        // Decide if maxes are updated
        if x_final > x_info.max {
            x_info.angle = launch_angle;
            x_info.max = x_final;
        }
        if y_peak > y_info.max {
            y_info.angle = launch_angle;
            y_info.max = y_peak;
        }
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

fn squared (num: f64) -> f64 {
    f64::powi(num, 2)
}
