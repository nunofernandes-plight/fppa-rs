//! DMA ring-buffer abstractions for managing transfer descriptors.

use crate::error::{FppaError, FppaResult};

use super::descriptor::DmaDescriptor;

/// A bounded, software-managed circular queue of DMA descriptors.
///
/// This structure manages descriptor metadata only. It does not allocate
/// DMA memory, submit transfers, or communicate with hardware.
#[derive(Debug)]
pub struct DmaRing {
    /// Storage for descriptor entries.
    entries: Vec<Option<DmaDescriptor>>,

    /// Index of the next descriptor to be removed.
    head: usize,

    /// Index where the next descriptor will be inserted.
    tail: usize,

    /// Number of descriptors currently stored.
    len: usize,
}

impl DmaRing {
    /// Creates a new DMA ring with the requested capacity.
    ///
    /// Any positive capacity is accepted.
    pub fn new(capacity: usize) -> FppaResult<Self> {
        if capacity == 0 {
            return Err(FppaError::InvalidInput(
                "DMA ring capacity must be greater than zero".to_string(),
            ));
        }

        Ok(Self {
            entries: vec![None; capacity],
            head: 0,
            tail: 0,
            len: 0,
        })
    }

    /// Returns the maximum number of descriptors the ring can hold.
    pub fn capacity(&self) -> usize {
        self.entries.len()
    }

    /// Returns the number of descriptors currently stored.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the ring contains no descriptors.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns true if the ring has reached its capacity.
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Inserts a descriptor at the tail of the ring.
    ///
    /// Returns an error if the ring is full.
    pub fn enqueue(&mut self, descriptor: DmaDescriptor) -> FppaResult<()> {
        if self.is_full() {
            return Err(FppaError::InvalidState(
                "cannot enqueue a descriptor into a full DMA ring".to_string(),
            ));
        }

        self.entries[self.tail] = Some(descriptor);
        self.tail = (self.tail + 1) % self.capacity();
        self.len += 1;

        Ok(())
    }

    /// Removes and returns the descriptor at the head of the ring.
    ///
    /// Returns None if the ring is empty.
    pub fn dequeue(&mut self) -> Option<DmaDescriptor> {
        let descriptor = self.entries[self.head].take()?;

        self.head = (self.head + 1) % self.capacity();
        self.len -= 1;

        Some(descriptor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dma::descriptor::DmaDirection;

    fn make_descriptor(length: usize) -> DmaDescriptor {
        DmaDescriptor::new(length, DmaDirection::ToDevice).unwrap()
    }

    #[test]
    fn rejects_zero_capacity() {
        assert!(DmaRing::new(0).is_err());
    }

    #[test]
    fn creates_ring_with_requested_capacity() {
        let ring = DmaRing::new(1024).unwrap();

        assert_eq!(ring.capacity(), 1024);
        assert_eq!(ring.len(), 0);
        assert!(ring.is_empty());
        assert!(!ring.is_full());
    }

    #[test]
    fn preserves_fifo_order() {
        let mut ring = DmaRing::new(3).unwrap();

        ring.enqueue(make_descriptor(128)).unwrap();
        ring.enqueue(make_descriptor(256)).unwrap();

        assert_eq!(ring.dequeue().unwrap().length(), 128);
        assert_eq!(ring.dequeue().unwrap().length(), 256);
        assert!(ring.is_empty());
    }

    #[test]
    fn rejects_enqueue_when_full() {
        let mut ring = DmaRing::new(1).unwrap();

        ring.enqueue(make_descriptor(128)).unwrap();

        assert!(ring.is_full());
        assert!(ring.enqueue(make_descriptor(256)).is_err());
        assert_eq!(ring.len(), 1);
    }

    #[test]
    fn reuses_slots_after_wraparound() {
        let mut ring = DmaRing::new(2).unwrap();

        ring.enqueue(make_descriptor(128)).unwrap();
        ring.enqueue(make_descriptor(256)).unwrap();

        assert_eq!(ring.dequeue().unwrap().length(), 128);

        ring.enqueue(make_descriptor(512)).unwrap();

        assert_eq!(ring.dequeue().unwrap().length(), 256);
        assert_eq!(ring.dequeue().unwrap().length(), 512);
        assert!(ring.is_empty());
    }

    #[test]
    fn returns_none_when_empty() {
        let mut ring = DmaRing::new(4).unwrap();

        assert!(ring.dequeue().is_none());
    }
}
