#include "pch.h"
#include "ManagedLibreHardwareMonitorWrapper.h"

using namespace ManagedLibreHardwareMonitorWrapper;

// function to create a new hardware monitor instance
void* CreateHardwareMonitor()
{
    // create a new instance of HardwareMonitorWrapper in managed memory
    HardwareMonitorWrapper^ instance = gcnew HardwareMonitorWrapper();

    // return a pointer to the instance wrapped in a gcroot
    return new gcroot<HardwareMonitorWrapper^>(instance);
}

// function to update the hardware monitor instance
void UpdateHardwareMonitor(void* handle)
{
    // cast the handle back to the original gcroot
    gcroot<HardwareMonitorWrapper^>* wrapperHandle = static_cast<gcroot<HardwareMonitorWrapper^>*>(handle);

    // call the Update method on the instance
    (*wrapperHandle)->Update();
}

// function to get a report from the hardware monitor instance
void GetReport(void* handle, char* buffer, int bufferSize)
{
    // cast the handle back to the original gcroot
    gcroot<HardwareMonitorWrapper^>* wrapperHandle = static_cast<gcroot<HardwareMonitorWrapper^>*>(handle);

    // call the GetReport method on the instance
    (*wrapperHandle)->GetReport(buffer, bufferSize);
}

// function to destroy the hardware monitor instance and clean up memory
void DestroyHardwareMonitor(void* handle)
{
    // cast the handle back to the original gcroot
    gcroot<HardwareMonitorWrapper^>* wrapperHandle = static_cast<gcroot<HardwareMonitorWrapper^>*>(handle);

    // delete the gcroot, releasing the managed memory
    delete wrapperHandle;
}