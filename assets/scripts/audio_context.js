export default class AudioContext {
  constructor() {
    this.ctx = new (window.AudioContext || window.webkitAudioContext)();
    this.audioBuffer = [];
    this.running = false;
    this.deltas = [];

    this.bufferSize = 4096;
    this.internalBufferScale = 2;
    this.channelCount = 2;

    this.scriptNode = this.ctx.createScriptProcessor(this.bufferSize, 0, 2);
    this.scriptNode.onaudioprocess = this.audioCallback.bind(this);
    this.scriptNode.connect(this.ctx.destination);

    this.sourceNode = this.ctx.createConstantSource();
    this.sourceNode.channelCount = this.channelCount;
    this.sourceNode.connect(this.scriptNode);

    this.sourceNode.start();
    this.ctx.resume().then(() => {
      this.running = true;
    });

    this.lastCbTime = performance.now();
  }

  audioCallback(evt) {
    const now = performance.now();
    const delta = now - this.lastCbTime;
    this.lastCbTime = now;
    // console.log("Callback delta:", delta);

    if (this.audioBuffer.length === 0) {
      console.warn("Buffer ran dry");
      return;
    }

    const buffer = evt.outputBuffer;
    const left = buffer.getChannelData(0);
    const right = buffer.getChannelData(1);

    for (let i = 0; i < buffer.length; i++) {
      if (this.audioBuffer.length > 0) {
        left[i] = this.audioBuffer.shift();
        right[i] = this.audioBuffer.shift();
      } else {
        console.warn("Buffer ran dry");
        break;
      }
    }
  }

  samplesToPull(deltaMs) {
    const deltaSeconds = this.sampleDelta(deltaMs) / 1000.0;
    return Math.floor(this.sampleRate() * deltaSeconds);
  }

  pushSamples(samples) {
    this.audioBuffer.push(...samples);
    let targetBufferLength =
      this.bufferSize * this.internalBufferScale * this.channelCount;

    if (this.audioBuffer.length > targetBufferLength) {
      this.audioBuffer = this.audioBuffer.slice(-targetBufferLength);
    }

    console.log(this.audioBuffer.length / this.sampleRate() / 2);
  }

  play() {
    if (this.ctx.state === "suspended") {
      this.ctx.resume();
    }
  }

  stop() {
    this.sourceNode.stop();
    this.running = false;
  }

  isRunning() {
    return this.running;
  }

  remainingSamples() {
    return this.audioBuffer.length;
  }

  sampleRate() {
    return this.ctx.sampleRate;
  }

  sampleDelta(delta) {
    this.deltas.unshift(delta);
    if (this.deltas.length > 64) {
      this.deltas.pop();
    }
    return this.deltas.reduce((a, b) => a + b, 0) / this.deltas.length;
  }
}

// Example usage:
// const audioHandler = new AudioHandler();
// audioHandler.play();
// audioHandler.pullSamples(gbAudio, deltaMs);
