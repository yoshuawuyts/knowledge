# crud

CRUD stands for Create, Read, Update, Delete.

## Read
Read is different in that it causes no side effects, making it purely
functional. It usually means that it won't trigger any events, as mutation
observers aren't interested in it.

## Update
Update can also be the creation of a new thing + the destruction of an old
thing, and being aware of that. Update is an illusion I guess?

## Delete
Delete is not necessarily needed; like with git you can keep an index of the
current HEAD + backwards pointers on new entries and never having to delete
anything. Welcome to weird town.
