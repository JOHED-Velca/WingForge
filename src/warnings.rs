use crate::geometry::FlyingWingDesign; // Rust, use the FlyingWingDesign type from the geometry
                                       // module.

pub fn print_warnings(design: &FlyingWingDesign) { // declare a public accessible function named
                                                   // print warnings. this requires 1 piece of
                                                   // information from whoever calls it.
                                                   // The info is stored in the design variable
                                                   // the value stored in design must be a
                                                   // referenceto a FlyingWingDesign object.
                                                   // Since it is a reference, we borrow it (&) and
                                                   // not copy it. since it is "&" and not "&mut"
                                                   // then we are only allowed to read it and not
                                                   // modify it.
                                                   // NO "-> []" so this function returns no value.
    let aspect_ratio = design.aspect_ratio();

    if aspect_ratio < 3.0 {
        println!("Warning: Low aspect ratio. This may be stable but inefficient.");
    } else if aspect_ratio > 8.0 {
        println!("Warning: High aspect ratio. This may be efficient but structurally weaker.");
    } else {
        println!("Aspect ratio looks reasonable for a small flying wing.");
    }

    if design.taper_ratio() < 0.35 {
        println!("Warning: Very narrow tips stallmore easily."); 
    }
}
