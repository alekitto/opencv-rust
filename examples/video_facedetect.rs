extern crate opencv;
use opencv::core;
use opencv::highgui;
use opencv::imgproc;
use opencv::objdetect;
use opencv::videoio;

fn run() -> Result<(), String> {
    let window = "video capture";
    let xml = "/usr/local/share/OpenCV/haarcascades/haarcascade_frontalface_alt.xml";
    highgui::named_window(window, 1)?;
    let mut cam = videoio::VideoCapture::index(1)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        println!("Using different camera");
        cam = videoio::VideoCapture::index(0)?;
    }
    let mut face = objdetect::CascadeClassifier::new(xml)?;
    loop {
        let mut frame = core::Mat::new()?;
        cam.read(&mut frame)?;
        if frame.size()?.width == 0 {
            ::std::thread::sleep(::std::time::Duration::from_secs(50));
            continue;
        }
        let mut gray = core::Mat::new()?;
        imgproc::cvt_color(
            &frame,
            &mut gray,
            imgproc::COLOR_BGR2GRAY,
            0
        )?;
        let mut reduced = core::Mat::new()?;
        imgproc::resize(
            &gray,
            &mut reduced,
            core::Size {
                width: 0,
                height: 0
            },
            0.25f64,
            0.25f64,
            imgproc::INTER_LINEAR
        )?;
        let mut faces = ::opencv::types::VectorOfRect::new();
        face.detect_multi_scale(
            &reduced,
            &mut faces,
            1.1,
            2,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size {
                width: 30,
                height: 30
            },
            core::Size {
                width: 0,
                height: 0
            }
        )?;
        println!("faces: {}", faces.len());
        for face in faces.iter() {
            println!("face {:?}", face);
            let scaled_face = core::Rect {
                x: face.x * 4,
                y: face.y * 4,
                width: face.width * 4,
                height: face.height * 4,
            };
            imgproc::rectangle(
                &frame,
                scaled_face,
                core::Scalar {
                    data: [0f64, -1f64, -1f64, -1f64]
                },
                1,
                8,
                0
            )?;
        }
        highgui::imshow(window, &frame)?;
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
