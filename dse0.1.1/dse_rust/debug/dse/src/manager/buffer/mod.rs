#[cfg(feature = "env_bit_64bit")]
pub mod env {
    #[cfg(feature = "std_use_LinkedList")]
    use std::collections::LinkedList;
    use std::fmt::Debug;

    use crate::manager::datum::env::Datum;

    #[derive(Debug)]
    pub struct BufferAttachment {
        count_step: usize,
    }

    #[cfg(feature = "log_mode_dev")]
    impl<DT> Debug for Buffers<DT>
    where
        DT: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Buffers")
                .field("id", &self.id)
                .field("parent_id", &self.parent_id)
                .field("buffer", &self.buffer)
                .field("counter", &self.counter)
                .field("attachment", &self.attachment)
                .finish()
        }
    }

    impl Default for BufferAttachment {
        fn default() -> Self {
            Self { count_step: 8 }
        }
    }

    #[derive(Default)]
    pub struct Buffers<DT> {
        pub id: u64,
        pub parent_id: u64,
        pub buffer: LinkedList<(usize, DT)>,
        pub counter: usize,
        pub attachment: BufferAttachment,
    }

    impl<DT> Buffers<DT> {
        pub fn is_empty(&self) -> bool {
            return self.buffer.is_empty();
        }
        pub fn push_buffer(&mut self, buf: DT) {
            self.buffer.push_back((self.counter, buf));
        }

        pub fn consume_all(&mut self) -> Datum<DT> {
            let mut _r = Datum::default().build_with_capacity(self.buffer.len());

            while !self.buffer.is_empty() {
                _r.alloc_data(self.buffer.pop_back().unwrap().1, None)
                    .end()
            }
            return _r;
        }

        pub fn consume_back(&mut self) -> Datum<DT> {
            return Datum::build_empty()
                .build_alloc_data(self.buffer.pop_back().unwrap().1, u64::MAX);
        }

        pub fn consume_front(&mut self) -> Datum<DT> {
            return Datum::build_empty()
                .build_alloc_data(self.buffer.pop_front().unwrap().1, u64::MAX);
        }

        pub fn release_buffer(&mut self) {
            let mut index = 0;
            for bi in self.buffer.iter() {
                if bi.0 < self.counter + self.attachment.count_step {
                    break;
                }
                index = index + 1;
            }
            for _i in 0..index {
                self.buffer.pop_front();
            }
            self.counter = self.counter + 1;
        }

        pub fn clear(&mut self) {
            self.buffer.clear();
        }
    }
}
