#[macro_export]
macro_rules! impl_tests {
    ($trait:ident) => {
        #[test]
        fn test_find_first() {
            unsafe {
                let mut haystack = [0u8; 100];
                for i in 0..100 {
                    haystack[i] = i as _;
                }
                assert_eq!($trait::find_next(0u8, &haystack), Some(0 * 1), "first u8");
                assert_eq!(
                    $trait::find_next(50u8, &haystack),
                    Some(50 * 1),
                    "middle u8"
                );
                assert_eq!($trait::find_next(99u8, &haystack), Some(99 * 1), "last u8");
                assert_eq!(
                    $trait::find_inclusive_range(10u8, 20u8, &haystack),
                    Some(10 * 1),
                    "inclusive u8"
                );
                assert_eq!(
                    $trait::find_exclusive_range(10u8, 20u8, &haystack),
                    Some(11 * 1),
                    "exclusive u8"
                );

                let mut haystack = [0u16; 100];
                for i in 0..100 {
                    haystack[i] = i as _;
                }
                let haystack =
                    core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 2);
                assert_eq!($trait::find_next(0u16, haystack), Some(0 * 2), "first u16");
                assert_eq!(
                    $trait::find_next(50u16, haystack),
                    Some(50 * 2),
                    "middle u16"
                );
                assert_eq!($trait::find_next(99u16, haystack), Some(99 * 2), "last u16");
                assert_eq!(
                    $trait::find_inclusive_range(10u16, 20u16, &haystack),
                    Some(10 * 2),
                    "inclusive u16"
                );
                assert_eq!(
                    $trait::find_exclusive_range(10u16, 20u16, &haystack),
                    Some(11 * 2),
                    "exclusive u16"
                );

                let mut haystack = [0u32; 100];
                for i in 0..100 {
                    haystack[i] = i as _;
                }
                let haystack =
                    core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 4);
                assert_eq!($trait::find_next(0u32, haystack), Some(0 * 4), "first u32");
                assert_eq!(
                    $trait::find_next(50u32, haystack),
                    Some(50 * 4),
                    "middle u32"
                );
                assert_eq!($trait::find_next(99u32, haystack), Some(99 * 4), "last u32");
                assert_eq!(
                    $trait::find_inclusive_range(10u32, 20u32, &haystack),
                    Some(10 * 4),
                    "inclusive u32"
                );
                assert_eq!(
                    $trait::find_exclusive_range(10u32, 20u32, &haystack),
                    Some(11 * 4),
                    "exclusive u32"
                );

                let mut haystack = [0u64; 100];
                for i in 0..100 {
                    haystack[i] = i as _;
                }
                let haystack =
                    core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 8);
                assert_eq!($trait::find_next(0u64, haystack), Some(0 * 8), "first u64");
                assert_eq!(
                    $trait::find_next(50u64, haystack),
                    Some(50 * 8),
                    "middle u64"
                );
                assert_eq!($trait::find_next(99u64, haystack), Some(99 * 8), "last u64");
                assert_eq!(
                    $trait::find_inclusive_range(10u64, 20u64, &haystack),
                    Some(10 * 8),
                    "inclusive u64"
                );
                assert_eq!(
                    $trait::find_exclusive_range(10u64, 20u64, &haystack),
                    Some(11 * 8),
                    "exclusive u64"
                );

                let mut haystack = [0f32; 100];
                for i in 0..100 {
                    haystack[i] = i as _;
                }
                let haystack =
                    core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 4);
                assert_eq!(
                    $trait::find_next(0.0f32, haystack),
                    Some(0 * 4),
                    "first f32"
                );
                assert_eq!(
                    $trait::find_next(50.0f32, haystack),
                    Some(50 * 4),
                    "middle f32"
                );
                assert_eq!(
                    $trait::find_next(99.0f32, haystack),
                    Some(99 * 4),
                    "last f32"
                );
                assert_eq!(
                    $trait::find_inclusive_range(10f32, 20f32, &haystack),
                    Some(10 * 4),
                    "inclusive f32"
                );
                assert_eq!(
                    $trait::find_exclusive_range(10f32, 20f32, &haystack),
                    Some(11 * 4),
                    "exclusive f32"
                );
                assert_eq!($trait::find_next(f32::NAN, haystack), None);

                let mut haystack = [0f64; 100];
                for i in 0..100 {
                    haystack[i] = i as _;
                }
                let haystack =
                    core::slice::from_raw_parts(haystack.as_ptr() as *const u8, haystack.len() * 8);
                assert_eq!(
                    $trait::find_next(0.0f64, haystack),
                    Some(0 * 8),
                    "first f64"
                );
                assert_eq!(
                    $trait::find_next(50.0f64, haystack),
                    Some(50 * 8),
                    "middle f64"
                );
                assert_eq!(
                    $trait::find_next(99.0f64, haystack),
                    Some(99 * 8),
                    "last f64"
                );
                assert_eq!(
                    $trait::find_inclusive_range(10f64, 20f64, &haystack),
                    Some(10 * 8),
                    "inclusive f64"
                );
                assert_eq!(
                    $trait::find_exclusive_range(10f64, 20f64, &haystack),
                    Some(11 * 8),
                    "exclusive f64"
                );
                assert_eq!($trait::find_next(f64::NAN, haystack), None);
            }
        }
    };
}
