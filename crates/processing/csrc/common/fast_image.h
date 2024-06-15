//
// Created by sobczal on 6/14/24.
//

#ifndef FAST_IMAGE_H
#define FAST_IMAGE_H

typedef struct
{
    int width;
    int height;
} CSize;

typedef struct
{
    float* data;
    CSize size;
} CFastImage;
#endif //FAST_IMAGE_H
