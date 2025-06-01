use std::ptr;
use winapi::um::wingdi::{CreateSolidBrush, RGB, SelectObject, Rectangle, SetPixel};
use winapi::um::winuser::{GetDC, GetDesktopWindow, InvalidateRect, RedrawWindow, ReleaseDC, UpdateWindow};
use winapi::shared::windef::{HDC, HBRUSH, RECT};
use gif::{Decoder, Frame};
use std::io::Cursor;

const GIF_DATA: &[u8] = include_bytes!("clippy.gif");
const FRAMES:usize =  18; // Number of frames to read from the GIF

fn main() {
    let mut positions: Vec<Vec<i32>> = vec![];
    // array of vectors
    let mut last_periodic_call = std::time::Instant::now();
    let cursor = Cursor::new(GIF_DATA);
    let mut decoder = Decoder::new(cursor).unwrap();
    let mut i = 0;   let width = (&decoder).width() as usize;
    let height = (&decoder).height() as usize;
    let palette = (&decoder).global_palette().unwrap().to_vec();
    while true {
        unsafe {

       
         // Get desktop device context
        let desktop_hwnd = GetDesktopWindow();
        let desktop_dc: HDC = GetDC(desktop_hwnd);
        
        if desktop_dc.is_null() {
            panic!("Failed to get desktop device context");
        }
        // Keep the program running to allow drawing
        std::thread::sleep(std::time::Duration::from_millis(1));
      
        // Call periodic() every 1000ms without blocking
        if last_periodic_call.elapsed() >= std::time::Duration::from_millis(10) {
            periodic(&mut positions);
            last_periodic_call = std::time::Instant::now();
        }
        if i >= FRAMES {
            // Reset the decoder to the first frame
            decoder = Decoder::new(Cursor::new(GIF_DATA)).unwrap();
            i = 0; // Reset frame index
            println!("Resetting decoder to the first frame");
        }
        let frame = decoder.read_next_frame().unwrap().unwrap();
        i += 1;
 
        
     
        println!("Color at (0, 0): {:?}", get_pixel_color(&frame, &palette, 0, 0));
        for y in (0..height).step_by(4) {
            for x in (0..width).step_by(4) {
             
                let xn = x / 4 as usize;
                let yn = y / 4 as usize;
                
                // println!("Color at ({}, {}): {:?}", 210, 200, get_pixel_color(&mut decoder, 210, 200));
                if let Some(color) = get_pixel_color(frame, &palette, x, y) {
                    // Draw the pixel at (x, y) with the color
                    if (color[0] > 130 && color[1] > 130 && color[2] > 130) {
                        continue; // Skip white pixels
                    }
                    SetPixel(desktop_dc, xn as i32, yn as i32, RGB(color[0], color[1], color[2]));
                    // set_pixel(x as i32, y as i32, color[0], color[1], color[2]);
                }
            }
        }
      println!("Finished processing frame {}", i);
        let rect = RECT {
            left: 0,
            top: 0,
            right: 500,
            bottom: 500,
        };
        
        // Invalidate the rectangle, causing Windows to redraw that region
        // The TRUE parameter means erase the background before redrawing
        InvalidateRect(desktop_hwnd, &rect, 1);
        // Update the window to reflect the changes
       
        RedrawWindow(hwnd, lprcUpdate, hrgnUpdate, flags);
        // Release the desktop device context
       ReleaseDC(desktop_hwnd, desktop_dc);
        // Draw a square at the center of the screen
        // go through the positions array and draw a red square at each position
        for i in 0..positions.len() {
            let pos = &positions[i];
            if pos.len() == 2 {
                let x = pos[0];
                let y = pos[1];
                let rr = rand::random::<u8>() % 255; // Random red value
                let rg = rand::random::<u8>() % 255; // Random green value
                let rb = rand::random::<u8>() % 255; // Random blue value
                
                positions[i][0] = x + (rand::random::<i32>() % 2); // Randomly adjust x position
                positions[i][1] = y + (rand::random::<i32>() % 2); // Randomly adjust y position
                // set_pixel(x, y, rr, rg, rb); // Draw a red pixel at the position
            }
        }
        
    }
    }
}
fn get_pixel_color(frame: &Frame<'_>, palette: &Vec<u8>, x1: usize, y1: usize) -> Option<[u8; 3]> {
   
    
    
  

    let width = frame.width as usize;
    let height = frame.height as usize;

    if x1 >= width || y1 >= height {
        return None; // Out of bounds
    }

    // Calculate the index in the buffer
    let idx = y1 * width + x1;
    let color_idx = frame.buffer[idx] as usize;

    // Get the palette (local or global)
    

    // Each color is 3 bytes: R, G, B
    let r = palette[color_idx * 3];
    let g = palette[color_idx * 3 + 1];
    let b = palette[color_idx * 3 + 2]; // no fricking idea how this code works, cs is cooked ai figured this all out :skull:

    Some([r, g, b])
}

fn periodic(positions: &mut Vec<Vec<i32>>) {
    // This function will be called every 1000ms
    // println!("Periodic function called at {:?}", std::time::Instant::now());
    let random_pos_x = rand::random::<i32>() % 2500; // Example of generating a random position
    let random_pos_y = rand::random::<i32>() % 1300; // Example of generating a random position
    let random_pos = vec![random_pos_x, random_pos_y];
    // println!("Generated random position: {:?}", random_pos);
    positions.push(random_pos); // Example of adding a vector to the positions array
    

}



fn draw_square(x1: i32, y1: i32, x2: i32, y2: i32, red: u8, green: u8, blue: u8) {
    unsafe {
        // Get desktop device context
        let desktop_hwnd = GetDesktopWindow();
        let desktop_dc: HDC = GetDC(desktop_hwnd);
        
        if desktop_dc.is_null() {
            panic!("Failed to get desktop device context");
        }

        // Create brush with specified color
        let brush: HBRUSH = CreateSolidBrush(RGB(red, green, blue));
        let old_brush = SelectObject(desktop_dc, brush as *mut _);

        // Draw rectangle from (x1, y1) to (x2, y2)
        Rectangle(
            desktop_dc,
            x1,
            y1,
            x2,
            y2,
        );

        // Cleanup
        SelectObject(desktop_dc, old_brush);
        ReleaseDC(desktop_hwnd, desktop_dc);
        
        println!("Square drawn to desktop from ({}, {}) to ({}, {}) with color RGB({}, {}, {})", x1, y1, x2, y2, red, green, blue);
    }
}

fn set_pixel(x: i32, y: i32, red: u8, green: u8, blue: u8) {
    unsafe {
        // Get desktop device context
        let desktop_hwnd = GetDesktopWindow();
        let desktop_dc: HDC = GetDC(desktop_hwnd);
        
        if desktop_dc.is_null() {
            panic!("Failed to get desktop device context");
        }

        // Set the pixel at (x, y) to the specified color
        SetPixel(desktop_dc, x, y, RGB(red, green, blue));

        // Cleanup
        ReleaseDC(desktop_hwnd, desktop_dc);
    }
}

fn refresh_screen_region(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        // Get desktop window handle
        let desktop_hwnd = GetDesktopWindow();
        
        // Create rectangle structure for the region to invalidate
      
        
        println!("Refreshed screen region from ({}, {}) to ({}, {})", x1, y1, x2, y2);
    }
}
