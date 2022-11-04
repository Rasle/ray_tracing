use std::sync::mpsc::*;
use minifb::{WindowOptions, Window, Key};
use triple_buffer::*;

pub enum RenderStatus {
    Processing,
    Done,
}

pub struct Render {
    buffer_output : Output<Vec<u32>>,
    receiver : Receiver<RenderStatus>,
}

impl Render {
    pub fn new(buffer_output : Output<Vec<u32>>, receiver : Receiver<RenderStatus>) -> Render {
        Render {
            buffer_output,
            receiver,
        }
    }

    pub fn render(mut self, mut render_data: Vec<u32>, width: u32, height: u32) {
        let options = WindowOptions {
            borderless: false,
            title: true,
            resize: true,
            scale: minifb::Scale::X1,
            scale_mode: minifb::ScaleMode::Center,
            topmost: false,
            transparency: false,
            none: false,
        };
        let mut window = Window::new(
            "Ray Tracer - ESC to exit",
            width as usize,
            height as usize,
            options,
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let mut counter = height - 1;
        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        while window.is_open() && !window.is_key_down(Key::Escape) {
            let progress = self.receiver.try_recv();
            match progress {
                Ok(status) => match status {
                    RenderStatus::Processing => {
                        if self.buffer_output.update() {
                            let output = self.buffer_output.output_buffer();
                            for (i, o) in output.iter().enumerate() {
                                let index =
                                    ((height - (counter as u32) - 1) * width + (i as u32)) as usize;
                                render_data[index] = *o;
                            }
                            window
                                .update_with_buffer(&render_data, width as usize, height as usize)
                                .unwrap();

                            counter -= 1;
                        }
                    }
                    RenderStatus::Done => window.update(),
                },
                Err(_) => window.update(),
            }
        }
    }
}