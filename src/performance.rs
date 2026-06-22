use crate::geometry::FlyingWingDesign;

pub fn print_performance(design: &FlyingWingDesign, weight_g: f64, thrust_g: f64) {
    println!("\n--- Performance Estimate ---");

    let wing_loading = wing_loading_g_dm2(weight_g, design.wing_area_dm2());
    let thrust_ratio = thrust_to_weight_ratio(thrust_g, weight_g);

    println!("Aircraft weight: {:.1} g", weight_g);
    println!("Estimated thrust: {:.1} g", thrust_g);
    println!("Wing Loading: {:.1} g/dm²", wing_loading);
    println!("Thrust-to-weight ratio: {:.2}", thrust_ratio);

    analyze_wing_loading(wing_loading);
    analyze_thrust_to_weight(thrust_ratio);
}

pub fn wing_loading_g_dm2(weight_g: f64, wing_area_dm2: f64) -> f64 {
    weight_g / wing_area_dm2 //divides grams by square decimeters to calculate g/dm²
}

pub fn thrust_to_weight_ratio(thrust_g: f64, weight_g: f64) -> f64 {
    thrust_g / weight_g
}

fn analyze_wing_loading(wing_loading: f64) {
    if wing_loading < 20.0 {
        println!("Wing loading result: light and slow-flying design.");
        println!("Trade-off: easier hand launch and slower landing, but more affected by wind.");
    } else if wing_loading <= 35.0 {
        println!("Wing loading result: balanced small UAV range."); // This prints the classification.
        println!("Trade-off: reasonable launch speed, landing speed, and wind handling.");
    } else if wing_loading <= 50.0 {
        println!("Wing loading result: fast and heavier-loaded wing."); // This prints the classification.
        println!("Warning: it may need more thrust, faster launch speed, and more landing space.");
    } else {
        println!("Wing loading result: very high for a small hand-launched wing."); // This prints the classification.
        println!("Warning: launch and stall speed may be demanding. Consider more wing area or less weight."); // This explains the consequence.
    }
}

fn analyze_thrust_to_weight(thrust_ratio: f64) {
    if thrust_ratio < 0.7 {
        println!("Thrust result: weak power setup.");
        println!("Warning: this may struggle to launch or climb safely.");
    } else if thrust_ratio < 1.0 {
        println!("Thrust result: flyable but not very strong.");
        println!("Trade-off: may work for efficient cruising, but launch and climb margin are limited.");
    } else if thrust_ratio <= 1.5 {
        println!("Thrust result: healthy thrust range."); // This prints the thrust classification.
        println!("Trade-off: good balance between launch safety, climb ability, and efficiency.");
    } else {
        println!("Thrust result: strong power setup.");
        println!("Trade-off: strong launch and climb, but may add current draw, heat, and battery stress.");
    }
}
