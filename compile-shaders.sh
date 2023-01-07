#!/bin/bash

shopt -s extglob dotglob nullglob

cd shaders
for d in */ ; do
    (
        cd "$d"
        for f in !(*.spv) ; do
            echo "Compiling $d$f..."
            glslc "$f" -o "$f.spv"
        done
    )
done
