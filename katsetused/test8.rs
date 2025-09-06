use windows::{
    core::Result,
    Devices::Bluetooth::BluetoothLEDevice,
    Devices::Enumeration::DeviceInformation,
    Devices::Bluetooth::GenericAttributeProfile::{GattDeviceService, GattCharacteristic},
};

fn main() -> Result<()> {
    // Step 1: Find BLE devices
    let selector = BluetoothLEDevice::GetDeviceSelector()?;
    let devices = windows::Devices::Enumeration::DeviceInformation::FindAllAsyncAqsFilter(&selector)?.get()?;

    for i in 0..devices.Size()? {
        let device_info: DeviceInformation = devices.GetAt(i)?;
        let device_id = device_info.Id()?;
        println!("Found BLE device: {} ({})", device_info.Name()?, device_id);

        // Step 2: Connect to device
        let ble_device = BluetoothLEDevice::FromIdAsync(&device_id)?.get()?;

        // Step 3: Enumerate services
        let services_result = ble_device.GetGattServicesAsync()?.get()?;
        for j in 0..services_result.Services()?.Size()? {
            let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
            println!("  Service UUID: {:?}", service.Uuid()?);

            // Step 4: Enumerate characteristics
            let characteristics_result = service.GetCharacteristicsAsync()?.get()?;
            let characteristics = characteristics_result.Characteristics()?;
            for k in 0..characteristics.Size()? {
                let characteristic: GattCharacteristic = characteristics.GetAt(k)?;
                println!("    Characteristic UUID: {:?}", characteristic.Uuid()?);
                println!("    Properties: {:?}", characteristic.CharacteristicProperties()?);

                // Step 5: Read value (if readable)
                use windows::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristicProperties;
                let props = characteristic.CharacteristicProperties()?;
                if props & GattCharacteristicProperties::Read == GattCharacteristicProperties::Read {
                    let value_result = characteristic.ReadValueAsync()?.get()?;
                    let value_buffer = value_result.Value()?;
                    use windows::Storage::Streams::DataReader;
                    let data_reader = DataReader::FromBuffer(&value_buffer)?;
                    let mut value = vec![0u8; value_buffer.Length()? as usize];
                    data_reader.ReadBytes(&mut value)?;
                    println!("    Value: {:?}", value);
                }
            }
        }
    }

    Ok(())
}