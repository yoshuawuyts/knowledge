# sqlite

## Usage
```js
var sqlite = require('sqlite3')

// either :memory: for in memory
// or a path filelocation on disk
var db = new sqlite.Database(':memory:')

db.on('error', function (err) {
  console.log(err)
})

db.on('open', function () {
  db.run('CREATE TABLE IF NOT EXISTS lorem (info TEXT)', function (err) {
    if (err) throw err

    var stmt = db.prepare('INSERT INTO lorem VALUES (?)')
    var i = 10
    while (--i >= 0) stmt.run('Ipsum ' + i)

    stmt.finalize(function (err) {
      if (err) throw err

      db.all('SELECT rowid AS id, info FROM lorem', function (err, rows) {
        if (err) throw err
        rows.forEach(function (row) {
          console.log(row)
        })
        db.close()
      })
    })
  })
})
```

## See Also
- https://github.com/yoshuawuyts/playground-sqlite
- https://github.com/mapbox/node-sqlite3/wiki/API
