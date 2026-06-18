use crate::geometry::FlyingWingDesign; // Bring the FlyingWingDesign type into this file

pub fn print_tradeoffs(design: &FlyingWingDesign) {
    let aspect_ratio = design.aspect_ratio();

    if aspect_ratio < 4.0 {
        println!("Trade-off: This wing should be compact and strong, but it may need more power.");
    } else if aspect_ratio > 6.5 {
        println!("Trade-off: This wing may glide better, but it may be harder to build strong.");
    } else {
        println!("Trade-off: This is a balanced beginner range for small UAV flying wing.");
    }

    println!("Changing wingspan affects area, stability, structure, weight, and required motor thrust.");
}
