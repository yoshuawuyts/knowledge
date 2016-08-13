# binaries
Stuff about binary applications. Information about static vs dynamic linking,
file formats, tools, optimization and more.

## Formats
### ELF
ELF files are Executable Linkable Format which consists of a symbol look-ups
and relocatable table, that is, it can be loaded at any memory address by the
kernel and automatically, all symbols used, are adjusted to the offset from
that memory address where it was loaded into.
- [wikipedia](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

### DWARF
DWARF is a widely used, standardized debugging data format. DWARF was
originally designed along with Executable and Linkable Format (ELF), although
it is independent of object file formats.

DWARF is a way of encoding debuggin symbols, which is information that
expresses which programming-language constructs generated a specific piece of
machine code in a given executable module.
- [DWARF on wikipedia](https://en.wikipedia.org/wiki/DWARF)
- [Debug symbol on wikipedia](https://en.wikipedia.org/wiki/Debug_symbol)

## Stripping binaries
- [don't strip docker binaries](https://github.com/docker/docker/blob/2a95488f7843a773de2b541a47d9b971a635bfff/project/PACKAGERS.md#stripping-binaries)
- [strip(1)](http://linux.die.net/man/1/strip)

## Tools
- [upx](http://upx.sourceforge.net/) - fast decompression, uncompresses and
  then `JMP`s to code
- [binwalk](https://github.com/devttys0/binwalk) - firmware analysis tool (can
  display entropy)

## See Also
- [shrink go binaries with this one weird trick](https://blog.filippo.io/shrink-your-go-binaries-with-this-one-weird-trick/)
- [elf-binaries-on-linux-understanding-and-analysis](https://linux-audit.com/elf-binaries-on-linux-understanding-and-analysis/)
