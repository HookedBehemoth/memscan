use crate::impl_tests;

pub struct Needle {}

macro_rules! impl_range {
    ($name:ident, $cmpg:tt, $cmpl:tt) => {
        pub unsafe fn $name<T: PartialEq + PartialOrd + Copy>(lb: T, ub: T, haystack: &[u8]) -> Option<usize> {
            let width = core::mem::size_of::<T>();
            let align = width - 1;

            let start = haystack.as_ptr();

            /* Align pointer to needle size */
            let mut ptr = (start as usize + align & !align) as *const u8;
            let end = start.add(haystack.len());

            /* Gather remaining values */
            while ptr.add(width) <= end {
                let val = *(ptr as *const T);
                if val $cmpg lb && val $cmpl ub {
                    return Some(ptr.offset_from(start) as usize);
                }
                ptr = ptr.add(width);
            }

            None
        }
    };
}

impl Needle {
    pub unsafe fn find_next<T: PartialEq>(needle: T, haystack: &[u8]) -> Option<usize> {
        let width = core::mem::size_of::<T>();
        let align = width - 1;

        let start = haystack.as_ptr();

        /* Align pointer to needle size */
        let mut ptr = (start as usize + align & !align) as *const u8;
        let end = start.add(haystack.len());

        /* Gather remaining values */
        while ptr.add(width) <= end {
            if *(ptr as *const T) == needle {
                return Some(ptr.offset_from(start) as usize);
            }
            ptr = ptr.add(width);
        }

        None
    }
    impl_range!(find_inclusive_range, >=, <=);
    impl_range!(find_exclusive_range, >, <);
}

impl_tests!(Needle);
