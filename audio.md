# audio
Audio, the synthesis (in whichever way) of sound.

## Chromatic scale
- each key/fret is a semitone
- 12 semitones is a full octave
- 2 semitones is a tone
- each scale is made up of patterns of tones and semitones
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
In order to change the tune with the web audio api do:
```js
const osc = ctx.createOscillator()

osc.type = 'sawtooth'
osc.frequency.value = 440
osc.detune.value = 3 * 100
```

## scale patterns
No matter what note you start on, if you play a pattern you'll get a scale. A
`|` denotes a tone (full jump, 2 frets). A `.` denotes a semitone (half jump, 1
fret).
```txt
||.|||.   [ ionian (major) ]
|.|||.|   [ dorian ]
.|||.||   [ phrygian ]
|||.||.   [ lydian ]
||.||.|   [ mixolydian ]
|.||.||   [ aeolian (minor) ]
.||.|||   [ locrian ]
```
Scale patterns determine the key signature. There are more patterns, but these
are the major ones.

## chords
### triad
- A triad is a chord with 3 notes.
- `1 3 5` of the key you are playing in

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

## envelopes
Envelopes wrap sounds and smear them over time.

### attack envelope
Attack envelopes ramp up the value.

```js
const attack = audioContext.createGain()
attack.gain.value = 0
attack.gain.setTargetAtTime(1, startTime, 0.1)
```

### release envelope
release envelopes decrease the value.

```js
const release = audioContext.createGain()
release.gain.setTargetAtTime(0, endTime, 0.2)
```

Keep in mind when adding a release envelope, the sound needs to keep playing
until the release finishes otherwise it will just stop.

```js
// provide enough time for the exponential falloff
oscillator.stop(endTime + 2)
```

## modulating audio parameters

## Modules
- [baudio](https://github.com/substack/baudio) - generate audio streams with functions
- [jsynth](https://github.com/NHQ/jsynth) - Generate audio/DSP with javascript functions in the browser

## Alda - programming music
- [alda manifest & introduction](http://daveyarwood.github.io/alda/2015/09/05/alda-a-manifesto-and-gentle-introduction/)

## Scalar modes
- 7 different modes
- a mode is the same scale, but with a different starting note
- they've all got a name
```txt
[ A major scale ]
a  b  c# d  e  f# g# a  (ionian mode)
b  c# d  e  f# g# a  b  (dorian mode)
```
- [wikipedia/music-modes](https://en.wikipedia.org/wiki/Mode_%28music%29)

## See Also
- [The science and mathematics of audio](https://www.youtube.com/watch?v=i_0DXxNeaQ0)
- [making algorithmic music with baudio](https://www.youtube.com/watch?v=2oz_SwhBixs)
- [understanding appregiators](http://www.residentadvisor.net/feature.aspx?2474&utm_content=buffer82714&utm_medium=social)
- [web audio school](http://mmckegg.github.io/web-audio-school/)
- [designing with sound](https://medium.com/@pablostanley/designing-musical-user-interfaces-4f30b41d7a83)
