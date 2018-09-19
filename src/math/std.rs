pub trait Std
{
    type Output;
    fn std(&mut self) -> Self::Output;
}

impl <I: Iterator<Item=f64>> Std for I
{
    type Output = f64;

    fn std(&mut self) -> f64
    {
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut n: u64 = 0;

        for x in self {
            a += x;
            b += x*x;
            n += 1;
        }

        let m = n as f64;

        ((b - a * a / m) / m).sqrt()
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_std()
    {
        let x = (0..10).map(|x| x as f64).std();
        assert!(x < 2.88 && x > 2.87);

        let y = vec![5.0; 100];
        let s = y.iter().std();
        assert!(s.abs() < 1e-6);
    }
}
