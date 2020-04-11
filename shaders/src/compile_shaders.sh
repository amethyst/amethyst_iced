#!/bin/sh

# Compiles triangle
glslc triangle.vert -o triangle.vert.spv
glslc triangle.frag -o triangle.frag.spv

# Compiles image
glslc image.vert -o image.vert.spv
glslc image.frag -o image.frag.spv

mv *.spv ../compiled
