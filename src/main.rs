extern crate rand;

mod frame;
mod modulo;
mod convolution;

use rand::distributions::normal::StandardNormal;
use convolution::laplace;
use frame::Periodic2DFrame;

use std::iter::Sum;

fn print_matrix(frame: &Periodic2DFrame, data: &[f64])
{
    let mut loc = frame.origin();
    while !frame.past_end(&loc) {
        print!("{}", data[loc.index]);
        match frame.advance(&mut loc) {
            0 => { print!(" "); }
            1 => { print!("\n"); }
            _ => panic!("error")
        }
    }
}

fn std(x: &[f64]) -> f64
{
    let s = x.len() as f64;
    let (a, b) = x.iter().fold(
        (0., 0.),
        |(a, b), &x| (a + x*x, b + x));
    ((a - b*b / s) / s).sqrt()
}

fn white_noise(x: &mut [f64], sigma: f64)
{
    for i in 0..x.len() {
        let StandardNormal(a) = rand::random();
        x[i] += sigma * a;
    }
}

/*
    let dt = 0.001;
    let tau: f64 = 0.1;
    let kappa = -0.05;
    let d_u = 0.017;
    let d_v = 0.071 / tau.sqrt();
    let lambda = 1.0;
    let sigma = 1.0;
    let dx2 = (10.0 / frame.width as f64).powi(2);

    let f = |u: f64| lambda * u - u.powi(3) - kappa;
    let r_u = |u: f64, v: f64| f(u) - sigma * v;
    let r_v = |u: f64, v: f64| (u - v) / tau;

*/

fn main() {
    let frame = Periodic2DFrame { width: 128, height: 128 };
    let mut u = vec![1.0; frame.size()];
    let mut v = vec![0.0; frame.size()];
    let mut q = vec![0.0; frame.size()];
    let mut p = vec![0.0; frame.size()];

    // white_noise(&mut u, 0.001);
    // white_noise(&mut v, 0.001);
    for i in 0..frame.size() {
        let loc = frame.index(i);
        let x: f64 = (loc.i as f64 - frame.width as f64 / 2.) * 40./64.;
        let y: f64 = (loc.j as f64 - frame.height as f64 / 2.) * 40./64.;
        if x*x + y*y < 1.0 {
            u[i] = 0.5;
            v[i] = 0.25;
        }
    }
    white_noise(&mut u, 0.001);

    let f = 0.025;
    let k = 0.06;
    let d_u = 2e-5;
    let d_v = 1e-5;

    let dt = 1.0;
    let dx2 = (2.0 / frame.width as f64).powi(2);

    let r_u = |u: f64, v: f64| - u * v * v + f * (1. - u);
    let r_v = |u: f64, v: f64| u * v * v - (f + k) * v;

    for i in 0..25000 {
        laplace(&frame, &u, &mut p);
        laplace(&frame, &v, &mut q);

        for i in 0..frame.size() {
            let u0 = u[i];
            let v0 = v[i];

            u[i] += (d_u * p[i] / dx2 + r_u(u0, v0)) * dt;
            v[i] += (d_v * q[i] / dx2 + r_v(u0, v0)) * dt;

            u[i] = u[i].max(0.0);
            v[i] = v[i].max(0.0);
            u[i] = u[i].min(1.0);
            v[i] = v[i].min(1.0);
        }

        //white_noise(&mut u, 0.0001 * dt);
        //white_noise(&mut v, 0.0001 * dt);

        eprintln!("iteration: {}, <u> = {}",
                  i,
                  u.iter().sum::<f64>() / frame.size() as f64);
        if i % 100 == 0 {
            print_matrix(&frame, &u);
            print!("\n\n");
            print_matrix(&frame, &v);
            print!("\n\n");
        }
    }

    print_matrix(&frame, &u);
    print!("\n\n");
    print_matrix(&frame, &v);
}
