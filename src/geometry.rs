pub struct FlyingWingDesign {
    pub wingspan_mm: f64, //declare a field inside the struct
    pub root_chord_mm: f64, // 'pub' means files/modules are allowed to access this field
    pub tip_chord_mm: f64, // 'f64' means the value will be a 64 bit floating point number
}

impl FlyingWingDesign { // Rust, attach functions directly to FlyingWingDesign
    pub fn from_wingspan(wingspan_mm: f64) -> Self { //build a basic design from the wingspan
        let root_chord_mm = wingspan_mm * 0.275; // estimate root chord as 27.5% of wingspan
        let tip_chord_mm = root_chord_mm * 0.5; // estimate tip chord as half of root chord

        Self { //Create a new instance of the FlyingWingDesign struct. "Self" is a shortcut for the
               //struct's name. In this case Rust mentally replaces "Self" with "FlyingWingDesign"
            wingspan_mm, // Fill the struct field named wingspan_mm with the value stored in the
                         // variable also named wingspan_mm. Rust allows this shorter syntax because
                         // both names match. Otherwise it would be: wingspan_mm: wingspan_mm
            root_chord_mm, // root_chord_mm: root_chord_mm
            tip_chord_mm, // tip_chord_mm: tip_chord_mm
        }
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

    pub fn aspect_ratio(&self) -> f64 {
        self.wingspan_mm.powi(2) / self.wing_area_mm2() // get the wingspan_mm value
                                                      // ".powi(2)": call the built in f64 function
                                                      // named powi.
                                                      // Raise the number to an integer power, use
                                                      // an exponent of 2.
                                                      // This is = wingspan^2 or 
                                                      // self.wingspan_mm * self.wingspan_mm
    }

    pub fn taper_ratio(&self) -> f64 {
        self.tip_chord_mm / self.root_chord_mm
    }
}
