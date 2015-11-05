# mail
Email is the emperor of asynchronous text-based communication. The formats are
well defined and tiny, and adoptation is _very_ high.

## mailcap
Mailcap specifies which extensions to use to open files with. Out of the box
`mutt` performs horribly with anything but plain-text. Here's an example
`.mailcap` file:
```mailcap
text/plain; cat %s; copiousoutput
text/html; links -dump %s; nametemplate=%s.html; copiousoutput
application/pdf; /usr/bin/evince %s
text/pdf; /usr/bin/evince %s
image/*; feh %s
```

## mutt
- [the homely mutt](http://stevelosh.com/blog/2012/10/the-homely-mutt/#mutt)
- [my mutt gmail setup](https://hynek.me/articles/my-mutt-gmail-setup/)

## Secure mail
Probably the best way of securing email is to host your own provider. After the
lavabit shutdown, it's probably a good option to use
[riseup](https://help.riseup.net/).

