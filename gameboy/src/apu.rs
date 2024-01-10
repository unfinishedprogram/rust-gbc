// Audio Processing Unit
// https://gbdev.io/pandocs/Audio_details.html#audio-details
pub struct APU {
	nr21: u8,
	nr22: u8,
}

// There are 4 sound channels each with a generator and a DAC
// Each generator produces values from 0 to 15 or 0x0-0XF
// The DAC then translates this into an "analog" value between -1 and 1

// The four analog channel outputs are then fed into the mixer, which selectively adds them (depending on NR51)
// into two analog outputs (Left and Right). Thus, the analog range of those outputs is 4× that of each channel, -4 to 4.
// Then these final outputs are scaled based on NR50 and output to the speakers.
// NOTE: this scaling can never silence a non-silent input.

// TODO: Implement PCM registers CGB only
