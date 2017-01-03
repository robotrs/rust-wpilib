fn main() {
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

    println!("cargo:rustc-link-search=native=ni-libraries");
    println!("cargo:rustc-link-search=native=athena/lib");
}
