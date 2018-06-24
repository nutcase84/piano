use string;

pub fn hammer(string: &mut string::String, stiffness: f32, size: usize, velocity: f32, position: usize, time: f32) {
	let length_strike = 0.01;
	let length_retract = 0.01;
	let length_total = length_strike + length_retract;
	let mut hammer_position = -10_f32;
	
	if time < length_total {
		if time < length_retract {
			hammer_position = 1_f32;//time/length_strike;
		} else {
			hammer_position = 1_f32-(time-length_strike)/length_retract;
		}
		
		hammer_position *= velocity/5_f32;
		
		for i in position..position+size {
			if string.y[i] < hammer_position {
				string.y[i] += (hammer_position-string.y[i])*stiffness;
			}
		}
	}
}

pub fn damper(string: &mut string::String, stiffness: f32, size: usize, position: usize, time: f32) -> bool {
	let length_strike = 0.05;
	let mut damper_position = -10_f32;
	
	if time < length_strike {
		damper_position = (time/length_strike)-1_f32;
	} else {
		damper_position = 1_f32;
	}
		
	for i in position..position+size {
		if string.y[i] < damper_position {
			string.y[i] += (damper_position-string.y[i])*stiffness;
		}
	}
	time > 4_f32*length_strike
}
