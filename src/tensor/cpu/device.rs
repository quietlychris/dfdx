use crate::shapes::{Shape, Unit};
use crate::tensor::{cpu::LendingIterator, storage_traits::*, Tensor};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{sync::Arc, vec::Vec};

#[cfg(feature = "no-std")]
use spin::Mutex;

#[cfg(not(feature = "no-std"))]
use std::sync::Mutex;

/// A device that stores data on the heap.
///
/// The [Default] impl seeds the underlying rng with seed of 0.
///
/// Use [Cpu::seed_from_u64] to control what seed is used.
#[derive(Clone, Debug)]
pub struct Cpu {
    pub(crate) rng: Arc<Mutex<StdRng>>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            rng: Arc::new(Mutex::new(StdRng::seed_from_u64(0))),
        }
    }
}

impl Cpu {
    /// Constructs rng with the given seed.
    pub fn seed_from_u64(seed: u64) -> Self {
        Self {
            rng: Arc::new(Mutex::new(StdRng::seed_from_u64(seed))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CpuError {
    /// Device is out of memory
    OutOfMemory,
    /// Not enough elements were provided when creating a tensor
    WrongNumElements,
}

impl std::fmt::Display for CpuError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("CpuError::OutOfMemory"),
            Self::WrongNumElements => f.write_str("CpuError::WrongNumElements"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CpuError {}

impl HasErr for Cpu {
    type Err = CpuError;
}

impl DeviceStorage for Cpu {
    type Vec<E: Unit> = Vec<E>;

    fn try_alloc_len<E: Unit>(&self, len: usize) -> Result<Self::Vec<E>, Self::Err> {
        self.try_alloc_zeros(len)
    }

    fn random_u64(&self) -> u64 {
        #[cfg(not(feature = "no-std"))]
        {
            self.rng.lock().unwrap().gen()
        }
        #[cfg(feature = "no-std")]
        {
            self.rng.lock().gen()
        }
    }

    fn len<E: Unit>(&self, v: &Self::Vec<E>) -> usize {
        v.len()
    }

    fn tensor_to_vec<S: Shape, E: Unit, T>(&self, tensor: &Tensor<S, E, Self, T>) -> Vec<E> {
        let mut buf = Vec::with_capacity(tensor.shape.num_elements());
        let mut iter = tensor.iter();
        while let Some(v) = iter.next() {
            buf.push(*v);
        }
        buf
    }

    fn try_synchronize(&self) -> Result<(), Self::Err> {
        Ok(())
    }
}
