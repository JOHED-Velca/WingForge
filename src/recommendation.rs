use crate::geometry::FlyingWingDesign;
use crate::performance::{thrust_to_weight_ratio, wing_loading_g_dm2};

pub fn print_recommendation(
    design: &FlyingWingDesign,
    weight_g: f64,
    thrust_g: f64,
) {
    println!("\n--- Final Recommendation ---");

    let wing_loading =
        wing_loading_g_dm2(weight_g, design.wing_area_dm2());

    let thrust_ratio =
        thrust_to_weight_ratio(thrust_g, weight_g);

    let cg_min = design.recommended_cg_min_mm();
    let cg_max = design.recommended_cg_max_mm();
    let cg_window = cg_max - cg_min;

    let mut risk_count = 0;

    if wing_loading > 35.0 {
        risk_count += 1;
    }

    if thrust_ratio < 1.0 {
        risk_count += 1;
    }

    if design.aspect_ratio() > 7.0 {
        risk_count += 1;
    }

    if design.elevon_area_percent() > 28.0 {
        risk_count += 1;
    }

    if cg_window < 8.0 {
        risk_count += 1;
    }

    if risk_count == 0 {
        println!("Design status: good starting point.");
    } else if risk_count <= 2 {
        println!("Design status: workable with some cautions.");
    } else {
        println!("Design status: risky and should be revised before construction.");
    }

    print_main_issues(
        design,
        wing_loading,
        thrust_ratio,
        cg_window,
    );

    println!(
        "First-flight CG: begin near {:.1} mm from the root leading edge.",
        cg_min
    );

    println!(
        "Move the CG rearward only in small steps after successful test flights."
    );
}

fn print_main_issues(
    design: &FlyingWingDesign,
    wing_loading: f64,
    thrust_ratio: f64,
    cg_window: f64,
) {
    println!("Main observations:");

    if wing_loading > 35.0 {
        println!(
            "- High wing loading: reduce weight or increase wing area."
        );
    }

    if thrust_ratio < 1.0 {
        println!(
            "- Limited thrust: consider a stronger propulsion setup."
        );
    }

    if design.aspect_ratio() > 7.0 {
        println!(
            "- High aspect ratio: make sure the wing structure is sufficiently strong."
        );
    }

    if design.elevon_area_percent() > 28.0 {
        println!(
            "- Large elevons: reduce control throws for the first flight."
        );
    }

    if cg_window < 8.0 {
        println!(
            "- Narrow CG range: balance the aircraft carefully."
        );
    }

    if wing_loading <= 35.0
        && thrust_ratio >= 1.0
        && design.aspect_ratio() <= 7.0
        && design.elevon_area_percent() <= 28.0
        && cg_window >= 8.0
    {
        println!("- No major issues detected by the current V1 checks.");
    }
}
