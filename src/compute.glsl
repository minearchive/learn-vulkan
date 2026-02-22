#version 460

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer DataBuffer {
    uint data[];
} dataBuffer;

void main() {
    uind idx = gl_GlobalInvocationID.x;
    dataBuffer.data[idx] *= 12;
}
