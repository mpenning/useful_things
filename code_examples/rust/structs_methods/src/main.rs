//use snafu::{prelude::ensure, prelude::*, Whatever};
use snafu::{prelude::ensure, prelude::Snafu, Whatever};

#[derive(Debug, Snafu)]
enum VehicleError {
    // InvalidYear error will show up as InvalidYearSnafu
    #[snafu(display("Year may not be less than 1900, but it was {year}"))]
    InvalidYear { year: i16 },
}

#[derive(Copy, Clone, Debug)]
enum Manufacturer {
    Nissan,
    Toyota,
    Ford,
    Chevrolet,
    Boeing,
    JohnDeere,
}

#[derive(Copy, Clone, Debug)]
enum Class {
    Car,
    Truck,
    SUV,
    TractorTrailer,
    Airplane,
}

#[derive(Copy, Clone)]
struct Vehicle {
    manufacturer: Manufacturer,
    year: i16,
    class: Class,
}

impl Vehicle {
    fn new(manufacturer: Manufacturer, year: i16, class: Class) -> Result<Self, VehicleError> {
        let mut this = &Self {
            manufacturer,
            year,
            class,
        };

        // Throw an error if year is invalid...
        ensure!(year >= 1900, InvalidYearSnafu { year });

        // Return the Vehicle instance
        Ok(*this)
    }
}

fn main() {
    let vehicle = Vehicle::new(Manufacturer::Toyota, 2020, Class::Car).unwrap();
    println!(
        "We are in a {} {:?} {:?}",
        vehicle.year.to_string(),
        vehicle.manufacturer,
        vehicle.class,
    );
}
