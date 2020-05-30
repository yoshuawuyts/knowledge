# Compiler

## Initial Setup
To prevent issues with the `cc` dependency when compiling in sudo mode, we must
first apply a patch:
```diff
diff --git a/src/bootstrap/bootstrap.py b/src/bootstrap/bootstrap.py
index fdd8784453bdb8c509b8bf4824921e327fb2246e..9b36215eafa21bef3de4b50f455317eabeb1f7fb 100644
--- a/src/bootstrap/bootstrap.py
+++ b/src/bootstrap/bootstrap.py
@@ -784,7 +784,7 @@ def bootstrap(help_triggered):

     if 'SUDO_USER' in os.environ and not build.use_vendored_sources:
         if os.environ.get('USER') != os.environ['SUDO_USER']:
-            build.use_vendored_sources = True
+            # build.use_vendored_sources = True
             print('info: looks like you are running this command under `sudo`')
             print('      and so in order to preserve your $HOME this will now')
             print('      use vendored sources by default. Note that if this')
```

We can then run the install script:
```sh
$ sudo -H ./x.py install
```
