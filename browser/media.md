# media

## Media Session API
```js
if ('mediaSession' in navigator) {
  navigator.mediaSession.metadata = new MediaMetadata({
    title: 'Never Gonna Give You Up',
    artist: 'Rick Astley',
    album: 'Whenever You Need Somebody',
    artwork: [
      { src: 'https://dummyimage.com/96x96',   sizes: '96x96',   type: 'image/png' },
      { src: 'https://dummyimage.com/128x128', sizes: '128x128', type: 'image/png' },
      { src: 'https://dummyimage.com/192x192', sizes: '192x192', type: 'image/png' },
      { src: 'https://dummyimage.com/256x256', sizes: '256x256', type: 'image/png' },
      { src: 'https://dummyimage.com/384x384', sizes: '384x384', type: 'image/png' },
      { src: 'https://dummyimage.com/512x512', sizes: '512x512', type: 'image/png' },
    ]
  })

  navigator.mediaSession.setActionHandler('play', function() {})
  navigator.mediaSession.setActionHandler('pause', function() {})
  navigator.mediaSession.setActionHandler('seekbackward', function() {})
  navigator.mediaSession.setActionHandler('seekforward', function() {})
  navigator.mediaSession.setActionHandler('previoustrack', function() {})
  navigator.mediaSession.setActionHandler('nexttrack', function() {})
}
```
- https://developers.google.com/web/updates/2017/02/media-session

## Customize Media Controls
```js
var video = document.querySelector('video')
video.controls // true
video.controlsList // "nofullscreen nodownload noremote" - "foobar" not present
video.controlsList.remove('noremote')
video.controlsList // "nofullscreen nodownload" - "noremote" not present
video.getAttribute('controlsList') // "nofullscreen nodownload"
```
- https://developers.google.com/web/updates/2017/03/chrome-58-media-updates

## Enable Auto Play on Android
sites installed using the improved Add to Home Screen flow are allowed to
autoplay audio and video served from origins included in the web app manifest's
scope without restrictions.

```json
{
  "name": "My Web App",
  "description": "An awesome app",
  "scope": "/foo",
  ...
}
```
- https://developers.google.com/web/updates/2017/03/chrome-58-media-updates

## Error: The Play Request Was Interrupted
Happend when `.pause()` is called before `.play()` is done - async races!

```js
button.addEventListener('click', onButtonClick);

  function onButtonClick() {
    // This will allow us to play video later...
    video.load();
    fetchVideoAndPlay();
  }

  function fetchVideoAndPlay() {
    fetch('https://example.com/file.mp4')
    .then(response => response.blob())
    .then(blob => {
      video.srcObject = blob;
      return video.play();
    })
    .then(_ => {
      // Video playback started ;)
    })
    .catch(e => {
      // Video playback failed ;(
    })
  }
```
- https://developers.google.com/web/updates/2017/06/play-request-was-interrupted
