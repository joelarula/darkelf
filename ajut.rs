                    match central.peripheral(&id).await {
                        Ok(peripheral) => {
                            match peripheral.properties().await {
                                Ok(Some(properties)) => {


                                    let mut device_name = properties.local_name.clone();
                                    let device_address = properties.address;

                                    // If the local_name is not present, some devices put it in the manufacturer data.
                                    // Let's check there. The name "TD5322A" might be part of this data.
                                    if device_name.is_none() {
                                        for (_company_id, data) in properties.manufacturer_data.iter() {
                                            let name_from_manu = String::from_utf8_lossy(data).to_string();
                                            if name_from_manu.starts_with("TD5322A") {
                                                device_name = Some(name_from_manu);
                                                 println!("Discovered TD5322A: ({})",  device_address);
                                                break;
                                            }
                                        }
                                     }

                                    let mut final_name = device_name.clone().unwrap_or_else(|| "Unknown Device".to_string());
                                    println!("Discovered: {} ({})", final_name, device_address);
                                   // println!("  -> All Properties: {:#?}", properties);

                                    let mut final_name: Option<String> = None;

                                    if let Some(local_name) = &properties.local_name {
                                        if local_name.starts_with(DEVICE_NAME_PREFIX) {
                                            final_name = Some(local_name.clone());
                                        }
                                    }

                                    // If no name yet, check manufacturer data
                                    if final_name.is_none() {
                                        for (_company_id, data) in &properties.manufacturer_data {
                                            let name_from_manu = String::from_utf8_lossy(data);
                                            if name_from_manu.starts_with(DEVICE_NAME_PREFIX) {
                                                final_name = Some(name_from_manu.trim().to_string());
                                                break; // Found it, no need to check more
                                            }
                                        }
                                    }

                                    if let Some(name) = final_name {
                                        println!("  -> MATCHED! Adding device: {}", name);
                                        discovered_ids.insert(id.clone(), true);
                                        found_devices.push(FoundDevice { name, address: peripheral.address(), id: peripheral.id() });
                                    }
                                }
                                Ok(None) => eprintln!("  -> No properties found for device."),
                                Err(e) => eprintln!("  -> Error getting properties: {}", e),
                            }
                        },
                        Err(e) => eprintln!("  -> Error getting peripheral: {}", e),
                    }