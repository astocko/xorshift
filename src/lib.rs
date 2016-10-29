extern crate rand;

pub mod splitmix64;
pub mod xoroshiro128;
pub mod xorshift128;
pub mod xorshift1024;


pub trait RngJump {
    fn jump(&mut self, count: usize);
}


// Taken from the lib.rs in the rand crate.
#[cfg(test)]
mod test {
    pub fn iter_eq<I, J>(i: I, j: J) -> bool
        where I: IntoIterator,
              J: IntoIterator<Item = I::Item>,
              I::Item: Eq
    {
        // make sure the iterators have equal length
        let mut i = i.into_iter();
        let mut j = j.into_iter();
        loop {
            match (i.next(), j.next()) {
                (Some(ref ei), Some(ref ej)) if ei == ej => {}
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}
