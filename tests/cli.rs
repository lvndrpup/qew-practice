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
        "30 cm = 11.81 in\nPillow fabric: 32 x 64 cm\nStuffing: about 135 g\n"
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
    for args in [&[][..], &["cat"], &["0"], &["-5"], &["nan"], &["inf"]] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "args {args:?}");
        assert!(out.stdout.is_empty(), "args {args:?}");
        assert!(!out.stderr.is_empty(), "args {args:?}");
    }
}

// Catches --help being treated as a bad height.
#[test]
fn help_prints_usage_and_example() {
    for flag in ["--help", "-h"] {
        let out = run(&[flag]);
        assert_eq!(out.status.code(), Some(0), "{flag}");
        let text = String::from_utf8(out.stdout).unwrap();
        assert!(text.starts_with("usage: plush HEIGHT_CM"), "{flag}: {text}");
        assert!(text.contains("plush 30"), "{flag}: {text}");
    }
}

// Catches inch input being read as cm, or converted the wrong way.
#[test]
fn twelve_inches_matches_30_48_cm() {
    let inches = run(&["12in"]);
    assert!(inches.status.success());
    assert_eq!(inches.stdout, run(&["30.48"]).stdout);
}

// Catches float noise like 32.480000000000004 in the printed sizes.
#[test]
fn sizes_print_without_float_noise() {
    let text = String::from_utf8(run(&["30.48"]).stdout).unwrap();
    assert_eq!(
        text,
        "30.48 cm = 12.00 in\nPillow fabric: 32.48 x 64.96 cm\nStuffing: about 142 g\n"
    );
}

// Catches mm input being read as cm, or scaled the wrong way.
#[test]
fn three_hundred_mm_matches_30_cm() {
    let mm = run(&["300mm"]);
    assert!(mm.status.success());
    assert_eq!(mm.stdout, run(&["30"]).stdout);
}

// Catches the --seam value being ignored or applied to only one edge.
#[test]
fn seam_flag_sets_the_allowance() {
    let text = String::from_utf8(run(&["--seam", "1.5", "30"]).stdout).unwrap();
    assert_eq!(
        text,
        "30 cm = 11.81 in\nPillow fabric: 33 x 66 cm\nStuffing: about 135 g\n"
    );
}

// Catches a bad seam slipping through as a pattern with no or negative allowance.
#[test]
fn bad_seam_fails_with_exit_2() {
    for args in [
        &["--seam", "0", "30"][..],
        &["--seam", "-1", "30"],
        &["--seam", "x", "30"],
        &["30", "--seam"],
    ] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "args {args:?}");
        assert!(out.stdout.is_empty(), "args {args:?}");
    }
}

// Catches width and height swapped, or the seam left off the width.
#[test]
fn width_flag_makes_a_rectangle() {
    let text = String::from_utf8(run(&["30", "--width", "40"]).stdout).unwrap();
    assert_eq!(text, "30 cm = 11.81 in\nPillow fabric: 42 x 64 cm\n");
}

// Catches a bad width slipping through as a pattern.
#[test]
fn bad_width_fails_with_exit_2() {
    for args in [
        &["30", "--width", "0"][..],
        &["30", "--width", "x"],
        &["30", "--width"],
    ] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "args {args:?}");
        assert!(out.stdout.is_empty(), "args {args:?}");
    }
}

// Catches a round pillow cut without seam allowance on its radius.
#[test]
fn round_flag_prints_cutting_radius() {
    let text = String::from_utf8(run(&["30", "--round"]).stdout).unwrap();
    assert_eq!(
        text,
        "30 cm = 11.81 in\nPillow fabric: 32 x 64 cm\nCutting radius: 16 cm\n"
    );
}

// Catches a circle that is also asked to have a width: that has no single answer.
#[test]
fn round_with_width_fails_with_exit_2() {
    let out = run(&["30", "--round", "--width", "40"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
}

// Catches stuffing printed for shapes the h*h*h/4 estimate does not fit.
#[test]
fn stuffing_only_for_square_pillows() {
    for args in [&["30", "--width", "40"][..], &["30", "--round"]] {
        let text = String::from_utf8(run(args).stdout).unwrap();
        assert!(!text.contains("Stuffing"), "args {args:?}: {text}");
    }
}
