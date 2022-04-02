use portaudio;
use std::sync::mpsc::*;
use kiss3d::window::Window;
use nalgebra::Point3;

fn main() {
    let pa = portaudio::PortAudio::new().expect("Unable to init PortAudio.");
    let mic_index = pa.default_input_device().expect("Unable to get default device.");
    let mic = pa.device_info(mic_index).expect("unable to get mic info.");

    let input_params = portaudio::StreamParameters::<f32>::new(mic_index, 2, true, mic.default_low_input_latency);

    let input_setting = portaudio::InputStreamSettings::new(input_params, mic.default_sample_rate, 256 * 12);

    let (sender, receiver) = channel();

    let callback = move |portaudio::InputStreamCallbackArgs { buffer, .. }| {
        match sender.send(buffer) {
            Ok(_) => portaudio::Continue,
            Err(_) => portaudio::Complete
        }
    };

    let mut stream = pa.open_non_blocking_stream(input_setting, callback).expect("Unable to create stream");
    stream.start().expect("unable to start stream");

    // while stream.is_active().unwrap() {
    //     while let Ok(buffer) = receiver.try_recv() {
    //         println!("{:?}", buffer.len())
    //     }
    // }

    let mut window = Window::new("Oscilloscope");
    let mut camera = kiss3d::planar_camera::FixedView::new();

    while window.render_with(None, Some(&mut camera), None) {
        if let Ok(buffer) = receiver.try_recv() {
            // let _result = buffer.iter().zip(buffer.iter().skip(2)).inspect(|(a, b)| {
            //     window.draw_point(&Point3::new(**a, **b, 1.0), &Point3::new(1.0, 1.0, 1.0));
            // }).collect::<Vec<_>>();
            let mut a = 0.0 as f32;
            let mut b = 0.0 as f32;
            for (i, val) in buffer.iter().enumerate() {
                if i % 2 == 0 {
                    a = *val;
                } else {
                    b = *val;
                    window.draw_point(&Point3::new(a, b, 1.0), &Point3::new(1.0, 1.0, 1.0));
                }
            }
        };
    }

}
