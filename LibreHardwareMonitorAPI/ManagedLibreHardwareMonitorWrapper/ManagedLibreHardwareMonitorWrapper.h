// ManagedLibreHardwareMonitorWrapper.h

#pragma once

using namespace System;
using namespace System::Runtime::InteropServices;

#include <locale>
#include <codecvt>
#include <vcclr.h>

namespace ManagedLibreHardwareMonitorWrapper
{
    public ref class HardwareMonitorWrapper
    {
    private:
        ManagedLibreHardwareMonitor::HardwareMonitor^ _hardwareMonitor;

    public:
        HardwareMonitorWrapper()
        {
            _hardwareMonitor = gcnew ManagedLibreHardwareMonitor::HardwareMonitor();
        }

        void Update()
        {
            _hardwareMonitor->Update();
        }

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

extern "C" __declspec(dllexport) void* CreateHardwareMonitor();
extern "C" __declspec(dllexport) void UpdateHardwareMonitor(void* instance);
extern "C" __declspec(dllexport) void GetReport(void* instance, char* buffer, int bufferSize);
extern "C" __declspec(dllexport) void DestroyHardwareMonitor(void* instance);