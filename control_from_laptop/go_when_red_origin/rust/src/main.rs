use futures::StreamExt;
use std::net::TcpStream;
use std::io::Write;
use hyper::Client;
use image::{Rgb, RgbImage};

use zune_core::colorspace::ColorSpace;
use zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;

use imageproc::filter;
use color::{ToHsv, FloatColor};
use angle::Angle;

#[tokio::main]
async fn main() {
	let mut tcp_stream = TcpStream::connect("192.168.1.11:6000").unwrap();
	let _ = tcp_stream.write(b"$010F040115#r\n");
	let _ = tcp_stream.write(b"$01160630307d#r\n");
	let _ = tcp_stream.write(b"$011504001A#r\n");
	let url = "http://192.168.1.11:6500/video_feed".parse::<http::Uri>().unwrap();
    let client = Client::new();
	let res = client.get(url).await.unwrap();
    if !res.status().is_success() {
        eprintln!("HTTP request failed with status {}", res.status());
        std::process::exit(1);
    }
	let content_type: mime::Mime = res
        .headers()
        .get(http::header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(content_type.type_(), "multipart");
	let boundary = content_type.get_param(mime::BOUNDARY).unwrap();
	let stream = res.into_body();
    let mut stream = multipart_stream::parse(stream, boundary.as_str());
	
	let decoder_options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
	
	let mut running = false;
	
    while let Some(p) = stream.next().await {
        let p = p.unwrap();
		let jpg_bytes = p.body;

		let mut jpeg_decoder = JpegDecoder::new_with_options(decoder_options, &jpg_bytes);
		let pixels = jpeg_decoder.decode().unwrap();
		let image_info = jpeg_decoder.info().unwrap();
		
		
		let img = RgbImage::from_raw(image_info.width as u32, image_info.height as u32, pixels).unwrap();
		print!("IN");

		let filtered = filter::filter3x3::<Rgb<u8>, f64, u8>(&img, &[0.0625, 0.125 , 0.0625,0.125 , 0.25  , 0.125 ,0.0625, 0.125 , 0.0625]);
		let red_area = filtered.pixels().filter(|rgb|{ 
			let hsv = color::Rgb::new(rgb.0[0],rgb.0[1],rgb.0[2]).to_hsv::<f32>().saturate();
			let (h,s,v) = (hsv.h * 0.5, hsv.s * 255.0, hsv.v * 255.0);
			s >= 80.0 && v >= 46.0 && (h.value() <= 5.0 || h.value() >= 175.0)
			
			}
		).count();
		let should_run = red_area as u64 * 4 > image_info.width as u64 * image_info.height as u64;
		if should_run != running{
			if should_run{
				let _ = tcp_stream.write(b"$01100800200039#r\n");
				running = true;
			}else{
				let _ = tcp_stream.write(b"$0115040721#r\n");
				running = false;
			}
		}
		
    }
}