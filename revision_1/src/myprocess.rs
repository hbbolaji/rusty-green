use std::process::{Command, Stdio};

pub fn test_external_process() {
    let mut process_1 = Command::new("which");
    process_1.arg("python3");
    process_1.stdout(Stdio::null());

    let process_1_result = process_1.output();
    if process_1_result.is_ok() {
        let result = process_1_result.ok().unwrap();
        println!("Was execution successful? {:?}", result.status.success());
        let output = String::from_utf8(result.stdout).unwrap().replace("\n", "");
        println!("{:?}", output);
    }

    let mut process_2 = Command::new("brew");
    process_2.arg("--version");

    let mut process_2_handle = process_2.spawn().unwrap();
    println!("Doing some more work!...");
    let process_2_result = process_2_handle.wait().unwrap();
    println!("{:?}", process_2_result.code().unwrap());
}
