use std::io;

const GRAVITY_FPS: f64 = 32.1741;

// #[derive(Debug)]
struct MaxInfo {angle: f64, max: f64}

fn main() {

    let body_height_message = "Please input the height of your body in ft:".to_string();
    let arm_length_message = "Please input the length of your arm in ft:".to_string();
    let arm_velocity_message = "Please input the velocity of your arm in mph:".to_string();
    let body_height: f64 = get_float64_input(body_height_message);
    let arm_length: f64 = get_float64_input(arm_length_message);
    let arm_velocity: f64 = get_float64_input(arm_velocity_message);

    // Convert from mph to fps
    let arm_velocity: f64 = (arm_velocity * 5280.0) / (60.0 * 60.0);
    
    // Determine launch angle that yields the greastest distance (x)
    let mut launch_angle: f64 = 90.0;
    let mut x_info = MaxInfo {angle:0.0, max:0.0};
    let mut y_info = MaxInfo {angle:0.0, max:0.0}; 
    while launch_angle >= 0.0 {

        let launch_angle_rad = launch_angle.to_radians();
        let launch_angle_cos = launch_angle_rad.cos();

        let arm_height: f64 = launch_angle_cos * arm_length;
        let launch_height: f64 = body_height + arm_height;
        let mut y: f64 = launch_height;
        let mut y_peak: f64 = y;

        let launch_x: f64 = launch_angle_rad.sin() * arm_length;
        let mut x: f64 = launch_x;

        while y >= 0.0 {
            let x_squared: f64 = f64::powi(x, 2);
            let arm_velocity_squared: f64 = f64::powi(arm_velocity, 2);
            y = launch_height + ( x * launch_angle_rad.tan() ) - ( (GRAVITY_FPS * x_squared) / (2.0 * arm_velocity_squared * f64::powi(launch_angle_cos, 2)) );
            y_peak = if y > y_peak {y} else {y_peak};
            x += 0.1;
        }
        // Decide if maxes are overwritten
        x_info = if x > x_info.max { 
            MaxInfo {angle:launch_angle, max:x} 
        } else {
            x_info
        };
        y_info = if y_peak > y_info.max { 
            MaxInfo {angle:launch_angle, max:y_peak} 
        } else {
            y_info
        };

        // Check the next angle
        launch_angle -= 0.1;
    }

    // Print the stuff
    println!("Best Distance was {:.2}ft at an angle of {:.2} degrees", x_info.max, x_info.angle);
    println!("Best Height was {:.2}ft at an angle of {:.2} degrees", y_info.max, y_info.angle);
}

fn get_float64_input (message: String) -> f64 {
    loop {
        println!("{}", message);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue, // go to the next iteration of the loop
        };
    }
}
