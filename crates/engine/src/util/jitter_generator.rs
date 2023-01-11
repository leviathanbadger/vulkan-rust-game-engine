use std::iter::{Zip};
use halton::{Sequence};
use nalgebra_glm as glm;

pub struct JitterGenerator {
    pub current_jitter: glm::Vec2,
    halton_sequence: Zip<Sequence, Sequence>
}

impl std::fmt::Debug for JitterGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JitterInfo")
            .field("current_jitter", &self.current_jitter)
            .finish()
    }
}

impl Default for JitterGenerator {
    fn default() -> Self {
        Self {
            current_jitter: glm::zero(),
            halton_sequence: halton::Sequence::new(2).zip(halton::Sequence::new(3))
        }
    }
}

impl JitterGenerator {
    pub fn next(&mut self) -> glm::Vec2 {
        let (q, w) = self.halton_sequence.next().unwrap();
        self.current_jitter = glm::vec2(q as f32 - 0.5, w as f32 - 0.5);

        self.current_jitter
    }
}
