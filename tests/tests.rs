//extern crate compiletest_rs as compiletest;

//use std::path::PathBuf;
/*
fn run_mode(mode: &'static str, custom_dir: Option<&'static str>) {
    let mut config = compiletest::Config::default();
    let cfg_mode = mode.parse().expect("Invalid mode");

    config.mode = cfg_mode;

    let dir = custom_dir.unwrap_or(mode);
    config.src_base = PathBuf::from(format!("tests/{}", dir));
    config.target_rustcflags = Some("-L target/debug -L target/debug/deps".to_string());
    config.clean_rmeta();

    compiletest::run_tests(&config);
} */

//#[test]
fn compile_test() {
    //eprintln!("***********************\r\n***************");
    //run_mode("compile-fail", None);
    //run_mode("run-pass", None);
    //run_mode("ui", None);

}
