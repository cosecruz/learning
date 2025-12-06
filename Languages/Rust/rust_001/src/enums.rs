pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Green => "Green",
        TrafficLight::Red => "Red",
        TrafficLight::Yellow => "Yellow",
    }
}

pub fn combinator() {
    let x = Some(5)
        .map(|x| x * 2)
        .and_then(|x| if x > 8 { Some(x) } else { None })
        .unwrap_or(0);
}
