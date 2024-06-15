cmake_minimum_required(VERSION 3.25)
project(picturify-processing-opencl C)

set(CMAKE_C_STANDARD 11)

add_library(picturify-processing-opencl STATIC picturify-processing-opencl.c
        common/fast_image.c
        common/fast_image.h)

install(TARGETS picturify-processing-opencl DESTINATION .)
