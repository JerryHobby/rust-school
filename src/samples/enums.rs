#[derive(Debug)]
enum DiskType {
    SSD,
    HDD,
    FDD,
    OPTICAL,
    THUMB,
}

#[derive(Debug)]
enum DiskSize {
    Bytes(u32),
    MB(u32),
    GB(u32),
    TB(u32),
}

#[derive(Debug)]
struct Disk {
    name: String,
    path: Option<String>,
    mount_point: Option<String>,
    disk_type: DiskType,
    capacity: DiskSize,
    free: DiskSize,
    removable: bool,
}

impl Disk {
    fn new(
        name: String,
        path: Option<String>,
        mount_point: Option<String>,
        disk_type: DiskType,
        capacity: DiskSize,
        free: DiskSize,
        removable: bool,
    ) -> Disk {
        Disk {
            name,
            path,
            mount_point,
            disk_type,
            capacity,
            free,
            removable,
        }
    }

    fn print(&self) {
        println!("Disk: {}", self.name);
        println!("    path: {}", self.path.as_ref().unwrap());
        println!("    mount_point: {}", self.mount_point.as_ref().unwrap());
        println!("    disk_type: {:?}", self.disk_type);
        println!("    capacity: {:?}", self.capacity);
        println!("    free: {:?}", self.free);
        println!("    removable: {}", self.removable);
    }
}

pub fn disk_test() {
    let mut drives: Vec<Disk> = Vec::new();

    drives.push(Disk::new(
        String::from("C:"),
        Some(String::from("C:\\Users\\jim\\Documents")),
        Some(String::from("C:")),
        DiskType::SSD,
        DiskSize::TB(2),
        DiskSize::GB(128),
        false,
    ));

    drives.push(Disk::new(
        String::from("Thumb Drive"),
        None,
        None,
        DiskType::FDD,
        DiskSize::GB(256),
        DiskSize::GB(128),
        true,
    ));

    drives.push(Disk::new(
        String::from("D:"),
        Some(String::from("D:\\")),
        Some(String::from("D:")),
        DiskType::HDD,
        DiskSize::MB(4700),
        DiskSize::GB(128),
        false,
    ));

    drives.push(Disk::new(
        String::from("Floppy"),
        None,
        None,
        DiskType::HDD,
        DiskSize::Bytes(720_000),
        DiskSize::Bytes(128_000),
        false,
    ));

    for drive in drives.iter() {
        match drive.disk_type {
            DiskType::SSD => {
                println!("Type: SSD");
            }
            DiskType::HDD => {
                println!("Type: HDD");
            }
            DiskType::FDD => {
                println!("Type: FDD");
            }
            DiskType::OPTICAL => {
                println!("Type: Optical");
            }
            DiskType::THUMB => {
                println!("Type: Thumb: {:?}", drive);
            }
        }
    }

    for drive in drives.iter() {
        match drive.path {
            None => {
                println!("Path: None: {:?}", drive.name);
            }
            _ => {
                println!("Path: Some: {:?}", drive.name);
            }
        }
    }

    for drive in drives.iter() {
        match drive.capacity {
            DiskSize::Bytes(c) => {
                println!("Size: GB: {:.2}", c as f64 / 1_000_000f64);
            }
            DiskSize::MB(c) => {
                println!("Size: GB: {:.2}", c as f64 / 1000f64);
            }
            DiskSize::GB(c) => {
                println!("Size: GB: {:.2}", c as f64 / 1.0);
            }
            DiskSize::TB(c) => {
                println!("Size: GB: {:.2}", c as f64 * 1000f64);
            }
        }
    }
}

enum Shape {
    Rectangle { width: u32, height: u32 },
    Square(u32),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Square(side) => (side * side) as f64,
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        }
    }
}

pub fn calc_shapes() {
    let shapes = [
        Shape::Rectangle {
            width: 10,
            height: 20,
        },
        Shape::Square(10),
        Shape::Circle(10.0),
    ];

    let space: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Rectangle { width, height } => {
                println!("Rectangle: {} x {} = {}", width, height, shape.area());
                shape.area()
            }
            Shape::Square(side) => {
                println!("Square: {} x {} = {}", side, side, shape.area());
                shape.area()
            }
            Shape::Circle(radius) => {
                println!("Circle: {} x {} = {}", radius, radius, shape.area());
                shape.area()
            }
        })
        .sum();

    println!("Total space: {:.2}", space);
}
