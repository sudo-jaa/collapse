use std::any::Any;
use std::collections::HashMap;
use arrayvec::ArrayVec;
use num::{Num, PrimInt};

pub type EasingFunction = fn(f64) -> f64;

pub trait Interpolatable {
    fn interpolate(&self, other: &Self, transition: f64, ease: Option<EasingFunction>) -> Self;

    fn lerp(value_a: f64, value_b: f64, transition: f64, ease: Option<EasingFunction>) -> f64 {
        let normalised_transition = transition / 100.0;
        match ease {
            None => {
                value_a + (value_b - value_a) * normalised_transition
            }
            Some(easing_function) => {
                value_a + (value_b - value_a) * easing_function(normalised_transition)
            }
        }
    }
}

impl Interpolatable for f64 {
    fn interpolate(&self, other: &Self, transition: f64, ease: Option<EasingFunction>) -> f64 {
        let output: f64 = Self::lerp(*self, *other, transition, ease);
        output
    }
}
