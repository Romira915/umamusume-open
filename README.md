# umamusume_open
Application for pinning to the Start menu and taskbar.

## Using
You can download it from the Releases page. 
If you have "umamusume" installed on your computer, you can start it via DMM.

## Icon
If you want to change the icon of the exe file, please compile it by yourself.

### What you need
- [rustup](https://www.rust-lang.org/)
- icon.ico (ico file)

1. Copy the icon file (icon.ico) to `./` or `./src`
2. Run `cargo build --release`
3. An exe file will be created. `./target/release/umamusume_open.exe`