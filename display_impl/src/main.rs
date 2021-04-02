use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Complex: {{real: {},imag: {}}}",
            self.real, self.imaginary
        )
    }
}
fn main() {
    println!(
        "Debug value: {:?}",
        Complex {
            imaginary: 3.2,
            real: 4.9
        }
    );
    println!(
        "Display value: {}",
        Complex {
            imaginary: 3.2,
            real: 4.9
        }
    );
}
