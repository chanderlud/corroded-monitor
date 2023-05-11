// ManagedLibreHardwareMonitorWrapper.h

#pragma once

using namespace System;
using namespace System::Runtime::InteropServices;

#include <locale>
#include <codecvt>
#include <vcclr.h>

// managed wrapper for the libre hardware monitor
namespace ManagedLibreHardwareMonitorWrapper
{
    public ref class HardwareMonitorWrapper
    {
    private:
        // declare an instance of the managed hardware monitor
        ManagedLibreHardwareMonitor::HardwareMonitor^ _hardwareMonitor;

    public:
        // constructor initializes the hardware monitor
        HardwareMonitorWrapper()
        {
            _hardwareMonitor = gcnew ManagedLibreHardwareMonitor::HardwareMonitor();
        }

        // method to update the hardware monitor
        void Update()
        {
            _hardwareMonitor->Update();
        }

        // method to get a report from the hardware monitor and store it in a buffer
        void GetReport(char* buffer, int bufferSize)
        {
            String^ report = _hardwareMonitor->GetReport();
            pin_ptr<const wchar_t> wstr = PtrToStringChars(report);
            size_t result = wcstombs(buffer, wstr, bufferSize);
            if (result == static_cast<size_t>(-1))
                buffer[0] = '\0';
        }
    };
}

// declare C-style functions for interacting with the wrapper
extern "C" __declspec(dllexport) void* CreateHardwareMonitor();
extern "C" __declspec(dllexport) void UpdateHardwareMonitor(void* instance);
extern "C" __declspec(dllexport) void GetReport(void* instance, char* buffer, int bufferSize);
extern "C" __declspec(dllexport) void DestroyHardwareMonitor(void* instance);