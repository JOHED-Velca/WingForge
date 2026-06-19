mod geometry; // Rust, look for file called geometry.rs and make its functions available here
mod warnings; // Rust, look for warnings.rs so this file can ask it to check the design
mod tradeoffs; // Rust, look for tradeoffs.rs so this file can print explanations
mod analysis;

use std::io; //Inside the standard library, use the io module so we can read user input

fn main() { //Rust, this is where the program starts running
    println!("Wing Forge V1 - Design Flying Wing Assistant"); //Print the title
    println!("Enter wingspan in millimeters:"); // Program ask the user for first design input
    
    let mut input = String::new(); //Create a mutable empty string

    io::stdin() //Rust, access the keyboard input stream
        .read_line(&mut input) //give read_line a mutable reference so it can write the user's text
        .expect("Failed to read input"); //carsh this message if you fail reading the input

    let wingspan_mm: f64 = input //Rust, take the text store in the variable input
        .trim() // remove extra spaces and the Enter key from the text
        .parse() // try to convert the text to a number
        .expect("Please enter a valid number"); // crash with this msg if text is not a number

    let design = geometry::FlyingWingDesign::from_wingspan(wingspan_mm); //create a flying wing
                                                                         //design usng the winspan

    println!("\n--- Geometry ---"); // Print the title
    println!("Wingspan: {:.1} mm", design.wingspan_mm); // Show the winspan with one decimal
    println!("Half span: {:.1} mm", design.half_span_mm()); // Show distance from center to one tip
    println!("");
    println!("Root chord: {:.1} mm", design.root_chord_mm); // show the root chord
    println!("Tip chord: {:.1} mm", design.tip_chord_mm); // show the tip chord
    println!("");
    println!("Sweep angle: {:.1}° mm", design.sweep_deg); // Show the winspan with one decimal
    println!("Sweep offset: {:.1} mm", design.sweep_offset_mm()); // Show the winspan with one decimal
    println!("");
    println!("Trailing edge length: {:.1} mm", design.trailing_edge_length_mm()); // Show the winspan with one decimal
    println!("");
    println!("Average chord: {:.1} mm", design.average_chord_mm()); // calc & show avrg chord
    println!("");
    println!("MAC: {:.1} mm", design.mean_aerodynamic_chord_mm()); // calc & show mean aerodynamic chord
    println!("MAC leading edge position: {:.1} mm", design.mac_le_x_position_mm()); // calc & show where MAC starts
    println!("");                                                                // from leading edge
    println!("Wing area: {:.1} mm²", design.wing_area_mm2()); // calc & show wing area
    println!("Wing area: {:.2} dm²", design.wing_area_dm2()); // calc & show wing area
    println!("");
    println!("Aspect ratio: {:.2} mm", design.aspect_ratio()); // calc & show aspect ratio
    println!("Taper ratio: {:.2} mm", design.taper_ratio()); // calc & show taper ratio
    println!("");
    println!("Recommended CG range: {:.1} mm to {:.1} mm from root leading edge", design.recommended_cg_min_mm(), design.recommended_cg_max_mm());
    println!("");
    println!("Elevon depth: {:.1} mm", design.elevon_depth_mm);
    println!("Elevon area: {:.1} mm²", design.elevon_area_mm2());
    println!("Elevon area percent: {:.1}%", design.elevon_area_percent());
    println!("");
    println!("\n--- Design Analysis ---");
    analysis::print_design_analysis(&design);
    println!("");
    println!("\n--- Warnings ---"); // Print a warning section
    warnings::print_warnings(&design); // ask the awrnings module to inspect the design

    println!("\n---Trade-offs---"); // Program, print a trade-off section
    tradeoffs::print_tradeoffs(&design); //Ask the tradeoffs module to explain consequences
}
