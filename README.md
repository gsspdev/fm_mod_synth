A simple fm synthesizer built in Rust

Make sure you have rust and cargo installed and from the command line run:
```
$ cargo run

```
Alternatively you can specify flags like so:

```
$ cargo run {osc1_amp} {osc1_freq} {osc1_waveform ("sin", "squ", "saw", "tri")} {osc2_amp} {osc2_freq} {osc1_waveform ("sin", "squ", "saw", "tri")}
```

So for an oscillator at 440hz and 0.5 amplitude to be modulated by another oscillator an octave below at half the amplitude (220hz and 0.25 amplitude) you wouldenter:

$ cargo run 0.5 440 squ 0.25 220 tri
```
