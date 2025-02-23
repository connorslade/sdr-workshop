# bfsk-transmitter

```plain
Usage: bfsk-transmitter [OPTIONS] [MESSAGES]...

Arguments:
  [MESSAGES]...  Message to send. Multiple can be supplied [default: "Hello World!"]

Options:
  -s, --sample-rate <SAMPLE_RATE>  Sample rate in Hz [default: 2000000]
  -f, --frequency <FREQUENCY>      Center frequency in Hz [default: 200000000]
  -o, --offset <OFFSET>            Offset from the center freqnency for the two symbols, in Hz [default: 1000]
  -g, --gain <GAIN>                Gain in dB, ranges from 0-47 [default: 47]
  -b, --baud <BAUD>                Baud (bits per second) [default: 10]
  -r, --repeat                     After transmission of all messages ends, loop through again
  -h, --help                       Print help
  -V, --version                    Print version
```
