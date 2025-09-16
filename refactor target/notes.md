Based on the code in app-service-minimal.js, the possible values for this.deviceInfo.deviceType are numeric and are set or parsed from device data as follows:

It is set in setDeviceInfo(deviceType, version, userType) as a string (likely hexadecimal, e.g., "01", "02").
It is retrieved and parsed as an integer in getDeviceInfo(), so it is used as a number in logic.
From the feature logic in getDeviceFeatures(), the code checks for these values:

0
1
2
3 (and possibly higher)
Specifically:

0: Used for some basic devices, with special handling if version >= 2.
1: Enables more features (e.g., ILDA, TTL analog, textModeFix01, etc.).
2: Enables even more features (e.g., xyCnf, arbPlay, etc.).
3 or higher: Unlocks additional features (e.g., textUpDown, picsPlay).
So, the possible values for this.deviceInfo.deviceType are at least 0, 1, 2, 3, and potentially higher integers, depending on the range of supported device types. The value is determined by parsing a two-character substring from device data, so it could be any integer that the device protocol supports, but the code only explicitly handles up to 3+.

Summary:

deviceType = 0: Basic device
deviceType = 1: Intermediate device (enables more features)
deviceType = 2: Advanced device (enables even more features)
deviceType = 3 or higher: Most advanced devices (all features enabled)
The actual value comes from the device's response data, so it depends on the connected hardware.