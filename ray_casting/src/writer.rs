use crate::framebuffer::FrameBuffer;
use crate::color::Color;

pub trait Writer {
    fn write_to_buffer(text: &str, initial_x_pos: usize, initial_y_pos: usize, block_size:usize, spacing:usize){
        let mut first_x = initial_x_pos;
        let mut first_y = initial_y_pos;
        let mut final_x = initial_x_pos + (block_size * 3);
        let mut final_y = initial_y_pos + (block_size * 5);

        for i in ..final_x{
            for j in first_y..final_y{

            }
        }
    }
}