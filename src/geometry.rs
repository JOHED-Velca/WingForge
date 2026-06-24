pub struct FlyingWingDesign {
    pub wingspan_mm: f64, //declare a field inside the struct
    pub root_chord_mm: f64, // 'pub' means files/modules are allowed to access this field
    pub tip_chord_mm: f64, // 'f64' means the value will be a 64 bit floating point number
    pub sweep_deg: f64, // declare a public field inside the FlyingWingDesign struct, make it public
                        // and make it expect a 64 bit floating point number.
    pub elevon_depth_mm: f64,
}

impl FlyingWingDesign { // Rust, attach functions directly to FlyingWingDesign
    
    pub fn new(
        wingspan_mm: f64,
        root_chord_mm: f64,
        tip_chord_mm: f64,
        sweep_deg: f64,
        elevon_depth_mm: f64,
    ) -> Self {
        Self {
            wingspan_mm,
            root_chord_mm,
            tip_chord_mm,
            sweep_deg,
            elevon_depth_mm,
        }
    }

    pub fn from_wingspan(wingspan_mm: f64) -> Self { //build a basic design from the wingspan
        let root_chord_mm = wingspan_mm * 0.275; // estimate root chord as 27.5% of wingspan
        let tip_chord_mm = root_chord_mm * 0.5; // estimate tip chord as half of root chord
        let sweep_deg = 25.0;
        let elevon_depth_mm = root_chord_mm * 0.14;

        Self { //Create a new instance of the FlyingWingDesign struct. "Self" is a shortcut for the
               //struct's name. In this case Rust mentally replaces "Self" with "FlyingWingDesign"
            wingspan_mm, // Fill the struct field named wingspan_mm with the value stored in the
                         // variable also named wingspan_mm. Rust allows this shorter syntax because
                         // both names match. Otherwise it would be: wingspan_mm: wingspan_mm
            root_chord_mm, // root_chord_mm: root_chord_mm
            tip_chord_mm, // tip_chord_mm: tip_chord_mm
            sweep_deg,
            elevon_depth_mm,
        }
    }

    pub fn half_span_mm(&self) -> f64 {
        self.wingspan_mm / 2.0
    }

    pub fn average_chord_mm(&self) ->f64 { // Create a function named average_chord_mm. "pub" allows code outside this module
                                           // to access it.
                                           // "&self": give me access to the current
                                           // FlyingWingDesign object, but only let me read it.
                                           // "&": means borrow the object instad of copy it.
                                           // "self": Rust special name for the current object.
                                           // "-> f64": the function MUST return a 64 bit float

        (self.root_chord_mm + self.tip_chord_mm) / 2.0 //Look inside the current FlyingWingDesign obj
                                                       //get the value stored in root_chord_mm
                                                       //get the value stored in tip_chord_mm
                                                       //return(self.root_chord_mm + self.tip_chord_mm)/2.0;
    }

    pub fn wing_area_mm2(&self) -> f64 { // create a public function named wing_area_mm2
                                         // "&self": borrow the current FlyingWingDesign obj
                                         // and give me access to that borrowed obj.
                                         //"-> f64": this function returns a 64 bit float number
        self.wingspan_mm * self.average_chord_mm() // "self": look in the current FlyingWingDesign
                                                   // ".wingspan_mm": read the value stored in it.
                                                   // multiply it by 
                                                   // call another function that is
                                                   // inside this same FlyingWingDesign obj.
    }

    pub fn wing_area_dm2(&self) -> f64 {
        self.wing_area_mm2() / 10_000.0
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.wingspan_mm.powi(2) / self.wing_area_mm2() // get the wingspan_mm value
                                                      // ".powi(2)": call the built in f64 function
                                                      // named powi.
                                                      // Raise the number to an integer power, use
                                                      // an exponent of 2.
                                                      // This is = wingspan^2 or 
                                                      // self.wingspan_mm * self.wingspan_mm
    }

    pub fn taper_ratio(&self) -> f64 { // Calculate how much smaller the tip is than the root
        self.tip_chord_mm / self.root_chord_mm
    }

    pub fn sweep_offset_mm(&self) -> f64 { // How far back the tip leading edge moves because of
                                           // of the sweep
        let sweep_rad = self.sweep_deg.to_radians(); // convert degrees to radians because ".tan()"
                                                     // expects radians
        self.half_span_mm() * sweep_rad.tan()
    }

    pub fn trailing_edge_length_mm(&self) -> f64 {
        let root_trailing_edge_x_mm = self.root_chord_mm;

        let tip_trailing_edge_x_mm =
            self.sweep_offset_mm() + self.tip_chord_mm;

        let trailing_edge_offset_mm =
            tip_trailing_edge_x_mm - root_trailing_edge_x_mm;

        (self.half_span_mm().powi(2) + trailing_edge_offset_mm.powi(2)).sqrt()
    }

