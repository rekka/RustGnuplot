// This file is released into Public Domain.
#![feature(unboxed_closures)]
#![feature(collections)]
#![feature(rustc_private)]
#![feature(env)]

extern crate gnuplot;

use std::num::Float;

use gnuplot::*;
use common::*;

mod common;

fn example(c: Common)
{
	let zw = 61us;
	let zh = 61us;
	let mut z1 = Vec::with_capacity((zw * zh) as usize);
	for i in 0..zh
	{
		for j in 0..zw
		{
			let y = 8.0 * (i as f64) / zh as f64 - 4.0;
			let x = 8.0 * (j as f64) / zw as f64 - 4.0;
			z1.push(x.cos() * y.cos() / ((x*x + y*y).sqrt() + 1.0));
		}
	}

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
	.set_title("Image", &[])
	.set_cb_range(Fix(-1.0), Fix(1.0))
	.set_cb_ticks(Some((Fix(0.25), 1)), &[], &[])
	.set_cb_label("Label", &[Rotate(0.0)])
	.image(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[]);

	c.show(&mut fg, "fg4.1.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes3d()
	.set_title("Surface", &[])
	.surface(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_x_label("X", &[])
	.set_y_label("Y", &[])
	.set_z_label("Z", &[])
	.set_z_range(Fix(-1.0), Fix(1.0))
	.set_z_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
	.set_cb_range(Fix(-1.0), Fix(1.0))
	.set_view(45.0, 45.0);

	c.show(&mut fg, "fg4.2.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes3d()
	.set_title("Cube Helix Palette", &[])
	.surface(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_x_label("X", &[])
	.set_y_label("Y", &[])
	.set_z_label("Z", &[])
	.set_z_range(Fix(-1.0), Fix(1.0))
	.set_z_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
	.set_cb_range(Fix(-1.0), Fix(1.0))
	.set_palette(HELIX)
	.set_view(45.0, 45.0);

	c.show(&mut fg, "fg4.3.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes3d()
	.set_title("Gray Palette", &[])
	.surface(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_x_label("X", &[])
	.set_y_label("Y", &[])
	.set_z_label("Z", &[])
	.set_z_range(Fix(-1.0), Fix(1.0))
	.set_z_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
	.set_palette(GRAY)
	.set_view(45.0, 45.0);

	c.show(&mut fg, "fg4.4.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes3d()
	.set_title("Black Body Palette", &[])
	.surface(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_x_label("X", &[])
	.set_y_label("Y", &[])
	.set_z_label("Z", &[])
	.set_z_range(Fix(-1.0), Fix(1.0))
	.set_z_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
	.set_palette(HOT)
	.set_view(45.0, 45.0);

	c.show(&mut fg, "fg4.5.gnuplot");

	let palette: [(f32, f32, f32, f32); 6] = [
		(0.00, 1.0, 0.0, 0.0),
		(0.33, 1.0, 0.0, 0.0),
		(0.33, 0.0, 1.0, 0.0),
		(0.66, 0.0, 1.0, 0.0),
		(0.66, 0.0, 0.0, 1.0),
		(1.00, 0.0, 0.0, 1.0)];

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes3d()
	.set_title("Custom Palette", &[])
	.surface(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_x_label("X", &[])
	.set_y_label("Y", &[])
	.set_z_label("Z", &[])
	.set_z_range(Fix(-1.0), Fix(1.0))
	.set_z_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
	.set_custom_palette(palette.iter().map(|&x| x))
	.set_view(45.0, 45.0);

	c.show(&mut fg, "fg4.5.gnuplot");
}

fn main()
{
	Common::new().map(|c| example(c));
}
