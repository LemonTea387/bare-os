use lazy_static::lazy_static;
use spin::Mutex;
use vga_buffer::ScreenWriter;

mod vga_buffer;

lazy_static! {
    pub static ref VGA_Writer: Mutex<ScreenWriter> = Mutex::new(ScreenWriter::default());
}
