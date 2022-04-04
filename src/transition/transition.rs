#![feature(generic_const_exprs)]

use std::any::Any;
use arrayvec::ArrayVec;
use num::{Num, PrimInt};

pub trait Transition {
    type OutputSelf;
    const S: usize;

    fn lerp(value_a: f64, value_b: f64, transition: f64, ease: Option<fn(f64) -> f64>) -> f64 {
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

    fn get_interpolatable_values_array(&self) -> [f64; Self::S];
    fn apply_interpolatable_values_array(&self, values: [f64; Self::S]) -> Self::OutputSelf;

    fn interpolate(&self, output: &Self, transition: f64, ease: Option<fn(f64) -> f64>) -> <Self as Transition>::OutputSelf where [f64; Self::S]: Sized {
        let origin_values: [f64; Self::S] = self.get_interpolatable_values_array();
        let target_values: [f64; Self::S] = output.get_interpolatable_values_array();

        let modified_interpolated_values: ArrayVec<f64, {Self::S}> = origin_values
            .iter()
            .zip(target_values)
            .map(|(origin, target)| Self::lerp(*origin, target, transition, ease)).collect();

        let unwrapped_interpolated_values: [f64; Self::S] = modified_interpolated_values.into_inner().unwrap_or_else(|_| panic!("failed"));

        self.apply_interpolatable_values_array(unwrapped_interpolated_values)
    }
}
