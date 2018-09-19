use std::ops::Rem;
use std::ops::Add;

#[inline(always)]
pub fn modulo<T: Copy + Rem<Output=T> + Add<Output=T>>(a: T, b: T) -> T
{
    ((a % b) + b) % b
}

pub trait Modulo<RHS = Self>
{
    type Output;

    fn modulo(self, rhs: RHS) -> Self::Output;
}

impl <T: Copy + Rem<Output=T> + Add<Output=T>> Modulo for T
{
    type Output = Self;

    fn modulo(self, rhs: T) -> T
    {
        modulo(self, rhs)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_modulo()
    {
        assert!(modulo(-3, 4) == 1);
        assert!((-3).modulo(4) == 1);

        assert!(42.modulo(6) == 0);
        assert!((-5).modulo(3) == 1);
    }
}
