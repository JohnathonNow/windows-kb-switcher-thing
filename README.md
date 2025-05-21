<img align="left" width="80" height="80" src="https://raw.githubusercontent.com/JohnathonNow/windows-kb-switcher-thing/refs/heads/main/resources/icon.png"/>

# Windows KB Switcher Thing

This program will run one of a number of different, user-specified commands depending on which keyboard
most recently emitted a keypress.
It will run this command once for each series of keypresses made with that keyboard.

The program runs in the system tray, and can be closed by choosing Exit from its context menu.

## Why does this exist

I have a Logitech K400 Plus wireless keyboard by my TV, which is hooked up to my desktop in another room. 
I would like my computer to automatically switch my audio device and display output to the TV when I type on my K400 Plus, 
and I'd like everything switched back when I type with the keyboard at my desk.

## Basic usage

The application will attempt to read configuration from a file in the working directory with the names 
`config.json`, `config.toml`, and `config.yaml`. A configuration will consist of a list of commands, where each 
command has three keys - `keyboard`, `cmd`, and `args`. `keyboard` specifies which keyboard the rule applies to -
when a key is pressed, each rule is checked, and the first rule that matches a keyboard is used. Keyboards
are matched if their device name contains the text in the `keyboard` field of a command. On a match,
the binary pointed to by the `cmd` field of the command is run with the arguments specified in the
`args` field.

Note that an empty string for the `keyboard` field works as a default rule, and will apply
to all keyboards not previously specified. A default rule like this should be the last one
in the config file.

You can find device names in the Windows Device Manager, under the `Device instance path` property. 
![image](https://github.com/user-attachments/assets/4adf2a00-4cd0-49fb-a03c-2c31de380d47)

## For example
Consider this configuration

```toml
[[commands]]
keyboard = "VID_046D&PID_C52B"
cmd = "cmd.exe"
args = [ "/c", "resources\\tv.bat" ]

[[commands]]
keyboard = ""
cmd = "cmd.exe"
args = [ "/c", "resources\\normal.bat" ]
```

Let's look at the first rule.
`VID_046D&PID_C52B` is the vendor id and product id of the wireless receiver of the aforementioned Logitech K400 Plus. That means
that whenever I type on that keyboard, this is the rule that will apply, as this string will appear in the product name. So when I type on
my K400 Plus and it is not the most recent keyboard I've used, it will run `cmd.exe` with the arguments `/c resources\\tv.bat`, which could,
say, set my audio device to the TV and enable video output to it in my graphics settings.

The other rule has an empty string for the `keyboard` field. This means that whenever I type on a different keyboard, and that keyboard
is not the most recent keyboard I've typed on, it will run the command specified by this rule. So when I type with the keyboard plugged
into my desktop directly, or any other keyboard besides the K400 Plus, it will run `cmd.exe` with the arguments `/c resources\\normal.bat`, which
could, say, set my audio device to my gaming headset and turn off video output to my TV.
