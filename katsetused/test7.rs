use windows::{
    core::{Error, Result},
    Devices::Bluetooth::BluetoothLEDevice,
    Devices::Bluetooth::GenericAttributeProfile::GattDeviceService,
    Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic,
    Devices::Enumeration::DeviceInformation,
};

fn main() -> Result<()> {
    println!("Scanning for BLE devices...");

    // Find all Bluetooth LE devices
    let selector = BluetoothLEDevice::GetDeviceSelector()?;
    let devices = DeviceInformation::FindAllAsync()?.get()?;

    for i in 0..devices.Size()? {
        let device_info: DeviceInformation = devices.GetAt(i)?;
        let name = device_info.Name()?.to_string();
        let id = device_info.Id()?;
        println!("Found device: {} ({})", name, id);

        // Check if the device is a Bluetooth LE device before attempting to connect

            println!("Found BLE device: {} ({})", name, id);
            // Connect to the device
            let ble_device_result = BluetoothLEDevice::FromIdAsync(&id)?.get();

            match ble_device_result {
                Ok(ble_device) => {
                    let services_result = ble_device.GetGattServicesAsync()?.get();

                    match services_result {
                        Ok(services) => {
                            for j in 0..services.Services()?.Size()? {
                                let service: GattDeviceService = services.Services()?.GetAt(j)?;
                                println!("  Service UUID: {:?}", service.Uuid()?);

                                let characteristics_result = service.GetCharacteristicsAsync()?.get();
                                match characteristics_result {
                                    Ok(characteristics_result) => {
                                        let characteristics = characteristics_result.Characteristics()?;
                                        for k in 0..characteristics.Size()? {
                                            let characteristic: GattCharacteristic = characteristics.GetAt(k)?;
                                            println!("    Characteristic UUID: {:?}", characteristic.Uuid()?);
                                            println!("    Properties: {:?}", characteristic.CharacteristicProperties()?);
                                        }
                                    }
                                    Err(e) => println!("    Error getting characteristics: {:?}", e),
                                }
                            }
                        }
                        Err(e) => println!("  Error getting services: {:?}", e),
                    }
                }
                Err(e) => println!("  Error connecting to device: {:?}", e),
            }

    }
    Ok(())
}