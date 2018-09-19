use modulo::Modulo;

pub struct Periodic2DFrame {
    pub width: u64,
    pub height: u64,
}

pub struct Location2D {
    pub index: usize,
    pub i: u64,
    pub j: u64,
}

impl Periodic2DFrame {
    pub fn origin(&self) -> Location2D
    {
        Location2D { index: 0, i: 0, j: 0 }
    }

    pub fn init(&self, si: i64, sj: i64) -> Location2D
    {
        let i: u64 = si.modulo(self.width as i64) as u64;
        let j: u64 = sj.modulo(self.height as i64) as u64;
        Location2D { index: (j*self.width + i) as usize, i: i, j: j }
    }

    pub fn index(&self, idx: usize) -> Location2D
    {
        let i: u64 = (idx as u64) % self.width;
        let j: u64 = (idx as u64) / self.width;
        Location2D { index: idx, i: i, j: j }
    }

    pub fn advance(&self, loc: &mut Location2D) -> u8
    {
        loc.index += 1;
        loc.i += 1;

        if loc.i % self.width == 0 {
            loc.i = 0;
            loc.j += 1;
            return 1;
        }

        return 0;
    }

    pub fn cycle(&self, loc: &mut Location2D, di: i64, dj: i64)
    {
        loc.i = (loc.i as i64 + di).modulo(self.width as i64) as u64;
        loc.j = (loc.j as i64 + dj).modulo(self.height as i64) as u64;
        loc.index = (loc.j * self.width + loc.i) as usize;
    }

    pub fn cycle_x(&self, loc: &mut Location2D)
    {
        loc.index += 1;
        loc.i += 1;

        if loc.i % self.width == 0 {
            loc.index -= self.width as usize;
            loc.i = 0;
        }
    }

    pub fn cycle_y(&self, loc: &mut Location2D)
    {
        loc.index += self.width as usize;
        loc.j += 1;

        if loc.j % self.height == 0 {
            loc.index -= (self.height * self.width) as usize;
            loc.j = 0;
        }
    }

    pub fn past_end(&self, loc: &Location2D) -> bool
    {
        loc.j >= self.height
    }

    pub fn size(&self) -> usize
    {
        (self.width * self.height) as usize
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_advance()
    {
        let frame = Periodic2DFrame {
            width: 3, height: 2 };
        let mut x = frame.origin();
        assert!(x.i == 0 && x.j == 0);
        frame.advance(&mut x);
        assert!(x.i == 1 && x.j == 0 && x.index == 1);
        frame.advance(&mut x);
        assert!(x.i == 2 && x.j == 0);
        frame.advance(&mut x);
        assert!(x.i == 0 && x.j == 1);
        frame.advance(&mut x);
        frame.advance(&mut x);
        assert!(x.i == 2 && x.j == 1 && x.index == 5);
        frame.advance(&mut x);
        assert!(frame.past_end(&x));
    }

    #[test]
    fn test_cycle()
    {
        let frame = Periodic2DFrame {
            width: 3, height: 2 };
        let mut x = frame.origin();
        frame.cycle_x(&mut x);
        frame.cycle_x(&mut x);
        frame.cycle_x(&mut x);
        assert!(x.i == 0 && x.j == 0 && x.index == 0);
        frame.cycle_y(&mut x);
        frame.cycle_y(&mut x);
        assert!(x.i == 0 && x.j == 0 && x.index == 0);
    }

    #[test]
    fn test_init()
    {
        let frame = Periodic2DFrame { width: 3, height: 2 };
        let mut x = frame.init(-1, -1);
        assert!(x.i == frame.width - 1);
        assert!(x.j == frame.height - 1);
        assert!(x.index == 5);
        frame.advance(&mut x);
        assert!(frame.past_end(&x));
    }
}
