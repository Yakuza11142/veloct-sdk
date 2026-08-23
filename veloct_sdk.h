// Veloct + Tesseract Unified Engine SDK
#ifndef VELOCT_SDK_H
#ifndef VELOCT_SDK_H
#define VELOCT_SDK_H

#include <stdint.h>
#include <stdbool.h>

// Initialize GPU Compute Context & Tesseract Spatial Pipeline
bool veloct_init_engine(void);

// Pass Raw IMU / Depth Sensors directly into Tesseract Engine
void veloct_push_sensor_data(const float* accel, const float* gyro, float dt);

// Execute High-Speed SIMD Physics Compute Kernel on GPU
void veloct_dispatch_compute_physics(uint32_t entity_count);

// Shutdown Engine & Free VRAM Buffer Memory
void veloct_shutdown(void);

#endif // VELOCT_SDK_H
