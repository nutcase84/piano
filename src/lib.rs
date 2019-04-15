#[macro_use]

extern crate vst;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
mod karplus_strong;
extern crate rand;
use self::rand::Rng;
mod event;
use event::{EventManager, Tuning};

struct Piano {
	event_manager: EventManager,
	tuning: Tuning,
	midi_queue: Vec<vst::event::MidiEvent>,
}

impl Default for Piano {
	fn default() -> Piano {
		Piano {
			event_manager: EventManager::new(),
			tuning: Tuning {
				dispersion: 1_f32,
				initial_displacement: Vec::new(),
				displacement_avg: Vec::new(),
				sample_rate: 48000.0,
				a4_frequency: 440.0,
				sub_sampling: 2,
			},
			midi_queue: Vec::new(),
		}
	}
}

impl Plugin for Piano {
	fn get_info(&self) -> Info {
		Info {
			name: "String Test".to_string(),
			unique_id: 0,
			inputs: 0,
			outputs: 1,
			parameters: 2,
			category: Category::Synth,
			..Default::default()
		}
	}
	fn init(&mut self) {
		let mut rng = rand::thread_rng();
		let mut sum = 0_f32;
		for i in 1..25000 { // TODO fixed inital displacement size
			let v = (rng.gen::<f32>()-0.5)*2.0;
			self.tuning.initial_displacement.push(v);
			sum += v;
			self.tuning.displacement_avg.push(sum/i as f32);
		}
	}
	fn process_events(&mut self, events: &Events) {
		for event in events.events() {
			match event {
				Event::Midi(ev) => {
					self.midi_queue.push(ev);
				},
				_ => (),
			}
		}
	}
	fn set_sample_rate(&mut self, rate: f32) {
		self.tuning.sample_rate = rate;
	}
	fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
		//let output_channels = buffer.output_count();
		let num_samples = buffer.samples();
		let (_, output_buffer) = buffer.split();

		for i in 0..num_samples {
			for ev in &mut self.midi_queue {
				if ev.delta_frames as usize == i {
					match ev.data[0] {
						128 => { // note off
							self.event_manager.note_off(ev.data[1]);
						},
						144 => { // note on
							self.event_manager.note_on(ev.data[1], &self.tuning);
						},
						176 => { // control (pedals)
							match ev.data[1] {
								64 => { // sustain/damper pedal
									if ev.data[2] >= 64 { // pedal on
										self.event_manager.sustain = true;
									} else { // pedal off
										self.event_manager.sustain = false;
									}
								},
								_ => (),
							}
						},
						_ => (),
					}
				}
			}
			
			let sample = self.event_manager.strings_update();

			// Write the same sample to each of the channels (make it mono)
			for out in output_buffer {
				out[i] = sample;
			}
		}
		self.midi_queue.clear()
	}
	fn get_parameter_name(&self, index: i32) -> String {
		match index {
			0 => "Sub Sampling".to_string(),
			1 => "Dispersion".to_string(),
			_ => "".to_string()
		}
	}
	fn get_parameter(&self, index: i32) -> f32 {
		match index {
			0 => self.tuning.sub_sampling as f32/10_f32,
			1 => self.tuning.dispersion,
			_ => 0.0
		}
	}
	fn get_parameter_text(&self, index: i32) -> String {
		match index {
			0 => self.tuning.sub_sampling.to_string(),
			_ => "".to_string()
		}
	}
	fn get_parameter_label(&self, index: i32) -> String {
		match index {
			0 => "X".to_string(),
			_ => "".to_string()
		}
	}
	fn set_parameter(&mut self, index: i32, value: f32) {
		match index {
			0 => self.tuning.sub_sampling = (value*10_f32) as usize,
			1 => self.tuning.dispersion = value,
			_ => ()
		}
	}
}

plugin_main!(Piano);
