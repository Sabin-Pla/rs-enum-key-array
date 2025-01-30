use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

use crate::*;

#[derive(Debug)]
pub struct IdMap<I: Idable, O: Sized > where [(); I::MAX]: {
    buf: [O; I::MAX],
}


impl<I: Idable, O: Sized > IdMap<I, O> where [(); I::MAX]: {
    pub fn new() -> Self {
        let o = MaybeUninit::<O>::uninit();
        unsafe { 
            // let buf = [o.assume_init(); I::MAX];
            let buf = unsafe { MaybeUninit::<[MaybeUninit<O>;  I::MAX]>::uninit().assume_init() };
            let buf = MaybeUninit::array_assume_init(buf);
            Self {
                buf,
            }
        }
    }
}

impl <I: Idable, O: Sized >  
Index<I> for IdMap<I, O> where [(); I::MAX]: {
    type Output = O;
    fn index(&self, idable: I) -> &Self::Output {
        &self.buf[idable.idx() as usize]
    }
}


impl<I: Idable, O: Sized > 
IndexMut<I> for IdMap<I, O> where [(); I::MAX]: {
    fn index_mut(&mut self, idable: I) -> &mut O {
        &mut self.buf[idable.idx() as usize]
    }
}