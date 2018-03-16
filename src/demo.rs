use Pcd8544;

type Buffer = [u8; 504];

static RUST_LOGO: &'static Buffer = include_bytes!("logo.bin");

fn empty_buffer() -> Buffer {
    [0u8; 504]
}

pub fn demo(pcd8544: &mut Pcd8544) {
    loop {
        for _ in 0..20 {
            pcd8544.draw_buffer(RUST_LOGO);
        }

        run_shader(pcd8544, 0..10, |x,y,t| x+y == t);
        run_shader(pcd8544, 0..10, |x,y,t| x-y == t);
        run_shader(pcd8544, 0..30, |x,y,t| x*x + y*y < t*t && x*x + y*y >= (t-1).pow(2));
    }
}

pub fn apply_shader<F: Fn(i32, i32, i32) -> bool>(buffer: &mut Buffer, t: i32, f: F) {
    for col in 0..84 {
        for row in 0..6 {
            let x = col as i32 - 42;
            let mut byte = 0x00;
            for bit in 0..8 {
                let y = 8 * (row as i32 - 3) + bit;
                byte += (f(x, y, t) as u8) << bit;
            }
            buffer[6*col + row] = byte;
        }
    }
}

pub fn run_shader<F: Fn(i32, i32, i32) -> bool>(pcd8544: &mut Pcd8544, t_range: ::core::ops::Range<i32>, f: F) -> Buffer {
    let mut buffer = empty_buffer();
    for t in t_range {
        apply_shader(&mut buffer, t, &f);
        pcd8544.draw_buffer(&buffer);
    }
    buffer
}
