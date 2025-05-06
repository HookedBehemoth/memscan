#[macro_export]
macro_rules! unroll {
    (6, $code:stmt) => {$code unroll!(5, $code)};
    (5, $code:stmt) => {$code unroll!(4, $code)};
    (4, $code:stmt) => {$code unroll!(3, $code)};
    (3, $code:stmt) => {$code unroll!(2, $code)};
    (2, $code:stmt) => {$code unroll!(1, $code)};
    (1, $code:stmt) => {$code}
}
#[macro_export]
macro_rules! impl_range {
    ($feat:expr, $name:ident, $cmpvec:ident, $cmpg:tt, $cmpl:tt, $ty:ty, $intr:ty, $stride:expr, $splat:ident, $load:ident, $movemask:ident) => {
        #[target_feature(enable = $feat)]
        unsafe fn $name(lb: $ty, ub: $ty, haystack: &[u8]) -> Option<usize> {
            use crate::unroll;

            let width = core::mem::size_of::<$ty>();
            let align = width - 1;
            let reg_width = core::mem::size_of::<$intr>();
            let reg_align = reg_width - 1;

            let start = haystack.as_ptr();
            let end = start.add(haystack.len());

            /* Splat mask onto wide register */
            let mask_lb = $splat(core::mem::transmute(lb));
            let mask_ub = $splat(core::mem::transmute(ub));

            /* Align pointer to needle size */
            let mut ptr = (start as usize + align & !align) as *const u8;
            let aligned_ptr = (ptr as usize + reg_align & !reg_align) as *const u8;

            fn single_range(val: $ty, lb: $ty, ub: $ty) -> bool {
                val $cmpg lb && val $cmpl ub
            }

            /* Load first unaligned access if necessary */
            while ptr < aligned_ptr {
                if single_range(*(ptr as *const $ty), lb, ub) {
                    return Some(ptr.offset_from(start) as usize);
                }
                ptr = ptr.add(width);
            }

            /* Align pointer to register width */
            ptr = (ptr as usize + reg_align & !reg_align) as *const u8;

            while ptr.add(reg_width * 6) <= end {
                unroll!(6, {
                    let result = $movemask($cmpvec($load(ptr as _), mask_lb, mask_ub));
                    if result != 0 {
                        let index = result.trailing_zeros() / $stride;
                        return Some(ptr.add(index as usize * width).offset_from(start) as usize);
                    }
                    ptr = ptr.add(reg_width);
                });
            }

            while ptr.add(reg_width) <= end {
                let result = $movemask($cmpvec($load(ptr as _), mask_lb, mask_ub));

                if result != 0 {
                    let index = result.trailing_zeros() / $stride;
                    return Some(ptr.add(index as usize * width).offset_from(start) as usize);
                }

                ptr = ptr.add(reg_width);
            }

            /* Gather remaining values */
            while ptr.add(width) <= end {
                if single_range(*(ptr as *const $ty), lb, ub) {
                    return Some(ptr.offset_from(start) as usize);
                }
                ptr = ptr.add(width);
            }

            None
        }
    };
}

#[macro_export]
macro_rules! impl_find {
    ($feat:expr, $trait:ident, $ty:ty, $intr:ty, $stride:expr, $splat:ident, $load:ident, $movemask:ident, $cmpeq:ident, $inclusive_range:ident, $exclusive_range:ident) => {
        impl $trait for $ty {
            #[target_feature(enable = $feat)]
            unsafe fn find_next(needle: $ty, haystack: &[u8]) -> Option<usize> {
                use crate::unroll;

                let width = core::mem::size_of::<$ty>();
                let align = width - 1;
                let reg_width = core::mem::size_of::<$intr>();
                let reg_align = reg_width - 1;

                let start = haystack.as_ptr();
                let end = start.add(haystack.len());

                /* Splat mask onto wide register */
                let mask = $splat(core::mem::transmute(needle));

                /* Align pointer to needle size */
                let mut ptr = (start as usize + align & !align) as *const u8;
                let aligned_ptr = (ptr as usize + reg_align & !reg_align) as *const u8;

                /* Load first unaligned access if necessary */
                while ptr < aligned_ptr {
                    if *(ptr as *const $ty) == needle {
                        return Some(ptr.offset_from(start) as usize);
                    }
                    ptr = ptr.add(width);
                }

                /* Align pointer to register width */
                ptr = (ptr as usize + reg_align & !reg_align) as *const u8;

                while ptr.add(reg_width * 6) <= end {
                    unroll!(6, {
                        let result = $movemask($cmpeq($load(ptr as _), mask));
                        if result != 0 {
                            let index = result.trailing_zeros() / $stride;
                            return Some(ptr.add(index as usize * width).offset_from(start) as usize);
                        }
                        ptr = ptr.add(reg_width);
                    });
                }

                while ptr.add(reg_width) <= end {
                    let result = $movemask($cmpeq($load(ptr as _), mask));

                    if result != 0 {
                        let index = result.trailing_zeros() / $stride;
                        return Some(ptr.add(index as usize * width).offset_from(start) as usize);
                    }

                    ptr = ptr.add(reg_width);
                }

                /* Gather remaining values */
                while ptr.add(width) <= end {
                    if *(ptr as *const $ty) == needle {
                        return Some(ptr.offset_from(start) as usize);
                    }
                    ptr = ptr.add(width);
                }

                None
            }
            impl_range!($feat, find_inclusive_range, $inclusive_range, >=, <=, $ty, $intr, $stride, $splat, $load, $movemask);
            impl_range!($feat, find_exclusive_range, $exclusive_range, >, <, $ty, $intr, $stride, $splat, $load, $movemask);
        }
    };
}
