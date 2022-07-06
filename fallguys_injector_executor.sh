#!/bin/bash
LOCK=/home/mysticdoll/fallguys_injctor.lock

exec 10>"${LOCK}"

flock -w40 -x 10 || {
    echo timeout
    exit 1;
}

bash -c "PowerShell.exe ./fallguys_injector.ps1 ${1}"

flock -u 10

exit 0
