use crate::{impl_find, impl_range, impl_tests};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

pub trait Needle {
    unsafe fn find_next(needle: Self, haystack: &[u8]) -> Option<usize>;
    unsafe fn find_inclusive_range(start: Self, end: Self, haystack: &[u8]) -> Option<usize>;
    unsafe fn find_exclusive_range(start: Self, end: Self, haystack: &[u8]) -> Option<usize>;
}

#[inline]
unsafe fn _mm_u8_inclusive_range(val: __m128i, gt: __m128i, lt: __m128i) -> __m128i {
    _mm_and_si128(
        _mm_or_si128(_mm_cmpgt_epi8(val, gt), _mm_cmpeq_epi8(val, gt)),
        _mm_or_si128(_mm_cmpgt_epi8(lt, val), _mm_cmpeq_epi8(lt, val)),
    )
}
#[inline]
unsafe fn _mm_u8_exclusive_range(val: __m128i, gt: __m128i, lt: __m128i) -> __m128i {
    _mm_and_si128(_mm_cmpgt_epi8(val, gt), _mm_cmpgt_epi8(lt, val))
}
impl_find!(
    "sse4.2",
    Needle,
    u8,
    __m128i,
    1,
    _mm_set1_epi8,
    _mm_load_si128,
    _mm_movemask_epi8,
    _mm_cmpeq_epi8,
    _mm_u8_inclusive_range,
    _mm_u8_exclusive_range
);
#[inline]
unsafe fn _mm_u16_inclusive_range(val: __m128i, gt: __m128i, lt: __m128i) -> __m128i {
    _mm_and_si128(
        _mm_or_si128(_mm_cmpgt_epi16(val, gt), _mm_cmpeq_epi16(val, gt)),
        _mm_or_si128(_mm_cmpgt_epi16(lt, val), _mm_cmpeq_epi16(lt, val)),
    )
}
#[inline]
unsafe fn _mm_u16_exclusive_range(val: __m128i, gt: __m128i, lt: __m128i) -> __m128i {
    _mm_and_si128(_mm_cmpgt_epi16(val, gt), _mm_cmpgt_epi16(lt, val))
}
impl_find!(
    "sse4.2",
    Needle,
    u16,
    __m128i,
    2,
    _mm_set1_epi16,
    _mm_load_si128,
    _mm_movemask_epi8,
    _mm_cmpeq_epi16,
    _mm_u16_inclusive_range,
    _mm_u16_exclusive_range
);
#[inline]
unsafe fn _mm_u32_inclusive_range(val: __m128i, gt: __m128i, lt: __m128i) -> __m128i {
    _mm_and_si128(
        _mm_or_si128(_mm_cmpgt_epi32(val, gt), _mm_cmpeq_epi32(val, gt)),
        _mm_or_si128(_mm_cmpgt_epi32(lt, val), _mm_cmpeq_epi32(lt, val)),
    )
}
#[inline]
unsafe fn _mm_u32_exclusive_range(val: __m128i, gt: __m128i, lt: __m128i) -> __m128i {
    _mm_and_si128(_mm_cmpgt_epi32(val, gt), _mm_cmpgt_epi32(lt, val))
}
impl_find!(
    "sse4.2",
    Needle,
    u32,
    __m128i,
    4,
    _mm_set1_epi32,
    _mm_load_si128,
    _mm_movemask_epi8,
    _mm_cmpeq_epi32,
    _mm_u32_inclusive_range,
    _mm_u32_exclusive_range
);
#[inline]
unsafe fn _mm_u64_inclusive_range(val: __m128i, gt: __m128i, lt: __m128i) -> __m128i {
    _mm_and_si128(
        _mm_or_si128(_mm_cmpgt_epi64(val, gt), _mm_cmpeq_epi64(val, gt)),
        _mm_or_si128(_mm_cmpgt_epi64(lt, val), _mm_cmpeq_epi64(lt, val)),
    )
}
#[inline]
unsafe fn _mm_u64_exclusive_range(val: __m128i, gt: __m128i, lt: __m128i) -> __m128i {
    _mm_and_si128(_mm_cmpgt_epi64(val, gt), _mm_cmpgt_epi64(lt, val))
}
impl_find!(
    "sse4.2",
    Needle,
    u64,
    __m128i,
    8,
    _mm_set1_epi64x,
    _mm_load_si128,
    _mm_movemask_epi8,
    _mm_cmpeq_epi64,
    _mm_u64_inclusive_range,
    _mm_u64_exclusive_range
);
#[inline]
unsafe fn _mm_f32_inclusive_range(val: __m128, gt: __m128, lt: __m128) -> __m128 {
    _mm_and_ps(_mm_cmpge_ps(val, gt), _mm_cmple_ps(val, lt))
}
#[inline]
unsafe fn _mm_f32_exclusive_range(val: __m128, gt: __m128, lt: __m128) -> __m128 {
    _mm_and_ps(_mm_cmpgt_ps(val, gt), _mm_cmplt_ps(val, lt))
}
impl_find!(
    "sse4.2",
    Needle,
    f32,
    __m128,
    1,
    _mm_set1_ps,
    _mm_load_ps,
    _mm_movemask_ps,
    _mm_cmpeq_ps,
    _mm_f32_inclusive_range,
    _mm_f32_exclusive_range
);
#[inline]
unsafe fn _mm_f64_inclusive_range(val: __m128d, gt: __m128d, lt: __m128d) -> __m128d {
    _mm_and_pd(_mm_cmpge_pd(val, gt), _mm_cmple_pd(val, lt))
}
#[inline]
unsafe fn _mm_f64_exclusive_range(val: __m128d, gt: __m128d, lt: __m128d) -> __m128d {
    _mm_and_pd(_mm_cmpgt_pd(val, gt), _mm_cmplt_pd(val, lt))
}
impl_find!(
    "sse4.2",
    Needle,
    f64,
    __m128d,
    1,
    _mm_set1_pd,
    _mm_load_pd,
    _mm_movemask_pd,
    _mm_cmpeq_pd,
    _mm_f64_inclusive_range,
    _mm_f64_exclusive_range
);

impl_tests!(Needle);
