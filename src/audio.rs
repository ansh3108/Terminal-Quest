use rodio::{source::Source, OutputStream, Sink};
use std::time::Duration;
use std::thread;

pub fn play_hit() {
    thread::spawn(|| {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                let source = rodio::source::SineWave::new(880.0)
                    .take_duration(Duration::from_millis(100))
                    .amplify(0.2);
                sink.append(source);
                sink.sleep_until_end();
            }
        }
    });
}

pub fn play_damage() {
    thread::spawn(|| {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                let source = rodio::source::SineWave::new(150.0)
                    .take_duration(Duration::from_millis(300))
                    .amplify(0.4);
                sink.append(source);
                sink.sleep_until_end();
            }
        }
    });
}

pub fn play_victory() {
    thread::spawn(|| {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                let n1 = rodio::source::SineWave::new(440.0).take_duration(Duration::from_millis(150)).amplify(0.2);
                let n2 = rodio::source::SineWave::new(554.0).take_duration(Duration::from_millis(150)).amplify(0.2);
                let n3 = rodio::source::SineWave::new(659.0).take_duration(Duration::from_millis(300)).amplify(0.2);
                
                sink.append(n1);
                sink.append(n2);
                sink.append(n3);
                sink.sleep_until_end();
            }
        }
    });
}