use std::io;

const V: f32 = 73.0/75.0;

fn main() {
    println!("Death calculator v1");
    println!("Enter neck diameter width (mm) ¬");
    let neck_width_input: String = input();
    println!("Enter blade-neck contact distance (mm) ¬");
    let blade_length_input: String = input();
    
    let neck_width: f32 = neck_width_input.parse::<f32>().unwrap();
    let blade_length: f32 = blade_length_input.parse::<f32>().unwrap();
    
    
    let n = (3.0*neck_width)/(V.asin().cos()*4.0);
    let shear_angle: f32 = ((3.0*neck_width)/(8.0*n)).asin()*2.0;
    let distance = (blade_length.powi(2) - ((3.0*neck_width)/8.0).powi(2)).powf(0.5);

    println!("Minimum blade length is {}mm", n);
    if blade_length < n {
        println!("Your blades are not long enough for a safe cut");
    } else {
        println!("Your blades are long enough for a safe cut");
        println!("You must stand {}mm away and close your shears to {} radians :)", distance, shear_angle);
    }
    
}


fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}



