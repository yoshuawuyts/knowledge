# 2018-09-23 Printer Setup

- list of printer drivers: https://wiki.archlinux.org/index.php/CUPS/Printer-specific_problems

```sh
yay -S printer driver brother-hll2300d
yay -S cups
```

- https://wiki.archlinux.org/index.php/CUPS
- https://bbs.archlinux.org/viewtopic.php?id=185298
- https://superuser.com/questions/177445/how-to-list-printer-names-acceptable-for-use-with-lpr#177461

```sh
$ sudo systemctl enable org.cups.cupsd.service
$ sudo systemctl start org.cups.cupsd.service
```

## Print to PDF
- https://www.archlinux.org/packages/?name=cups-pdf


## Web Interface

http://localhost:631/

## Commands
```sh
$ sudo lpinfo -v   # List available printers
$ lpinfo -m        # list the model files. There should be a Brother `.ppd` in here
$ sudo lpadmin -p name -E -v 'usb://my-printer' -m something.ppd  # Enable a printer

$ sudo lpadmin -p BROTHER -E -v 'usb://Brother/HL-L2310D%20series?serial=E78096C8N348699' -m HL2030.ppd  # Enable the Brother printer
$ lpoptions -d BROTHER  # Set brother printer as default
```

![](https://d2mxuefqeaa7sj.cloudfront.net/s_356A1BFEA94683F3AA7AE048AEDA78D53EE3BE773C73B4E871EB279A307ADD2B_1537730970098_2018-09-23-212913_3840x2160.png)

# Booklets
- https://aur.archlinux.org/packages/booklet
- https://unix.stackexchange.com/questions/89121/lpr-how-to-specify-the-page-range-3-to-end#89129

Convert a PDF from the command line. Output will be written with the `-booklet.pdf` appendix.
Print using:

```sh
$ lp *booklet.pdf -d BROTHER
```

# Duplex Printing
> Change "Duplex={OFF}" to "Duplex={ON} in both
> /usr/share/brother/Printer/DCP7065DN/inf/{brDCP7065DNfunc,brDCP7065DNrc}.
> Note: I also changed the duplex setting in CUPS, system-config-printer and in
> evince's printer dialog box, so I don't know if all these are necessary, or
> just the "Duplex" changes in the config files.

