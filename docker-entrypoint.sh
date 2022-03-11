#!/bin/sh
# CUBE **requires** all of selfpath, selfexec, and execshell to be defined,
# which doesn't make sense for binary applications like this one.
# `docker-entrypoint.sh` is a script which transparently executes its
# arguments as a command.

exec "$@"
