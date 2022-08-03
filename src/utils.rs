use std::mem::MaybeUninit;

#[macro_export]
macro_rules! unsafe_read_type {
    ($ty:ty, $file:expr, $index:expr) => {{
        let mut config: $ty = unsafe { std::mem::zeroed() };
        let config_size = std::mem::size_of::<$ty>();
        unsafe {
            use std::io::Read;
            let config_slice = std::slice::from_raw_parts_mut(&mut config as *mut _ as *mut u8, config_size);
            let mut data_slice = &$file[$index..];
            data_slice.read_exact(config_slice).unwrap();
        }
        $index += config_size;
        config
        }};
}
pub use unsafe_read_type;

pub fn init_optional_array_to_blank<T, const C: usize>() -> [Option<T>; C] {
    let mut data: [MaybeUninit<Option<T>>; C] = unsafe { MaybeUninit::uninit().assume_init() };
        for p in &mut data[..] {
            p.write(None);
        }

    unsafe { MaybeUninit::array_assume_init(data) }
}