using LibreHardwareMonitor.Hardware;
using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json;

namespace ManagedLibreHardwareMonitor
{
    public class HardwareMonitor
    {
        private Computer _computer;

        // constructor initializes the computer object and its properties
        public HardwareMonitor()
        {
            _computer = new Computer
            {
                IsCpuEnabled = true,
                IsGpuEnabled = true,
                IsMemoryEnabled = true,
                IsMotherboardEnabled = false,
                IsControllerEnabled = false,
                IsNetworkEnabled = true,
                IsStorageEnabled = true
            };

            // open connection to the computer and update hardware information
            _computer.Open();
            _computer.Accept(new UpdateVisitor());
        }

        // update the hardware information
        public void Update()
        {
            _computer.Accept(new UpdateVisitor());
        }

        // get a JSON report of the hardware information
        public string GetReport()
        {
            Hardware[] parsed_hardware = ParseHardware(_computer.Hardware);
            string jsonString = JsonConvert.SerializeObject(parsed_hardware);
            return jsonString;
        }

        // parse the hardware data into a custom data structure
        private Hardware[] ParseHardware(IEnumerable<IHardware> hardwareList)
        {
            return hardwareList.Select(h => new Hardware
            {
                HardwareType = h.HardwareType,
                Name = h.Name,
                SubHardware = ParseHardware(h.SubHardware),
                Sensors = parseSensors(h.Sensors)
            }).ToArray();
        }

        // parse the sensor data into a custom data structure
        private Sensor[] parseSensors(IEnumerable<ISensor> sensorList)
        {
            return sensorList.Select(s => new Sensor
            {
                SensorType = s.SensorType,
                Name = s.Name,
                Index = s.Index,
                Value = s.Value.GetValueOrDefault(),
                Min = s.Min.GetValueOrDefault(),
                Max = s.Max.GetValueOrDefault()
            }).ToArray();
        }
    }

    // visitor class to handle updating hardware information
    public class UpdateVisitor : IVisitor
    {
        public void VisitComputer(IComputer computer)
        {
            computer.Traverse(this);
        }
        public void VisitHardware(IHardware hardware)
        {
            hardware.Update();
            foreach (IHardware subHardware in hardware.SubHardware) subHardware.Accept(this);
        }
        public void VisitSensor(ISensor Sensor) { }
        public void VisitParameter(IParameter Parameter) { }
    }

    // custom class to represent hardware
    public class Hardware
    {
        public HardwareType HardwareType { get; set; }

        public string Name { get; set; }

        public Hardware[] SubHardware { get; set; }

        public Sensor[] Sensors { get; set; }
    }

    // custom class to represent sensors
    public class Sensor
    {
        public SensorType SensorType { get; set; }

        public string Name { get; set; }

        public int Index { get; set; }

        public float Value { get; set; }

        public float Min { get; set; }

        public float Max { get; set; }
    }
}