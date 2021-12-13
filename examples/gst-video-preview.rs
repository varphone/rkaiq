use gst::prelude::*;
use gst_video::VideoMeta;
use rkaiq::prelude::*;
use rkaiq::types::{OpMode, WorkingMode};
use std::thread;
use std::time::Duration;

fn delay_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    gst::init().unwrap();

    // 初始化 AIQ 上下文
    let sne_ent_name = "m00_b_ov5695 4-0036-1";
    let ctx = Context::new(sne_ent_name, "/etc/iqfiles").unwrap();
    ctx.set_exp_mode(OpMode::Manual).unwrap();
    ctx.set_wb_mode(OpMode::Manual).unwrap();
    ctx.prepare(2592, 1944, WorkingMode::Normal).unwrap();

    // 创建 GStreamer 管线
    let pipeline_cmd =
        "v4l2src io-mode=mmap name=vsrc ! video/x-raw,format=NV12,width=1920,height=1080 ! waylandsink";
    let mut context = gst::ParseContext::new();
    let pipeline =
        gst::parse_launch_full(&pipeline_cmd, Some(&mut context), gst::ParseFlags::empty())
            .unwrap();
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();

    let elm = pipeline.by_name("vsrc").unwrap();
    let pad = elm.static_pad("src").unwrap();
    pad.add_probe(gst::PadProbeType::BUFFER, move |pad, probe_info| {
        if let Some(gst::PadProbeData::Buffer(ref buffer)) = probe_info.data {
            println!("{:?}", buffer);
            if let Some(meta) = buffer.meta::<VideoMeta>() {
                println!("{:?}", meta);
            } else if let Some(ref caps) = pad.current_caps() {
                println!("{:?}", caps);
            } else {
            }
        }
        gst::PadProbeReturn::Ok
    });

    let gst_bus = pipeline.bus().unwrap();

    // 将视频管线放入单独线程中执行
    let gst_thread = {
        let bus = gst_bus.clone();
        thread::spawn(move || {
            pipeline
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");

            for msg in bus.iter_timed(gst::ClockTime::NONE) {
                use gst::MessageView;

                match msg.view() {
                    MessageView::Eos(..) => break,
                    MessageView::Error(err) => {
                        println!("{}", err.error());
                        break;
                    }
                    _ => (),
                }
            }

            pipeline
                .set_state(gst::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
        })
    };

    // 启动 AIQ 控制系统
    ctx.start().unwrap();

    // 1 秒钟后设置对比度为 0
    delay_ms(1000);
    ctx.set_saturation(0).unwrap();

    // 5 秒钟后设置对比度为 100
    delay_ms(5000);
    ctx.set_saturation(100).unwrap();

    // 5 秒钟后设置对比度为 255
    delay_ms(5000);
    ctx.set_saturation(255).unwrap();

    // 5 秒钟后停止 GStreamer 管线
    delay_ms(5000);
    gst_bus.post(&gst::message::Eos::new()).unwrap();
    gst_thread.join().unwrap();

    // 停止 AIQ 控制系统
    ctx.stop(false).unwrap();
}
