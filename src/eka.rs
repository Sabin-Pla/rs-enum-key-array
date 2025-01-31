use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

use crate::*;

#[derive(Debug)]
pub struct EKA<I: Idable, O: Sized > where [(); I::MAX]: {
    buf: [O; I::MAX],
}


impl<I: Idable, O: Sized > EKA<I, O> where [(); I::MAX]: {
    pub unsafe fn uninitialized() -> Self {
        let buf =  MaybeUninit::<[MaybeUninit<O>;  I::MAX]>::uninit().assume_init();
        let buf = MaybeUninit::array_assume_init(buf);
        Self {
            buf
        }
    }

    pub unsafe fn zeroed() -> Self {
        let buf =  MaybeUninit::<[MaybeUninit::<O>;  I::MAX]>::zeroed().assume_init();
        let buf = MaybeUninit::array_assume_init(buf);
        Self {
            buf
        }
    }
}

impl<I: Idable, O: Sized + Copy + Default > EKA<I, O> where [(); I::MAX]: {
    pub fn new() -> Self {
        Self {
            buf:  [O::default();  I::MAX]
        }
    }
}


impl <I: Idable, O: Sized >  
Index<I> for EKA<I, O> where [(); I::MAX]: {
    type Output = O;
    fn index(&self, idable: I) -> &Self::Output {
        &self.buf[idable.idx() as usize]
    }
}


impl<I: Idable, O: Sized > 
IndexMut<I> for EKA<I, O> where [(); I::MAX]: {
    fn index_mut(&mut self, idable: I) -> &mut O {
        &mut self.buf[idable.idx() as usize]
    }
}

impl<I: Idable, O: Sized > 
Index<usize> for EKA<I, O> where [(); I::MAX]: {
    type Output = O;
    fn index(&self, idx: usize) -> &O {
        &self.buf[idx]
    }
}

impl<I: Idable, O: Sized > 
IndexMut<usize> for EKA<I, O> where [(); I::MAX]: {
    fn index_mut(&mut self, idx: usize) -> &mut O {
        &mut self.buf[idx]
    }
}