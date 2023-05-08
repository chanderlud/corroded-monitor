using LibreHardwareMonitor.Hardware;
using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json;

namespace ManagedLibreHardwareMonitor
{
    public class HardwareMonitor
    {
        private Computer _computer;

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

            _computer.Open();
            _computer.Accept(new UpdateVisitor());
        }

        public void Update()
        {
            _computer.Accept(new UpdateVisitor());
        }

        public string GetReport()
        {
            Hardware[] parsed_hardware = ParseHardware(_computer.Hardware);
            string jsonString = JsonConvert.SerializeObject(parsed_hardware);
            return jsonString;
        }

        private Hardware[] ParseHardware(IEnumerable<IHardware> hardwareList)
        {
            return hardwareList.Select(h => new Hardware
            {
                HardwareType = h.HardwareType,
                Name = h.Name,
                SubHardware = ParseHardware(h.SubHardware),
                Sensors = ParseSensors(h.Sensors)
            }).ToArray();
        }

        private Sensor[] ParseSensors(IEnumerable<ISensor> sensorList)
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
        public void VisitSensor(ISensor sensor) { }
        public void VisitParameter(IParameter parameter) { }
    }
       
    public class Hardware
    {
        public HardwareType HardwareType { get; set; }

        public string Name { get; set; }

        public Hardware[] SubHardware { get; set; }

        public Sensor[] Sensors { get; set; }
    }

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