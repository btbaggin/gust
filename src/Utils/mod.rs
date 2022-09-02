use std::mem::MaybeUninit;

mod timer;
mod tween;
pub use timer::{FrameTimer, Timer};

#[macro_export]
macro_rules! entity_as {
    ($entity:ident, $ty:ty) => {
        $entity.as_any().downcast_ref::<$ty>()
    }
}

#[macro_export]
macro_rules! entity_as_mut {
    ($entity:ident, $ty:ty) => {
        entity.as_any_mut().downcast_mut::<$ty>()
    }
}

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

pub fn init_optional_array_to_blank<T, const C: usize>() -> [Option<T>; C] {
    let mut data: [MaybeUninit<Option<T>>; C] = unsafe { MaybeUninit::uninit().assume_init() };
        for p in &mut data[..] {
            p.write(None);
        }

    unsafe { MaybeUninit::array_assume_init(data) }
}

#[macro_export]
macro_rules! singleton {
    ($name:ident: $ty:ty = $init:expr) => {
        use paste::paste;
        paste! {
            static mut [<$name:upper _VAR>]: Option<$ty> = None;
            static [<$name:upper _INIT>]: std::sync::Once = std::sync::Once::new();

            pub fn $name<'a>() -> &'a mut $ty {
                use std::borrow::BorrowMut;
                [<$name:upper _INIT>].call_once(|| {
                    // Since this access is inside a call_once, before any other accesses, it is safe
                    unsafe {
                        *[<$name:upper _VAR>].borrow_mut() = Some($init);
                    }
                });
                // As long as this function is the only place with access to the static variable,
                // giving out a read-only borrow here is safe because it is guaranteed no more mutable
                // references will exist at this point or in the future.
                unsafe { [<$name:upper _VAR>].as_mut().unwrap() }
            }
        }
    }
}