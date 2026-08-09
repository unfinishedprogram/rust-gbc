// AudioWorkletProcessor runs on a dedicated audio thread, avoiding main-thread jank.
class GBCAudioProcessor extends AudioWorkletProcessor {
  constructor() {
    super();

    // 16384 stereo pairs =~ 340ms @ 48kHz
    this.bufferCapacity = 32768;
    this.mask = this.bufferCapacity - 1;
    this.buffer = new Float32Array(this.bufferCapacity);
    this.readHead = 0;
    this.writeHead = 0;
    this.size = 0;

    this.port.onmessage = ({ data: samples }) => {
      for (let i = 0; i < samples.length; i++) {
        if (this.size < this.bufferCapacity) {
          this.buffer[this.writeHead] = samples[i];
          this.writeHead = (this.writeHead + 1) & this.mask;
          this.size++;
        } else {
          // Buffer full: drop oldest pair to stay current rather than accumulating lag
          this.readHead = (this.readHead + 2) & this.mask;
          this.size -= 2;
          this.buffer[this.writeHead] = samples[i];
          this.writeHead = (this.writeHead + 1) & this.mask;
          this.size++;
        }
      }
    };
  }

  process(_inputs, outputs) {
    const left = outputs[0][0];
    const right = outputs[0][1];

    for (let i = 0; i < left.length; i++) {
      if (this.size >= 2) {
        left[i] = this.buffer[this.readHead];
        this.readHead = (this.readHead + 1) & this.mask;
        right[i] = this.buffer[this.readHead];
        this.readHead = (this.readHead + 1) & this.mask;
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
