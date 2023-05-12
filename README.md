# Corroded Monitor
A hardware monitor built on [Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) in Rust.
Corroded Monitor is currently Windows only as Libre Hardware Monitor is Windows only, however the GUI is cross-platform so 
support for other operating systems is goal.

![Corroded Monitor CPU Utilization](https://chanchan.dev/static/images/corroded-monitor-utilization.png)

### Contributing
Contributions are welcome! Please open an issue or pull request if you have any suggestions or bug reports. 
If you want to improve the hardware support, you should consider contributing to the [Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)
project. Cross-platform support is a goal of this project, so if you have any ideas on how to improve that, please let me know.

### Building
1. Build the LibreHardwareMonitorAPI solution in Visual Studio. VS 2022 is recommended, but other versions should work. Make sure to use the release profile. The build output will be in `LibreHardwareMonitorAPI\x64\Release`.
2. Build Corroded Monitor with cargo
3. Copy `LibreHardwareMonitorLib.dll`, `ManagedLibreHardwareMonitor.dll`, `ManagedLibreHardwareMonitorWrapper.dll`, `Newtonsoft.Json.dll`, and `corroded_monitor.exe` to the same directory.
