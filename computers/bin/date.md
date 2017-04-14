# date(1)
Create dates stuff

## Get year-month-day
Probably what you wanna have for prefixes, like always:
```sh
$ date +"%Y-%m-%d"
# => 2016-08-14

$ date -I
# => 2016-08-14
```

## Get day-month-year
```sh
$ date +"%m-%d-%Y"
# => 08-14-2016
```

## Get unix epoch time
Time since unix epoch, unix epoch, epoch time
```sh
$ date +'%s'
# => 1471126359
```

## Get ISO date
ISO date can have varying specificity. It defaults to "date" but also accepts
"hours", "minutes", "seconds" and "ns":
```sh
$ date -I
# => 2016-08-14

$ date --iso-8601=seconds
# => 2016-08-14T00:19:11+02:00
```

## See Also
- http://www.cyberciti.biz/faq/linux-unix-formatting-dates-for-display/
