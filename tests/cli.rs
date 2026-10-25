use std::process::Command;

// Catches the binary drifting from the library maths or the output format.
#[test]
fn thirty_cm_prints_inches_and_fabric() {
    let out = Command::new(env!("CARGO_BIN_EXE_plush"))
        .arg("30")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8(out.stdout).unwrap(),
        "30 cm = 11.81 in\nPillow fabric: 32 x 64 cm\n"
    );
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_plush"))
        .args(args)
        .output()
        .unwrap()
}

// Catches bad input slipping through as a wrong pattern instead of a clear refusal.
#[test]
fn bad_heights_fail_with_exit_2() {
    for args in [&[][..], &["cat"], &["0"], &["-5"]] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "args {args:?}");
        assert!(out.stdout.is_empty(), "args {args:?}");
        assert!(!out.stderr.is_empty(), "args {args:?}");
    }
}
