#!/bin/bash

chmod +x scadm
./scadm propolis load-program libsidecar_lite.so

for i in {1..25}; do
    ./scadm propolis add-route4 0.0.0.0 0 1 1.2.3.4 
done
