const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub type Buffer = [[bool; WIDTH]; HEIGHT];


// display struct to represent the display of the emulator
// one buffer entry represents if the "pixel" is "on" or not
pub struct Display {
    display_buffer: Buffer,
}

impl Display {
    pub fn new() -> Display {
        Display {
            display_buffer: [[false; WIDTH]; HEIGHT],
        }
    }

    // given a memory slice where each entry is "drawn" into the buffer with start position x_start and y_start
    // each byte has its own row on the buffer starting with y_start and is ascending
    // buffer gets 'True' when the byte has an 1 in that position else 'False'
    // when a "pixel/entry" is switched from "True" to "False" we return "True" else "False"
    // parameters: x_start: u8 starting column
    //             y_start: u8 starting row
    //             memory: &[u8]  slice of the memory max length is 0xF
    // return: true  when a entry changes from true to false
    //         false else
    pub fn draw(&mut self, x_start: u8, y_start: u8, memory: &[u8]) -> bool {
        let mut pixel_changed = false;

        for (row, byte) in memory.iter().enumerate() {
            let y = (y_start as usize + row) % HEIGHT;

            for bit in 0..8 {
                let x = (x_start as usize + bit) % WIDTH;
                let current_bit = (byte >> (7 - bit)) & 1;

                let current_pixel = self.display_buffer[y][x] as u8;
                let new_pixel = current_bit ^ current_pixel;

                self.display_buffer[y][x] = new_pixel != 0;

                if new_pixel == 0 && current_pixel == 1 {
                    pixel_changed = true;
                }
            }
        }
        pixel_changed
    }

    pub fn get_buffer(&mut self) -> Buffer {
        self.display_buffer
    }

    // resets the buffer to all_false
    pub fn clear(&mut self) {
        self.display_buffer = [[false; WIDTH]; HEIGHT];
    }
}