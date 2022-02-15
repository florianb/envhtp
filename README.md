# envhtp

This command line tool converts environment variables whose keys start with "user_" into htpasswd compatible username/password pairs.

The goal is to easily create Traefik-compatible htpasswd secrets (for BasicAuth middleware, see https://doc.traefik.io/traefik/middlewares/http/basicauth) in a CI/CD environment. The htpasswd content is Base64 encoded for this purpose and then output to stdout.

**Warning:** htpasswd-files generated with this tool are not considered to be super secure. Do **not** use this to protect production sites or sensitive data. The reason is that wether this implementation nor the underlying bcrypt implementation is audited (as far as i know) and the random number generator used to create the hashes may be insecure.

However - i use it to protect review deployments.

```
$ envhtp --help

envhtp 1.0.0
Florian Neumann <florian.neumann@mindkeeper.solutions>
Command line tool to convert env variables beginning with user to a htpasswd file.

USAGE:
    envhtp [OPTIONS]

OPTIONS:
    -h, --help                 Print help information
    -n, --no-base64            Do not encode htpasswd with Base64
    -p, --pattern <PATTERN>    Pattern to match the beginning of env variable names. Uses remaining
                               part of env key as username [default: user_]
    -V, --version              Print version information
```

### Examples

```
$ USER_Florian=Florian ./envhtp
RmxvcmlhbjokMmIkMDUkWFNLZHZPeWNBZlRLR2ExZkdWczZFLkZvRG53Q3ltT2l5eGdGOXRKSmMxWnd0RGV4YlhEek8K

$ USER_Florian=Florian ./envhtp -n
Florian:$2b$05$ok43ZDvLYUL8EVtzfyf5/OVc9iBgsn12xWjzqL43EXD5.kCGknXWq

$ lil_flo=florian lil_you=you ./envhtp -np 'lil_'
flo:$2b$05$Qs22hV79ExJEyxOnYh3SieQzTNUS1WxOOcrzuNEf8kAwYDxiZaz8q
you:$2b$05$2zFo5l7oW099/Wj4Tn5J4OHnWoQ8FWPHylbwGApon7Bv0CwVvu1Pe
```

Pipe into a file for further usage:

```
$ lil_flo=florian lil_you=you ./envhtp -np 'lil_' > .htpasswd

$ cat .htpasswd 
flo:$2b$05$X/diD1PdqfDkqginEZRVN.FdrX5etQY4c57VuelrdIOc9ijWGFsX6
you:$2b$05$5.JlIjRmZ9T7Oa045p0frOOfxmJCEwRKOcJTkyLQYCpna8baxZKRq

$ htpasswd -vb .htpasswd you you
Password for user you correct.
```

---
MIT License – Copyright (c) 2022 – Florian Neumann
