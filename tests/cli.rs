use std::process::Command;

// Catches the binary drifting from the library maths or the output format.
#[test]
fn thirty_cm_prints_inches_and_fabric() {
    let out = Command::new(env!("CARGO_BIN_EXE_plush")).arg("30").output().unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8(out.stdout).unwrap(),
        "30 cm = 11.81 in\nPillow fabric: 32 x 64 cm\n"
    );
}
