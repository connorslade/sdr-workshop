# transmitter

```plain
Usage: transmitter [OPTIONS] <COMMAND>

Commands:
  audio  Transmit audio with AM or FM
  bfsk   Transmit binary data with frequency shift keying
  help   Print this message or the help of the given subcommand(s)

Options:
  -s, --sample-rate <SAMPLE_RATE>  Sample rate in Hz [default: 2000000]
  -f, --frequency <FREQUENCY>      Center frequency in Hz [default: 200000000]
  -g, --gain <GAIN>                Gain in dB, ranges from 0-47 [default: 47]
  -h, --help                       Print help
  -V, --version                    Print version
```

```plain
Transmit audio with AM or FM

Usage: transmitter audio [OPTIONS] [SONGS]...

Arguments:
  [SONGS]...  Path to .wav files

Options:
  -b, --bandwidth <BANDWIDTH>  Bandwidth, in Hz [default: 19000]
  -h, --help                   Print help
```

```plain
Transmit binary data with frequency shift keying

Usage: transmitter bfsk [OPTIONS] [MESSAGES]...

Arguments:
  [MESSAGES]...  Message to send. Multiple can be supplied [default: "Hello World!"]

Options:
  -o, --offset <OFFSET>        Offset from the center freqnency for the two symbols, in Hz [default: 1000]
  -b, --baud <BAUD>            Baud (bits per second) [default: 10]
  -r, --repeat                 After transmission of all messages ends, loop through again
  -n, --no-transmission-flags  Don't transmit the 0x02 and 0x03 bytes denoting the start and end of a message
  -h, --help                   Print help
```
