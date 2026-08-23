// ============================================================================
// VELOCT SPATIAL ENGINE - NATIVE SIMD MATH CORE
// File: veloct_math.rs
// Architecture: Direct CPU Hardware Intrinsics (ARM Neon / x86_64 AVX-512)
// Zero External Dependencies | Zero Heap Allocations
// ============================================================================

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vec4Simd {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4Simd {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    // Hardware-accelerated SIMD vector addition
    #[inline(always)]
    pub unsafe fn add_simd(a: Self, b: Self) -> Self {
        #[cfg(target_arch = "x86_64")] {
            let va = _mm_load_ps(&a.x as *const f32);
            let vb = _mm_load_ps(&b.x as *const f32);
            let vr = _mm_add_ps(va, vb);
            let mut out = Vec4Simd::new(0.0, 0.0, 0.0, 0.0);
            _mm_store_ps(&mut out.x as *mut f32, vr);
            out
        }
        #[cfg(target_arch = "aarch64")] {
            let va = vld1q_f32(&a.x as *const f32);
            let vb = vld1q_f32(&b.x as *const f32);
            let vr = vaddq_f32(va, vb);
            let mut out = Vec4Simd::new(0.0, 0.0, 0.0, 0.0);
            vst1q_f32(&mut out.x as *mut f32, vr);
            out
        }
    }

    // Branchless Inverse Square Root (Fast Vector Normalization)
    #[inline(always)]
    pub unsafe fn fast_rsqrt_simd(val: f32) -> f32 {
        #[cfg(target_arch = "x86_64")] {
            let v = _mm_set_ss(val);
            let r = _mm_rsqrt_ss(v);
            _mm_cvtss_f32(r)
        }
        #[cfg(target_arch = "aarch64")] {
            let v = vdupq_n_f32(val);
            let r = vrsqrteq_f32(v);
            vgetq_lane_f32::<0>(r)
        }
    }
}
