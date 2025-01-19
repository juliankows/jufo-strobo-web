use std::{collections::VecDeque, error::Error, io::Cursor};

use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};
use itertools::Itertools;
use log::{debug, error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() -> String {
	console_error_panic_hook::set_once();
	let err = console_log::init_with_level(log::Level::Trace);
	if let Err(e) = err {
		error!("{e:?}")
	}
	format!("Hello, World!")
}
type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;
#[wasm_bindgen]
pub fn read(
	image_data: Vec<JsValue>,
	width: u32,
	height: u32,
	from_first: bool,
	threshold: u8,
) -> Result<Vec<u8>, JsError> {
	let mut images = image_data
		.into_iter()
		.filter_map(|x| {
			let vecx = js_sys::Uint8Array::from(x).to_vec();
			RgbaImage::from_raw(width, height, vecx)
		})
		.collect::<VecDeque<_>>();
	let mut target = images.pop_front().ok_or(JsError::new("no first image"))?;
	let mut mask = RgbaImage::new(width, height);
	let mut compare_target = target.clone();

	for (i, mut img) in images.iter_mut().enumerate() {
		debug!("image {i}");
		let newcompare = if from_first { None } else { Some(img.clone()) };
		compare(&mut img, &compare_target, threshold);
		if let Some(n) = newcompare {
			compare_target = n
		};
		target
			.pixels_mut()
			.zip(mask.pixels())
			.zip(img.pixels())
			.for_each(|((t, m), i)| {
				if i.0[3] < 128 || m.0[0] > 128 {
					return;
				}
				t.0 = i.0
			});
		mask.pixels_mut().zip(img.pixels()).for_each(|(m, i)| {
			if i.0[3] > 128 {
				// alpha
				m.0 = [255, 255, 255, 255]
			}
		});
	}
	let mut output_png = Vec::new();
	target
		.write_to(&mut Cursor::new(&mut output_png), image::ImageFormat::Png)
		.map_err(jserrmap)?;
	Ok(output_png)
}
fn jserrmap<T: Error>(e: T) -> JsError {
	JsError::new(&format!("{e:?}"))
}
fn compare(a: &mut Image, b: &Image, threshold: u8) {
	a.iter_mut()
		.zip(b.iter())
		.tuples::<(_, _, _, _)>()
		.for_each(|((ar, br), (ag, bg), (ab, bb), (aa, _))| {
			let vr = if *ar > *br { *ar - br } else { br - *ar };
			let vg = if *ag > *bg { *ag - bg } else { bg - *ag };
			let vb = if *ab > *bb { *ab - bb } else { bb - *ab };
			let res = if vr.max(vg).max(vb) > threshold {
				255
			} else {
				0
			};
			//*ar = res;
			//*ag = res;
			//*ab = res; // white image
			*aa = res // alpha
		});
}
