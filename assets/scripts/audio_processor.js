// AudioWorkletProcessor runs on a dedicated audio thread, avoiding main-thread jank.
class GBCAudioProcessor extends AudioWorkletProcessor {
  constructor() {
    super();

    // Ring buffer: stereo interleaved, sized for ~200ms of audio at 48kHz
    this.bufferCapacity = 4800 * 2;
    this.buffer = new Float32Array(this.bufferCapacity);
    this.readHead = 0;
    this.writeHead = 0;
    this.size = 0;

    this.port.onmessage = (e) => {
      const samples = e.data;
      const capacity = this.bufferCapacity;

      for (let i = 0; i < samples.length; i++) {
        if (this.size < capacity) {
          this.buffer[this.writeHead] = samples[i];
          this.writeHead = (this.writeHead + 1) % capacity;
          this.size++;
        } else {
          // Drop oldest samples to stay in sync
          this.readHead = (this.readHead + 2) % capacity;
          this.size -= 2;
          this.buffer[this.writeHead] = samples[i];
          this.writeHead = (this.writeHead + 1) % capacity;
          this.size++;
        }
      }
    };
  }

  process(_inputs, outputs) {
    const left = outputs[0][0];
    const right = outputs[0][1];
    const capacity = this.bufferCapacity;

    for (let i = 0; i < left.length; i++) {
      if (this.size >= 2) {
        left[i] = this.buffer[this.readHead];
        this.readHead = (this.readHead + 1) % capacity;

        right[i] = this.buffer[this.readHead];
        this.readHead = (this.readHead + 1) % capacity;
        this.size -= 2;
      } else {
        left[i] = 0;
        right[i] = 0;
      }
    }

    return true; // Keep processor alive
  }
}

registerProcessor("gbc-audio-processor", GBCAudioProcessor);
