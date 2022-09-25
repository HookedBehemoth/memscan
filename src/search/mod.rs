#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx2;
#[macro_use]
mod generic_simd;
pub mod primitive;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse42;
pub mod tests;

pub struct MemorySearch<'a, T: Sized + PartialEq> {
    needle: T,
    haystack: &'a [u8],
    current: usize,
}

impl<'a, T: Sized + PartialEq> MemorySearch<'a, T> {
    pub fn new(needle: T, haystack: &'a [u8]) -> Self {
        Self {
            needle,
            haystack,
            current: 0,
        }
    }
}

pub struct InclusiveRangeSearch<'a, T: Sized + PartialEq> {
    lower_bounds: T,
    upper_bounds: T,
    haystack: &'a [u8],
    current: usize,
}

impl<'a, T: Sized + PartialEq> InclusiveRangeSearch<'a, T> {
    pub fn new(lower_bounds: T, upper_bounds: T, haystack: &'a [u8]) -> Self {
        Self {
            lower_bounds,
            upper_bounds,
            haystack,
            current: 0,
        }
    }
}

pub struct ExclusiveRangeSearch<'a, T: Sized + PartialEq> {
    lower_bounds: T,
    upper_bounds: T,
    haystack: &'a [u8],
    current: usize,
}

impl<'a, T: Sized + PartialEq> ExclusiveRangeSearch<'a, T> {
    pub fn new(lower_bounds: T, upper_bounds: T, haystack: &'a [u8]) -> Self {
        Self {
            lower_bounds,
            upper_bounds,
            haystack,
            current: 0,
        }
    }
}

macro_rules! export_part {
    ($struct:ident, $func:ident, $ty:ty, $( $needle:ident),+) => {
        impl<'a> Iterator for $struct<'a, $ty> {
            type Item = usize;
            fn next(&mut self) -> Option<Self::Item> {
                if self.current >= self.haystack.len() {
                    None
                } else if let Some(result) = unsafe {
                    let haystack = &self.haystack[self.current..];
                    if core_detect::is_x86_feature_detected!("avx2") {
                        avx2::Needle::$func($(self.$needle),+,  haystack)
                    } else if core_detect::is_x86_feature_detected!("sse4.2") {
                        sse42::Needle::$func($(self.$needle),+, haystack)
                    } else {
                        primitive::Needle::$func($(self.$needle),+, haystack)
                    }
                } {
                    let result = self.current + result;
                    self.current = result + core::mem::size_of::<$ty>();
                    Some(result)
                } else {
                    self.current = 0;
                    None
                }
            }
        }
    };
}

macro_rules! export_fwd {
    ($name:ident, $func:ident, $ty:ty, $( $needle:ident),+) => {
        #[no_mangle]
        pub fn $name($($needle: $ty),+, haystack: &[u8]) -> Option<usize> {
            unsafe {
                if core_detect::is_x86_feature_detected!("avx2") {
                    avx2::Needle::$func($($needle),+, haystack)
                } else if core_detect::is_x86_feature_detected!("sse4.2") {
                    sse42::Needle::$func($($needle),+, haystack)
                } else {
                    primitive::Needle::$func($($needle),+, haystack)
                }
            }
        }
    };
}

macro_rules! export {
    ($find_first:ident, $find_inclusive_range:ident, $find_exclusive_range:ident, $ty:ty) => {
        export_part!(MemorySearch, find_next, $ty, needle);
        export_part!(
            InclusiveRangeSearch,
            find_inclusive_range,
            $ty,
            lower_bounds,
            upper_bounds
        );
        export_part!(
            ExclusiveRangeSearch,
            find_exclusive_range,
            $ty,
            lower_bounds,
            upper_bounds
        );

        export_fwd!($find_first, find_next, $ty, needle);
        export_fwd!(
            $find_inclusive_range,
            find_inclusive_range,
            $ty,
            lower_bounds,
            upper_bounds
        );
        export_fwd!(
            $find_exclusive_range,
            find_exclusive_range,
            $ty,
            lower_bounds,
            upper_bounds
        );
    };
}

export!(find_first_u8, find_inclusive_u8, find_exclusive_u8, u8);
export!(find_first_u16, find_inclusive_u16, find_exclusive_u16, u16);
export!(find_first_u32, find_inclusive_u32, find_exclusive_u32, u32);
export!(find_first_u64, find_inclusive_u64, find_exclusive_u64, u64);
export!(find_first_f32, find_inclusive_f32, find_exclusive_f32, f32);
export!(find_first_f64, find_inclusive_f64, find_exclusive_f64, f64);

