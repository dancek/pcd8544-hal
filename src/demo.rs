use Pcd8544;

pub fn rust_logo() -> [u8; 504] {
    let logo = include_bytes!("logo.bin");
    *logo
}

pub fn demo(pcd8544: &mut Pcd8544) {
    let logo = rust_logo();
    pcd8544.draw_buffer(&logo);
}

