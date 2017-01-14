use std::env;

fn main() {
    let arm = env::var("TARGET").unwrap().contains("arm");

    // Assume that building for arm means we want to build for the RIO, and anything else is for
    // simulation.
    if arm {
        println!("cargo:rustc-link-search=native=ni-libraries");
        println!("cargo:rustc-link-search=native=athena/lib");
        for lib in ["HALAthena",
                    "wpiutil",
                    "FRC_NetworkCommunication",
                    "RoboRIO_FRC_ChipObject",
                    "NiFpga",
                    "NiFpgaLv",
                    "niriosession",
                    "spi",
                    "i2c",
                    "visa",
                    "NiRioSrv",
                    "niriodevenum"]
            .iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
    }
}
