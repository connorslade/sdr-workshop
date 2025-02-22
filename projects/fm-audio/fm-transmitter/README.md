# fm-transmitter

The workshop only covers receiving / decoding FM signals, because transmitting SDRs are very expensive.
This program is to be used by the instructor to transmit a signal for the students to decode.

## Usage

Pass a list of .wav files to play at arguments.
In this example song will be played first, then song2, them back to song1, and it will repeat indefanataly.

```bash
$ fm-transmitter song.wav song2.wav
```
