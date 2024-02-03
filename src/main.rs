use std::io;

enum Planets {
    Sun,
    Mercury,
    Venus,
    Moon,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

fn calculate_weight(w: f32, p: Planets) -> Option<f32> {
    match p {
        Planets::Sun => Some((w / 9.807) * 274.0),
        Planets::Mercury => Some((w / 9.807) * 3.7),
        Planets::Venus => Some((w / 9.807) * 8.87),
        Planets::Moon => Some((w / 9.807) * 1.62),
        Planets::Mars => Some((w / 9.807) * 3.71),
        Planets::Jupiter => Some((w / 9.807) * 24.79),
        Planets::Saturn => Some((w / 9.807) * 10.44),
        Planets::Uranus => Some((w / 9.807) * 8.87),
        Planets::Neptune => Some((w / 9.807) * 11.15),
    }
}

fn main() {
    loop {
        println!("Enter your weight:");
        let mut weight = String::new();
        io::stdin()
            .read_line(&mut weight)
            .expect("Cannot read weight!");

        println!("Select a planet:");
        println!(
            "[1]Sun\n[2]Mercury\n[3]Venus\n[4]Moon\n[5]Mars\n[6]Jupiter\n[7]Saturn\n[8]Uranus\n[9]Neptune"
        );
        let mut planet = String::new();
        io::stdin()
            .read_line(&mut planet)
            .expect("Cannot get planet!");

        let weight: f32 = match weight.trim().parse::<f32>() {
            Ok(w) => match planet.trim() {
                "1" => calculate_weight(w, Planets::Sun).unwrap(),
                "2" => calculate_weight(w, Planets::Mercury).unwrap(),
                "3" => calculate_weight(w, Planets::Venus).unwrap(),
                "4" => calculate_weight(w, Planets::Moon).unwrap(),
                "5" => calculate_weight(w, Planets::Mars).unwrap(),
                "6" => calculate_weight(w, Planets::Jupiter).unwrap(),
                "7" => calculate_weight(w, Planets::Saturn).unwrap(),
                "8" => calculate_weight(w, Planets::Uranus).unwrap(),
                "9" => calculate_weight(w, Planets::Neptune).unwrap(),
                _ => {
                    println!("Invalid option!");
                    return;
                }
            },
            Err(e) => {
                println!("Error while selecting planet: {}", e);
                return;
                //continue;
            }
        };

        println!("Your weigh {} kilograms on the planet you chose.", weight);
    }
}
