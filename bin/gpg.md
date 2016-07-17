# pgp - OpenPGP

## Generate a new key
To generate a new key do:
```sh
$ gpg --gen-key
```
This prompts a selection menu. The following values are recommended:
- type of key: RSA & RSA
- keysize: 4096
- key expiration: 0 (infinite)
- passphrase: something long and memorable

At this point our key should be pretty good; but we might want to make it
stronger.

## Fortifying a key
It's cool if we can make our keys prefer stronger hashes. When we have a key we
should edit it:
```sh
$ gpg --edit-key <email>
gpg> setpref SHA512 SHA384 SHA256 SHA224 AES256 AES192 AES CAST5 ZLIB BZIP2 ZIP Uncompressed
gpg> save
```

## Listing keys
```sh
$ gpg --list-secret-keys --keyid-format LONG
```

## Exporting keys
Print the `gpg` key in ASCII armor format:
```sh
$ gpg --armor --export <key_id>
```

## See Also
- https://help.riseup.net/en/security/message-security/openpgp/best-practices
- https://help.github.com/articles/generating-a-new-gpg-key/
- https://alexcabal.com/creating-the-perfect-gpg-keypair/
