use std::{collections::VecDeque, error::Error, io::Cursor};

use image::{ImageBuffer, Rgba, RgbaImage};
use itertools::Itertools;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() -> String {
	console_error_panic_hook::set_once();
	console_log::init_with_level(log::Level::Trace).unwrap();
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
	let mut compare_target = target.clone();

	for mut img in images.iter_mut() {
		let newcompare = if from_first { None } else { Some(img.clone()) };
		compare(&mut img, &compare_target, threshold);
		if let Some(n) = newcompare {
			compare_target = n
		};
		target
			.iter_mut()
			.zip(img.iter())
			.for_each(|(t, s)| *t = t.saturating_add(*s));
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
			*ar = res;
			*ag = res;
			*ab = res;
			*aa = 255 // alpha
		});
}
