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

/**
 * Note:
 *  This should be
 *      val >= gt & val <= lt
 *  however since avx lacks le & ge it's written as such
 *      (val > gt | val == gt) & (lt > val | val == lt)
 *  The compiler optimizes these to
 *      !(val < gt) & !(lt < val)
 *  Writing it like that ourselfs requires temporaries.
 */
#[inline]
unsafe fn _mm256_u8_inclusive_range(val: __m256i, gt: __m256i, lt: __m256i) -> __m256i {
    _mm256_and_si256(
        _mm256_or_si256(_mm256_cmpgt_epi8(val, gt), _mm256_cmpeq_epi8(val, gt)),
        _mm256_or_si256(_mm256_cmpgt_epi8(lt, val), _mm256_cmpeq_epi8(lt, val)),
    )
}
#[inline]
unsafe fn _mm256_u8_exclusive_range(val: __m256i, gt: __m256i, lt: __m256i) -> __m256i {
    _mm256_and_si256(_mm256_cmpgt_epi8(val, gt), _mm256_cmpgt_epi8(lt, val))
}
impl_find!(
    "avx2",
    Needle,
    u8,
    __m256i,
    1,
    _mm256_set1_epi8,
    _mm256_load_si256,
    _mm256_movemask_epi8,
    _mm256_cmpeq_epi8,
    _mm256_u8_inclusive_range,
    _mm256_u8_exclusive_range
);
#[inline]
unsafe fn _mm256_u16_inclusive_range(val: __m256i, gt: __m256i, lt: __m256i) -> __m256i {
    _mm256_and_si256(
        _mm256_or_si256(_mm256_cmpgt_epi16(val, gt), _mm256_cmpeq_epi16(val, gt)),
        _mm256_or_si256(_mm256_cmpgt_epi16(lt, val), _mm256_cmpeq_epi16(lt, val)),
    )
}
#[inline]
unsafe fn _mm256_u16_exclusive_range(val: __m256i, gt: __m256i, lt: __m256i) -> __m256i {
    _mm256_and_si256(_mm256_cmpgt_epi16(val, gt), _mm256_cmpgt_epi16(lt, val))
}
impl_find!(
    "avx2",
    Needle,
    u16,
    __m256i,
    2,
    _mm256_set1_epi16,
    _mm256_load_si256,
    _mm256_movemask_epi8,
    _mm256_cmpeq_epi16,
    _mm256_u16_inclusive_range,
    _mm256_u16_exclusive_range
);
#[inline]
unsafe fn _mm256_u32_inclusive_range(val: __m256i, gt: __m256i, lt: __m256i) -> __m256i {
    _mm256_and_si256(
        _mm256_or_si256(_mm256_cmpgt_epi32(val, gt), _mm256_cmpeq_epi32(val, gt)),
        _mm256_or_si256(_mm256_cmpgt_epi32(lt, val), _mm256_cmpeq_epi32(lt, val)),
    )
}
#[inline]
unsafe fn _mm256_u32_exclusive_range(val: __m256i, gt: __m256i, lt: __m256i) -> __m256i {
    _mm256_and_si256(_mm256_cmpgt_epi32(val, gt), _mm256_cmpgt_epi32(lt, val))
}
impl_find!(
    "avx2",
    Needle,
    u32,
    __m256i,
    4,
    _mm256_set1_epi32,
    _mm256_load_si256,
    _mm256_movemask_epi8,
    _mm256_cmpeq_epi32,
    _mm256_u32_inclusive_range,
    _mm256_u32_exclusive_range
);
#[inline]
unsafe fn _mm256_u64_inclusive_range(val: __m256i, gt: __m256i, lt: __m256i) -> __m256i {
    _mm256_and_si256(
        _mm256_or_si256(_mm256_cmpgt_epi64(val, gt), _mm256_cmpeq_epi64(val, gt)),
        _mm256_or_si256(_mm256_cmpgt_epi64(lt, val), _mm256_cmpeq_epi64(lt, val)),
    )
}
#[inline]
unsafe fn _mm256_u64_exclusive_range(val: __m256i, gt: __m256i, lt: __m256i) -> __m256i {
    _mm256_and_si256(_mm256_cmpgt_epi64(val, gt), _mm256_cmpgt_epi64(lt, val))
}
impl_find!(
    "avx2",
    Needle,
    u64,
    __m256i,
    8,
    _mm256_set1_epi64x,
    _mm256_load_si256,
    _mm256_movemask_epi8,
    _mm256_cmpeq_epi64,
    _mm256_u64_inclusive_range,
    _mm256_u64_exclusive_range
);
#[inline]
unsafe fn _mm256_f32_eq(val: __m256, eq: __m256) -> __m256 {
    _mm256_cmp_ps(val, eq, _CMP_EQ_OQ)
}
#[inline]
unsafe fn _mm256_f32_inclusive_range(val: __m256, gt: __m256, lt: __m256) -> __m256 {
    _mm256_and_ps(
        _mm256_cmp_ps(val, gt, _CMP_GE_OQ),
        _mm256_cmp_ps(val, lt, _CMP_LE_OQ),
    )
}
#[inline]
unsafe fn _mm256_f32_exclusive_range(val: __m256, gt: __m256, lt: __m256) -> __m256 {
    _mm256_and_ps(
        _mm256_cmp_ps(val, gt, _CMP_GT_OQ),
        _mm256_cmp_ps(val, lt, _CMP_LT_OQ),
    )
}
impl_find!(
    "avx2",
    Needle,
    f32,
    __m256,
    1,
    _mm256_set1_ps,
    _mm256_load_ps,
    _mm256_movemask_ps,
    _mm256_f32_eq,
    _mm256_f32_inclusive_range,
    _mm256_f32_exclusive_range
);
#[inline]
unsafe fn _mm256_f64_eq(val: __m256d, eq: __m256d) -> __m256d {
    _mm256_cmp_pd(val, eq, _CMP_EQ_OQ)
}
#[inline]
unsafe fn _mm256_f64_exclusive_range(val: __m256d, gt: __m256d, lt: __m256d) -> __m256d {
    _mm256_and_pd(
        _mm256_cmp_pd(val, gt, _CMP_GT_OQ),
        _mm256_cmp_pd(val, lt, _CMP_LT_OQ),
    )
}
#[inline]
unsafe fn _mm256_f64_inclusive_range(val: __m256d, gt: __m256d, lt: __m256d) -> __m256d {
    _mm256_and_pd(
        _mm256_cmp_pd(val, gt, _CMP_GE_OQ),
        _mm256_cmp_pd(val, lt, _CMP_LE_OQ),
    )
}
impl_find!(
    "avx2",
    Needle,
    f64,
    __m256d,
    1,
    _mm256_set1_pd,
    _mm256_load_pd,
    _mm256_movemask_pd,
    _mm256_f64_eq,
    _mm256_f64_inclusive_range,
    _mm256_f64_exclusive_range
);

impl_tests!(Needle);
