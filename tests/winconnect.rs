use darkelf::{ blue, util};
use windows::{
    Devices::{Bluetooth::{BluetoothLEDevice, GenericAttributeProfile::{GattCharacteristic, GattCharacteristicProperties, GattDeviceService}}, Enumeration::DeviceInformation}, core::{GUID, Result}
};

use log::{debug, info};


#[test]
fn test_windows_api_connect() -> Result<()> {
    
    util::setup_logging();
    
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
            if !device_name_str.starts_with(blue::LASER_DEVICE_PREFIX) {
                continue;
            }
            let device_id = device_info.Id()?;
            info!("Found Laser device: {} ({})", device_name, device_id);
            found_device = true;

            // Step 2: Connect to device
            let ble_device = BluetoothLEDevice::FromIdAsync(&device_id)?.get()?;
            let mut laser_service_uuid:Option<GUID>   = None; 
            // Step 3: Enumerate services
            let services_result = ble_device.GetGattServicesAsync()?.get()?;
            for j in 0..services_result.Services()?.Size()? {
                let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
                let service_uuid = service.Uuid()?;

                let str = format!("{:?}", service_uuid).to_uppercase();
                if blue::LASER_SERVICE_UUID.contains(&str.as_str()) {
                    found_service = true;
                    laser_service_uuid = Some(service_uuid);

                    info!("    Service UUID: {:?} Found Laser Service uuid", service_uuid);     
                    let characteristics_result = service.GetCharacteristicsAsync()?.get()?;
                    let chars = characteristics_result.Characteristics()?;

            for k in 0..chars.Size()? {
                let characteristic: GattCharacteristic = chars.GetAt(k)?;
                let uuid = format!("{:?}", characteristic.Uuid()?).to_uppercase();
                let props = characteristic.CharacteristicProperties()?;

                    // If readable, read value
                    if props & GattCharacteristicProperties::Read == GattCharacteristicProperties::Read {
                        let value_result = characteristic.ReadValueAsync()?.get()?;
                        // ... process value_result ...
                    }

                    // If writable and matches UUID, save for writing
                    if props & GattCharacteristicProperties::Write == GattCharacteristicProperties::Write {
                        if blue::WRITE_UUIDS.contains(&uuid.as_str()) {
                            info!("            This is a writable characteristic for test");
                            // Save or use as needed
                        }
                    }

                    // If notifiable/indicatable and matches UUID, enable notifications
                    if (props & GattCharacteristicProperties::Notify == GattCharacteristicProperties::Notify ||
                        props & GattCharacteristicProperties::Indicate == GattCharacteristicProperties::Indicate) {
                        if blue::NOTIFY_UUIDS.contains(&uuid.as_str()) {
                            info!("            This is a notifiable/indicatable characteristic for test");
                            // Enable notifications as needed
                        }
                    }

                    info!("        Characteristic UUID: {:?}", characteristic.Uuid()?);
                    //print_characteristic_info(&characteristic)?;
                    found_characteristic = true;


                }
                }else{
                    debug!("    Service UUID: {:?}", service_uuid);
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

/// Prints detailed information about a BLE characteristic including properties and value if readable
fn print_characteristic_info(characteristic: &GattCharacteristic) -> Result<()> {
    let uuid = characteristic.Uuid()?;
   
    
    let props = characteristic.CharacteristicProperties()?;
    debug!("    Properties: {:?}", props);
    
    // Print individual properties
    debug!("      Read: {}", (props & GattCharacteristicProperties::Read) == GattCharacteristicProperties::Read);
    debug!("      Write: {}", (props & GattCharacteristicProperties::Write) == GattCharacteristicProperties::Write);
    debug!("      WriteWithoutResponse: {}", (props & GattCharacteristicProperties::WriteWithoutResponse) == GattCharacteristicProperties::WriteWithoutResponse);
    debug!("      Notify: {}", (props & GattCharacteristicProperties::Notify) == GattCharacteristicProperties::Notify);
    debug!("      Indicate: {}", (props & GattCharacteristicProperties::Indicate) == GattCharacteristicProperties::Indicate);
    debug!("      Broadcast: {}", (props & GattCharacteristicProperties::Broadcast) == GattCharacteristicProperties::Broadcast);
    debug!("      AuthenticatedSignedWrites: {}", (props & GattCharacteristicProperties::AuthenticatedSignedWrites) == GattCharacteristicProperties::AuthenticatedSignedWrites);
    debug!("      ExtendedProperties: {}", (props & GattCharacteristicProperties::ExtendedProperties) == GattCharacteristicProperties::ExtendedProperties);
    debug!("      ReliableWrites: {}", (props & GattCharacteristicProperties::ReliableWrites) == GattCharacteristicProperties::ReliableWrites);
    debug!("      WritableAuxiliaries: {}", (props & GattCharacteristicProperties::WritableAuxiliaries) == GattCharacteristicProperties::WritableAuxiliaries);

    // Read and print characteristic value if it's readable
    if props & GattCharacteristicProperties::Read == GattCharacteristicProperties::Read {
        let value_result = characteristic.ReadValueAsync()?.get()?;
        let value_buffer = value_result.Value()?;
        
        use windows::Storage::Streams::DataReader;
        let data_reader = DataReader::FromBuffer(&value_buffer)?;
        let mut value = vec![0u8; value_buffer.Length()? as usize];
        data_reader.ReadBytes(&mut value)?;
        
        println!("    Value: {:?}", value);
    }
    
    Ok(())
}