    pub fn mean_aerodynamic_chord_mm(&self) -> f64 { // calculate MAC, a better chord reference than
                                                     // average chord
        let taper = self.taper_ratio(); // store taper ratio so the formula is easier to read
        (2.0 / 3.0) * self.root_chord_mm * ((1.0 + taper + taper.powi(2)) / (1.0 + taper)) // STD
                                                                                           // tapered
                                                                                           // wing
                                                                                           // MAC
                                                                                           // formula
    }

    pub fn mac_y_position_mm(&self) -> f64 { // Calculate how outboard the MAC is from the wing
                                             // centerline
        let taper = self.taper_ratio(); // Store taper ratio
        (self.wingspan_mm / 6.0) * ((1.0 + 2.0 * taper) / (1.0 + taper)) // Calculate spanwise MAC
                                                                         // position from centerline
    }

    pub fn mac_le_x_position_mm(&self) -> f64 { // calculate how far back the leading edge of MAC is
        let sweep_rad = self.sweep_deg.to_radians(); // convert sweep angle from degres to radians
        self.mac_y_position_mm() * sweep_rad.tan() // Leading edge offset at MAC position
    }

    pub fn recommended_cg_min_mm(&self) -> f64 { // Calculate forward CG estimate from root leading
                                                 // edge
        self.mac_le_x_position_mm() + self.mean_aerodynamic_chord_mm() * 0.15 // beginner forward CG
                                                                              // around 15% of MAC
    }

    pub fn recommended_cg_max_mm(&self) -> f64 { // Calculate rear CG estimate from root leading
                                                 // edge
        self.mac_le_x_position_mm() + self.mean_aerodynamic_chord_mm() * 0.20 // beginner rear CG
                                                                              // around 20 percent
                                                                              // MAC
    }

    pub fn elevon_area_mm2(&self) -> f64 { // estimate total elevon area for both sides
        self.trailing_edge_length_mm() * self.elevon_depth_mm * 2.0 // one elevon per side, so
                                                                    // multiply by 2
    }

    pub fn elevon_area_percent(&self) -> f64 { // calculate elevon area compared to full wing area
        (self.elevon_area_mm2() / self.wing_area_mm2()) * 100.0 // convert area ratio to percentage
    }
}

#[cfg(test)] // Only compile when running cargo test
mod tests { // Create a private module that holds the tests for geometry.rs
    use super::*; // Import everything from the parent geometry.rs file into this

    fn assert_near(actual: f64, expected: f64, tolerance: f64) {
        let difference = (actual - expected).abs();

        assert!(
            difference <= tolerance,
            "actual value {} was not within {} of expected value {}",
            actual,
            tolerance,
            expected,
        );
    }

    fn reference_design() -> FlyingWingDesign {
        FlyingWingDesign::new(
            800.0,
            150.0,
            75.0,
            27.0,
            25.0,
        )
    }

    #[test]
    fn calculates_basic_planform_geometry() {
        let design = reference_design();

        assert_near(design.half_span_mm(), 400.0, 0.001);
        assert_near(design.average_chord_mm(), 112.5, 0.001);
        assert_near(design.wing_area_mm2(), 90_000.0, 0.001);
        assert_near(design.wing_area_dm2(), 9.0, 0.001);
        assert_near(design.aspect_ratio(), 7.111, 0.001);
        assert_near(design.taper_ratio(), 0.5, 0.001);
    }

    #[test]
    fn calculates_sweep_and_trailing_edge_geometry() {
        let design = reference_design();

        assert_near(design.sweep_offset_mm(), 203.810, 0.001);
        assert_near(design.trailing_edge_length_mm(), 420.229, 0.001);
    }
    
    #[test]
    fn calculates_mac_and_cg_estimates() {
        let design = reference_design();

        assert_near(design.mean_aerodynamic_chord_mm(), 116.667, 0.001);
        assert_near(design.mac_y_position_mm(), 177.778, 0.001);
        assert_near(design.mac_le_x_position_mm(), 90.582, 0.001);
        assert_near(design.recommended_cg_min_mm(), 108.082, 0.001);
        assert_near(design.recommended_cg_max_mm(), 113.916, 0.001);
    }

    #[test]
    fn calculates_elevon_area_from_corrected_trailing_edge() { // Test elevon area calculations
        let design = reference_design(); // Create the reference design for this test

        assert_near(design.elevon_area_mm2(), 21_011.429, 0.001); // Check total elevon area for
                                                                  // both sides
        assert_near(design.elevon_area_percent(), 23.346, 0.001); //Check elevon area as percent of
                                                                  //wing area
    } // End elevon area test
} // End tests module
