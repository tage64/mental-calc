use boxfnonce::BoxFnOnce;
use num_traits::{identities::Zero, NumOps};
use rand::distributions::uniform::{SampleUniform, Uniform};
use rand::prelude::*;
use rand::rngs::ThreadRng;
use std::cmp::Ord;
use std::str::FromStr;

/// The type yielded by a task producer.
pub struct Task<T> {
    /// The question in text.
    pub text: String,
    /// A closure accepting an answer. It returns true if the answer was right.
    pub give_answer: BoxFnOnce<'static, (T,), bool>,
}

/// A number valid in a task producer.
pub trait Number: Ord + NumOps + Zero + FromStr + ToString + SampleUniform + Copy {}

impl<T> Number for T where T: Ord + NumOps + Zero + FromStr + ToString + SampleUniform + Copy {}

/// A task producer is something which can yield an infinite number of tasks.
pub trait TaskProducer {
    type Number: Number;
    /// Yield a new task.
    fn next(&mut self) -> Task<Self::Number>;
}

pub struct Add<T: Number> {
    result_max: T,
    distribution: Uniform<T>,
    rng: ThreadRng,
}

impl<T: Number> Add<T> {
    pub fn new(result_min: T, result_max: T) -> Self {
        Self {
            distribution: Uniform::from(result_min..=result_max),
            result_max,
            rng: rand::thread_rng(),
        }
    }
}

impl<T: Number + 'static> TaskProducer for Add<T> {
    type Number = T;

    fn next(&mut self) -> Task<Self::Number> {
        loop {
            let left = self.distribution.sample(&mut self.rng);
            let right = self.distribution.sample(&mut self.rng);
            let result = left + right;
            if result > self.result_max {
                continue;
            }
            break Task {
                text: format!("{} + {}", left.to_string(), right.to_string()),
                give_answer: BoxFnOnce::from(move |answer| answer == result),
            };
        }
    }
}

pub struct Sub<T: Number> {
    result_min: T,
    left_distribution: Uniform<T>,
    right_distribution: Uniform<T>,
    rng: ThreadRng,
}

impl<T: Number> Sub<T> {
    pub fn new(result_min: T, result_max: T) -> Self {
        Self {
            left_distribution: Uniform::from(result_min..=result_max),
            right_distribution: Uniform::from(T::zero()..=result_max - result_min),
            result_min,
            rng: rand::thread_rng(),
        }
    }
}

impl<T: Number + 'static> TaskProducer for Sub<T> {
    type Number = T;

    fn next(&mut self) -> Task<Self::Number> {
        loop {
            let left = self.left_distribution.sample(&mut self.rng);
            let right = self.right_distribution.sample(&mut self.rng);
            let result = left - right;
            if result < self.result_min {
                continue;
            }
            break Task {
                text: format!("{} - {}", left.to_string(), right.to_string()),
                give_answer: BoxFnOnce::from(move |answer| answer == result),
            };
        }
    }
}

pub struct Mul<T: Number> {
    distribution: Uniform<T>,
    rng: ThreadRng,
}

impl<T: Number> Mul<T> {
    pub fn new(factor_min: T, factor_max: T) -> Self {
        Self {
            distribution: Uniform::from(factor_min..=factor_max),
            rng: rand::thread_rng(),
        }
    }
}

impl<T: Number + 'static> TaskProducer for Mul<T> {
    type Number = T;

    fn next(&mut self) -> Task<Self::Number> {
        let left = self.distribution.sample(&mut self.rng);
        let right = self.distribution.sample(&mut self.rng);
        let result = left * right;
        Task {
            text: format!("{} * {}", left.to_string(), right.to_string()),
            give_answer: BoxFnOnce::from(move |answer| answer == result),
        }
    }
}

/// A task producer where you can mix different task producers. A task producer is randomly selected upon each call to next.
pub struct Mixed<T: Number> {
    task_producers: Vec<Box<TaskProducer<Number = T>>>,
    distribution: Uniform<usize>,
    rng: ThreadRng,
}

impl<T: Number> Mixed<T> {
    pub fn new(task_producers: Vec<Box<TaskProducer<Number = T>>>) -> Self {
        let len = task_producers.len();
        Self {
            task_producers,
            distribution: Uniform::from(0..len),
            rng: rand::thread_rng(),
        }
    }
}

impl<T: Number + 'static> TaskProducer for Mixed<T> {
    type Number = T;

    fn next(&mut self) -> Task<Self::Number> {
        self.task_producers[self.distribution.sample(&mut self.rng)].next()
    }
}
