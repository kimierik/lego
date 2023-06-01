#!/bin/sh

ADRESS=$(hostname -i)
echo "frontend got ip : $ADRESS";
trunk serve --address=$ADRESS #serve on the propper ip address
