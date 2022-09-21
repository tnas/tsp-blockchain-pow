#!/bin/bash

export URL_GZ_ALL_TSP=http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/tsp/ALL_tsp.tar.gz
export GZ_FILE_ALL_TSP=ALL_tsp.tar.gz
export TAR_FILE_ALL_TSP=ALL_tsp.tar
export INSTANCES_FOLDER=ALL_tsp

rm -f $GZ_FILE_ALL_TSP
rm -rf $INSTANCES_FOLDER

wget $URL_GZ_ALL_TSP

mkdir $INSTANCES_FOLDER
mv $GZ_FILE_ALL_TSP $INSTANCES_FOLDER
cd $INSTANCES_FOLDER
tar -xvf $GZ_FILE_ALL_TSP
gunzip *.gz

rm -f *.tour
rm -f *.problems
rm -f $TAR_FILE_ALL_TSP
