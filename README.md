# adb-ext

![adb-ext](https://github.com/Atomofiron/green-pain/assets/14147217/aeef69e9-41d0-47ee-8744-35d170ce707a)

# Install
<br>:white_check_mark: MacOS x86_64
<br>:white_check_mark: MacOS ARM
<br>:white_check_mark: Linux x86_64
<br>:zzz: Linux ARM
<br>:no_entry_sign: Windows
```
curl -sSfL https://github.com/atomofiron/adb-ext/raw/main/stuff/install.sh | sh
```

<details>
  <summary>MacOS feature (on each installation or update)</summary>
  <br>
  0. execute the adb-ext<br>
  1. click Cancel<br>
  2. System Settings > Privacy & Security > Allow Anyway<br>
  3. execute the adb-ext again<br>
  4. click Open<br>
  <br>
<img width="978" alt="adb-ext-macos-feature" src="https://github.com/atomofiron/adb-ext/assets/14147217/d765de66-a273-4b93-a8a4-8441e40c06ba">
</details>

# Run
resolve usb adb permission (Linux only)
```
adb fix
```
common use of ADB
```
$ adb shell
Select a device
  Oneplus 7T
> Nothing Phone (1)
  Cancel
Spacewar:/ $
```
pull the 3 last screenshots from device\
sources: `/sdcard/Pictures/Screenshots/`, `/sdcard/DCIM/Screenshots/`\
destination: `~/Android/Screenshots/`
```
lss 3
```
pull the 3 last screencasts from device\
sources: `/sdcard/Pictures/Screenshots/`, `/sdcard/DCIM/Screen recordings/`, `/sdcard/Movies/`\
destination: `~/Android/Screencasts/`
```
lsc 3
```
make a new screenshot and pull it\
default destination: `~/Android/Screencasts/`
```
mss [destination]
```
where `destination` can be:
`.`, `./name`, `./name.png`, `./stuff/`, `./stuff/name`, `./stuff/name.png`, `~`, `~/name`, `~/name.png`, `~/stuff/`, `~/stuff/name`, `~/stuff/name.png`, `name`, `name.png`, `stuff/name`, `stuff/name.png`
