# Corroded Monitor
A hardware monitor built on [Libre Hardware Monitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor) in Rust.

![Corroded Monitor CPU Utilization](https://chanchan.dev/static/images/corroded-monitor-utilization.png)

### Building
1. Build the LibreHardwareMonitorAPI solution in Visual Studio. VS 2022 is recommended, but other versions should work. Make sure to use the release profile. The build output will be in `LibreHardwareMonitorAPI\x64\Release`.
2. Build Corroded Monitor with cargo
3. Copy `LibreHardwareMonitorLib.dll`, `ManagedLibreHardwareMonitor.dll`, `ManagedLibreHardwareMonitorWrapper.dll`, `Newtonsoft.Json.dll`, and `corroded_monitor.exe` to the same directory.
