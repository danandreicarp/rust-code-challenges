#![allow(unused)]

enum Scale {
    Celsius,
    Fahrenheit,
}

struct Temperature {
    degrees: f32,
    scale: Scale,
}

impl Temperature {
    fn new(degrees: f32) -> Self {
        Temperature {
            degrees,
            scale: Scale::Celsius,
        }
    }

    fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Fahrenheit => (self.degrees - 32.0) * 5.0 / 9.0,
            Scale::Celsius => self.degrees,
        }
    }

    fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees * 9.0 / 5.0 + 32.0,
            Scale::Fahrenheit => self.degrees,
        }
    }
}

fn main() {
    let temp = Temperature::new(20.0);

    println!("fun fact: 20°C is an integer in celsius and fahrenheit");
    println!(
        "          {:.1}°C = {:.1}°F",
        temp.to_celsius(),
        temp.to_fahrenheit()
    );
}

#[test]
fn one_degree() {
    let cold = Temperature::new(1.0);
    assert!((cold.to_fahrenheit() - 33.8) < 0.01);
    assert!((cold.to_fahrenheit() - 33.8) >= 0.0);
}

#[test]
fn boiling() {
    let hot = Temperature::new(100.0);
    assert!((hot.to_fahrenheit() - 212.0) < 0.01);
    assert!((hot.to_fahrenheit() - 212.0) >= 0.0);
}

#[test]
fn freezing() {
    let freezing = Temperature {
        degrees: Temperature::new(0.0).to_fahrenheit(),
        scale: Scale::Fahrenheit,
    };

    assert!(freezing.to_celsius() < 0.001);
    assert!(freezing.to_celsius() > -0.01);
}

#[test]
fn comfortable() {
    let freezing = Temperature {
        degrees: 71.6,
        scale: Scale::Fahrenheit,
    };

    assert!(freezing.to_fahrenheit() < 71.61);
    assert!(freezing.to_fahrenheit() > 71.59);
    assert!(freezing.to_celsius() < 22.01);
    assert!(freezing.to_celsius() > 21.99);
}
