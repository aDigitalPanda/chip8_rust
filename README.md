# chip8_rust by aDigitalPanda

A basic Chip-8 emulator implementation written in Rust.

## How to run
Find yourself some chip-8 ROMs online. Like [those](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html). Copy them into the **chip_8** directory, prefered into a new directory *xy*.  
In **main.rs** change *GAME_DIR* value like this:
```rust
const GAME_DIR: &str = "xy/" // or "" if ROMs are in chip-8 directory
```
To run it, use
```bash
cargo run ROM_NAME
```

## Inspiration for implementing
I wanted to learn Rust and after completing the [book](https://doc.rust-lang.org/book/title-page.html) I searched for a new project.  
After some time I heard about Chip-8, highly praised to be an *easy* project to get used to an language. So here I am.  
I can only recommend to everyone who wants to 'learn' a new language to try implement a Chip-8 emulator.

## References
Here are some references i used to understand Chip-8 and other aspect of it.  
- Cowgod's [Chip-8](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) Technical Reference  
- Matthew Mikolay [Mastering Chip-8](https://github.com/mattmikolay/chip-8/wiki/Mastering-CHIP%E2%80%908)  
- [Chip-8 Sprites](http://www.emulator101.com/chip-8-sprites.html)  
- John Earnest [Chip8 Programming](https://github.com/JohnEarnest/Octo/blob/gh-pages/docs/Chip8%20Programming.md)
