use crate::geometry::FlyingWingDesign; // 'use' imports a type, 'crate::' means start from this
                                       // project's root module
pub fn print_design_analysis(design: &FlyingWingDesign) { //This declares a public function named
                                                          //print_... and it receives a read-only
                                                          //reference to a FlyingWingDesign called
                                                          //design. 'Borrowing'
    analyze_aspect_ratio(design); // Calls another function and passes the same design reference to
                                  // analyze aspect ratio.
    analyze_taper_ratio(design); 
    analyze_elevons(design);
    analyze_cg(design);
}

fn analyze_aspect_ratio(design: &FlyingWingDesign) {
    let aspect_ratio = design.aspect_ratio();

    if aspect_ratio < 3.5 {
        println!("Aspect ratio {:.2}: very compact wing.", aspect_ratio);
        println!("Trade-off: strong and compact, but less efficient and may need more thrust.");
        println!("");
    } else if aspect_ratio <= 6.0 {
        println!("Aspect ratio {:.2}: balanced flying-wing range.", aspect_ratio);
        println!("Trade-off: good beginner compromise between stability, strengh and efficiency.");
        println!("");
    } else {
        println!("Aspect ratio {:.2}: glider like wing.", aspect_ratio);
        println!("Trade-off: better glide efficiency, but more bending stress and harder construction.");
        println!("");
    }
}

fn analyze_taper_ratio(design: &FlyingWingDesign){
    let taper_ratio = design.taper_ratio();
 
    if taper_ratio < 0.35 {
        println!("Taper ratio {:.2}: Agressive taper.", taper_ratio);
        println!("Warning: narrow tips can increase tip-stall risk.");
        println!("");
    } else if taper_ratio <= 0.65 {
        println!("Taper ratio {:.2}: reasonable flying-wing taper.", taper_ratio);
        println!("Trade-off: reduces drag and weight without making the tips too small.");
        println!("");
    } else {
        println!("Taper ratio {:.2}: mild taper.", taper_ratio);
        println!("Trade-off: easier build and gentler stall, but more area near the tips.");
        println!("");
    }
}

fn analyze_elevons(design: &FlyingWingDesign){
    let elevon_percent = design.elevon_area_percent();
 
    if elevon_percent < 15.0 {
        println!("Elevon area {:.1}%: possibly too small.", elevon_percent);
        println!("Warning: control authority may be weak, especially at low speed.");
        println!("");
    } else if elevon_percent <= 25.0 {
        println!("Elevon area {:.1}%: healthy beginner range.", elevon_percent);
        println!("Trade-off: should give good control authority without being extreme.");
        println!("");
    } else {
        println!("Elevon area {:.1}%: large elevons.", elevon_percent);
        println!("Trade-off: strong control response, but bigger drag and more sensitivity.");
        println!("");
    }
}

fn analyze_cg(design: &FlyingWingDesign){
    let cg_min = design.recommended_cg_min_mm();
    let cg_max = design.recommended_cg_max_mm();
    let cg_range = cg_max - cg_min;

    println!("CG range: {:.1} mm to {:.1} mm from root leading edge.", cg_min, cg_max);
    println!("CG window size: {:.1} mm", cg_range);

    if cg_range < 8.0 {
        println!("Warning: small CG window. Balance will need to be careful.");
    } else {
        println!("Trade-off: start near the forward CG value for a safer first flight.")
    }

}
