use frame::Periodic2DFrame;
use frame::Location2D;

fn co_cycle(af: &Periodic2DFrame, aloc: &mut Location2D,
            cf: &Periodic2DFrame, cloc: &mut Location2D)
{
    match af.advance(aloc)
    {
        0 => { cf.cycle_x(cloc); }
        1 => { cf.cycle(cloc, -(af.width as i64 - 1), 1); }
        _ => panic!("Unexpected resut from location advance.")
    }
}

pub fn convolve(input_frame: &Periodic2DFrame,
            input_pixels: &[f64],
            kernel_frame: &Periodic2DFrame,
            kernel_pixels: &[f64],
            output_pixels: &mut [f64])
{
    let mut input_loc = input_frame.origin();
    let di = (kernel_frame.width / 2) as i64;
    let dj = (kernel_frame.height / 2) as i64;

    while !input_frame.past_end(&input_loc) {
        let mut i = input_frame.init(
            (input_loc.i as i64) - di,
            (input_loc.j as i64) - dj);
        let mut j = kernel_frame.origin();
        let mut tot: f64 = 0.0;

        while !kernel_frame.past_end(&j) {
            tot += input_pixels[i.index] * kernel_pixels[j.index];
            co_cycle(&kernel_frame, &mut j, &input_frame, &mut i);
        }

        output_pixels[input_loc.index] = tot;
        input_frame.advance(&mut input_loc);
    }
}

pub fn laplace(input_frame: &Periodic2DFrame,
               input_pixels: &[f64],
               output_pixels: &mut [f64])
{
    let kernel_frame = Periodic2DFrame { width: 3, height: 3 };
    let k = vec![ 0.,  1., 0.,
                  1., -4., 1.,
                  0.,  1., 0. ];

    convolve(input_frame, input_pixels, &kernel_frame, &k, output_pixels);
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_convolution()
    {
        let frame = Periodic2DFrame { width: 5, height: 5 };
        let mut x = vec![0.0; frame.size()];
        let mut y = vec![0.0; frame.size()];
        x[0] = 1.0;

        let kernel_frame = Periodic2DFrame { width: 3, height: 3 };
        let k = vec![ 0.,  1., 0.,
                      1., -4., 1.,
                      0.,  1., 0. ];

        convolve(&frame, &x, &kernel_frame, &k, &mut y);

        println!("{:?}", y);
        assert!(y[0] == -4.0);
        assert!(y[24] == 0.0);
        assert!(y[20] == 1.0);
    }

    #[test]
    fn test_laplace()
    {
        let frame = Periodic2DFrame { width: 5, height: 5 };
        let mut x = vec![0.0; frame.size()];
        let mut y = vec![0.0; frame.size()];
        x[0] = 1.0;

        laplace(&frame, &x, &mut y);

        println!("{:?}", y);
        assert!(y[0] == -4.0);
        assert!(y[24] == 0.0);
        assert!(y[20] == 1.0);
    }
}
