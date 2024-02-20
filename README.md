# send-time-to-hid-keyboard

A program to send the current time to a connected keyboard via [hidapi](https://docs.rs/hidapi/latest/hidapi/). The time is sent to the keyboard in the format "12:34". The time is sent fully formatted rather than as numbers because the memory may be limited on the keyboard. This solution therefore minimises keyboard-side code and thereby saves space.

This program should work with essentially any HID-device, but was designed with QMK keyboards in mind. Setting it up for use with any QMK keyboard should require only minimal configuration.

![A Sofle keyboard receiving the current time from the PC](https://github.com/Elyviere/send-time-to-hid-keyboard/assets/23457506/eb770a1f-f3b7-4318-a23d-0f0966f0e319)

## Running the program
No readily-compiled version of the program is currently provided, as configuration is likely necessary inside the code in order to identify your keyboard. Instead, the user is required to [install rust](https://www.rust-lang.org/tools/install), and compile the code themselves. Installing rust should provide access to the `cargo` command, which will be used to compile and run the program. Also, you'll need to install libudev to your system. Alternatively, you can run the below commands if these packages are available in your distribution:
```
sudo apt update
sudo apt install rustc cargo pkg-config libudev-dev
```

Next, clone the git repository using a terminal or powershell, in a folder of your choice.
```
git clone https://github.com/Elyviere/send-time-to-hid-keyboard.git
```
Now for some configuration. In order for the program to correctly identify your keyboard, you will need to provide four parameters. The only way to configure this currently is to edit them directly in the code, so open the file `send-time-to-hid-keyboard/src/main.rs` in a text editor of your choice. The lines you need to edit are the following:
```
const USAGE_PAGE: u16 = 0xFF60;
const USAGE: u16 = 0x61;
const VENDOR_ID: u16 = 0xFC32;
const PRODUCT_ID: u16 = 0x0287;
```
The only part you need to change are the letters and numbers following the `0x` on each line. For example, if your device's PRODUCT_ID is `1398`, that line would be changed to the following:
```
const PRODUCT_ID: u16 = 0x1398;
```
Note: If you're using a keyboard running QMK software, only the VENDOR_ID and PRODUCT_ID will need to change, unless you've edited the usage values in the QMK software. The above id's are for a FalbaTech Sofle wired keyboard. Finding the id's of your keyboard is beyond the scope of this readme, but the qmk guide for [RAW-Hid](https://github.com/qmk/qmk_firmware/blob/master/docs/feature_rawhid.md#sending-data-to-the-keyboard-idsending-data-to-the-keyboard) has some further information.

And finally, once those values are set correctly, it's time to start up the program.
```
cd send-time-to-hid-keyboard
cargo run
```
Note that the program will not be able to find your device until you complete the setup for the keyboard as well (see [Receiving side](#receiving-side-keyboard)).

WSL Note: If using WSL in Windows, unfortunately the HID device won't be picked up by the WSL distro. Instead, you'll need to run the program in Powershell.

### Separate executable

If you want a stand-alone executable, in order to make the program start when your PC powers up, run the following command:
`cargo build --release`
This will create an executable in `/target/release/send-time-to-hid-keyboard`, with a matching extension. In Windows, this would be `send-time-to-hid-keyboard.exe`. When running the program however, you won't get any visual cue that the program has started. Instead, use a terminal or the task manager if you want confirmation that the process is running. It is aptly named "Send-time-to-hid-keyboard".

Now you can add this to your PC's startup executables, following any guide for your specific OS to do so.

## Receiving side (keyboard)
If using QMK on the keyboard, the time can be simply shown on an OLED with the following code in your `keymap.c`:
```
#ifdef RAW_ENABLE
void raw_hid_receive(uint8_t *data, uint8_t length) {
    oled_write((char *)data, false);
}
#endif
```
You'll also need to add the following in your `rules.mk`
```
RAW_ENABLE = yes
```

For other software, please refer to your manufacturer.

### Text location and Split-keyboards
Note that this code will print the time at the top of the main-side keyboard if using split keyboards. 

- For hints on printing the time on the other half of the keyboard, please see the method `raw_hid_receive` in my personal repository: [qmk-firmware-elyviere - Luna.c](https://github.com/Elyviere/qmk_firmware_elyviere/blob/main/keyboards/sofle/keymaps/elyviere/luna.c). 
- To move the location of the time, use the `oled_set_cursor(uint8_t col, uint8_t line)` method or equivalent (further info in the [Further Reading](#further-reading) section below).

## Further reading
Please see the relevant QMK help pages for further information:

[RAW-Hid](https://github.com/qmk/qmk_firmware/blob/master/docs/feature_rawhid.md)

[OLED Driver](https://github.com/qmk/qmk_firmware/blob/master/docs/feature_oled_driver.md)

## Attributions
Icon  designed by [Freepik](https://www.freepik.com/) from Flaticon
