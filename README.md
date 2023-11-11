# green-pain

![green-pain](https://github.com/Atomofiron/green-pain/assets/14147217/aeef69e9-41d0-47ee-8744-35d170ce707a)

# Install
<br>:white_check_mark: MacOS x86_64
<br>:white_check_mark: MacOS ARM
<br>:white_check_mark: Linux x86_64
<br>:zzz: Linux ARM
<br>:no_entry_sign: Windows
```
curl -sSfL https://github.com/Atomofiron/green-pain/raw/main/stuff/install.sh | sh
```

# Run
resolve usb adb permission (Linux only)
```
green-pain
```
<details>
  <summary>MacOS feature</summary>
  <br>
  0. execute the green-pain<br>
  1. click Cancel<br>
  2. System Settings > Privacy & Security > Allow Anyway<br>
  3. execute the green-pain again<br>
  4. click Open<br>
  <br>
<img width="978" alt="green-pain-macos-feature" src="https://github.com/atomofiron/green-pain/assets/14147217/d765de66-a273-4b93-a8a4-8441e40c06ba">
</details>

common use of ADB
```
$ adb shell
1) Oneplus 7T
2) Nothing Phone (1)
Select a device (default 1): 2
Spacewar:/ $
```
pull the 3 last screenshots from device\
sources: `/sdcard/Pictures/Screenshots/`, `/sdcard/DCIM/Screenshots/`\
destination: `~/Android/Screenshots`
```
lss 3
```
pull the 3 last screencasts from device\
sources: `/sdcard/Pictures/Screenshots/`, `/sdcard/DCIM/Screen recordings/`, `/sdcard/Movies/`\
destination: `~/Android/Screencasts`
```
lsc 3
```
