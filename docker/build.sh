#!/bin/sh

docker pull alpine
DIR=`dirname $0`

(cd $DIR/..;
  docker build --compress --force-rm -f docker/Dockerfile -t s3-signed-put .)