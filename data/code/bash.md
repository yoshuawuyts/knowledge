# bash

## Bash file testing
```
-b filename     block special file
-c filename     special character file
-d dirname      check for directory existence
-e filename     check for file existence
-f filename     check for regular file existence not a directory
-G filename     check if file exists and is owned by effective group ID.
-g filename     true if file exists and is set-group-id.
-k filename     sticky bit
-L filename     symbolic link
-O filename     true if file exists and is owned by the effective user id.
-r filename     check if file is a readable
-S filename     check if file is socket
-s filename     check if file is nonzero size
-u filename     check if file set-user-id bit is set
-w filename     check if file is writable
-x filename     check if file is executable
```

__example__
```sh
#!/bin/bash
file=./file
if [ ! -e "$file" ]; then
  echo "File does not exist"
else
  echo "File exists"
fi
```
