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
use ex::{fs, fs::File};
use std::{env,
          fmt::Write,
          io::{Read, Write as IoWrite},
          path::Path,
          process::Command};

fn svd2rust(svd_path: &str) -> ex::io::Result<String> {
    let mut svd_xml = String::new();
    File::open(svd_path)?.read_to_string(&mut svd_xml).unwrap();
    let generated = svd2rust::generate(svd_xml.as_str(), svd2rust::Target::None, false).unwrap();

    let mut file = File::create("svd/esp32.svd.rs").expect("Couldn't create lib.rs file");
    let mut data = String::new();
    write!(data, "{}", generated.lib_rs).expect("Could not output code");

    let newlined = data.replace("] ", "]\n");
    file.write_all(newlined.as_ref()).expect("Could not write code to lib.rs");

    Ok(newlined)
}

// Example custom build script.
fn main() -> ex::io::Result<()> {
    println!("BEGIN generate_bindings_from_build_rs: {:?}", std::env::current_exe().unwrap());
    if env::var("CARGO_MANIFEST_DIR").is_err() {
        env::set_var("CARGO_MANIFEST_DIR", env::current_dir().unwrap().to_str().unwrap());
    }
    let base_dir: String = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Tell Cargo that if the given file changes, to rerun this build script.
    let svd = format!("{}/svd/esp32.base.svd", base_dir);
    let svd_patched = format!("{}/svd/esp32.base.svd.patched", base_dir);
    let yaml = format!("{}/svd/patches/esp32.yaml", base_dir);
    let svd_final = format!("{}/svd/esp32.svd", base_dir);
    let generate_file_sample = format!("{}/src/gpio.rs", base_dir);
    let lib_rs = format!("{}/src/lib.rs", base_dir);

    println!("cargo:rerun-if-changed={}", svd);
    println!("cargo:rerun-if-changed={}", svd_patched);
    println!("cargo:rerun-if-changed={}", yaml);
    println!("cargo:rerun-if-changed={}", svd_final);
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-changed={}", svd);

    if Path::new(generate_file_sample.as_str()).exists() {
        let in_m = fs::metadata("svd/esp32.base.svd")?;
        let out_m = fs::metadata(generate_file_sample)?;
        if in_m.modified()? < out_m.modified()? {
            println!("src/* files newer than {}, skipping any regeneration.", svd);
            return Ok(());
        }
    }

    // Delete the output if it exists.
    if Path::new(&svd_patched).exists() {
        fs::remove_file(&svd_patched)?;
    }

    println!("Patching SVD {} -> {}", svd, svd_final);
    // generate patched file.
    Command::new("svd").arg("patch").arg(yaml)
        .output()
        .expect("failed to run svd tool. This is a python tool that should have been installed via: pip3 install --upgrade --user svdtools");
    fs::copy(&svd_patched, &svd_final)?;

    // println!("Generating rust source from AVD");
    let rs = svd2rust(&svd_final)?;

    // convert it into the rust form.
    println!("Converting sources into commonly accepted rust format.");
    if form::create_directory_structure("src", rs).is_err() {
        println!("Unable to convert to common rust format.");
    }

    println!("Formatting rust source files.");
    Command::new("cargo").arg("fmt").output().expect("failed to run cargo format");

    println!("Cleaning up some lint warnings that are no longer supported in generated cdde.");
    clean_up_some_lint_warnings_in_the_generated_code(&lib_rs)?;

    // Tell git to ignore changes to the generated src/lib.rs
    Command::new("git").arg("update-index")
                       .arg("--assume-unchanged")
                       .arg("src/lib.rs")
                       .output()
                       .expect("failed to suppress change notification on generated lib.rs");

    println!("All done.");
    Ok(())
}

fn clean_up_some_lint_warnings_in_the_generated_code(lib_rs: &String) -> ex::io::Result<()> {
    let mut lib_src = String::new();
    File::open(&lib_rs)?.read_to_string(&mut lib_src).unwrap();
    let cleaned_some_invalid_lints = lib_src.replace("#![deny(legacy_directory_ownership)]", "//#![deny(legacy_directory_ownership)]")
                                            .replace("#![deny(plugin_as_library)]", "//#![deny(plugin_as_library)]")
                                            .replace("#![deny(safe_extern_statics)]", "//#![deny(safe_extern_statics)]")
                                            .replace("#![deny(unions_with_drop_fields)]", "//#![deny(unions_with_drop_fields)]");

    File::create(&lib_rs)?.write_all(cleaned_some_invalid_lints.as_bytes()).unwrap();

    Ok(())
}
