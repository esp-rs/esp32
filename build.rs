// OUTPUT=esp32.svd
// BASE=esp32.base.svd
//
// all: clean patch generate form fmt build
//
// clean:
// rm -rf src/
//
// patch:
// rm -f svd/$(OUTPUT)
// svd patch svd/patches/esp32.yaml
// mv svd/$(BASE).patched svd/$(OUTPUT)
//
// generate:
// svd2rust --target none -i svd/$(OUTPUT)
//
// form:
// form -i lib.rs -o src/
// rm lib.rs
//
// fmt:
// cargo fmt
//
// build:
// cargo clean
// cargo build
use std::fmt::Write;
use std::fs;
use std::fs::File;
use std::io::{Read, Write as IoWrite};
use std::path::Path;
use std::process::Command;

fn svd2rust(svd_path: &str) -> std::io::Result<String> {
    let mut svd_xml = String::new();
    File::open(svd_path)?.read_to_string(&mut svd_xml)?;
    let generated = svd2rust::generate(svd_xml.as_str(), svd2rust::Target::None, false).unwrap();

    let mut file = File::create("svd/esp32.svd.rs").expect("Couldn't create lib.rs file");
    let mut data = String::new();
    write!(data, "{}", generated.lib_rs).expect("Could not output code");

    let newlined = data.replace("] ", "]\n");
    file.write_all(newlined.as_ref())
        .expect("Could not write code to lib.rs");

    Ok(newlined)
}

// Example custom build script.
fn main() -> std::io::Result<()> {
    // Tell Cargo that if the given file changes, to rerun this build script.
    let svd = "svd/esp32.base.svd";
    let svd_patched = "svd/esp32.base.svd.patched";
    let yaml = "svd/patches/esp32.yaml";
    let svd_final = "svd/esp32.svd";
    let generate_file_sample = "src/gpio.rs";

    if Path::new(generate_file_sample).exists() {
        let in_m = fs::metadata("svd/esp32.base.svd")?;
        let out_m = fs::metadata(generate_file_sample)?;
        if in_m.modified()? < out_m.modified()? {
            println!("No need for changes.");
            return Ok(());
        }

    }
    println!("cargo:rerun-if-changed={}", svd);

    // Delete the output if it exists.
    if Path::new(svd_patched).exists() {
        fs::remove_file(svd_patched)?;
    }

    // println!("Patching SVD {} -> {}", svd, svd_final);
    // generate patched file.
    Command::new("svd").arg("patch").arg(yaml)
        .output()
        .expect("failed to run svd tool. This is a python tool that should have been installed via: pip3 install --upgrade --user svdtools");
    fs::copy(svd_patched, svd_final)?;

    // println!("Generating rust source from AVD");
    let rs = svd2rust(svd_final)?;

    // convert it into the rust form.
    // println!("Converting sources into commonly accepted rust format.");
    if form::create_directory_structure("src", rs).is_err() {
        println!("Unable to convert to common rust format.");
    }

    // println!("Formatting rust source files.");
    Command::new("cargo")
        .arg("fmt")
        .output()
        .expect("failed to run cargo format");

    Ok(())
}
