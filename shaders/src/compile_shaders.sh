#!/bin/sh

glslc triangle.vert -o triangle.vert.spv
glslc triangle.frag -o triangle.frag.spv
mv *.spv ../compiled
