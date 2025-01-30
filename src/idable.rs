
pub trait Idable {
    const MAX: usize = 0;
    fn idx(&self) -> usize;
}


