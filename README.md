# adb-ext

https://github.com/user-attachments/assets/3545cbfe-9c8d-4e8a-a65c-446cfb46dd00

<details>
  <summary>commands</summary>

 <br>lss [count]
 <br>mss | shot [destination]
 <br>lsc [count]
 <br>msc | rec | record [destination]
 <br>bounds
 <br>taps
 <br>pointer
 <br>animscale [scale]
 <br>[f]port | [f]land | [no]accel
 <br>sdk [path]
 <br>run path/to/app.apk
 <br>steal app.package.name
 <br>adb-ext update
</details>

<details>
  <summary>fix on Linux</summary>

  ![adb-ext](https://github.com/Atomofiron/green-pain/assets/14147217/aeef69e9-41d0-47ee-8744-35d170ce707a)
</details>

:white_check_mark: MacOS x86_64\
:white_check_mark: MacOS ARM\
:white_check_mark: Linux x86_64\
:zzz: Linux ARM\
:white_check_mark: Windows

# Install

### GNU/Linux, MacOS
```
curl -sSfL https://github.com/atomofiron/adb-ext/raw/main/stuff/install.sh | sh
```

<details>
  <summary>MacOS feature (on each installation or update)</summary>
  <br>
  1. run the adb-ext<br>
  2. click Cancel<br>
  3. System Settings > Privacy & Security > Allow Anyway<br>
  4. execute the adb-ext again<br>
  5. click Open<br>
<img width="978" alt="adb-ext-macos-feature" src="https://github.com/atomofiron/adb-ext/assets/14147217/d765de66-a273-4b93-a8a4-8441e40c06ba">
</details>

### Windows
1. [Download](https://github.com/atomofiron/adb-ext/releases/latest/download/adb-ext.exe)
2. run `adb-ext.exe deploy`
3. add `%LOCALAPPDATA%/Programs/adb-ext` into **top** of the `PATH`

# How to use
resolve usb adb permission (Linux only)
```
adb fix
```
common use of ADB
```
% adb shell
Select a device
  Oneplus 7T
> Nothing Phone (1)
  Cancel
Spacewar:/ $
```
run `adb-ext` for interactive mode (optional)
```
% adb-ext
input command or 'exit', 'quit', Ctrl-D to exit
adb-ext> devices
List of devices attached

✔ adb-ext> foo
adb: unknown command foo
✘ adb-ext> exit
%
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

for update
```
adb-ext update
```
