use std::path::Path;

use vauncher_core as vcore;

#[test]
fn vm_from_file_currentdir() {
    // let vm = Vm::from_file("./tests/vm/vauncher_currentdir.json").unwrap();

    // let path = &*vm.root();
    // let path = Path::new(path).join("tests").join("vm").join("mods");

    // assert!(&path.join("mod_1").is_dir());
    // assert!(&path.join("mod_1").join("package.json").is_file());
    // assert!(&path.join("mod_1").join("vss-new-models.js").is_file());
    // assert!(&path.join("mod_2").is_dir());
}

#[test]
fn vm_from_file_withoutdir() {
    // let vm = Vm::from_file("./tests/vm/vauncher_withoutdir.json").unwrap();

    // let path = &*vm.root();
    // let path = Path::new(path);

    // assert!(path.join("mod_1").is_dir());
    // assert!(path.join("mod_1").join("package.json").is_file());
    // assert!(path.join("mod_1").join("vss-new-models.js").is_file());
    // assert!(path.join("mod_2").is_dir());
}
