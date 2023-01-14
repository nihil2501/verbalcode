#!/bin/sh

if [ "$1" = 'up' ]; then
	wg-quick up ./tunnel.conf
elif [ "$1" = 'down' ]; then
	wg-quick down ./tunnel.conf
else
	echo "invalid arg"
fi