# sqlite

## import csv to SQLite
```sh
$ sqlite3
sqlite> create table d(date,level,status,url,token,client,server,request,host);
sqlite> .mode csv
sqlite> .import tmp.csv d
```
- [import-csv-to-sqlite](http://stackoverflow.com/questions/14947916/import-csv-to-sqlite)
