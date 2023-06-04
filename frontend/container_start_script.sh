#!/bin/sh

ADRESS=$(hostname -i)
echo "frontend got ip : $ADRESS";
#serve on the propper ip address
trunk serve --address=$ADRESS --proxy-backend=http://"$BACKENDIP":3000/api
