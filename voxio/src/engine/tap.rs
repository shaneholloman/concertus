use crossbeam::queue::ArrayQueue;
use std::sync::Arc;

#[derive(Clone)]
pub struct SampleTap {
    buffer: Arc<ArrayQueue<f32>>,
    capacity: usize,
}

impl SampleTap {
    pub(crate) fn new(capacity: usize) -> Self {
        SampleTap {
            buffer: Arc::new(ArrayQueue::new(capacity)),
            capacity,
        }
    }

    pub(crate) fn push(&self, samples: &[f32]) {
        samples.into_iter().for_each(|&s| {
            let _ = self.buffer.force_push(s);
        });
    }

    pub(crate) fn get_latest(&self, amount: usize) -> Vec<f32> {
        let output_len = amount.min(self.capacity);

        let mut output = Vec::with_capacity(output_len);
        for _ in 0..output_len {
            match self.buffer.pop() {
                Some(s) => output.push(s),
                None => break,
            }
        }

        output
    }
}
