#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void* CreateHardwareMonitor();
void UpdateHardwareMonitor(void* handle);
void GetReport(void* handle, char* buffer, int bufferSize);
void DestroyHardwareMonitor(void* handle);

#ifdef __cplusplus
}
#endif
