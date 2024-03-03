# AuthMonitor

![CI status](https://github.com/FastAlien/AuthMonitor/actions/workflows/ci.yml/badge.svg)

A daemon that powers off computer after nth incorrect login attempt. If you use disk encryption, which is
strongly recommended, in most cases the password used for disk encryption is much stronger than user password, so this
tool can be used as additional protection when notebook is lost or stolen. If you don't use disk encryption it won't
be useful for you.

# Installation

## Create a service user

Find adm group ID:

```shell
ADM_GROUP_ID=$(grep "adm:" /etc/group | cut -f 3 -d ":")
```

Add auth-manager user:

```shell
sudo adduser --system --no-create-home --gid ${ADM_GROUP_ID} -N --shell /sbin/nologin auth-monitor
``` 

## Copy service configuration

```shell
sudo cp -rv ./etc /ect
```

