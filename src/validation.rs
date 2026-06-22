use crate::geometry::FlyingWingDesign;

pub fn print_validation(design: &FlyingWingDesign) {
    println!("\n--- Geometry Validation ---");

    validate_positive_dimensions(design);
    validate_chord_relationship(design);
    validate_sweep_angle(design);
    validate_aspect_ratio(design);
    validate_elevon_size(design);
}

fn validate_positive_dimensions(design: &FlyingWingDesign) {
    if design.wingspan_mm <= 0.0 {
        println!("Invalid: wingspan must be greater than 0 mm.");
    }

    if design.root_chord_mm <= 0.0 {
        println!("Invalid: root chord must be greater than 0 mm.");
    }

    if design.tip_chord_mm <= 0.0 {
        println!("Invalid: tip chord must be greater than 0.");
    }

    if design.elevon_depth_mm <= 0.0 {
        println!("Invalid: elevon depth must be greater than 0 mm.");
    }
}

fn validate_chord_relationship(design: &FlyingWingDesign) {
    if design.tip_chord_mm > design.root_chord_mm {
        println!("Warning: tip chord is larger than root chord. This is unusual for this V1 flying-wing layout.");
    }

    if design.tip_chord_mm < design.root_chord_mm * 0.30 {
        println!("Warning: tip chord is very small compared to root chord. Tip stall risk may increase.");
    }
}

fn validate_sweep_angle(design: &FlyingWingDesign) {
    if design.sweep_deg < 0.0 {
        println!("Warning: negative sweep is unusual for this beginner flying wing assistant.");
    } else if design.sweep_deg < 15.0 {
        println!("Note: low sweep may reduce pitch stability margin for a tailess flying wing.");
    } else if design.sweep_deg > 40.0 {
        println!("Warning: high sweep may reduce usable wing area and complicate handling.");
    } else {
        println!("Sweep angle check: OK.");
    }
}

fn validate_aspect_ratio(design: &FlyingWingDesign) {
    let aspect_ratio = design.aspect_ratio();

    if aspect_ratio < 3.0 {
        println!("Warning: very low aspect ratio. The wing may need more thrust and ly less efficiently.");
    } else if aspect_ratio > 7.0 {
        println!("Warning: high aspect ratio for a small flying wing. Structure and CG sensitivity may become more important.");
    } else {
        println!("Aspect ratio check: OK.");
    }
}

fn validate_elevon_size(design: &FlyingWingDesign) {
    let elevon_percent = design.elevon_area_percent();

    if elevon_percent < 12.0 {
        println!("Warning: elevons may be too small for strong pitch and roll control.");
    } else if elevon_percent > 28.0 {
        println!("Warning: elevons are very large. The aircraft may feel sensitive and draggy.");
    } else {
        println!("Elevon size check: OK.");
    }
}
