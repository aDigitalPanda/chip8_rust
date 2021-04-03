const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub type Buffer = [[bool; WIDTH]; HEIGHT];

pub struct Display {
    display_buffer: Buffer,
}

impl Display {
    pub fn new() -> Display {
        Display {
            display_buffer: [[false; WIDTH]; HEIGHT],
        }
    }

    //memory gibt einen Bereich der Länge äzwischen 1 - 15
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

    pub fn clear(&mut self) {
        self.display_buffer = [[false; WIDTH]; HEIGHT];
    }
}