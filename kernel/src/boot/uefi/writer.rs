
pub static mut GOP_WRITER: Option<GopConsoleWriter> = None;

pub struct GopConsoleWriter {
    pub info:        hw::uefi::GraphicsOutputModeInformation,
    pub framebuffer: &'static mut [u32],
    pub x:           u32,
    pub y:           u32,
    pub c:           u32
}

impl GopConsoleWriter {
    fn static_write(s: &str) {
        unsafe { GOP_WRITER.as_mut().unwrap_unchecked() }.write(s);
    }

    fn write(&mut self, s: &str) {
        if self.y + hw::font::HEIGHT >= self.info.vertical_resolution {
            self.y = 0;
            self.framebuffer.fill(0);
        }

        for ch in s.chars() {
            let ch = match ch {
                ' ' => {
                    self.x += hw::font::LENGTH;
                    continue;
                },
                '\t' => {
                    self.x += 4 * hw::font::LENGTH;
                    continue;
                },
                '\n' => {
                    self.x = 0;
                    self.y += hw::font::HEIGHT;
                    continue;
                },
                ch @ '!'..='~' => ch - 0x21,
                _ => 0x5E,
            };

            self.x += hw::font::LENGTH;

            for i in 0..hw::font::LENGTH {
                for j in 0..hw::font::HEIGHT {
                    let c = hw::font::FONT[ch as usize][i][j];
                    let c = (self.c & 0x000000FF) * c | (self.c & 0x0000FF00) * c | (self.c & 0x00FF0000) * c | (self.c & 0xFF000000) * c;
                    let x = self.x + i as u32;
                    let y = self.y + j as u32;

                    if y >= self.info.horizontal_resolution || x >= self.info.vertical_resolution {
                        return;
                    }

                    self.framebuffer[(y * self.info.horizontal_resolution + x) as usize] = self.c * c;
                }
            }
        }
    }
}