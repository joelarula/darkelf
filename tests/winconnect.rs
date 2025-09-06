
use windows::{
    core::Result,
    Devices::Bluetooth::BluetoothLEDevice,
    Devices::Enumeration::DeviceInformation,
    Devices::Bluetooth::GenericAttributeProfile::{GattDeviceService, GattCharacteristic}
                    
};

#[test]
fn test_windows_api_connect() -> Result<()> {

    pretty_env_logger::init();

    let result = (|| {
        // Step 1: Find BLE devices
        let selector = BluetoothLEDevice::GetDeviceSelector()?;
        let devices = windows::Devices::Enumeration::DeviceInformation::FindAllAsyncAqsFilter(&selector)?.get()?;

    let mut found_device = false;
    let mut found_service = false;
    let mut found_characteristic = false;

        for i in 0..devices.Size()? {
            let device_info: DeviceInformation = devices.GetAt(i)?;
            let device_name = device_info.Name()?;
            let device_name_str = device_name.to_string_lossy();
            if !device_name_str.starts_with("TD5322A") {
                continue;
            }
            let device_id = device_info.Id()?;
            let device_id_str = device_id.to_string_lossy();
            if let Some(addr) = device_id_str.split('#').nth(1) {
                info!("Device address: {}", addr);
            }

            println!("Found BLE device: {} ({})", device_name, device_id);
            found_device = true;

            // Step 2: Connect to device
            let ble_device = BluetoothLEDevice::FromIdAsync(&device_id)?.get()?;

            // Step 3: Enumerate services
            let services_result = ble_device.GetGattServicesAsync()?.get()?;
            for j in 0..services_result.Services()?.Size()? {
                let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
                let service_uuid = service.Uuid()?;
                println!("  Service UUID: {:?}", service_uuid);
                if format!("{:?}", service_uuid).to_uppercase() == "0000FF00-0000-1000-8000-00805F9B34FB" {
                    found_service = true;
                }

                // Step 4: Enumerate characteristics
                let characteristics_result = service.GetCharacteristicsAsync()?.get()?;
                let characteristics = characteristics_result.Characteristics()?;
                for k in 0..characteristics.Size()? {
                    let characteristic: GattCharacteristic = characteristics.GetAt(k)?;
                    println!("    Characteristic UUID: {:?}", characteristic.Uuid()?);
                    let props = characteristic.CharacteristicProperties()?;
                    println!("    Properties: {:?}", props);
                    println!("      Read: {}", (props & GattCharacteristicProperties::Read) == GattCharacteristicProperties::Read);
                    println!("      Write: {}", (props & GattCharacteristicProperties::Write) == GattCharacteristicProperties::Write);
                    println!("      WriteWithoutResponse: {}", (props & GattCharacteristicProperties::WriteWithoutResponse) == GattCharacteristicProperties::WriteWithoutResponse);
                    println!("      Notify: {}", (props & GattCharacteristicProperties::Notify) == GattCharacteristicProperties::Notify);
                    println!("      Indicate: {}", (props & GattCharacteristicProperties::Indicate) == GattCharacteristicProperties::Indicate);
                    println!("      Broadcast: {}", (props & GattCharacteristicProperties::Broadcast) == GattCharacteristicProperties::Broadcast);
                    println!("      AuthenticatedSignedWrites: {}", (props & GattCharacteristicProperties::AuthenticatedSignedWrites) == GattCharacteristicProperties::AuthenticatedSignedWrites);
                    println!("      ExtendedProperties: {}", (props & GattCharacteristicProperties::ExtendedProperties) == GattCharacteristicProperties::ExtendedProperties);
                    println!("      ReliableWrites: {}", (props & GattCharacteristicProperties::ReliableWrites) == GattCharacteristicProperties::ReliableWrites);
                    println!("      WritableAuxiliaries: {}", (props & GattCharacteristicProperties::WritableAuxiliaries) == GattCharacteristicProperties::WritableAuxiliaries);
                    found_characteristic = true;

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

        assert!(found_device, "No BLE devices found with name starting with TD5322A");
        assert!(found_service, "No TD5322A device contains service 0000FF00-0000-1000-8000-00805F9B34FB");
        assert!(found_characteristic, "No characteristics found on any TD5322A device");
        Ok(())
    })();

    match result {
        Err(e) if format!("{:?}", e).contains("HRESULT(0x00000000)") => Ok(()), // treat as success
        other => other,
    }
}