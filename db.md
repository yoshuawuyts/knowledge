# db
databases

## DynamoDB
[Dynamodb](https://aws.amazon.com/dynamodb/) is a scalable key-value store
(NoSQL) database from Amazon. It doesn't have a schema, and can have multiple
indexes to store data.

### tables
Before data can be written to a database, a table must be created. Because it
uses hashes as keys, a theoretically infinite amount of data can be stored
within the table. A way to differentiate between the different value is by
composing the hashes using prefixes. Generally you'll use a single table per
application.

### key design
Key design is crucial to performant dynamodb instances.

Dynamo defines 2 key types:
- __hash keys__: exact, bound to table
- __range queries__: conditional, bound to hash key. Operators: `gt`, `lt`.

If you want to query data without using map-reduce you'll need to combine hash
keys with range queries.

Say we want to create a bidirectional relationship between two actors, we'd
compose the keys as follows:

```txt
hash key                 |   range key
-------------------------------------------------------------
content_type.entity_id   |   FAN_OF.content_type.entity_id
content_type.entity_id   |   FANNED_BY.content_type.entity_id
```

Dynamo instances are spun up for the amount of requests. Writes to a hash key
are locked to a process. If you're appending a ton of data to a single key
you'll be in big big trouble (e.g. using < 20% of resources, making dynamoDB
5x as expensive as need be).

Writes are 5x as expensive as reads.

- [falling in and out of love with dynamodb](http://0x74696d.com/posts/falling-in-and-out-of-love-with-dynamodb-part-ii/)

### queries
Queries can either be created using nested JS objects or
[key-condition expressions](http://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Query.html#DDB-Query-request-KeyConditionExpression).
Using key expressions is recommended, as conditions will be phased out.

```js
{
  TableName: 'string',                // required, table to read from
  KeyConditionExpression: 'string',   // key value to be retrieved
  ExpressionAttributeValues: {},      // value mappings for the expression
  ConsistentRead: false,              // consistency model
}
```

Example using the
[`dynamo-streams`](https://www.npmjs.com/package/dynamo-streams) module:
```js
db.createQueryStream({
  TableName: table,
  KeyConditionExpression: '#K = :key', // target expression
  ExpressionAttributeNames: {
    '#K': 'key' // specify column, ignoring AWS's reserved words.
                // Column name here is called 'key'
  },
  ExpressionAttributeValues: {
    ':key': { S: 'bar' }  // specify keys, maps onto expressions.
                          // 'S' means 'type: String'
  }
})
```

__KeyConditionExpressions__
- `a = b` true if the attribute a is equal to the value b
- `a < b` true if a is less than b
- `a <= b` true if a is less than or equal to b
- `a > b` true if a is greater than b
- `a >= b` true if a is greater than or equal to b
- `a BETWEEN b AND c` true if a is greater than or equal to b, and less than or
  equal to c
- `begins_with (a, substr)` true if the value of attribute a begins with a
  particular substring

__resources__
- [dynamodb/query-and-scan](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#QueryAndScan.Query)
- [dynamodb/query-property](http://docs.aws.amazon.com/AWSJavaScriptSDK/latest/AWS/DynamoDB.html#query-property)
- [dynamodb/api-query](http://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Query.html)
- [aws-blog/key-condition-expression](https://aws.amazon.com/blogs/aws/dynamodb-update-improved-json-editing-key-condition-expressions/)
- [dynamodb/key-condition-expression](http://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Query.html#DDB-Query-request-KeyConditionExpression)

### modules
- [dynamo-streams](https://github.com/jed/dynamo-streams) - A stream-flavored
  wrapper for the AWS DynamoDB JavaScript API
- [dynamo-client](https://github.com/jed/dynamo-client) - A low-level client
  for accessing DynamoDB from node.js
- [dynamo-down](https://github.com/jed/dynamo-down) - A leveldown API
  implementation on AWS DynamoDB
- [dynalite](https://github.com/mhart/dynalite) - A mock implementation of
  Amazon's DynamoDB built on LevelDB

### links
- [falling in and out of love with dynamodb](http://0x74696d.com/posts/falling-in-and-out-of-love-with-dynamodb-part-ii/)
- [dynamodb for js cheat sheet](http://www.markomedia.com.au/dynamodb-for-javascript-cheatsheet/)
