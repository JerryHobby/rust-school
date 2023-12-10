use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality(miles: u32) -> (Age, u32) {
    // Check if car has accumulated miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }

    // Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver", "Black", "White"];
    let valid_colors = colors.len();

    // Prevent panic: Check color index for colors array, reset as needed
    // If color > valid_colors, reduce color to valid index
    let mut color = order as usize;
    while color > valid_colors {
        color = color % valid_colors + 1;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {
        // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    } // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

pub fn hash_map_demo() {
    let mut orders: HashMap<i32, Car> = HashMap::new();
    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Order cars, increment "order" for each request
    // Car order #1: Used, Hard top
    for order in 1..=25 {
        car = car_factory(order, (1000 * (order % 3)) as u32);
        orders.insert(order, car);
        //println!("Car order {}: {:?}", order, orders.get(&order));
    }

    // -- unsorted data --
    // for(order, car) in &orders {
    //     //println!("Car ordered {}", order);
    //     println!("Car ordered {}: {:?}", order, &car);
    // }

    // force sorting by key
    for index in 1..=orders.len() as i32 {
        println!("Car ordered {}: {:?}", index, orders.get(&index));
    }
}
