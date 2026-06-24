mod geometry; // This loads the geometry.rs file as a module.
//mod warnings; // This loads the warnings.rs file as a module.
mod tradeoffs; // This loads the tradeoffs.rs file as a module.
mod analysis; // This loads the analysis.rs file as a module.
mod input; // This loads the input.rs file as a module.
mod validation;
mod performance;
mod recommendation;

fn main() { // This declares the main function where the program starts.
    println!("Wing Forge V1 - Flying Wing Assistant"); // This prints the program title.

    let choice = input::read_choice(
        "\nChoose design mode:\n1) Auto design from wingspan\n2) Manual geometry input"
    ); // This calls read_choice from input.rs and stores the user's menu choice.

    let design = if choice == 1 { // This creates a design using automatic mode if the user enters 1.
        let wingspan_mm = input::read_number("Enter wingspan in millimeters:"); // This asks for wingspan and stores it as an f64.
        geometry::FlyingWingDesign::from_wingspan(wingspan_mm) // This creates a design using only the wingspan.
    } else { // This runs when the user chooses anything other than 1.
        let wingspan_mm = input::read_number("Enter wingspan in millimeters:"); // This asks for full wingspan.
        let root_chord_mm = input::read_number("Enter root chord in millimeters:"); // This asks for root chord.
        let tip_chord_mm = input::read_number("Enter tip chord in millimeters:"); // This asks for tip chord.
        let sweep_deg = input::read_number("Enter leading edge sweep angle in degrees:"); // This asks for sweep angle.
        let elevon_depth_mm = input::read_number("Enter elevon depth in millimeters:"); // This asks for elevon depth.

        geometry::FlyingWingDesign::new(
            wingspan_mm,
            root_chord_mm,
            tip_chord_mm,
            sweep_deg,
            elevon_depth_mm,
        ) // This creates a design from the manual values.
    };

    let weight_g = input::read_number("Enter estimated aircraft weight in grams:");
    let thrust_g = input::read_number("Enter estimated motor thrust in grams:");
    print_geometry(&design); // This calls the geometry printing helper function.
  
    validation::print_validation(&design); //This calls the validation module and apsses a read_only

    performance::print_performance(&design, weight_g, thrust_g);

    println!("\n--- Trade-offs ---"); // This prints the trade-offs section title.
    tradeoffs::print_tradeoffs(&design); // This prints basic trade-off explanations.

    println!("\n--- Design Analysis ---"); // This prints the design analysis section title.
    analysis::print_design_analysis(&design); // This prints deeper assistant-style analysis.

    recommendation::print_recommendation(
        &design,
        weight_g,
        thrust_g,
    );
}

fn print_geometry(design: &geometry::FlyingWingDesign) { // This declares a private helper function that receives a read-only reference to FlyingWingDesign.
    println!("\n--- Geometry ---"); // This prints the geometry section title.
    println!("Wingspan: {:.1} mm", design.wingspan_mm); // This prints the wingspan with 1 decimal.
    println!("Half span: {:.1} mm", design.half_span_mm()); // This prints the calculated half span.
    println!("Root chord: {:.1} mm", design.root_chord_mm); // This prints the root chord.
    println!("Tip chord: {:.1} mm", design.tip_chord_mm); // This prints the tip chord.
    println!("Sweep angle: {:.1}°", design.sweep_deg); // This prints the sweep angle.
    println!("Sweep offset: {:.1} mm", design.sweep_offset_mm()); // This prints the calculated sweep offset.
    println!("Trailing edge length: {:.1} mm", design.trailing_edge_length_mm()); // This prints the trailing edge length.
    println!("Average chord: {:.1} mm", design.average_chord_mm()); // This prints the average chord.
    println!("MAC spanwise position: {:.1} mm from centerline.", design.mac_y_position_mm()); // This prints the mean aerodynamic chord.
    println!("MAC leading edge position: {:.1} mm aft of root leading edge.", design.mac_le_x_position_mm()); // This prints the MAC leading edge position.
    println!("Wing area: {:.1} mm²", design.wing_area_mm2()); // This prints wing area in square millimeters.
    println!("Wing area: {:.2} dm²", design.wing_area_dm2()); // This prints wing area in square decimeters.
    println!("Aspect ratio: {:.2}", design.aspect_ratio()); // This prints aspect ratio with 2 decimals.
    println!("Taper ratio: {:.2}", design.taper_ratio()); // This prints taper ratio with 2 decimals.
    println!("Preliminary geometric CG estimate: {:.1} mm to {:.1} mm from root leading edge", design.recommended_cg_min_mm(), design.recommended_cg_max_mm()); // This prints the recommended CG range.
    println!("Elevon depth: {:.1} mm", design.elevon_depth_mm); // This prints elevon depth.
    println!("Elevon area: {:.1} mm²", design.elevon_area_mm2()); // This prints total elevon area.
    println!("Elevon area percent: {:.1}%", design.elevon_area_percent()); // This prints elevon area as percentage of wing area.
}