#[test]
fn test_exported() {
    unsafe {
        let mut haystack = [0u8; 100];
        for i in 0..100 {
            haystack[i] = i as _;
        }
        assert_eq!(find_first_u8(0u8, &haystack), Some(0 * 1), "first u8");
        assert_eq!(find_first_u8(50u8, &haystack), Some(50 * 1), "middle u8");
        assert_eq!(find_first_u8(99u8, &haystack), Some(99 * 1), "last u8");

        let mut haystack = [0u16; 100];
        for i in 0..100 {
            haystack[i] = i as _;
        }
        let haystack =
            core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 2);
        assert_eq!(find_first_u16(0u16, haystack), Some(0 * 2), "first u16");
        assert_eq!(find_first_u16(50u16, haystack), Some(50 * 2), "middle u16");
        assert_eq!(find_first_u16(99u16, haystack), Some(99 * 2), "last u16");

        let mut haystack = [0u32; 100];
        for i in 0..100 {
            haystack[i] = i as _;
        }
        let haystack =
            core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 4);
        assert_eq!(find_first_u32(0u32, haystack), Some(0 * 4));
        assert_eq!(find_first_u32(50u32, haystack), Some(50 * 4));
        assert_eq!(find_first_u32(99u32, haystack), Some(99 * 4));

        let mut haystack = [0u64; 100];
        for i in 0..100 {
            haystack[i] = i as _;
        }
        let haystack =
            core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 8);
        assert_eq!(find_first_u64(0u64, haystack), Some(0 * 8));
        assert_eq!(find_first_u64(50u64, haystack), Some(50 * 8));
        assert_eq!(find_first_u64(99u64, haystack), Some(99 * 8));

        let mut haystack = [0f32; 100];
        for i in 0..100 {
            haystack[i] = i as _;
        }
        let haystack =
            core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 4);
        assert_eq!(find_first_f32(0.0f32, haystack), Some(0 * 4));
        assert_eq!(find_first_f32(50.0f32, haystack), Some(50 * 4));
        assert_eq!(find_first_f32(99.0f32, haystack), Some(99 * 4));
        assert_eq!(find_first_f32(f32::NAN, haystack), None);

        let mut haystack = [0f64; 100];
        for i in 0..100 {
            haystack[i] = i as _;
        }
        let haystack =
            core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 8);
        assert_eq!(find_first_f64(0.0f64, haystack), Some(0 * 8));
        assert_eq!(find_first_f64(50.0f64, haystack), Some(50 * 8));
        assert_eq!(find_first_f64(99.0f64, haystack), Some(99 * 8));
        assert_eq!(find_first_f64(f64::NAN, haystack), None);
    }
}

#[test]
fn test_iter() {
    macro_rules! test_iter {
        ($ty:ty) => {
            let width = core::mem::size_of::<$ty>();
            let mut haystack = [0 as $ty; 100];
            haystack[0] = 1 as _;
            haystack[13] = 1 as _;
            haystack[25] = 1 as _;
            haystack[50] = 1 as _;
            haystack[99] = 1 as _;
            let haystack = unsafe {
                core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * width)
            };
            let search = MemorySearch::new(1 as $ty, &haystack);
            let mut iter = search.into_iter();
            assert_eq!(iter.next(), Some(0 * width));
            assert_eq!(iter.next(), Some(13 * width));
            assert_eq!(iter.next(), Some(25 * width));
            assert_eq!(iter.next(), Some(50 * width));
            assert_eq!(iter.next(), Some(99 * width));
            assert_eq!(iter.next(), None);

            let width = core::mem::size_of::<$ty>();
            let mut haystack = [0 as $ty; 100];
            for i in 0..100 {
                haystack[i] = i as _;
            }
            let haystack = unsafe {
                core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * width)
            };
            let search = InclusiveRangeSearch::new(30 as $ty, 40 as $ty, &haystack);
            let mut iter = search.into_iter();
            for i in 30..=40 {
                assert_eq!(iter.next(), Some(i * width));
            }
            assert_eq!(iter.next(), None);

            let search = ExclusiveRangeSearch::new(30 as $ty, 40 as $ty, &haystack);
            let mut iter = search.into_iter();
            for i in 31..40 {
                assert_eq!(iter.next(), Some(i * width));
            }
            assert_eq!(iter.next(), None);
        };
    }

    test_iter!(u8);
    test_iter!(u16);
    test_iter!(u32);
    test_iter!(u64);
    test_iter!(f32);
    test_iter!(f64);
}
