#!/bin/sh
echo $(basename $1)
scp "$1" "pi@10.0.0.168:~/developer/test/$(basename $1)"
ssh pi@10.0.0.168 "~/developer/test/$(basename $1)"