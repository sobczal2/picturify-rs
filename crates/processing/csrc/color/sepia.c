//
// Created by sobczal on 6/15/24.
//

#include "sepia.h"

#include <stdio.h>
#include <CL/cl.h>

const char* sepiaKernelSource = 
"__kernel void sepia(__global float* data, int width, int height) { \n"
"    int x = get_global_id(0); \n"
"    int y = get_global_id(1); \n"
"    int idx = (y * width + x) * 4; \n"
"    float r = data[idx]; \n"
"    float g = data[idx + 1]; \n"
"    float b = data[idx + 2]; \n"
"    float tr = 0.393 * r + 0.769 * g + 0.189 * b; \n"
"    float tg = 0.349 * r + 0.686 * g + 0.168 * b; \n"
"    float tb = 0.272 * r + 0.534 * g + 0.131 * b; \n"
"    data[idx] = (tr > 1.0) ? 1.0 : tr; \n"
"    data[idx + 1] = (tg > 1.0) ? 1.0 : tg; \n"
"    data[idx + 2] = (tb > 1.0) ? 1.0 : tb; \n"
"} \n";

int picturify_sepia(const CFastImage* fast_image) {
    cl_int CL_err = CL_SUCCESS;
    cl_uint numPlatforms = 0;

    // Get platform IDs
    CL_err = clGetPlatformIDs(0, NULL, &numPlatforms);
    if (CL_err != CL_SUCCESS || numPlatforms <= 0) return CSTATUS_OPENCL_ERROR;

    cl_platform_id* platforms = (cl_platform_id*) malloc(numPlatforms * sizeof(cl_platform_id));
    CL_err = clGetPlatformIDs(numPlatforms, platforms, NULL);
    if (CL_err != CL_SUCCESS) {
        free(platforms);
        return CSTATUS_OPENCL_ERROR;
    }

    // Get a GPU device
    cl_device_id device;
    CL_err = clGetDeviceIDs(platforms[0], CL_DEVICE_TYPE_GPU, 1, &device, NULL);
    if (CL_err != CL_SUCCESS) {
        free(platforms);
        return CSTATUS_OPENCL_ERROR;
    }
    free(platforms);

    // Create context
    cl_context context = clCreateContext(NULL, 1, &device, NULL, NULL, &CL_err);
    if (CL_err != CL_SUCCESS) return CSTATUS_OPENCL_ERROR;

    // Create command queue
    cl_command_queue queue = clCreateCommandQueue(context, device, 0, &CL_err);
    if (CL_err != CL_SUCCESS) {
        clReleaseContext(context);
        return CSTATUS_OPENCL_ERROR;
    }

    // Create buffer
    size_t dataSize = fast_image->size.width * fast_image->size.height * 4 * sizeof(float);
    cl_mem imageBuffer = clCreateBuffer(context, CL_MEM_READ_WRITE | CL_MEM_COPY_HOST_PTR, dataSize, fast_image->data, &CL_err);
    if (CL_err != CL_SUCCESS) {
        clReleaseCommandQueue(queue);
        clReleaseContext(context);
        return CSTATUS_OPENCL_ERROR;
    }

    // Create program from source
    cl_program program = clCreateProgramWithSource(context, 1, &sepiaKernelSource, NULL, &CL_err);
    if (CL_err != CL_SUCCESS) {
        clReleaseMemObject(imageBuffer);
        clReleaseCommandQueue(queue);
        clReleaseContext(context);
        return CSTATUS_OPENCL_ERROR;
    }

    // Build program
    CL_err = clBuildProgram(program, 1, &device, NULL, NULL, NULL);
    if (CL_err != CL_SUCCESS) {
        char buildLog[16384];
        clGetProgramBuildInfo(program, device, CL_PROGRAM_BUILD_LOG, sizeof(buildLog), buildLog, NULL);
        printf("Error in kernel: %s\n", buildLog);
        clReleaseProgram(program);
        clReleaseMemObject(imageBuffer);
        clReleaseCommandQueue(queue);
        clReleaseContext(context);
        return CSTATUS_OPENCL_ERROR;
    }

    // Create kernel
    cl_kernel kernel = clCreateKernel(program, "sepia", &CL_err);
    if (CL_err != CL_SUCCESS) {
        clReleaseProgram(program);
        clReleaseMemObject(imageBuffer);
        clReleaseCommandQueue(queue);
        clReleaseContext(context);
        return CSTATUS_OPENCL_ERROR;
    }

    // Set kernel arguments
    CL_err = clSetKernelArg(kernel, 0, sizeof(cl_mem), &imageBuffer);
    CL_err |= clSetKernelArg(kernel, 1, sizeof(int), &fast_image->size.width);
    CL_err |= clSetKernelArg(kernel, 2, sizeof(int), &fast_image->size.height);
    if (CL_err != CL_SUCCESS) {
        clReleaseKernel(kernel);
        clReleaseProgram(program);
        clReleaseMemObject(imageBuffer);
        clReleaseCommandQueue(queue);
        clReleaseContext(context);
        return CSTATUS_OPENCL_ERROR;
    }

    // Define the global and local work sizes
    size_t globalWorkSize[2] = {fast_image->size.width, fast_image->size.height};

    // Execute the kernel
    CL_err = clEnqueueNDRangeKernel(queue, kernel, 2, NULL, globalWorkSize, NULL, 0, NULL, NULL);
    if (CL_err != CL_SUCCESS) {
        clReleaseKernel(kernel);
        clReleaseProgram(program);
        clReleaseMemObject(imageBuffer);
        clReleaseCommandQueue(queue);
        clReleaseContext(context);
        return CSTATUS_OPENCL_ERROR;
    }

    // Read back the results
    CL_err = clEnqueueReadBuffer(queue, imageBuffer, CL_TRUE, 0, dataSize, fast_image->data, 0, NULL, NULL);
    if (CL_err != CL_SUCCESS) {
        clReleaseKernel(kernel);
        clReleaseProgram(program);
        clReleaseMemObject(imageBuffer);
        clReleaseCommandQueue(queue);
        clReleaseContext(context);
        return CSTATUS_OPENCL_ERROR;
    }

    // Clean up
    clReleaseKernel(kernel);
    clReleaseProgram(program);
    clReleaseMemObject(imageBuffer);
    clReleaseCommandQueue(queue);
    clReleaseContext(context);

    return CSTATUS_OK;
}