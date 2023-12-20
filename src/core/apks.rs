use std::process::exit;
use crate::core::adb_command::AdbArgs;
use crate::core::destination::Destination;
use crate::core::ext::OutputExt;
use crate::core::r#const::{PULL, SHELL};
use crate::core::selector::{resolve_device, run_adb_with};

pub fn steal_apk(package: String, dst: Option<String>) {
    let pm_command = format!("pm path {package}");
    let args = AdbArgs::run(&[SHELL, pm_command.as_str()]);
    let device = resolve_device();
    let output = run_adb_with(&device, args);
    if !output.status.success() {
        output.print_err();
        exit(output.code());
    }
    let default_name = format!("{package}.apk");
    let destination = dst
        .unwrap_or(default_name.clone())
        .with_file(default_name.as_str());
    // the output line is "package:/data/data/[…]/base.apk"
    let path = &output.stdout().clone()[8..];
    let args = AdbArgs::spawn(&[PULL, path, destination.as_str()]);
    let output = run_adb_with(&device, args);
    exit(output.code());
}

pub fn run_apk() {
    //format!("aapt d xmltree {}.apk AndroidManifest.xml");
    //format!("adb shell am start -a android.intent.action.MAIN -n {}/{}");
}

/*
N: android=http://schemas.android.com/apk/res/android
  E: manifest (line=2)
    E: application (line=19)
      E: activity (line=31)
        A: android:name(0x01010003)="demo.atomofiron.insets.activity.MainActivity" (Raw: "demo.atomofiron.insets.activity.MainActivity")
        E: intent-filter (line=34)
          E: action (line=35)
            A: android:name(0x01010003)="android.intent.action.MAIN" (Raw: "android.intent.action.MAIN")
          E: category (line=37)
            A: android:name(0x01010003)="android.intent.category.LAUNCHER" (Raw: "android.intent.category.LAUNCHER")
*/
