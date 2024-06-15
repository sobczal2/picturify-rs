#include "picturify-processing-opencl.h"

#include <stdio.h>

#include "common/fast_image.h"

void half_image(const CFastImage* fast_image)
{
    printf("width: %d, height: %d\n", fast_image->size.width, fast_image->size.height);
    for(int i = 0; i < fast_image->size.height; i++)
    {
        for(int j = 0; j < fast_image->size.width * 4; j++)
        {
            fast_image->data[i * fast_image->size.width * 4 + j] /= 2;
        }
    }
}
