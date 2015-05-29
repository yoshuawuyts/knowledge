# audio
Audio, the synthesis (in whichever way) of sound.

## Chromatic scale
```
                         -3  -1   1       4   6       9   11
                       -4  -2   0   2   3   5   7   8   10  12
  .___________________________________________________________________________.
  :  | |  |  | | | |  |  | | | | | |  |  | | | |  |  | | | | | |  |  | | | |  :
  :  | |  |  | | | |  |  | | | | | |  |  | | | |  |  | | | | | |  |  | | | |  :
  :  | |  |  | | | |  |  | | | | | |  |  | | | |  |  | | | | | |  |  | | | |  :
<-:  |_|  |  |_| |_|  |  |_| |_| |_|  |  |_| |_|  |  |_| |_| |_|  |  |_| |_|  :->
  :   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   :
  : A | B | C | D | E | F | G | A | B | C | D | E | F | G | A | B | C | D | E :
  :___|___|___|___|___|___|___|___|___|___|___|___|___|___|___|___|___|___|___:
    ^                           ^           ^               ^           ^
  220 Hz                      440 Hz      523.25 Hz       880 Hz     1174.65 Hz
(-1 Octave)                 (middle A)                 (+1 Octave)
```

## Filters
Filters can modify values. Using `.connect()` they are analogous to `through`
streams for audio.
```js
const f = audioContext.createBiquadFilter()
f.type = 'highpass'
f.frequency.value = 10000
f.connect(ctx.destination)
```

### filter types
```txt
lowpass    only values lower
highpass   only values higher
bandpass   only values in range
lowshelf
highshelf
peaking
notch
allpass
```

## Modules
- [baudio](https://github.com/substack/baudio) - generate audio streams with functions
- [jsynth](https://github.com/NHQ/jsynth) - Generate audio/DSP with javascript functions in the browser

## See Also
- [The science and mathematics of audio](https://www.youtube.com/watch?v=i_0DXxNeaQ0)
- [making algorithmic music with baudio](https://www.youtube.com/watch?v=2oz_SwhBixs)
- [understanding appregiators](http://www.residentadvisor.net/feature.aspx?2474&utm_content=buffer82714&utm_medium=social)
- [web audio school](http://mmckegg.github.io/web-audio-school/)
