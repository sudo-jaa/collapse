use arrayvec::ArrayVec;
use num::{Num, PrimInt};
use std::any::Any;
use std::collections::HashMap;

pub type EasingFunction = fn(f64) -> f64;

pub trait Interpolatable {
    type AO;
    fn interpolate(
        &self,
        other: &Self,
        transition: f64,
        ease: Option<EasingFunction>,
        asynchronous_options: Option<Self::AO>,
    ) -> Self;

    fn lerp(value_a: f64, value_b: f64, transition: f64, ease: Option<EasingFunction>) -> f64 {
        match ease {
            None => value_a + (value_b - value_a) * transition,
            Some(easing_function) => value_a + (value_b - value_a) * easing_function(transition),
        }
    }

    fn offset_transition(transition: f64, offset: f64) -> f64 {
        if transition >= offset {
            transition * transition * transition
        } else {
            0.0
        }
    }
}

#[derive(Debug)]
pub struct Interpolationf64AsyncOptions(pub f64);

impl Interpolatable for f64 {
    type AO = Interpolationf64AsyncOptions;
    fn interpolate(
        &self,
        other: &Self,
        transition: f64,
        ease: Option<EasingFunction>,
        asynchronous_options: Option<Self::AO>,
    ) -> f64 {
        println!();
        let transition_adjusted = if asynchronous_options.is_some() {
            Self::offset_transition(transition, asynchronous_options.unwrap().0)
        } else {
            transition
        };
        // println!("lerp with {}", transition_adjusted);
        let output: f64 = Self::lerp(*self, *other, transition_adjusted, ease);
        output
    }
}
