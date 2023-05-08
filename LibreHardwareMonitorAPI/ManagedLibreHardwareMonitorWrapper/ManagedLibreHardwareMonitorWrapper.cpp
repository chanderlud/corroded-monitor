// ManagedLibreHardwareMonitorWrapper.cpp

#include "pch.h"
#include "ManagedLibreHardwareMonitorWrapper.h"

using namespace ManagedLibreHardwareMonitorWrapper;

void* CreateHardwareMonitor()
{
    HardwareMonitorWrapper^ instance = gcnew HardwareMonitorWrapper();
    return new gcroot<HardwareMonitorWrapper^>(instance);
}

void UpdateHardwareMonitor(void* handle)
{
    gcroot<HardwareMonitorWrapper^>* wrapperHandle = static_cast<gcroot<HardwareMonitorWrapper^>*>(handle);
    (*wrapperHandle)->Update();
}

void GetReport(void* handle, char* buffer, int bufferSize)
{
    gcroot<HardwareMonitorWrapper^>* wrapperHandle = static_cast<gcroot<HardwareMonitorWrapper^>*>(handle);
    (*wrapperHandle)->GetReport(buffer, bufferSize);
}

void DestroyHardwareMonitor(void* handle)
{
    gcroot<HardwareMonitorWrapper^>* wrapperHandle = static_cast<gcroot<HardwareMonitorWrapper^>*>(handle);
    delete wrapperHandle;
}