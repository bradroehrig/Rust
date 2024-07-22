enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Green;

    match light {
    TrafficLight::Red => println!("Stop!"),
    TrafficLight::Yellow => println!("Caution!"),
    TrafficLight::Green => println!("Go!"),
    }
}