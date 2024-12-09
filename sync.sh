#!/bin/sh

MIRROR=`cat sync.config`

rsync -a private/inputs private/puzzles $MIRROR
rsync -a $MIRROR/inputs $MIRROR/puzzles private
