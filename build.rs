use std::env;

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

    let path = env::current_dir().unwrap();

    println!("cargo:rustc-link-search=native={}/ni-libraries", path.display());
    println!("cargo:rustc-link-search=native={}/athena/lib", path.display());
}
