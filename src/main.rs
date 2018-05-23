extern crate portaudio;
extern crate portmidi;
mod string;

// Portaudio Settings
const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

fn main() {
	let portaudio = portaudio::PortAudio::new()
	let mut portaudio_settings = portaudio.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER);

	let callback = move |portaudio::OutputStreamCallbackArgs {buffer, frames, ..}| {
		
	};
}
