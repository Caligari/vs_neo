

pub fn interpolate_float ( a: f32, b: f32, progress: f32 ) -> f32 {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    assert!(!progress.is_nan());
    ((1.0 - progress) * a) + (progress * b)
}

