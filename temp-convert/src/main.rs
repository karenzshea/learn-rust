use std::io;

fn main() {
    println!("Input a temperature:");

    let temperature: f32;
    loop {
        let mut _t: String = String::new();
        io::stdin().read_line(&mut _t)
            .expect("stdin read_line failed");
        temperature = match _t.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} was not a number", _t);
                continue;
            }
        };
        break;
    }
    println!("Input a scale [F/C]:");
    let mut scale: String = String::new();
    loop {
        io::stdin().read_line(&mut scale)
            .expect("stdin read_line failed");
        scale = scale.trim().to_string();
        let converted: f32 = match scale.as_ref() {
            "F" => fahrenheit_to_celsius(temperature),
            "C" => celsius_to_fahrenheit(temperature),
            &_ => {
                println!("Scale needs to be either F or C");
                continue;
            }
        };
        let target = if scale == "F" { "C" } else { "F" };
        println!("{} {} converts to {} {}", temperature, scale, converted, target);
        break;
    }
}

fn fahrenheit_to_celsius(degrees: f32) -> f32 {
    (degrees - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(degrees: f32) -> f32 {
    (degrees * (9.0 / 5.0)) + 32.0
}
