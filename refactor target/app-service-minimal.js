(this["webpackJsonp"] = this["webpackJsonp"] || []).push([

    ["app-service"], {
			
        "appStateManager": function(e, t, r) {
            "use strict";
            (function(logger) {

                var r = {
                    globalData: {

                        // List of Bluetooth service UUIDs to connect to
                        mserviceuuids: [],
                        // List of Bluetooth characteristic UUIDs for transmitting (TX) data
                        mtxduuids: [],
                        // List of Bluetooth characteristic UUIDs for receiving (RX) data
                        mrxduuids: [],
                        // 0,1,2
                        muuidSel: 0,
                        bleManualDisCnn: !1,
                        BLEConnectionStateChangeSet: !1,
                        BluetoothAdapterOpen: false,
                        ble_device: null,
                        blu_state: 0,
                        blu_connect_stop: !1,
                        blu_connected: 0,
                        //  global "stop BLE operations" flag
                        blu_readyRec: !1,
                        blu_cnn_call_back: null,
                        blu_rec_call_back: null,
                        blu_rec_content: null,
                        blu_data_canSend: !1,
                        blu_data_cmdSending: !1,
                        blu_data_send_interval: 100,
                        deviceInfo: {},
                        cmd: {
                            curMode: 0,
                            settingData: {
                                dmx: 0,
                                ch: 0,
                                xy: 0,
                                light: 1,
                                cfg: 0,
                                lang: 0,
                                valArr: [1, 10, 10, 10, 10]
                            },
                            // prjIndex: The current project index (default 0).
                            // public: Shared settings for all projects (rdMode, runSpeed, txColor, soundVal).
                            // prjItem: An object mapping project indices (as strings) to their specific settings, each with pyMode, prjSelected (an array of 4 numbers), 
                            // and ckValues (an array, initially empty)      
                            prjData: {
                                prjIndex: 0,
                                public: {
                                    rdMode: 0, // audio trigger mode, 0 or 255
                                    runSpeed: 50,
                                    txColor: 9, //colorDisplayOrder
                                    soundVal: 20 // setting sound sensitivity
                                },
                                prjItem: {
                                    2: {
                                        pyMode: 0, // 0,255  loop playback, tick play
                                        prjSelected: [0, 0, 0, 0], /// Pattern selection (bitfield)
                                        ckValues: [] // selected items
                                    },
                                    3: {
                                        pyMode: 0,
                                        prjSelected: [0, 0, 0, 0], /// Pattern selection (bitfield)
                                        ckValues: []
                                    },
                                    5: {
                                        pyMode: 0,
                                        prjSelected: [0, 0, 0, 0], /// Pattern selection (bitfield)
                                        ckValues: []
                                    },
                                    6: {
                                        pyMode: 0,
                                        prjSelected: [0, 0, 0, 0], 
                                        ckValues: []
                                    }
                                }
                            },

                        },

                        // sets the value of blu_data_send_interval in the global data object to the value passed as e.
                        // It is used to update the interval (in milliseconds) at which Bluetooth data is sent.
                        setbluDataSendInterval: function(interval) {
                            this.blu_data_send_interval = interval
                        },

                        // invokes the registered Bluetooth receive callback with the provided data, 
                        //  if a callback is set.
                        setRecCallBack: function(data) {
                            var callbackFunc = this.blu_rec_call_back;
                            null != callbackFunc && callbackFunc(data)
                        },

                        // updates the Bluetooth connection state, saves the device if connected, 
                        // and notifies any registered callback.
                        setBluCnnState: function(connectionState , isManualChange) {
                            this.blu_connected = connectionState , 2 == this.blu_connected && this.saveDevice();
                            var connectionCallback  = this.blu_cnn_call_back;
                            null != connectionCallback  && connectionCallback (connectionState , isManualChange)
                        },

                        setCmdMode: function(mode) {
                            this.cmd["curMode"] = mode, 
                            this.cmd["prjData"].prjIndex = mode
                        },
                        getCmdData: function(commandKey) {
                            return this.cmd[commandKey]
                        },

                        //  function is a setter used to update command-related data within the cmd object. 
                        // It takes two arguments: e, which is the key indicating which section of the command data to update, 
                        // and t, which is the new data to set.
                        // If the key e is "prjData", the function updates the public property of cmd.prjData with t.public. 
                        // If t.prjIndex is not 1, it also updates the corresponding prjItem entry using the index as a string. 
                        // Additionally, it synchronizes the runSpeed and txColor properties in cmd.textData with those from 
                        // t.public. The use of void ensures the function returns undefined after these assignments,
                        //  exiting early for this case.

                        // For all other keys, the function simply assigns t to cmd[e]. If the key is "textData", 
                        // it also updates cmd.prjData.public.runSpeed and cmd.prjData.public.txColor 
                        // to match the new values in t. This ensures that related properties stay consistent across different 
                        // sections of the command data structure.

                        setCmdData: function(key, data) {
                            if ("prjData" == key) return this.cmd[key].public = data.public, 1 != data.prjIndex 
                                && (this.cmd[key].prjItem[data.prjIndex + ""] = data.item), 
                                    this.cmd.textData.runSpeed = data.public.runSpeed, 
                                    void(this.cmd.textData.txColor = data.public.txColor);
                            this.cmd[key] = data, "textData" == key 
                                && (this.cmd.prjData.public.runSpeed = data.runSpeed, this.cmd.prjData.public.txColor = data.txColor)
                        },

                        // restores the last used Bluetooth UUID configuration by reading a saved index 
                        // and updating the relevant UUID arrays for device communication.
                        readSetting: function() {
                            switch (this.muuidSel = this.readData("lastsel") || 0, this.muuidSel) {
                                case 0:
                                    this.mserviceuuids = ["0000FF00-0000-1000-8000-00805F9B34FB"], 
                                    this.mtxduuids = ["0000FF02-0000-1000-8000-00805F9B34FB"], 
                                    this.mrxduuids = "0000FF01-0000-1000-8000-00805F9B34FB";
                                    break;
                                case 1:
                                    this.mserviceuuids = ["0000FFE0-0000-1000-8000-00805F9B34FB"], 
                                    this.mtxduuids = ["0000FFE1-0000-1000-8000-00805F9B34FB"], 
                                    this.mrxduuids = ["0000FFE1-0000-1000-8000-00805F9B34FB"];
                                    break;
                                case 2:
                                    this.mserviceuuids = ["0000FF00-0000-1000-8000-00805F9B34FB"], 
                                    this.mtxduuids = ["0000FF02-0000-1000-8000-00805F9B34FB"], 
                                    this.mrxduuids = "0000FF01-0000-1000-8000-00805F9B34FB";
                                    break
                            }
                        },

                        setDeviceInfo: function(deviceType, version, userType) {
                            this.deviceInfo["deviceType"] = deviceType, 
                            this.deviceInfo["version"] = version, 
                            this.deviceInfo["userType"] = userType
                        },
                        getDeviceInfo: function() {
                            var e = his.deviceInfo["deviceType"] ;
                            "" == e && (e = 0);
                            var t = this.deviceInfo["version"];
                            "" == t && (t = 0);
                            var r = this.deviceInfo["userType"] ;
                            return "" == r && (r = 0), {
                                deviceType: parseInt(e),
                                version: parseInt(t),
                                userType: parseInt(r)
                            }
                        },

                        // getDeviceFeatures() returns an object indicating which features are supported by the current device, 
                        // based on its type and version.
                        getDeviceFeatures: function() {
                            var features = {
                                    textStopTime: false,
                                    textDecimalTime: false,
                                    displayType: 0,
                                    showOutDoorTips: false,
                                    xyCnf: false,
                                    arbPlay: false,
                                    ilda: false,
                                    // device supports TTL analog
                                    ttlAn: false,
                                    picsPlay: false,
                                    textUpDown: false,
                                    animationFix: false
                                },
                                deviceType = this.deviceInfo["deviceType"],
                                version = this.deviceInfo["version"];
                            // Determine device features based on deviceType and version
                            // Each feature is enabled according to specific deviceType/version rules
                            //  "00 - 02" represents a device with type 0 and version 2,
                            if (
                                (deviceType === 1 && version >= 1) ||
                                (deviceType === 0 && version >= 2) ||
                                deviceType >= 2
                            ) {
                                features.textStopTime = true;
                                features.textDecimalTime = true;
                            }

                            if (
                                (deviceType === 1 && version >= 2) ||
                                deviceType > 1
                            ) {
                                features.showOutDoorTips = true;
                            }

                            if (deviceType === 1 && version === 1) {
                                features.textModeFix01 = true;
                            }

                            if (deviceType === 2) {
                                features.xyCnf = true;
                            }

                            if (deviceType === 1 || deviceType === 2) {
                                features.ilda = true;
                                features.ttlAn = true;
                            }

                            if (deviceType >= 2 || version >= 3) {
                                features.arbPlay = true;
                            }

                            if (deviceType >= 3 || version >= 4) {
                                features.textUpDown = true;
                            }

                            if (deviceType >= 3 || version >= 5) {
                                features.picsPlay = true;
                            }

                            if (deviceType === 1) {
                                features.animationFix = true;
                            }

                            features.displayType = deviceType;
                            return features;
                        },


                        savelastsel: function(t) {
                            this.saveData("lastsel", t), logger("log", "Writelastsel ", t, " at App.vue:328")
                        },

                        // restores the last used Bluetooth device from persistent storage into the ble_device property.
                        readDevice: function() {
                            this.ble_device = this.readData("device")
                        },
                        saveDevice: function() {
                            this.saveData("device", this.ble_device)
                        },

                        clearDevice: function() {
                            this.ble_device = null, this.saveDevice()
                        },

                        createBLEConnection: function(deviceId) {
                            var r = this,
                                callbackFunc = arguments.length > 1 && void 0 !== arguments[1] 
                                    ? arguments[1] 
                                    : null;
 
                            this.blu_connected = -1, 
                            uni.createBLEConnection({
                                deviceId: deviceId,
                                timeout: 6e3,
                                success: function(e) {
                                    callbackFunc && callbackFunc(!0)
                                },
                                fail: function(h) {
                                    r.bleManualDisCnn 
                                        ? callbackFunc && callbackFunc(!1) 
                                        : r.doCloseBLEConnection(deviceId, (function(e) {
                                            callbackFunc && callbackFunc(!1)
                                    }))
                                }
                            })
                        },

                        doCloseBLEConnection: function(deviceId) {
                            var callbackFunc = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null;

                            this.bleManualDisCnn = true;
                            var n = this,
                                h = setTimeout((function() {
                                    h = null, n.bleManualDisCnn = !1, callbackFunc && callbackFunc(!0)
                                }), 200);
                            uni.closeBLEConnection({
                                deviceId: deviceId,
                                success: function(t) {
                                    logger("log", "doCloseBLEConnection success", t, " at App.vue:384"), h 
                                        && callbackFunc && callbackFunc(!0)
                                },
                                fail: function(t) {
                                    logger("log", "doCloseBLEConnection fail", t, " at App.vue:389"), h 
                                        && callbackFunc && callbackFunc(!1)
                                },
                                complete: function() {
                                    logger("log", "doCloseBLEConnection complete", " at App.vue:394"), h 
                                        && clearTimeout(h), this.bleManualDisCnn = !1
                                }
                            })
                        },
                        closeBLEConnection: function() {
                            var callbackFunc = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            if (this.blu_connected) {
                                var device = this.ble_device;
                                device ? this.doCloseBLEConnection(device.deviceId, (function(r) {
                                    logger("log", "do callback", " at App.vue:409"), 
                                    callbackFunc && callbackFunc(r)
                                })) : callbackFunc && callbackFunc(!0)
                            } else callbackFunc && callbackFunc(!0)
                        },
                        doCloseBluetoothAdapter: function() {
                            var callbackFunc = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            this.bleOpenCloseCount--, this.BluetoothAdapterOpen = !1, uni.closeBluetoothAdapter({
                                success: function(e) {
                                    callbackFunc && callbackFunc(!0)
                                },
                                fail: function(r) {
                                    logger("log", "closeBluetoothAdapter fail", r, " at App.vue:424"), 
                                    callbackFunc && callbackFunc(!1)
                                }
                            })
                        },
                        closeBluetoothAdapter: function() {
                            var callbackFunc = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            this.BluetoothAdapterOpen ? this.doCloseBluetoothAdapter(callbackFunc) : callbackFunc && callbackFunc(!0)
                        },
                        openBluetoothAdapter: function() {
                            var t = this,
                                callbackFunc = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            if (this.BluetoothAdapterOpen) callbackFunc && callbackFunc(!0);
                            else {
                                this.bleOpenCloseCount++;
                                var n = this;
                                uni.openBluetoothAdapter({
                                    success: function(e) {
                                        t.BluetoothAdapterOpen = !0, t.setBLEConnectionStateChange(), callbackFunc && callbackFunc(!0)
                                    },
                                    fail: function(h) {
                                        t.doCloseBluetoothAdapter(), 10001 === h.errCode 
                                            && t.showModalTips(n.$t("\u8bf7\u68c0\u67e5\u624b\u673aBluetooth\u662f\u5426\u542f\u7528"), !0), 103 == h.errno ? t.showModalTips(n.$t("\u8bf7Settings\u5c0f\u7a0b\u5e8fBluetooth\u6743\u9650"), !0) : t.showModalTips("Open Bluetooth Adapter Fail"), callbackFunc && callbackFunc(!1)
                                    }
                                })
                            }
                        },
                        setBLEConnectionStateChange: function() {
                            if (!this.BLEConnectionStateChangeSet) {
                                this.BLEConnectionStateChangeSet = !0;
                                var t = this;
                                uni.onBLEConnectionStateChange((function(result) {
                                    t.blu_data_cmdSending = !1, 
                                    result.connected || (logger("log", "setBLEConnectionStateChange", t.bleManualDisCnn, " at App.vue:471"), 
                                    t.bleManualDisCnn || t.doCloseBLEConnection(result.deviceId), 
                                    t.ble_device && t.ble_device.deviceId != result.deviceId || (t.blu_data_canSend = !1, 
                                    t.setBluCnnState(0, !0)))
                                }))
                            }
                        },
                    },
                    onLaunch: function() {
                        this.globalData.getDeviceInfo(),
                        this.globalData.getSysinfo();
                    },
                    onHide: function() {
                        this.globalData.closeBLEConnection((function(t) {
                            this.globalData.blu_state = 0,
                            this.globalData.setBluCnnState(0, false), 
                            this.globalData.closeBluetoothAdapter()
                        }))
                    }
                };
                t.default = r
            }).call(this, r("enhancedConsoleLogger")["default"])
        },


		"mainPageComponent": function(e, t, r) {
        "use strict";
            (function(logger) {

                var app = r("appStateManager")["default"],
                deviceCommandUtil = r("deviceCommandUtils "),
                uni = r("uni"),
                bleDeviceController = (r("handDrawFileManager"), r("bleDeviceControlUtils ")),
                module = {
                        data: function() {
                            var deviceFeatures = app.globalData.getDeviceFeatures();
                            return {
                                modeCmdSend: "",
                                functions: [
                                    {
                                        tag: 0,
                                        name: "DMX",
                                        show: !0
                                    },
                                    {
                                        tag: 1,
                                        name: "Random playback",
                                        show: !0
                                    },
                                    {
                                        tag: 2,
                                        name: "Timeline playback",
                                        show: !0
                                    },
                                    {
                                        tag: 3,
                                        name: "Animation playback",
                                        show: !0
                                    },
                                    {
                                        tag: 4,
                                        name: "Text playback",
                                        show: !0
                                    },
                                    {
                                        tag: 5,
                                        name: "Christmas broadcast",
                                        show: !0
                                    },
                                    {
                                        tag: 5,
                                        name: "ILDA",
                                        show: !1
                                    },
                                    {
                                        tag: 6,
                                        name: "Outdoor playback",
                                        show: !0
                                    },
                                    {
                                        tag: 7,
                                        name: "Personalized programming",
                                        show: !0
                                    },
                                    {
                                        tag: 8,
                                        name: "Hand-drawn doodle",
                                        show: !0
                                    },
                                    {
                                        tag: 9,
                                        name: "Playlist",
                                        show: !0
                                    }
                                ],
                                features: deviceFeatures,
                                deviceOn: false,
                                prjIndex: -1,
                                cnnDevice: "Not connected",
                                cnnState: false,
                                randomCheck: [],
                                initShow: false,
                            }
                        },
                        onLoad: function() {
                            this.genRandomCheck();
                        },
                        onShow: function() {
                            this.features = app.globalData.getDeviceFeatures();
                            if (!this.data.initShow) {
                                this.data.initShow = true;
                                this.methods.bluInitPro();
                            }             
                        },
                        methods: {
                            bluInitPro: function() {
                                app.globalData.blu_cnn_call_back = this.blu_cnn_call_back, 
                                app.globalData.blu_rec_call_back = this.blu_rec_call_back, 
                                bleDeviceController.cnnPreBlu()                      
                            },
                            goQueryCmd: function() {
                                bleDeviceController.gosend(!1, deviceCommandUtil.getQueryCmd(this.data.randomCheck));
                            },
                            blu_cnn_call_back: function(connectionStatus, resultCode) {
                                if (1 != connectionStatus) {
                                    var device = app.globalData.ble_device;
                                    logger("log", "blu_cnn_call_back1", connectionStatus, resultCode); 
                                    if (connectionStatus && device && "characteristicId" in device) 
                                        logger("log", "Connected", device.name), 
                                        this.cnnDevice = device.name, 
                                        this.cnnState = true, 
                                        this.goQueryCmd();
                                    else {
                                        this.cnnState = false, 
                                        this.deviceOn = false, 
                                        this.prjIndex = -1;
                                    }
                                }
                            },

                            blu_rec_call_back: function(data) {
                                logger("log", "blu_rec_call_back");
                                this.checkRcvData(data, this.randomCheck) 
                                    ? (bleDeviceController.setCanSend(true), 
                                        bleDeviceController.setCmdData(data), 
                                        this.prjIndex = app.globalData.cmd.curMode) 
                                    : logger("log", "Abnormality in reading device parameters");
                            },
                            
                            // fill the this.randomCheck array with 4 random integers between 0 and 255 (inclusive). 
                            genRandomCheck: function() {
                                for (var e = 0; e < 4; e++) this.randomCheck[e] = Math.floor(256 * Math.random())
                            },
                        
                            // Validates the received data string and random check array.
                            // Decodes and checks a checksum validation code.
                            // Updates device status and features if valid.
                            // Returns true if the data is valid and processed, otherwise false.
                            checkRcvData: function(data, randomVerify) {
                                if (4 != randomVerify.length || data.length < 24) return !1;
                                for (var r = data.substr(data.length - 24, 8), n = [], h = 0; h < 4; h++) {
                                    var i = 0,
                                        c = randomVerify[h];
                                    0 == h && (i = (c + 55 >> 1) - 10 & 255), 1 == h && (i = 7 + (c - 68 << 1) & 255), 2 == h && (i = 15 + (c + 97 >> 1) & 255), 3 == h && (i = 87 + (c - 127 >> 1) & 255), n.push(i)
                                }
                                for (var o = [], s = 0; s < 4; s++) {
                                    var l = r[2 * s] + r[2 * s + 1],
                                        p = parseInt(l, 16);
                                    o.push(p)
                                }
                                for (var d = 0; d < 4; d++)
                                    if (o[d] != n[d]) return !1;
                                var b = data.substr(data.length - 16, 2);
                                this.deviceOn = 0 != parseInt(b, 16);
                                var deviceType = data.substr(data.length - 14, 2),
                                    version = data.substr(data.length - 12, 2),
                                    userType = data.substr(data.length - 10, 2);
                                return  (app.globalData.setDeviceInfo(deviceType, version, userType), 
                                    this.features = app.globalData.getDeviceFeatures()), 
                                    this.features = app.globalData.getDeviceFeatures(), !0
                            },

                            cnnLaser: function() {
                                bleDeviceController.cnnLaser()
                            },

                            // decides whether to switch to DMX mode and send a command, 
                            // navigate to the settings page, or prompt the user to turn on the device, 
                            // based on the clicked tag and device/debug state.
                            settingClick: function(e) {
                                var tag = e.currentTarget.dataset.tag;
                                if (0 != tag || this.deviceOn ) 
                                    return this.prjIndex != tag && 0 == tag 
                                        ? (this.prjIndex = tag, app.globalData.setCmdMode(tag), void this.sendCmd()) 
                                        : void uni.navigateTo({ url: "/pages/setting/setting?dmx=" + tag });
      
                            },
                            
                            // This function handles toggling the device's power state and 
                            // sending the appropriate command, with user feedback if the action cannot be performed.
                            onOffChange: function(t) {
                                if (bleDeviceController.getCanSend()) {
                                    this.deviceOn = !this.deviceOn;
                                    var command = "B0B1B2B300B4B5B6B7";
                                    this.deviceOn && (command = "B0B1B2B3FFB4B5B6B7"), bleDeviceController.gosend(!1, command)
                                } else this.cnnState 
                                    ? app.globalData.showModalTips(this.$t("The current device cannot be identified"), !0) 
                                    : app.globalData.showModalTips(this.$t("Please connect first 5Bluetooth"), !0)
                            },
                            
                            // testFunc injects a specific test command into the device controller 
                            // and synchronizes the project index with the current command mode, 
                            // probably to simulate or test device behavior with known data.
                            testFunc: function() {
                                bleDeviceController.setCmdData("E0E1E2E3B0B1B2B300B4B5B6B7C0C1C2C30400098080800080003309FFFFFF320000000000000000000000000000000000000000000000000000000000000000000000000000FF035393C06600000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001000A00FFFFFF020000000000000004050607D0D1D2D3820000FF28000000000000000000003200FF00FF28000000000000000000FF3200FFD4D5D6D7F0F1F2F300000000070102030405060700004466F4F5F6F743E3A317F0000000E4E5E6E7"), 
                                this.prjIndex = app.globalData.cmd.curMode
                            },

                            // sendCmd generates a device command string based on the current mode and features, 
                            // stores it, and triggers the sending process.
                            sendCmd: function() {
                                var command = deviceCommandUtil.getCmdStr(app.globalData.cmd, {
                                    features: this.features
                                });
                                this.modeCmdSend = command, this.doSendCmd()
                            },

                            // doSendCmd tries to send the current command. If it fails, it retries every 100ms until successful. 
                            // When successful, it clears the command buffer.
                            doSendCmd: function() {
                                if ("" != this.modeCmdSend) {
                                    var result = bleDeviceController.gosend(!1, this.modeCmdSend),
                                        t = this;
                                    result ? this.modeCmdSend = "" : setTimeout((function() {
                                        t.doSendCmd()
                                    }), 100)
                                }
                            },
                            //  routes the user to the correct function or project page, 
                            // sending the necessary command to the device, 
                            // and handles device state checks and navigation logic.
                            prjClick: function(e) {
                                
                                var mode = e.currentTarget.dataset.tag;
                                
                                if (0 != mode)
                                    if (this.deviceOn ) {
                                        
                                        if (this.prjIndex != mode || 5 == mode && this.features.ilda) return this.prjIndex = mode, 
                                            app.globalData.setCmdMode(mode), void this.sendCmd();
                                        
                                        this.sendCmd(), 
                                        
                                        4 == mode && uni.navigateTo({
                                            url: "/sub/pages/text/text"
                                        }), 

                                        7 == mode && uni.navigateTo({
                                            url: "/sub2/pages/pgs/pgs"
                                        }), 
                                        
                                        8 == mode && uni.navigateTo({
                                            url: "/sub/pages/draw/draw"
                                        }), 
                                        9 == mode && uni.navigateTo({
                                            url: "/sub/pages/listMaster/listMaster"
                                        }), 
                                        
                                        mode >= 1 && mode <= 6 && 4 != mode && uni.navigateTo({
                                            url: "/pages/prj/prj?tag=" + mode
                                        })
                                    } 
                                else this.settingClick(e)
                            },


                            sendLastCmd: function(e) {
                                this.lastCmdTime = (new Date).getTime(), this.sendCmd2(e)
                            },
             
                        }
                    };
                t.default = module
            }).call(this, r("enhancedConsoleLogger")["default"])
        },


        "DeviceConfigPageController": function(e, t, r) {
            "use strict";
            (function(logger) {
                    app = getApp(),
                    deviceCommandUtil = r("deviceCommandUtils "),
                    bleDeviceController = r("bleDeviceControlUtils "),
                    module = (r("geometryAndUuidUtils"), {
                        data: function() {
                            var deviceFeatures = app.globalData.getDeviceFeatures();
                            return {
                                showCtr: {
                                    light1: true,
                                    light2: true,
                                    light3: true,
                                    lightExt: false
                                },
                                deviceInfo: 0,
                                features: deviceFeatures,
                                SetCmdSend: "",
                                version: "",
                                machine: "",
                                dmx: 0,

                                // channel
                                ch: 0,
                                xy: 0,
                                light: 1,
                                cfg: 0,
                                valArr: [1, 10, 10, 10, 10],
                                valRange: [
                                    [1, 512],
                                    [10, 100],
                                    [0, 255],
                                    [0, 255],
                                    [0, 255]
                                ]
                            }
                        },
                        
                        onLoad: function(e) {
                            var t = this;
                            this.version = plus.runtime.version, this.version = this.version.trim();
                            var settingData = app.globalData.getCmdData("settingData");
                            settingData.dmx = e.dmx;
                            var n = this;
                            Object.keys(settingData).forEach((function(e) {
                                n.$set(t, e, settingData[e])
                            })), 
                            this.deviceInfo = app.globalData.getDeviceInfo(), 
                            this.machine = ("0" + this.deviceInfo.deviceType).slice(-2) + " - " + ("0" + this.deviceInfo.version).slice(-2), 
                            this.initData()
                        },

                        onUnload: function() {
                            var settingData = {
                                ch: this.ch, // channel
                                dmx: this.dmx, // 0 or 1
                                xy: this.xy, // 0-7 Normal: X+Y+ X+Y- X-Y- X-Y+ Interchange: X+Y+ X+Y- X-Y- X-Y+
                                light: this.light, // 1 single ,2 dual ,3 full
                                cfg: this.cfg,  //  0 ttl 255 analog
                                lang: this.lang,

                            };
                            app.globalData.setCmdData("settingData", settingData)
                        },

                        methods: {
                            sendCmd: function() {
                                var settingData = {
                                    ch: this.ch,
                                    dmx: this.dmx,
                                    xy: this.xy,
                                    light: this.light,
                                    cfg: this.cfg,
                                    lang: this.lang,
                                    valArr: this.valArr
                                };
                                app.globalData.setCmdData("settingData", settingData);
                                var command = deviceCommandUtil.getSettingCmd(app.globalData.cmd.settingData);
                                this.SetCmdSend = command, this.doSendCmd()
                            },
                            doSendCmd: function() {
                                if ("" != this.SetCmdSend) {
                                    var commandResult = bleDeviceController.gosend(!1, this.SetCmdSend),
                                        t = this;
                                    commandResult ? this.SetCmdSend = "" : setTimeout((function() {
                                        t.doSendCmd()
                                    }), 100)
                                }
                            },
                            initData: function() {
                                if (1 == this.deviceInfo.deviceType) {
                                    this.valRange = [
                                        [1, 512],
                                        [10, 100],
                                        [0, 100],
                                        [0, 100],
                                        [0, 100]
                                    ];
                                    for (var e = 2; e < 5; e++) this.valArr[e] > this.valRange[e][1] && (this.valArr[e] = this.valRange[e][1])
                                }(1 == this.deviceInfo.deviceType 
                                    || 0 == this.deviceInfo.deviceType 
                                        && this.deviceInfo.version >= 1) && (
                                            this.showCtr = {
                                                light1: false,
                                                light2: false,
                                                light3: false,
                                                lightExt: true
                                            })
                            },
   


                        }
                    });
                t.default = module
            }).call(this, r("enhancedConsoleLogger")["default"])
        },

        "bleDeviceProjectConfigPageComponent": function(e, t, r) {
            "use strict";
            (function(e) {

                var app = getApp(),
                    deviceCommandUtils = r("deviceCommandUtils "),
                    deviceBleController = r("bleDeviceControlUtils "),
                    module = {
                        data: function() {
                            var deviceFeatures = app.globalData.getDeviceFeatures();
                            return {

                                prjIndex: 0,
                                sendCmdParmsTimer: null,
                                showOutDoorTips: false,
                                features: deviceFeatures,
                                colorDisplayOrder: [{
                                    name: "Red",
                                    color: "red",
                                    order: 0,
                                    idx: 1
                                }, {
                                    name: "yellow",
                                    color: "yellow",
                                    order: 1,
                                    idx: 4
                                }, {
                                    name: "green",
                                    color: "green",
                                    order: 2,
                                    idx: 2
                                }, {
                                    name: "Cyan",
                                    color: "#00FFFF",
                                    order: 3,
                                    idx: 5
                                }, {
                                    name: "blue",
                                    color: "blue",
                                    order: 4,
                                    idx: 3
                                }, {
                                    name: "purple",
                                    color: "purple",
                                    order: 5,
                                    idx: 6
                                }, {
                                    name: "white",
                                    color: "white",
                                    order: 6,
                                    idx: 7
                                }, {
                                    name: "Jump",
                                    color: "transparent",
                                    order: 7,
                                    idx: 8
                                }, {
                                    name: "RGB",
                                    color: "transparent",
                                    order: 8,
                                    idx: 9
                                }],
                                public: {
                                    txColor: 0, //colorDisplayOrder
                                    rdMode: 0, // Audio trigger mode	0, 255
                                    runSpeed: 10, // 1–100 (typical) Playback speed
                                    soundVal: 20 // Sound sensitivity
                                },
                                item: {
                                    pyMode: 0, // Playback mode
                                    prjSelected: [0, 0, 0, 0], // Pattern selection (bitfield)
                                    ckValues: [] //Checkbox states (UI) 
                                },
                            }
                        },
                        onLoad: function(e) {
                            var prjIndex = e.tag,
                                prjData = app.globalData.getCmdData("prjData"),
                                projectConfig  = {},
                                publicSettings  = prjData.public;
                            if (1 == prjIndex) projectConfig  = {
                                public: publicSettings ,
                                prjIndex: prjIndex
                            };
                            else {
                                var index = prjData.prjItem[prjIndex + ""];
                                projectConfig  = {
                                    public: publicSettings ,
                                    item: index,
                                    prjIndex: prjIndex
                                };
                                var selectionBits = this.getCkValues(projectConfig.item.prjSelected);
                                projectConfig .item["ckValues"] = selectionBits
                            }
                            this.prjIndex = projectConfig .prjIndex, 
                            this.public = projectConfig .public, 
                            this.item = projectConfig .item, 
                            6 == this.prjIndex && this.features.showOutDoorTips && (this.showOutDoorTips = !0)
                        },


                        methods: {
                            sendCmd: function() {
                                var commandParams = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                                this.refreshShow(), 
                                    app.globalData.setCmdData("prjData", {
                                        prjIndex: this.prjIndex,
                                        public: this.public,
                                        item: this.item
                                    }), 
                                    null == commandParams && (commandParams = {}), 
                                    commandParams["features"] = app.globalData.getDeviceFeatures();
                                var command = deviceCommandUtils.getCmdStr(app.globalData.cmd, commandParams),
                                    r = deviceBleController.gosend(!1, command);
                                return r
                            },


                            // This function takes an array of numbers, where each number is treated as a 16-bit value. 
                            // It extracts each bit (from least significant to most significant) from every number 
                            // and flattens all bits into a single array of 0s and 1s. 
                            // This is useful for converting compact bitfield representations into a simple array of booleans for UI display or logic.
                            getCkValues: function(e) {
                                for (var t = [], r = 0; r < e.length; r++)
                                    for (var n = e[r], h = 0; h < 16; h++) {
                                        var a = n >> h & 1;
                                        t.push(a)
                                    }
                                return t
                            },
                            //This function performs the inverse operation. 
                            // It takes an array of bits (0s and 1s) and packs them into an array of four 16-bit integers. 
                            // For every group of 16 bits, it calculates the corresponding integer 
                            // by setting the appropriate bits, then stores it in the result array. 
                            // This is useful for compressing a long list of boolean flags into a smaller, 
                            // more efficient format for storage or transmission.
                            getprjSelected: function(e) {
                                for (var t = 0, r = [0, 0, 0, 0], n = 0; n < e.length; n++) {
                                    var h = n % 16;
                                    1 == e[n] && (t += Math.pow(2, h)), 15 == h && (r[(n + 1) / 16 - 1] = t, t = 0)
                                }
                                return r
                            },
                            // This function handles clicks on "auto" selection buttons. Depending on the value of t, 
                            // it toggles all bits in ckValues 
                            // (if t == 2, it inverts each bit; if t == 3, it clears all bits; otherwise, it sets all bits to 1).
                            //  After updating the bit array, it uses getprjSelected to pack the bits into integers, 
                            // updates the relevant properties using Vue's $set for reactivity, and sends the updated command.
                            selectAutoBtnClick: function(selectionAction) {
                                for (var r = selectionAction, selectionBits  = this.item.ckValues, h = 0; h < selectionBits .length; h++) 2 == r ? 1 == selectionBits [h] ? selectionBits [h] = 0 : selectionBits [h] = 1 : selectionBits [h] = 3 == r ? 0 : 1;
                                var packedBits = this.getprjSelected(selectionBits );
                                this.item.prjSelected = packedBits, 
                                this.item.ckValues = selectionBits, 
                                this.sendCmd()
                            },
                            // This function responds to checkbox group changes. 
                            // It receives the indices of checked boxes, 
                            // then sets the corresponding bits in a four-element integer array (r). 
                            // Each checked index is mapped to a specific bit in the appropriate integer.
                            //  The packed result is stored in prjSelected, and the updated state is sent.
                            checkboxChange: function(indices) {
                                var packedBits = [0, 0, 0, 0];
                                for (var n = indices.detail.value, h = 0; h < n.length; h++) {
                                    var a = n[h] - 1,
                                        i = Math.floor(a / 16),
                                        c = a % 16,
                                        o = 1 << c;
                                    packedBits[i] = packedBits[i] | o
                                }
                                this.item.prjSelected = packedBits,
                                this.sendCmd()
                            },

                            btnSelectClick: function(selectedIndex ) {
                                if (0 != this.item.pyMode) {
                                    var  selectionBits = this.item.ckValues,
                                        commandParams = null;
                                    1 == selectionBits[selectedIndex ] 
                                        ? selectionBits[selectedIndex ] = 0 
                                            : (selectionBits[selectedIndex ] = 1, commandParams = {
                                        prjParm: {
                                            prjIndex: this.prjIndex,
                                            selIndex: selectedIndex  + 1
                                        }
                                    });
                                    var packedBits = this.getprjSelected(selectionBits);
                                    this.item.prjSelected = packedBits,
                                    this.item.ckValues = selectionBits, 
                                    this.sendCmdParms(commandParams)
                                }
                            },

                            sendCmdParms: function(commandParams) {
                              this.sendCmd(commandParams);
                            }
 
                        }
                    };
                t.default = module
            }).call(this, r("enhancedConsoleLogger")["default"])
        },

       "deviceCommandUtils ": function(e, t, r) {
            (function(t) {
                var arrayConverionUtil = r("arrayConversionHelper");

                function createIteratorHelper(e, t) {
                    var r = "undefined" !== typeof Symbol && e[Symbol.iterator] || e["@@iterator"];
                    if (!r) {
                        if (Array.isArray(e) || (r = function(e, t) {
                                if (!e) return;
                                if ("string" === typeof e) return arrayLikeToArray(e, t);
                                var r = Object.prototype.toString.call(e).slice(8, -1);
                                "Object" === r && e.constructor && (r = e.constructor.name);
                                if ("Map" === r || "Set" === r) return Array.from(e);
                                if ("Arguments" === r || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r)) return arrayLikeToArray(e, t)
                            }(e)) || t && e && "number" === typeof e.length) {
                            r && (e = r);
                            var n = 0,
                                h = function() {};
                            return {
                                s: h,
                                n: function() {
                                    return n >= e.length ? {
                                        done: !0
                                    } : {
                                        done: !1,
                                        value: e[n++]
                                    }
                                },
                                e: function(e) {
                                    throw e
                                },
                                f: h
                            }
                        }
                        throw new TypeError("Invalid attempt to iterate non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.")
                    }
                    var i, c = !0,
                        o = !1;
                    return {
                        s: function() {
                            r = r.call(e)
                        },
                        n: function() {
                            var e = r.next();
                            return c = e.done, e
                        },
                        e: function(e) {
                            o = !0, i = e
                        },
                        f: function() {
                            try {
                                c || null == r.return || r.return()
                            } finally {
                                if (o) throw i
                            }
                        }
                    }
                }

                function arrayLikeToArray(e, t) {
                    (null == t || t > e.length) && (t = e.length);
                    for (var r = 0, n = new Array(t); r < t; r++) n[r] = e[r];
                    return n
                }

                function toFixedWidthHex(value) {
                    var width = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 4,
                        roundedValue = Math.round(value);
                    roundedValue < 0 && (roundedValue = 32768 | -roundedValue);
                    var hexStringResult = ("0000" + roundedValue.toString(16)).slice(-width);
                    return hexStringResult
                }

                function combineNibbles(e, t) {
                    var r = e << 4 | 15 & t;
                    return r
                }

                function splitIntoSegmentsBySumLimit(e, t) {
                    for (var r = 0, n = [], h = 0, a = 0, i = 0; i < e.length; i++)
                        if (r + e[i] <= t) a += 1, n.push([h, a]), r += e[i];
                        else {
                            tempWidth = r;
                            while (1) {
                                if (tempWidth <= t) {
                                    a += 1, n.push([h, a]), r = tempWidth + e[i];
                                    break
                                }
                                if (tempWidth > t && tempWidth - e[h] < t) {
                                    a += 1, n.push([h, a]), r += e[i];
                                    break
                                }
                                tempWidth -= e[h], r -= e[h], h += 1, a -= 1
                            }
                        } return n
                }

                function generateSegmentedLayoutData(e, t) {
                    for (var r = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : 0, n = -1, h = [], a = [], c = 200, s = 0, l = 0, p = 0; p < e.length; p++) n != e[p][0] && (n = e[p][0], h.push(e[p][2] * t), s += e[p][2], a.push(e[p][3] * t), l += e[p][3]);
                    if (127 == r || 127 == r) {
                        for (var d = 0, b = [], g = 0; g < 9; g++) {
                            n++;
                            var j = [{
                                x: 0,
                                y: l / 2 + c / 2 + d,
                                z: 0
                            }];
                            b.push([n, j, c, c]), d += c, a.push(c * t)
                        }
                        for (var x = splitIntoSegmentsBySumLimit(a, 800), V = "", f = "", F = 0; F < x.length; F++) V += toFixedWidthHex(x[F][0], 2), f += toFixedWidthHex(x[F][1], 2);
                        return [e.concat(b), V, f, -d * t / 2]
                    }
                    for (var k = 0, m = [], P = 0; P < 9; P++) {
                        n++;
                        var u = [{
                            x: s / 2 + c / 2 + k,
                            y: 0,
                            z: 0
                        }];
                        m.push([n, u, c, c]), k += c, h.push(c * t)
                    }
                    for (var X = splitIntoSegmentsBySumLimit(h, 800), N = "", H = "", z = 0; z < X.length; z++) N += toFixedWidthHex(X[z][0], 2), H += toFixedWidthHex(X[z][1], 2);
                    return [e.concat(m), N, H, -k * t / 2]
                }

                function encodeLayoutToCommandData(e, r, n, h) {
                    var a = arguments.length > 4 && void 0 !== arguments[4] ? arguments[4] : 0;
                    if (0 == e.length) return null;
                    var o = 0,
                        l = 0,
                        p = -1,
                        d = "",
                        b = "",
                        g = toFixedWidthHex(a, 2),
                        j = "",
                        x = "",
                        V = 8,
                        f = .5,
                        F = V,
                        k = 0,
                        m = "00";
                    m = n.textDecimalTime ? toFixedWidthHex(Math.floor(10 * r), 2) : toFixedWidthHex(Math.floor(r), 2), t("log", "time = ", m, " at utils/funcTools.js:337"), V >= 8 && (F = 0);
                    var P = !1;
                    if (P) t("error", "20241210 - \u5f53\u524d\u4ee3\u7801\u4e3a\u5750\u6807\u8c03\u5f0f\u6a21\u5f0f\uff0c\u4e0d\u53ef\u53d1\u7248", " at utils/funcTools.js:345"), xyss = e, se1 = 0, se2 = 0, xOffset = 0;
                    else {
                        var u = generateSegmentedLayoutData(e, f, h);
                        xyss = u[0], se1 = u[1], se2 = u[2], xOffset = u[3]
                    }
                    for (var X = 0; X < xyss.length; X++) {
                        p != xyss[X][0] && (p = xyss[X][0], l > 0 && (j += toFixedWidthHex(k, 2), k = 0), l++, x += toFixedWidthHex(Math.round(Number(xyss[X][2] * f)), 2), V >= 8 && xyss[X][1].length > 1 && F++), F >= 8 && (F = 1);
                        var N = xyss[X][1];
                        k += N.length;
                        for (var H = 0; H < N.length; H++) {
                            o++;
                            var z = N[H],
                                Q = Math.round(Number(z.x * f) + xOffset),
                                R = Math.round(Number(z.y * f)),
                                v = Number(z.z),
                                I = F;
                            0 == H && (I = 0, v = 1), H == N.length - 1 && (v = 1), 1 == N.length && (v = Number(z.z)), n.textStopTime && N.length > 1 && (0 == I ? v = 2 : (H < N.length - 1 && 0 == N[H + 1].s || H == N.length - 1) && (v = 3)), d = d + toFixedWidthHex(Q) + toFixedWidthHex(R) + toFixedWidthHex(combineNibbles(I, v), 2), P && (b = b + "\n{" + Q + "," + R + "," + I + "," + v + "},")
                        }
                    }
                    return P && t("log", "\u6587\u5b57\u5750\u6807(\u7ed8\u56fe\u8f6f\u4ef6\u683c\u5f0f)", b, " at utils/funcTools.js:408"), j += toFixedWidthHex(k, 2), 0 == o ? null : {
                        cnt: o,
                        charCount: l,
                        cmd: d,
                        charWidthCmd: x,
                        charPointCmd: j,
                        se1: se1,
                        se2: se2,
                        ver: g,
                        time: m
                    }
                }

                function padHexStringToByteLength(e, t) {
                    for (var r = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : "00", n = Math.floor(e.length / 2), h = e, a = n; a < t; a++) h += r;
                    return h
                }

                function getBitmaskAndIndex(e) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : -1,
                        r = e - 1;
                    t > -1 && (r = t - e);
                    var n = Math.trunc(r / 16),
                        h = r % 16,
                        a = Math.pow(2, h),
                        i = 0;
                    return i = t > -1 ? 65535 & a : 65535 & ~a, {
                        idx: n,
                        val: i,
                        decBy: t
                    }
                }

                function applyBitmaskUpdates(e, t) {
                    var r, a = arrayConverionUtil(t),
                        i = createIteratorHelper(e);
                    try {
                        for (i.s(); !(r = i.n()).done;) {
                            var c = r.value,
                                o = getBitmaskAndIndex(c, -1);
                            if (o.idx < a.length) {
                                var s = a[o.idx] & o.val;
                                if (a[o.idx] != s) {
                                    a[o.idx] = s;
                                    var l = getBitmaskAndIndex(c, 50);
                                    l.idx < a.length && (a[l.idx] = a[l.idx] | l.val)
                                }
                            }
                        }
                    } catch (p) {
                        i.e(p)
                    } finally {
                        i.f()
                    }
                    return a
                }

                function getFeatureValue(e, t) {
                    if (e.hasOwnProperty("features")) {
                        var r = e.features;
                        if (r.hasOwnProperty(t)) return r[t]
                    }
                    return null
                }

                function encodeDrawPointCommand(e, t, r, n) {
                    for (var h = arguments.length > 4 && void 0 !== arguments[4] ? arguments[4] : "00", a = "", o = "", s = 0; s < 15; s++) s <= 11 ? o += toFixedWidthHex(t.cnfValus[s], 2) : 13 == s ? getFeatureValue({
                        features: r
                    }, "picsPlay") ? o += toFixedWidthHex(-1 == n ? 10 * t.cnfValus[12] : 10 * n, 2) : o += "00" : 14 == s && r.textStopTime ? o += toFixedWidthHex(t.txPointTime, 2) : o += "00";
                    if ("00" == h) {
                        o += h;
                        for (var l = 0; l < e.length; l++) {
                            var p = e[l],
                                d = p[3];
                            r.textStopTime && (0 == p[2] ? d = 2 : (l < e.length - 1 && 0 == e[l + 1][2] || l == e.length - 1) && (d = 3)), a = a + toFixedWidthHex(p[0].toFixed()) + toFixedWidthHex(p[1].toFixed()) + toFixedWidthHex(combineNibbles(p[2], d), 2)
                        }
                        a = o + toFixedWidthHex(e.length) + a
                    } else o += h, a = o;
                    return a
                }

                function drawPointStrToCmd(e, t) {
                    var r = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null,
                        n = "";
                    return n = null == r ? t.picsPlay ? "f0f1f200" + e + "f4f5f6f7" : "f0f1f2f3" + e + "f4f5f6f7" : "f0f1f2" + toFixedWidthHex(r, 2) + e + "f4f5f6f7", n.toUpperCase()
                }
                e.exports = {
                    test: function(e) {
                        return "hello---" + e
                    },
                    inArray: function(e, t, r) {
                        for (var n = 0; n < e.length; n++)
                            if (e[n][t] === r) return n;
                        return -1
                    },
                    ab2hex: function(e) {
                        var t = Array.prototype.map.call(new Uint8Array(e), (function(e) {
                            return ("00" + e.toString(16)).slice(-2) + ""
                        }));
                        return t.join("").toUpperCase()
                    },
                    ab2Str: function(e) {
                        var t = new Uint8Array(e),
                            r = String.fromCharCode.apply(null, t);
                        return r
                    },
                    stringToBytes: function(e) {
                        for (var t, r, n = [], h = 0; h < e.length; h++) {
                            t = e.charCodeAt(h), r = [];
                            do {
                                r.push(255 & t), t >>= 8
                            } while (t);
                            n = n.concat(r.reverse())
                        }
                        return n
                    },
                    getXtsCmd: function(e) {
                        for (var t = e.split("\n"), r = 0, n = "", h = 0; h < t.length; h++) {
                            var a = t[h];
                            if ("" != a) {
                                r++;
                                var i = a.split(","),
                                    c = Number(i[0]) + 400,
                                    o = Number(i[1]) + 400;
                                n = n + ("00" + c.toString(16)).slice(-4) + ("00" + o.toString(16)).slice(-4) + ("00" + Number(i[2]).toString(16)).slice(-2) + ("00" + Number(i[3]).toString(16)).slice(-2)
                            }
                        }
                        return 0 == r ? "" : (n = "55667788" + ("00" + r.toString(16)).slice(-2) + n + "88776655", n)
                    },
                    getXysCmd: function(e) {
                        var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 0,
                            r = 0,
                            n = 0,
                            h = -1,
                            a = "",
                            o = "",
                            l = "",
                            p = "",
                            d = toFixedWidthHex(t, 2),
                            b = 8,
                            g = .5,
                            j = b,
                            x = 0;
                        b >= 8 && (j = 0);
                        var V = generateSegmentedLayoutData(e, g);
                        xyss = V[0], se = V[1] + V[2], xOffset = V[3];
                        for (var f = 0; f < xyss.length; f++) {
                            h != xyss[f][0] && (h = xyss[f][0], n > 0 && (l += toFixedWidthHex(x, 2), x = 0), n++, p += toFixedWidthHex(Math.round(Number(xyss[f][2] * g)), 2), b >= 8 && xyss[f][1].length > 1 && j++), j >= 8 && (j = 1);
                            var F = xyss[f][1];
                            x += F.length;
                            for (var k = 0; k < F.length; k++) {
                                r++;
                                var m = F[k],
                                    P = Math.round(Number(m.x * g) + xOffset),
                                    u = Math.round(Number(m.y * g)),
                                    X = Number(m.z),
                                    N = j;
                                0 == k && (N = 0, X = 1), k == F.length - 1 && (X = 1), 1 == F.length && (X = Number(m.z)), a = a + toFixedWidthHex(P) + toFixedWidthHex(u) + toFixedWidthHex(combineNibbles(N, X), 2), o = o + "\n" + P + "," + u + ",(" + N + "," + X + "),"
                            }
                        }
                        return l += toFixedWidthHex(x, 2), 0 == r ? "" : (a = "A0A1A2A3" + toFixedWidthHex(r) + toFixedWidthHex(n, 2) + a + p + l + se + d + "A4A5A6A7", a.toUpperCase())
                    },
                    getXysCmdArr: function(e, r, n) {
                        for (var h = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : 0, a = [], c = 0; c < e.length; c++) {
                            var o = e[c].xys,
                                s = n;
                            255 == n && null != e[c].XysRight ? o = e[c].XysRight : 127 == n && null != e[c].XysUp ? o = e[c].XysUp : 128 == n && null != e[c].XysDown ? o = e[c].XysDown : s = 0;
                            var p = encodeLayoutToCommandData(o, e[c].time, r, s, h);
                            null != p && a.push(p)
                        }
                        if (0 == a.length) return "";
                        for (var d = 0, b = 0, g = "", j = "", x = "", V = "", f = "", F = "", k = "", m = "", P = 0; P < a.length; P++) d += a[P].cnt, b += a[P].charCount, toFixedWidthHex(a[P].cnt), g += toFixedWidthHex(a[P].charCount, 2), j += a[P].cmd, x += a[P].charWidthCmd, V += a[P].charPointCmd, f += a[P].se1, F += a[P].se2, k += a[P].ver, m += a[P].time;
                        t("log", d, b, " at utils/funcTools.js:308");
                        var u = toFixedWidthHex(a.length, 2),
                            X = "A0A1A2A3" + toFixedWidthHex(d) + toFixedWidthHex(b, 2) + j + u + g + x + V + f + F + k + m + "A4A5A6A7";
                        return X.toUpperCase()
                    },


                    getCmdStr: function(commandConfig) {
                        var featureParams = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null,
                            curModeHex = toFixedWidthHex(commandConfig.curMode, 2),
                            reservedHex  = toFixedWidthHex(0, 2),
                            colorHex = toFixedWidthHex(commandConfig.textData.txColor, 2),
                            c = toFixedWidthHex(commandConfig.textData.txSize / 100 * 255, 2),
                            o = toFixedWidthHex(commandConfig.textData.txSize / 100 * 255, 2),
                            runSpeedHex = toFixedWidthHex(commandConfig.textData.runSpeed / 100 * 255, 2),
                            l = "00",
                            p = toFixedWidthHex(commandConfig.textData.txDist / 100 * 255, 2),
                            audioTriggerModeHex  = toFixedWidthHex(commandConfig.prjData.public.rdMode, 2),
                            soundSensitivityHex  = toFixedWidthHex(commandConfig.prjData.public.soundVal / 100 * 255, 2),
                            x = "ffffffff0000";

                        if (null != featureParams) {
                            if (x = "", featureParams.hasOwnProperty("groupList"))
                                for (var V = 0; V < featureParams.groupList.length; V++) x += toFixedWidthHex(featureParams.groupList[V].color, 2);
                            x += "ffffffff", x = x.substring(0, 8), getFeatureValue(featureParams, "textStopTime") 
                                && (x += toFixedWidthHex(commandConfig.textData.txPointTime, 2)), x += "0000", x = x.substring(0, 12)
                        }

                        var f = "",
                            projectItems = commandConfig.prjData.prjItem;
                        for (var index in projectItems) {
                            var projectItem = projectItems[index],
                                playBackMode = 0 == projectItem.pyMode ? 0 : 128;
                            0 != playBackMode && null != featureParams 
                                && featureParams.hasOwnProperty("prjParm") 
                                    && featureParams.prjParm.prjIndex == index 
                                        && (3 == index && getFeatureValue(featureParams, "animationFix") 
                                            && [2, 4, 11, 13, 19].includes(featureParams.prjParm.selIndex) 
                                                ? playBackMode |= 50 - featureParams.prjParm.selIndex 
                                                : playBackMode |= featureParams.prjParm.selIndex);
                            var playBackModeHex = toFixedWidthHex(playBackMode, 2),
                                X = "",
                                selectionBits = arrayConverionUtil(projectItem.prjSelected);
                            3 == index && getFeatureValue(featureParams, "animationFix") && (selectionBits = applyBitmaskUpdates([2, 4, 11, 13, 19], selectionBits));
                            for (var H = 0; H < selectionBits.length; H++) X = toFixedWidthHex(selectionBits[H]) + X;
                            f = f + playBackModeHex + X
                        }
                        var runDirection = "";
                        getFeatureValue(featureParams, "arbPlay") && (runDirection += toFixedWidthHex(commandConfig.textData.runDir, 2));
                        for (var padding = "", R = Math.floor(runDirection.length / 2), v = R; v < 44; v++) padding += "00";
                        var command = "c0c1c2c3" + curModeHex + reservedHex  + colorHex + c + o + runSpeedHex + l + p + audioTriggerModeHex  + soundSensitivityHex  + x + f + runDirection + padding + "c4c5c6c7";
                        return command.toUpperCase()
                    },
                    
                    getShakeCmdStr: function(e) {
                        var r = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null,
                            n = "";
                        if (getFeatureValue(r, "xyCnf")) {
                            n = "00", r.hasOwnProperty("xyCnfSave") && !r.xyCnfSave && (n = "ff");
                            var a = e.subsetData.xyCnf;
                            a.auto ? n += toFixedWidthHex(a.autoValue, 2) : n += toFixedWidthHex(255 - a.autoValue, 2), n += toFixedWidthHex(a.phase, 2);
                            var c, o = createIteratorHelper(a.xy);
                            try {
                                for (o.s(); !(c = o.n()).done;) {
                                    var s = c.value;
                                    n += toFixedWidthHex(s.value, 2)
                                }
                            } catch (d) {
                                o.e(d)
                            } finally {
                                o.f()
                            }
                            t("log", "xyCnf", JSON.stringify(a), " at utils/funcTools.js:551")
                        }
                        n = padHexStringToByteLength(n, 16, "00");
                        var l = "10111213" + n + "14151617";
                        return l.toUpperCase()
                    },
                    getDrawPointStr: encodeDrawPointCommand,
                    getDrawCmdStr: function(e, t, r) {
                        var n = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : "00",
                            h = encodeDrawPointCommand(e, t, r, -1, n);
                        return drawPointStrToCmd(h, r)
                    },
                    drawPointStrToCmd: drawPointStrToCmd,
                    getPisCmdStr: function(e, r) {
                        for (var n = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null, h = r.cnfValus, a = "01", c = toFixedWidthHex(e, 2), o = a + c, s = 0; s <= 12; s++) o += toFixedWidthHex(h[s], 2);
                        var l = toFixedWidthHex(10 * r.playTime, 2);
                        if (o += l, getFeatureValue(n, "xyCnf")) {
                            for (var d = 14; d <= 18; d++) o += toFixedWidthHex(h[d], 2);
                            t("log", "13-17", h[14], h[15], h[16], h[17], h[18], " at utils/funcTools.js:516"), o = padHexStringToByteLength(o, 24, "00")
                        } else o = padHexStringToByteLength(o, 18, "00");
                        var b = "d0d1d2d3" + o + "d4d5d6d7";
                        return b.toUpperCase()
                    },
                    getPisListCmdStr: function(e) {
                        for (var r = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null, n = toFixedWidthHex(128 | e.length, 2), h = "FF", a = "", c = 0; c < e.length; c++) {
                            for (var o = "", s = e[c], l = 0; l <= 12; l++) o += toFixedWidthHex(s.cnfValus[l], 2);
                            var d = toFixedWidthHex(10 * s.playTime, 2);
                            if (o += d, getFeatureValue(r, "xyCnf")) {
                                for (var b = s.cnfValus, j = 14; j <= 18; j++) o += toFixedWidthHex(b[j], 2);
                                t("log", "pgs 14-18", b[14], b[15], b[16], b[17], b[18], " at utils/funcTools.js:488"), o = padHexStringToByteLength(o, 21, "00")
                            } else o = padHexStringToByteLength(o, 15, "00");
                            a = a + o + h
                        }
                        return a = "d0d1d2d3" + n + "00" + a + "d4d5d6d7", a.toUpperCase()
                    },
                    getSettingCmd: function(settingData) {
                        var channel = toFixedWidthHex(settingData.valArr[0]),
                            channelSetting = toFixedWidthHex(settingData.ch, 2),
                            displayValue = toFixedWidthHex(settingData.valArr[1], 2),
                            xy = toFixedWidthHex(settingData.xy, 2),
                            redValue = toFixedWidthHex(settingData.valArr[2], 2),
                            greenValue = toFixedWidthHex(settingData.valArr[3], 2),
                            blueValue = toFixedWidthHex(settingData.valArr[4], 2),
                            lightMode = toFixedWidthHex(settingData.light, 2),
                            ttlAnalog = toFixedWidthHex(settingData.cfg, 2);
                        0 == settingData.cfg && (redValue = "FF", greenValue = "FF", blueValue = "FF");
                        var lang = toFixedWidthHex(settingData.lang, 2),
                            command = "00010203" + channel + channelSetting + displayValue + xy + redValue + greenValue + blueValue + lightMode + ttlAnalog + lang + "000000000004050607";
                        return command.toUpperCase()
                    },
                    getCmdValue: function(startPattern , endPattern , inputString ) {
                        var matcher = new RegExp(startPattern  + "(.+?)" + endPattern ),
                            matchResult = matcher.exec(inputString );
                        return null !== matchResult ? matchResult[1] : (t("log", "No matching string found that meets the requirements", startPattern , endPattern , " at utils/funcTools.js:7"), "")
                    },
                    getQueryCmd: function(randomData) {
                        for (var encodedRandomBytes  = "", i = 0; i < randomData.length; i++) encodedRandomBytes  += toFixedWidthHex(randomData[i], 2);
                        var queryCommand  = "E0E1E2E3" + encodedRandomBytes  + "E4E5E6E7";
                        return queryCommand .toUpperCase()
                    },
                    getDrawLineStr: function(e, t) {
                        for (var r = "", n = 0; n < e.length; n++) {
                            var h = e[n];
                            r = r + toFixedWidthHex(h.pt.x) + toFixedWidthHex(h.pt.y) + toFixedWidthHex(combineNibbles(h.color, h.z), 2)
                        }
                        return r = "10111213" + toFixedWidthHex(t) + toFixedWidthHex(e.length, 2) + r + "14151617", r.toUpperCase()
                    },
                    getFeaturesValue: getFeatureValue
                }
            }).call(this, r("enhancedConsoleLogger")["default"])
        },
  
        "bleDeviceControlUtils" : function(e, t, r) {
            (function(t) {
                var appStateManager = getApp(),
                    deviceCommandUtils = r("deviceCommandUtils ");

 
                function discoverAndConfigureCharacteristics(deviceId , serviceId , retryCount) {
                    var callback  = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : null;
                    if (appStateManager.globalData.blu_connect_stop) callback  && callback (!1);
                    else {
                        var i = !1,
                            c = "",
                            s = -1;
                        uni.getBLEDeviceCharacteristics({
                            deviceId: deviceId ,
                            serviceId: serviceId ,
                            success: function(h) {
                                s = 0, t("log", "getBLEDeviceCharacteristics success", h.characteristics, " at utils/bluCtrl.js:173");
                                for (var o = function(o) {
                                        var characteristicInfo = h.characteristics[o];
                                        characteristicInfo.properties.read && (c = characteristicInfo.uuid, i && uni.readBLECharacteristicValue({
                                            deviceId: deviceId ,
                                            serviceId: serviceId ,
                                            characteristicId: characteristicInfo.uuid,
                                            success: function(e) {
                                                t("log", "readBLECharacteristicValue1:", e, " at utils/bluCtrl.js:184")
                                            },
                                            fail: function(e) {
                                                t("log", "readBLECharacteristicValue1-fail:", e, " at utils/bluCtrl.js:187")
                                            }
                                        })), characteristicInfo.properties.write && -1 != appStateManager.globalData.mtxduuids.indexOf(characteristicInfo.uuid) && (appStateManager.globalData.ble_device.characteristicId = characteristicInfo.uuid, appStateManager.globalData.ble_device.serviceId = serviceId , s++), (characteristicInfo.properties.notify || characteristicInfo.properties.indicate) && -1 != appStateManager.globalData.mrxduuids.indexOf(characteristicInfo.uuid) && uni.notifyBLECharacteristicValueChange({
                                            deviceId: deviceId ,
                                            serviceId: serviceId ,
                                            characteristicId: characteristicInfo.uuid,
                                            state: !0,
                                            success: function(h) {
                                                appStateManager.globalData.blu_readyRec = !0, i = !0, "" != c && uni.readBLECharacteristicValue({
                                                    deviceId: deviceId ,
                                                    serviceId: serviceId ,
                                                    characteristicId: characteristicInfo.uuid,
                                                    success: function(e) {
                                                        t("log", "readBLECharacteristicValue2:", e, " at utils/bluCtrl.js:217")
                                                    },
                                                    fail: function(e) {
                                                        t("log", "readBLECharacteristicValue2-fail:", e, " at utils/bluCtrl.js:220")
                                                    }
                                                }), appStateManager.globalData.setBluCnnState(2, !1), callback  && callback (!0)
                                            },
                                            fail: function(e) {
                                                s > 0 && (appStateManager.globalData.blu_readyRec = !0, i = !0, appStateManager.globalData.setBluCnnState(2, !1), callback  && callback (!0))
                                            }
                                        })
                                    }, l = 0; l < h.characteristics.length; l++) o(l)
                            },
                            fail: function(e) {
                                0 == retryCount && appStateManager.globalData.showModalTips(translate("Connection failed") + "-1002"), s = -2
                            },
                            complete: function() {
                                s <= 0 && (retryCount > 0 ? setTimeout((function() {
                                    discoverAndConfigureCharacteristics(deviceId , serviceId , --retryCount, callback )
                                }), 1500) : (uni.hideLoading(), callback  && callback (!1), appStateManager.globalData.showModalTips(translate("Connection failed") + "-1001")))
                            }
                        })
                    }
                }


                function discoverAndSetupServices(e, r) {
                    var h = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : 3,
                        a = r.callback;
                    if (appStateManager.globalData.blu_connect_stop) a && a(!1);
                    else if (h <= 0) a && a(!1);
                    else {
                        appStateManager.globalData.blu_readyRec = !1;
                        var i = e,
                            c = !1;
                        uni.getBLEDeviceServices({
                            deviceId: i,
                            success: function(e) {
                                t("log", "services: ", e, " at utils/bluCtrl.js:301");
                                for (var r = 0; r < e.services.length; r++)
                                    if (-1 != appStateManager.globalData.mserviceuuids.indexOf(e.services[r].uuid)) {
                                        c = !0, setupCharacteristicNotification(i, e.services[r].uuid, a);
                                        break
                                    }
                            },
                            fail: function(e) {
                                t("log", "getBLEDeviceServices fail:", JSON.stringify(e), " at utils/bluCtrl.js:311")
                            },
                            complete: function() {
                                c || setTimeout((function() {
                                    discoverAndSetupServices(i, r, --h)
                                }), 1e3)
                            }
                        })
                    }
                }

                function connectToDevice(device ) {
                    var showMsg  = arguments.length > 1 && void 0 !== arguments[1] && arguments[1],
                        connectionCallback = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null;
                    if (appStateManager.globalData.blu_connect_stop) connectionCallback && connectionCallback(!1);
                    else if (void 0 != device  && "" != device  && null != device ) {
                        appStateManager.globalData.readSetting(), appStateManager.globalData.blu_readyRec = !1;
                        var h = device .deviceId;
                        appStateManager.globalData.createBLEConnection(h, (function(e) {
                            e ? (appStateManager.globalData.setBluCnnState(1, !1), discoverAndSetupServices(h, {
                                showMsg: showMsg ,
                                callback: connectionCallback
                            })) : (uni.hideLoading(), showMsg  && appStateManager.globalData.showModalTips(translate("Connection failed"), !0), connectionCallback && connectionCallback(!1))
                        }))
                    } else connectionCallback && connectionCallback(!1)
                }



                function canSendBleData() {
                    return appStateManager.globalData.blu_data_canSend
                }

                function logHexBytes(byteArray) {
                    for (var r = "", n = 0; n < byteArray.length; n++) n % 2 == 0 ? ("" != r && (r += ", "), r = r + "0x" + byteArray[n]) : r += byteArray[n];
                    t("log", r, " at utils/bluCtrl.js:494")
                }

               function extractHexValue (startByte , byteLength , hexString) {
                    var n = 2 * (startByte  - 1),
                        h = n + 2 * byteLength ,
                        a = hexString.slice(n, h),
                        i = parseInt(a, 16);
                    return i
                }

                function clampOrDefault(value, min, max, defaultValue ) {
                    return isNaN(value) || value < min || value > max ? defaultValue  : value
                }

                function setupCharacteristicNotification(deviceId , serviceId ) {
                    var callback  = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null;
                    appStateManager.globalData.blu_connect_stop ? callback  && callback (!1) 
                        : (uni.onBLECharacteristicValueChange((function(characteristicEvent) {
                        var dataBytes = new Uint8Array(characteristicEvent.value),
                            valueHex = deviceCommandUtils.ab2hex(characteristicEvent.value);
                        deviceCommandUtils.ab2Str(characteristicEvent.value); - 1 != appStateManager.globalData.mrxduuids.indexOf(characteristicEvent.characteristicId) ? appStateManager.globalData.blu_readyRec && dataBytes.length > 0 && processReceivedDataFragment(valueHex) : t("error", "no same characteristicId: ", appStateManager.globalData.mrxduuids, characteristicEvent.characteristicId, " at utils/bluCtrl.js:270")
                    })), discoverAndConfigureCharacteristics(deviceId , serviceId , 1, callback ))
                }


                // The processReceivedDataFragment function is designed to handle incoming fragments of data, 
                // likely from a Bluetooth device, and assemble them into complete messages based on specific
                //  start and end markers. It works by maintaining a buffer, blu_rec_content, in the global 
                // application state (appStateManager.globalData). When a new fragment (dataFragment) arrives,
                //  the function checks if the buffer is null. If so, it only initializes the buffer if the 
                // fragment starts with the expected start marker "E0E1E2E3". Otherwise, it appends the new 
                // fragment to the existing buffer.

                // Once the buffer is updated, the function checks if it is non-empty. It then searches for 
                // the last occurrence of the start marker ("E0E1E2E3") and the end marker ("E4E5E6E7") w
                // ithin the buffer. If the end marker is found at the very end of the buffer, 
                // it extracts the complete message from the start marker to the end marker, 
                // calls a callback (setRecCallBack) to process the complete message, 
                // and clears the buffer. If the end marker is found elsewhere, 
                // it keeps only the data from the last start marker onward, 
                // likely waiting for more data to complete the message. Finally,
                //  it updates the buffer in the global state with the remaining or new data.
                function processReceivedDataFragment(dataFragment) {
                    var receiveBuffer = appStateManager.globalData.blu_rec_content;
                    if (null == receiveBuffer ? dataFragment.startsWith("E0E1E2E3") && (receiveBuffer = dataFragment) : receiveBuffer += dataFragment, "" != receiveBuffer) {
                        var r = receiveBuffer.lastIndexOf("E0E1E2E3"),
                            h = receiveBuffer.lastIndexOf("E4E5E6E7"),
                            currentMessage = receiveBuffer;
                        h > 0 && (h == receiveBuffer.length - 8 ? (currentMessage = receiveBuffer.slice(r, h + 8), appStateManager.globalData.setRecCallBack(currentMessage), currentMessage = null) : currentMessage = receiveBuffer.slice(r)), appStateManager.globalData.blu_rec_content = currentMessage
                    }
                }


                function sendBleDataBuffers(sendContext) {
                    var lastSendTimestamp  = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 0,
                        sendInterval = 20,
                        a = appStateManager.globalData.blu_data_send_interval;
                    if (appStateManager.globalData.platform.app && "android" == appStateManager.globalData.platform.system && (sendInterval = 40), sendContext.showMsg) {
                        sendContext.count;
                        var i = Math.floor((sendContext.count - sendContext.sendBufs.length) / sendContext.count * 100),
                            c = (new Date).getTime();
                        (100 == i || c - appStateManager.globalData.blu_data_lastShowTime > 200) && (appStateManager.globalData.blu_data_lastShowTime = c, sendContext.callBack ? (uni.hideLoading(), sendContext.callBack(0, i)) : uni.showLoading({
                            mask: !0
                        }))
                    }
                    if (0 != sendContext.sendBufs.length) {
                        var nowTimestamp = (new Date).getTime();
                        lastSendTimestamp  = 0 == lastSendTimestamp  ? nowTimestamp : lastSendTimestamp ;
                        var timeUntilNextSend = sendInterval - (nowTimestamp - lastSendTimestamp ),
                            delayBeforeSend  = timeUntilNextSend > 0 ? timeUntilNextSend : 1;
                        setTimeout((function() {
                            var currentBuffer  = sendContext.sendBufs.shift();
                            "split" != currentBuffer  ? (t("log", "send date---", (new Date).getTime() / 1e3, " at utils/bluCtrl.js:441"), uni.writeBLECharacteristicValue({
                                deviceId: sendContext.device.deviceId,
                                serviceId: sendContext.device.serviceId,
                                characteristicId: sendContext.device.characteristicId,
                                value: currentBuffer ,
                                success: function(t) {
                                    sendBleDataBuffers(sendContext, nowTimestamp)
                                },
                                fail: function(r) {
                                    t("log", "writeBLECharacteristicValue fail", r, " at utils/bluCtrl.js:454"), setTimeout((function() {
                                        sendContext.fail(r)
                                    }), sendInterval)
                                },
                                complete: function(e) {}
                            })) : setTimeout((function() {
                                t("log", "sleep---", a, sendInterval, " at utils/bluCtrl.js:436"), sendBleDataBuffers(sendContext, nowTimestamp)
                            }), a - (nowTimestamp - lastSendTimestamp ))
                        }), delayBeforeSend )
                    } else setTimeout((function() {
                        sendContext.success({})
                    }), sendInterval)
                }


                function splitHexStringToBuffers(hexString) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 20;
                    if ("" == hexString) return [];
                    var r = new Uint8Array(hexString.match(/[\da-f]{2}/gi).map((function(e) {
                        return parseInt(e, 16)
                    })));
                    if (null == r) return [];
                    var n = r.buffer.byteLength,
                        h = 0,
                        a = [];
                    while (n > 0) {
                        var i = n % t,
                            c = void 0;
                        n >= t ? (c = new Uint8Array(r.subarray(h, h + t)).buffer, n -= t, h += t) : (c = new Uint8Array(r.subarray(h, h + i)).buffer, n -= i, h += i), a.push(c)
                    }
                    return a
                }

                function hexStringToBufferSequence(hexString) {
                    var hexSegments = hexString.toUpperCase().split("Z");
                    t("log", hexSegments, " at utils/bluCtrl.js:530");
                    for (var bufferList = [], h = 0; h < hexSegments.length; h++) {
                        t("log", h, hexSegments[h], " at utils/bluCtrl.js:533");
                        var a = splitHexStringToBuffers(hexSegments[h]);
                        a.length > 0 && (bufferList.length > 0 && bufferList.push("split"), bufferList = bufferList.concat(a))
                    }
                    return bufferList
                }

                function sendBleBuffersPromise(dataBuffers, deviceInfo, showProgress) {
                    var progressCallback = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : null;
                    return new Promise((function(h, a) {
                        sendBleDataBuffers({
                            device: deviceInfo,
                            sendBufs: dataBuffers,
                            count: dataBuffers.length,
                            showMsg: showProgress,
                            callBack: progressCallback,
                            success: function(e) {
                                h(e)
                            },
                            fail: function(e) {
                                a(e)
                            }
                        })
                    }))
                }

        


                e.exports = {

                // manages the process of sending data over Bluetooth Low Energy (BLE) and handles various edge 
                    // cases and UI feedback.
                    // In summary, gosend returns true if the send process (real or simulated) is started or handled, 
                    // and false if it is blocked due to an ongoing send or invalid data. The actual BLE send is asynchronous,
                    //  so the return value does not indicate send success, only that the process was started or handled.
                    gosend: function(showProgress, hexData) {
                        var sendCallback = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null;
                        if (canSendBleData() 
                                ? logHexBytes(hexData) 
                            :  0 == hexData.length || !canSendBleData() 
                                && !hexData.startsWith("E0E1E2E3")) 
                                    return 0 == hexData.length || (t("log", "Simulate sending ------- 20ms", 
                                        appStateManager.globalData.blu_data_cmdSending, 
                                        " at utils/bluCtrl.js:552"), 
                                        !appStateManager.globalData.blu_data_cmdSending 
                                        && (appStateManager.globalData.blu_data_cmdSending = !0, 
                                            
                                            setTimeout((function() {
                                                appStateManager.globalData.blu_data_cmdSending = !1, 
                                                sendCallback && sendCallback(1, 100)
                                            }), 20), !0));
                        if (appStateManager.globalData.blu_data_cmdSending) 
                            return t("error", "last cmd is sending", " at utils/bluCtrl.js:563"), !1;
                        if (2 != appStateManager.globalData.blu_connected) 
                            return appStateManager.globalData.showModalTips(translate("Bluetooth not connected")), !0;
                        showProgress && (appStateManager.globalData.blu_data_lastShowTime = (new Date).getTime(), 
                        sendCallback ? sendCallback(0, 0) : uni.showLoading({
                            mask: !0
                        }));
                        var bufferSequence = hexStringToBufferSequence(hexData);
                        if (0 == bufferSequence.length) return !1;
                        if (appStateManager.globalData.blu_data_cmdSending) return !1;
                        appStateManager.globalData.blu_data_cmdSending = !0;
                        var i = bufferSequence,
                            c = appStateManager.globalData.ble_device;
                        return sendBleBuffersPromise(i, c, showProgress, sendCallback).then((function(r) {
                            showProgress && uni.hideLoading(), appStateManager.globalData.blu_data_cmdSending = !1, t("log", "bluSend succ", " at utils/bluCtrl.js:592"), sendCallback && sendCallback(1, 100)
                        })).catch((function(r) {
                            showProgress && uni.hideLoading(), t("log", "Sending failed", r, " at utils/bluCtrl.js:596"), appStateManager.globalData.blu_data_cmdSending = !1, sendCallback && sendCallback(-1, 0)
                        })), !0
                    },

                    // Initiates BLE connection if not already connected
                    cnnPreBlu: function() {
                        if (0 == appStateManager.globalData.blu_state) {
                            appStateManager.globalData.blu_state = 1, 
                            appStateManager.globalData.blu_connect_stop = !1, 
                            appStateManager.globalData.readDevice();
                            var e = appStateManager.globalData.ble_device;
                            void 0 != e && "" != e && null != e ? appStateManager.globalData.openBluetoothAdapter((function(r) {
                                r && connectToDevice(e, !1, (function(e) {
                                    1 == appStateManager.globalData.blu_state && (appStateManager.globalData.blu_state = 0)
                                   
                                }))
                            })) : appStateManager.globalData.blu_state = 0
                        }
                    },

                    setCanSend: function(canSend) {
                        appStateManager.globalData.blu_data_canSend = canSend
                    },
                    getCanSend: canSendBleData,
       
                    drawProgress: function(canvas, size , progress ) {
                        canvas.beginPath(), canvas.setFillStyle("#4C4C4C");
                        var n = size  - 0,
                            h = n;
                        canvas.moveTo(20, 0), canvas.lineTo(0 + n - 20, 0), canvas.arcTo(0 + n, 0, 0 + n, 20, 20), canvas.lineTo(0 + n, 0 + h - 20), canvas.arcTo(0 + n, 0 + h, 0 + n - 20, 0 + h, 20), canvas.lineTo(20, 0 + h), canvas.arcTo(0, 0 + h, 0, 0 + h - 20, 20), canvas.lineTo(0, 20), canvas.arcTo(0, 0, 20, 0, 20), canvas.fill();
                        var a = size  / 2,
                            i = a,
                            c = size  / 3,
                            o = -Math.PI / 2,
                            s = 2 * Math.PI * progress  / 100 + o;
                        canvas.setLineWidth(size  / 30), canvas.beginPath(), canvas.arc(a, i, c, 0, 2 * Math.PI), canvas.setStrokeStyle("#616161"), canvas.stroke(), canvas.beginPath(), canvas.arc(a, i, c, o, s), canvas.setStrokeStyle("#ECECEC"), canvas.stroke(), canvas.beginPath();
                        var l = progress  + "%",
                            p = size  / 5;
                        canvas.setFillStyle("#ECECEC"), canvas.setFontSize(p);
                        var d = canvas.measureText(l).width;
                        canvas.fillText(progress  + "%", a - d / 2, i + p / 3), canvas.fill(), canvas.draw()
                    },
                    
                    // responsible for parsing and updating the application's global state based on a device's response data,
                    // typically received as a buffer or hex string. It performs several key tasks:
                    setCmdData: function(deviceResponseData ) {
                        
                        deviceCommandUtils.getCmdValue("B0B1B2B3", "B4B5B6B7", deviceResponseData );
                        
                        var mainCommandData  = deviceCommandUtils.getCmdValue("C0C1C2C3", "C4C5C6C7", deviceResponseData );
                        
                        appStateManager.globalData.cmd.curMode = clampOrDefault(extractHexValue (1, 1, mainCommandData ), 0, 12, 0), 
                        
                        appStateManager.globalData.cmd.prjData.prjIndex = clampOrDefault(extractHexValue (1, 1, mainCommandData ), 0, 12, 0),
                        
                        appStateManager.globalData.cmd.prjData.public.txColor = clampOrDefault(extractHexValue (3, 1, mainCommandData ), 0, 9, 0), 
                        
                        appStateManager.globalData.cmd.textData.txColor = appStateManager.globalData.cmd.prjData.public.txColor, 
                        appStateManager.globalData.cmd.textData.txSize = clampOrDefault(Math.round(extractHexValue (4, 1, mainCommandData ) / 255 * 100), 10, 100, 60), 
                        appStateManager.globalData.cmd.textData.runSpeed = clampOrDefault(Math.round(extractHexValue (6, 1, mainCommandData ) / 255 * 100), 0, 255, 128), 
                        
                        appStateManager.globalData.cmd.prjData.public.runSpeed = appStateManager.globalData.cmd.textData.runSpeed, 
                        
                        appStateManager.globalData.cmd.textData.txDist = clampOrDefault(Math.round(extractHexValue (8, 1, mainCommandData ) / 255 * 100), 10, 100, 60), 
                        
                        appStateManager.globalData.cmd.prjData.public.rdMode = clampOrDefault(extractHexValue (9, 1, mainCommandData ), 0, 255, 0), 
                        appStateManager.globalData.cmd.prjData.public.soundVal = clampOrDefault(Math.round(extractHexValue (10, 1, mainCommandData ) / 255 * 100), 0, 255, 0), 
                        
                        appStateManager.globalData.cmd.textData.txPointTime = clampOrDefault(extractHexValue (15, 1, mainCommandData ), 0, 100, 50), 
                        
                        appStateManager.globalData.cmd.drawData.pisObj.txPointTime = clampOrDefault(extractHexValue (16, 1, mainCommandData ), 0, 100, 50), 
                        appStateManager.globalData.cmd.textData.refresh = !0;
                        var projectItems = appStateManager.globalData.cmd.prjData.prjItem,
                            projectItemStartIndex  = 17;
                        for (var itemKey in projectItems) {
                            var projectItem = projectItems[itemKey];
                            projectItem.pyMode = clampOrDefault(extractHexValue (projectItemStartIndex , 1, mainCommandData ), 0, 255, 0), 
                            projectItem.prjSelected[3] = extractHexValue (projectItemStartIndex  + 1, 2, mainCommandData ), 
                            projectItem.prjSelected[2] = extractHexValue (projectItemStartIndex  + 3, 2, mainCommandData ), 
                            projectItem.prjSelected[1] = extractHexValue (projectItemStartIndex  + 5, 2, mainCommandData ), 
                            projectItem.prjSelected[0] = extractHexValue (projectItemStartIndex  + 7, 2, mainCommandData ), 
                            projectItemStartIndex  += 9
                        }
                        appStateManager.globalData.cmd.textData.runDir = clampOrDefault(extractHexValue (projectItemStartIndex , 1, mainCommandData ), 0, 255, 0), 
                        projectItemStartIndex  += 1;
                        for (var p = appStateManager.globalData.cmd.subsetData, d = 0; d < 6; d++) 
                            0 == d ? p.xyCnf.auto = p.xyCnf.autoValue == clampOrDefault(extractHexValue (projectItemStartIndex  + d, 1, mainCommandData ), 0, 255, 0) 
                                    : 1 == d ? p.xyCnf.phase = clampOrDefault(extractHexValue (projectItemStartIndex  + d, 1, mainCommandData ), 0, 255, 0) 
                                            : p.xyCnf.xy[d - 2].value = clampOrDefault(extractHexValue (projectItemStartIndex  + d, 1, mainCommandData ), 0, 255, 0);
                        
                        var settingCommandData = deviceCommandUtils.getCmdValue("00010203", "04050607", deviceResponseData );
                        appStateManager.globalData.cmd.settingData.valArr[0] = clampOrDefault(extractHexValue (1, 2, settingCommandData),1, 512, 1), 
                        appStateManager.globalData.cmd.settingData.ch = extractHexValue (3, 1, settingCommandData), 
                        appStateManager.globalData.cmd.settingData.valArr[1] = clampOrDefault(extractHexValue (4, 1, settingCommandData), 10, 100, 10), 
                        appStateManager.globalData.cmd.settingData.xy = clampOrDefault(extractHexValue (5, 1, settingCommandData), 0, 7, 0),
                        appStateManager.globalData.cmd.settingData.valArr[2] = clampOrDefault(extractHexValue (6, 1, settingCommandData), 0, 255, 255), 
                        appStateManager.globalData.cmd.settingData.valArr[3] = clampOrDefault(extractHexValue (7, 1, settingCommandData), 0, 255, 255), 
                        appStateManager.globalData.cmd.settingData.valArr[4] = clampOrDefault(extractHexValue (8, 1, settingCommandData), 0, 255, 255), 
                        appStateManager.globalData.cmd.settingData.light = clampOrDefault(extractHexValue (9, 1, settingCommandData), 1, 3, 3), 
                        appStateManager.globalData.cmd.settingData.cfg = clampOrDefault(extractHexValue (10, 1, settingCommandData), 0, 255, 0);

                        var featureCommandData = deviceCommandUtils.getCmdValue("D0D1D2D3", "D4D5D6D7", deviceResponseData );
                        if ("" != featureCommandData) {
                            var j = appStateManager.globalData.getDeviceFeatures(),
                                x = 16;
                            t("log", "features", JSON.stringify(j), " at utils/bluCtrl.js:96"), 
                            deviceCommandUtils.getFeaturesValue({
                                features: j
                            }, "xyCnf") && (x = 22);
                            for (var featureConfigList = [], f = clampOrDefault(extractHexValue (1, 1, featureCommandData), 0, 255, 0), F = 127 & f, k = 0; k < F; k++) {
                                for (var m = {
                                        playTime: 0,
                                        cnfValus: []
                                    }, P = 0; P < x; P++) {
                                    var u = clampOrDefault(extractHexValue (3 + k * x + P, 1, featureCommandData), 0, 255, 0);
                                    m.cnfValus.push(u), 13 == P && (m.playTime = (u / 10).toFixed())
                                }
                                t("log", "pis.cnfValus", JSON.stringify(m.cnfValus), " at utils/bluCtrl.js:111"), featureConfigList.push(m)
                            }
                            appStateManager.globalData.cmd.pgsData.pisList = featureConfigList
                        }
                        var drawConfigData = deviceCommandUtils.getCmdValue("F0F1F2F3", "F4F5F6F7", deviceResponseData );
                        if ("" != drawConfigData)
                            for (var drawConfigObject = appStateManager.globalData.cmd.drawData.pisObj, configIndex  = 0; configIndex  < 15; configIndex ++) {
                                var drawConfigParam = clampOrDefault(extractHexValue (configIndex  + 1, 1, drawConfigData), 0, 255, 0);
                                configIndex  < drawConfigObject.cnfValus.length && (drawConfigObject.cnfValus[configIndex ] = drawConfigParam), 14 == configIndex  && (drawConfigObject.txPointTime = drawConfigParam)
                            }
                    }
                }


            }).call(this, r("enhancedConsoleLogger")["default"])
        },


        "textPlaybackPageComponent ": function(e, t, r) {
            "use strict";
            (function(e) {
                var n = r("esModuleInteropHelper");
                Object.defineProperty(t, "__esModule", {
                    value: !0
                }), t.default = void 0;
                var h = n(r("uniPopupComponentExportWrapper")),
                    app = getApp(),
                    deviceCommandUtils = r("deviceCommandUtils "),
                    bleManager = r("bleDeviceControlUtils "),
                    handwritingCanvasHelper = r("handwritingCanvasHelper"),
                    handDrawFileManager = r("handDrawFileManager"),
                    codePointAt = r("codePointAt"),
                    textLineVectorizer = r("textLineVectorizer "),
                    fontGeometryUtils = r("fontGeometryUtils "),
                    b = {
                        data: function() {
                            var fontIndex = 0 | app.globalData.readData("text_fontIdex"),
                                t = app.globalData.getDeviceFeatures(),
                                r = 650 * app.globalData.screen_width_float,
                                textObject = [{
                                    text: "",
                                    update: 0,
                                    color: 9,
                                    fontIdex: fontIndex,
                                    time: 5,
                                    xys: [],
                                    XysRight: [],
                                    XysUp: [],
                                    XysDown: []
                                }];
                            return {
                                screen_width: app.globalData.screen_width_str,
                                scUnit: app.globalData.screen_width_float,
                                pageWidth: app.globalData.screen_width_page,
                                rtl: app.globalData.rtl,
                                ntitle: this.$t("Text playback"),
                                inputNote: this.$t("Please enter text"),
                                fontIdex: fontIndex,
                                fontNameList: [],
                                fontLoadIdex: -1,
                                showChineseWarn: !1,
                                canvasShow: !0,
                                sendColorTag: !0,
                                lastSendTxtCmdTime: 0,
                                lastSendTxtCmdComplete: !0,
                                popupTimeIndex: 0,
                                showSending: !1,
                                needRefresh: !0,
                                firstShow: !0,
                                textRv: !0,
                                textInput: !1,
                                timeInput0: !1,
                                timeInput1: !1,
                                timeInput2: !1,
                                timeInput3: !1,
                                inputCursor0: !1,
                                inputCursor1: !1,
                                inputCursor2: !1,
                                inputCursor3: !1,
                                maxStrCount: 0,
                                features: t,
                                currSelectedFile: "",
                                colorDisplayOrder: [{
                                    name: "Red",
                                    color: "red",
                                    order: 0,
                                    idx: 1
                                }, {
                                    name: "yellow",
                                    color: "yellow",
                                    order: 1,
                                    idx: 4
                                }, {
                                    name: "green",
                                    color: "green",
                                    order: 2,
                                    idx: 2
                                }, {
                                    name: "Cyan",
                                    color: "#00FFFF",
                                    order: 3,
                                    idx: 5
                                }, {
                                    name: "blue",
                                    color: "blue",
                                    order: 4,
                                    idx: 3
                                }, {
                                    name: "purple",
                                    color: "purple",
                                    order: 5,
                                    idx: 6
                                }, {
                                    name: "white",
                                    color: "white",
                                    order: 6,
                                    idx: 7
                                }, {
                                    name: "Jump",
                                    color: "transparent",
                                    order: 7,
                                    idx: 8
                                }, {
                                    name: "RGB",
                                    color: "transparent",
                                    order: 8,
                                    idx: 9
                                }],
                                defGroupList: textObject,
                                maxChar: 100, // max charcters
                                maxPoints: 2048, // max points
                                textData: {
                                    verTag: 0,
                                    runDir: 0,
                                    arrColor: ["red", "green", "blue", "yellow", "#00FFFF", "purple", "white"],
                                    txPointTime: 50,
                                    txColor: 9, // text color
                                    txSize: 50, // text size
                                    txDist: 50, // text distance
                                    runSpeed: 50, // run speed
                                    groupIdex: 0,
                                    groupList: textObject
                                },
                                position: {
                                    x: r,
                                    y: r
                                },
                                startPosition: {
                                    x: 0,
                                    y: 0
                                }
                            }
                        },
                        onLoad: function() {
                            var fontRegistryModule = r("fontRegistryModule ");
                            this.fontNameList = fontRegistryModule.getFontNameList();
                            var textData = app.globalData.getCmdData("textData");
                            this.textData = textData;
                            for (var n = 0; n < this.textData.groupList.length; n++) 
                                null == this.textData.groupList[n].fontIdex 
                                    && (this.textData.groupList[n].fontIdex = this.fontIdex)
                        },

                        computed: {
                            textTime: {
                                get: function() {
                                    for (var e = [], t = 0; t < this.textData.groupList.length; t++) {
                                        var r = parseFloat(this.textData.groupList[t].time);
                                        this.features.textDecimalTime ? e.push(r.toFixed(1)) : e.push(r.toFixed(0))
                                    }
                                    return e
                                }
                            }
                        },
                        methods: {
                            sendTxtCmd: function() {
                                var e = this;
                                app.globalData.setCmdData("textData", this.textData);
                                var runDir = this.textData.runDir,
                                    command = deviceCommandUtils.getXysCmdArr(this.textData.groupList, this.features, runDir, this.textData.verTag),
                                    n = this;
                                bleManager.gosend(!0, command);
         
                            },

                            sendCmd: function() {
                                app.globalData.setCmdData("textData", this.textData);
                                var command = deviceCommandUtils.getCmdStr(app.globalData.cmd, {
                                    features: this.features,
                                    groupList: this.textData.groupList
                                });
                                bleManager.gosend(!1, command) && (this.sendColorTag = !1)
                            },
                            sendRvXysCmd: function() {
                                var settingData = app.globalData.cmd.settingData;
                                0 == this.textData.runDir ? settingData.xy = 0 : settingData.xy = 3;
                                var settingCommand = deviceCommandUtils.getSettingCmd(app.globalData.cmd.settingData);
                                bleManager.gosend(!1, settingCommand) && this.sendCmdBtn(null)
                            },
                            readFontBase64: function(fontIdex) {
                                var t = this,
                                    callback = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null;
                                if (this.fontLoadIdex != fontIdex) {
                                    var fontRegistryModule = r("fontRegistryModule "),
                                        fontList = fontRegistryModule.getFontList(this),
                                        file = fontList[fontIdex].file,
                                        mode = fontList[fontIdex].mode,
                                        sn = fontList[fontIdex].sn;
                                    fontGeometryUtils.readTTF(file, mode, (function(data, mode) {
                                        fontGeometryUtils.fontData = {
                                            data: data,
                                            mode: mode,
                                            sn: sn
                                        }, t.fontLoadIdex = fontIdex, callback && callback()
                                    }))
                                } else callback && callback()
                            },
                            setFontIdex: function(e) {
                                this.fontIdex != e && (this.fontIdex = e, app.globalData.saveData("text_fontIdex", this.fontIdex))
                            },
                            onFontChange: function(e) {
                                var t = e.detail.value;
                                this.setFontIdex(t), this.textData.groupList[this.textData.groupIdex].fontIdex = t,
                                     this.textData.groupList[this.textData.groupIdex].update = 1
                            },
                            slPointTimeChange: function(e) {
                                var t = e.detail.value;
                                this.textData.txPointTime = t, 
                                    this.sendCmd()
                            },
                            slTxSizeChange: function(e) {
                                var t = e.detail.value;
                                this.textData.txSize = t, 
                                    handwritingCanvasHelper.doDrawPicEx(this), this.sendCmd()
                            },
                            btnColorChange: function(e) {
                                var t = parseInt(e.currentTarget.dataset.tag);
                                this.textData.groupList[this.textData.groupIdex].color = t, 
                                    this.$set(this.textData, "txColor", t), 
                                    handwritingCanvasHelper.doDrawPicEx(this), this.sendCmd()
                            },
                            slTxDistChange: function(e) {
                                var t = e.detail.value;
                                this.textData.txDist = t, 
                                this.sendCmd()
                            },
                            slRunSpeedChange: function(e) {
                                var t = e.detail.value;
                                this.textData.runSpeed = t, 
                                this.sendCmd()
                            },
                            radioRunDirectionChange: function(e) {
                                var t = e.detail.value;
                                "textUp" == t ? this.$set(this.textData, "runDir", 127) : "textDown" == t 
                                    ? this.$set(this.textData, "runDir", 128) 
                                        : "right" == t 
                                            ? this.$set(this.textData, "runDir", 255) 
                                                : this.$set(this.textData, "runDir", 0), 
                                                this.sendColorTag = !0, this.sendCmdBtn(null)
                            },
                            radioDisplayChange: function(e) {
                                var t = e.detail.value;
                                "h" == t 
                                    ? this.$set(this.textData, "verTag", 0) 
                                    : this.$set(this.textData, "verTag", 255)
                            },
                            createXys: function(inputText) {
                                var textPlaybackPageComponent = this,
                                    callback = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null,
                                    fallBackShapes = [
                                        [0, [{
                                            x: -194,
                                            y: 129,
                                            z: 1
                                        }, {
                                            x: -119,
                                            y: 153,
                                            z: 0
                                        }, {
                                            x: -100,
                                            y: 174,
                                            z: 1
                                        }], 234, 234],
                                        [0, [{
                                            x: -111,
                                            y: -105,
                                            z: 1
                                        }, {
                                            x: -42,
                                            y: -105,
                                            z: 1
                                        }], 234, 234],
                                        [0, [{
                                            x: -114,
                                            y: 153,
                                            z: 1
                                        }, {
                                            x: -114,
                                            y: -105,
                                            z: 1
                                        }, {
                                            x: -186,
                                            y: -108,
                                            z: 1
                                        }], 234, 234],
                                        [1, [{
                                            x: 189,
                                            y: -105,
                                            z: 1
                                        }, {
                                            x: 50,
                                            y: -102,
                                            z: 1
                                        }, {
                                            x: 58,
                                            y: -76,
                                            z: 0
                                        }, {
                                            x: 170,
                                            y: 44,
                                            z: 0
                                        }, {
                                            x: 186,
                                            y: 89,
                                            z: 0
                                        }, {
                                            x: 176,
                                            y: 142,
                                            z: 0
                                        }, {
                                            x: 149,
                                            y: 164,
                                            z: 0
                                        }, {
                                            x: 90,
                                            y: 166,
                                            z: 0
                                        }, {
                                            x: 53,
                                            y: 150,
                                            z: 0
                                        }, {
                                            x: 37,
                                            y: 121,
                                            z: 1
                                        }], 234, 234]
                                    ];
                                fallBackShapes = [
                                    [0, [{
                                        x: -216,
                                        y: 174,
                                        z: 1
                                    }, {
                                        x: -219,
                                        y: 88,
                                        z: 1
                                    }, {
                                        x: -366,
                                        y: 88,
                                        z: 1
                                    }, {
                                        x: -366,
                                        y: -88,
                                        z: 1
                                    }, {
                                        x: -339,
                                        y: -88,
                                        z: 1
                                    }, {
                                        x: -336,
                                        y: -66,
                                        z: 1
                                    }, {
                                        x: -219,
                                        y: -66,
                                        z: 1
                                    }, {
                                        x: -216,
                                        y: -210,
                                        z: 1
                                    }, {
                                        x: -187,
                                        y: -210,
                                        z: 1
                                    }, {
                                        x: -184,
                                        y: -66,
                                        z: 1
                                    }, {
                                        x: -64,
                                        y: -66,
                                        z: 1
                                    }, {
                                        x: -62,
                                        y: -88,
                                        z: 1
                                    }, {
                                        x: -35,
                                        y: -88,
                                        z: 1
                                    }, {
                                        x: -35,
                                        y: 88,
                                        z: 1
                                    }, {
                                        x: -184,
                                        y: 88,
                                        z: 1
                                    }, {
                                        x: -187,
                                        y: 174,
                                        z: 1
                                    }, {
                                        x: -216,
                                        y: 174,
                                        z: 0
                                    }], 400, 400],
                                    [0, [{
                                        x: -187,
                                        y: 59,
                                        z: 1
                                    }, {
                                        x: -64,
                                        y: 62,
                                        z: 1
                                    }, {
                                        x: -62,
                                        y: -37,
                                        z: 1
                                    }, {
                                        x: -184,
                                        y: -40,
                                        z: 1
                                    }, {
                                        x: -187,
                                        y: 59,
                                        z: 0
                                    }], 400, 400],
                                    [0, [{
                                        x: -339,
                                        y: 59,
                                        z: 1
                                    }, {
                                        x: -219,
                                        y: 62,
                                        z: 1
                                    }, {
                                        x: -216,
                                        y: -37,
                                        z: 1
                                    }, {
                                        x: -336,
                                        y: -40,
                                        z: 1
                                    }, {
                                        x: -339,
                                        y: 59,
                                        z: 0
                                    }], 400, 400],
                                    [1, [{
                                        x: 29,
                                        y: 152,
                                        z: 1
                                    }, {
                                        x: 29,
                                        y: -208,
                                        z: 1
                                    }, {
                                        x: 56,
                                        y: -208,
                                        z: 1
                                    }, {
                                        x: 58,
                                        y: -189,
                                        z: 1
                                    }, {
                                        x: 338,
                                        y: -189,
                                        z: 1
                                    }, {
                                        x: 341,
                                        y: -208,
                                        z: 1
                                    }, {
                                        x: 368,
                                        y: -208,
                                        z: 1
                                    }, {
                                        x: 368,
                                        y: 152,
                                        z: 1
                                    }, {
                                        x: 29,
                                        y: 152,
                                        z: 0
                                    }], 400, 400],
                                    [1, [{
                                        x: 58,
                                        y: 128,
                                        z: 1
                                    }, {
                                        x: 341,
                                        y: 126,
                                        z: 1
                                    }, {
                                        x: 338,
                                        y: -165,
                                        z: 1
                                    }, {
                                        x: 56,
                                        y: -162,
                                        z: 1
                                    }, {
                                        x: 58,
                                        y: 128,
                                        z: 0
                                    }], 400, 400],
                                    [1, [{
                                        x: 253,
                                        y: -32,
                                        z: 1
                                    }, {
                                        x: 237,
                                        y: -45,
                                        z: 1
                                    }, {
                                        x: 272,
                                        y: -82,
                                        z: 1
                                    }, {
                                        x: 288,
                                        y: -66,
                                        z: 1
                                    }, {
                                        x: 253,
                                        y: -32,
                                        z: 0
                                    }], 400, 400],
                                    [1, [{
                                        x: 85,
                                        y: 86,
                                        z: 1
                                    }, {
                                        x: 85,
                                        y: 62,
                                        z: 1
                                    }, {
                                        x: 186,
                                        y: 59,
                                        z: 1
                                    }, {
                                        x: 184,
                                        y: 6,
                                        z: 1
                                    }, {
                                        x: 90,
                                        y: 6,
                                        z: 1
                                    }, {
                                        x: 90,
                                        y: -18,
                                        z: 1
                                    }, {
                                        x: 186,
                                        y: -21,
                                        z: 1
                                    }, {
                                        x: 186,
                                        y: -93,
                                        z: 1
                                    }, {
                                        x: 77,
                                        y: -96,
                                        z: 1
                                    }, {
                                        x: 77,
                                        y: -120,
                                        z: 1
                                    }, {
                                        x: 320,
                                        y: -120,
                                        z: 1
                                    }, {
                                        x: 320,
                                        y: -96,
                                        z: 1
                                    }, {
                                        x: 210,
                                        y: -93,
                                        z: 1
                                    }, {
                                        x: 213,
                                        y: -18,
                                        z: 1
                                    }, {
                                        x: 304,
                                        y: -18,
                                        z: 1
                                    }, {
                                        x: 304,
                                        y: 6,
                                        z: 1
                                    }, {
                                        x: 213,
                                        y: 6,
                                        z: 1
                                    }, {
                                        x: 210,
                                        y: 59,
                                        z: 1
                                    }, {
                                        x: 312,
                                        y: 62,
                                        z: 1
                                    }, {
                                        x: 312,
                                        y: 86,
                                        z: 1
                                    }, {
                                        x: 85,
                                        y: 86,
                                        z: 0
                                    }], 400, 400]
                                ], fallBackShapes = [
                                    [0, [{
                                        x: 170,
                                        y: 112,
                                        z: 1
                                    }, {
                                        x: 141,
                                        y: 142,
                                        z: 1
                                    }, {
                                        x: 117,
                                        y: 118,
                                        z: 0
                                    }, {
                                        x: -112,
                                        y: 118,
                                        z: 0
                                    }, {
                                        x: -136,
                                        y: 131,
                                        z: 1
                                    }, {
                                        x: -136,
                                        y: -42,
                                        z: 0
                                    }, {
                                        x: -155,
                                        y: -120,
                                        z: 0
                                    }, {
                                        x: -184,
                                        y: -176,
                                        z: 1
                                    }, {
                                        x: -152,
                                        y: -136,
                                        z: 0
                                    }, {
                                        x: -126,
                                        y: -72,
                                        z: 0
                                    }, {
                                        x: -115,
                                        y: 112,
                                        z: 1
                                    }, {
                                        x: 170,
                                        y: 112,
                                        z: 0
                                    }], 400, 400],
                                    [0, [{
                                        x: -16,
                                        y: 182,
                                        z: 1
                                    }, {
                                        x: 16,
                                        y: 126,
                                        z: 1
                                    }, {
                                        x: 24,
                                        y: 158,
                                        z: 1
                                    }, {
                                        x: -16,
                                        y: 182,
                                        z: 0
                                    }], 400, 400]
                                ];
                                var cleanedInput = inputText.replace("\n", "");
                                "" != cleanedInput ? (uni.showLoading({
                                    title: this.$t("Generating coordinate points..."),
                                    mask: !0
                                }), this.readFontBase64(this.fontIdex, (function() {
                                    var textCoordinates = textLineVectorizer.getXXYY(codePointAt, fontGeometryUtils.fontData, cleanedInput, textPlaybackPageComponent.textRv);
                                    uni.hideLoading(), fallBackShapes = textCoordinates.xxyy, fontGeometryUtils.ifHasChinese(textCoordinates.notRec) && 1001 == fontGeometryUtils.fontData.sn && app.globalData.showModalTips(textPlaybackPageComponent.$t("Due to capacity limitations, some Chinese characters are not included in the font library. For the complete font library, please refer to the APP version"), !0);
                                    var i = textPlaybackPageComponent.getSumSizeExclude(textPlaybackPageComponent.textData.groupIdex),
                                        c = handwritingCanvasHelper.getTxXySize(fallBackShapes);
                                    c.chCount + i.chCount > textPlaybackPageComponent.maxChar 
                                        ? app.globalData.showModalTips(textPlaybackPageComponent.$t("The number of text coordinate points has exceeded 2048, please re-enter."), !0) 
                                        : c.ptCount + i.ptCount > textPlaybackPageComponent.maxPoints 
                                            ? app.globalData.showModalTips(textPlaybackPageComponent.$t("The number of text coordinate points has exceeded 2048, please re-enter."), !0) 
                                            : callback && callback(fallBackShapes, textCoordinates.xxyyRight, textCoordinates.xxyyUp, textCoordinates.xxyyDown)
                                }))) : callback && callback([])
                            },
                            getSumSizeExclude: function() {
                                for (var excludedIndex = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null, charCount = 0, pointCount = 0, n = 0; n < this.textData.groupList.length; n++)
                                    if (null == excludedIndex || n != excludedIndex) {
                                        var h = handwritingCanvasHelper.getTxXySize(this.textData.groupList[n].xys);
                                        charCount += h.chCount, pointCount += h.ptCount
                                    } return {
                                    chCount: charCount,
                                    ptCount: pointCount
                                }
                            },
                            inputEvent: function(e) {
                                var t = e.detail.value;
                                this.textData.groupList[this.textData.groupIdex].update = 1, 
                                this.$set(this.textData.groupList[this.textData.groupIdex], "text", t)
                            },
                            onTimeBlur: function(t) {
                                var r = this.textData.groupList[t].time;
                                e("log", "time", r, " at sub/pages/text/text.js:973"), 
                                this.features.textDecimalTime ? r < 1 || r > 25.5 ? (app.globalData.showModalTips(this.$t("text_time_range"), !0), this.$set(this.textData.groupList[t], "time", 5)) : this.$set(this.textData.groupList[t], "time", Math.floor(10 * r) / 10) : r < 1 || r > 255 ? (app.globalData.showModalTips(this.$t("\u8bf7\u8f93\u51651-255\u8303\u56f4\u7684\u6570\u503c"), !0), this.$set(this.textData.groupList[t], "time", 5)) : this.$set(this.textData.groupList[t], "time", Math.floor(r))
                            },
                            onGroupChange: function(e) {
                                this.textData.groupIdex != e.detail.value && (this.$set(this.textData, "groupIdex", e.detail.value), handwritingCanvasHelper.doDrawPicEx(this))
                            },
                            refreshCanvasDraw: function() {
                                var e = this;
                                this.$nextTick((function() {
                                    var t = uni.createSelectorQuery().in(e);
                                    t.select("#myCanvas").boundingClientRect((function(t) {
                                        e.cvWH = {
                                            w: t.width,
                                            h: t.height
                                        }, setTimeout((function() {
                                            handwritingCanvasHelper.doDrawPicEx(e)
                                        }), 100)
                                    })).exec()
                                }))
                            },
                            createXyByIdex: function(e) {
                                var t = this,
                                    r = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null;
                                if (0 != this.textData.groupList[e].update) {
                                    var n = this.textData.groupList[e].text;
                                    this.createXys(n, (function(n, h, i, c) {
                                        if (0 == n.length) return app.globalData.showModalTips(t.inputNote, !0), void(r && r());
                                        t.textData.groupList[e].xys = n, t.textData.groupList[e].XysRight = h, t.textData.groupList[e].XysUp = i, t.textData.groupList[e].XysDown = c, t.textData.groupList[e].update = 0, r && r()
                                    }))
                                } else r && r()
                            },
                            checkCurrentGroupOk: function() {
                                var e = this.textData.groupList[this.textData.groupIdex].xys,
                                    t = this.textData.groupList[this.textData.groupIdex].text,
                                    r = this.textData.groupList[this.textData.groupIdex].fontIdex,
                                    n = !0,
                                    h = "";
                                if (1007 == this.fontNameList[r].sn) return !0;
                                for (var i = 0; i < e.length; i++) {
                                    var c = e[i][0];
                                    if (e[i][1].length <= 1 && " " != t[c]) {
                                        for (var o = !0, s = 0; s < h.length; s++)
                                            if (h[s] == t[c]) {
                                                o = !1;
                                                break
                                            } o && (h += t[c]), n = !1
                                    }
                                }
                                if (!n) {
                                    var l = this.fontNameList[r].name;
                                    return h.length > 4 && (h = h.substring(0, 3) + "..."), 
                                    app.globalData.showModalTips(this.$t("\u7b2c") + (this.textData.groupIdex + 1) + this.$t("\u7ec4\u5b57\u4f53") + this.$t(l) + this.$t("\u4e0d\u652f\u6301\u6587\u672c") + '"' + h + '",' + this.$t("\u8bf7\u4fee\u6539\u5b57\u4f53\u6216\u6587\u672c\u540e\u91cd\u8bd5"), !0), !1
                                }
                                return !0
                            },
                            addGroup: function(e) {
                                var t = this;
                                if (this.textData.groupList.length >= 4) app.globalData.showModalTips(this.$t("\u6700\u591a4\u4e2a\u5206\u7ec4"), !0);
                                else if ("" != this.textData.groupList[this.textData.groupIdex].text.trim()) {
                                    var r = this;
                                    this.createXyByIdex(this.textData.groupIdex, (function() {
                                        "" != t.textData.groupList[t.textData.groupIdex].text.trim() 
                                            ? r.checkCurrentGroupOk() && (r.textData.groupList.push({
                                            text: "",
                                            time: 5,
                                            color: 9,
                                            update: 0,
                                            fontIdex: r.fontIdex,
                                            xys: []
                                        }), r.$set(r.textData, "groupIdex", r.textData.groupList.length - 1),
                                            r.$set(t.textData, "txColor", r.textData.groupList[r.textData.groupIdex].color), 
                                            r.refreshCanvasDraw(), r.sendColorTag = !0, r.textInput = !1, setTimeout((function() {
                                            r.textInput = !0
                                        }), 100)) : app.globalData.showModalTips(t.inputNote, !0)
                                    }))
                                } else app.globalData.showModalTips(this.inputNote, !0)
                            },
                            oprEdit: function(e) {
                                var t = this;
                                this.textData.groupIdex !== e && ("" != this.textData.groupList[this.textData.groupIdex].text.trim() ? this.createXyByIdex(this.textData.groupIdex, (function() {
                                    t.checkCurrentGroupOk() && (t.$set(t.textData, "groupIdex", e), t.setFontIdex(t.textData.groupList[t.textData.groupIdex].fontIdex), t.$set(t.textData, "txColor", t.textData.groupList[t.textData.groupIdex].color), t.refreshCanvasDraw())
                                })) : app.globalData.showModalTips(this.inputNote, !0))
                            },
                            changeTimeClick: function(e, t) {
                                var r = 0;
                                this.features.textDecimalTime ? (r = parseFloat(this.textData.groupList[t].time), r += .1 * e, r < 1 && (r = 1), r > 25.5 && (r = 25.5), r = Math.round(10 * r) / 10) : (r = parseInt(this.textData.groupList[t].time), r += e, r < 1 && (r = 1), r > 255 && (r = 255)), this.$set(this.textData.groupList[t], "time", r)
                            },
                            setTimeInput: function(e) {
                                this.popupTimeIndex = e, this.$refs.popupTime.open("bottom")
                            },
                            previwBtn: function(e) {
                                var t = this;
                                this.createXyByIdex(this.textData.groupIdex, (function() {
                                    handwritingCanvasHelper.doDrawPicEx(t)
                                }))
                            },
                            sendCmdBtn: function(e) {
                                var t = this;
                                this.lastSendTxtCmdComplete && this.createXyByIdex(this.textData.groupIdex, (function() {
                                    "" != t.textData.groupList[t.textData.groupIdex].text.trim() ? t.checkCurrentGroupOk() && (t.sendColorTag && t.sendCmd(), t.lastSendTxtCmdComplete = !1, t.sendTextCmdMustOk((new Date).getTime()), handwritingCanvasHelper.doDrawPicEx(t)) : app.globalData.showModalTips(t.inputNote, !0)
                                }))
                            },

                            restoreDeskTop: function() {
                                var t = handDrawFileManager.getTextFileData("saveDeskTopFile_002", !0);
                                if (t) {
                                    var r = t.data.features,
                                        n = this.features;
                                    if (r.textDecimalTime != n.textDecimalTime || r.textModeFix01 != n.textModeFix01 || !(n.textUpDown && r.hasOwnProperty("textUpDown") && r.textUpDown) && n.textUpDown) return e("log", "\u5f53\u524d\u4fdd\u5b58\u7684\u6587\u672c\u683c\u5f0f\u4e0d\u652f\u6301", " at sub/pages/text/text.js:1211"), this.textData.groupList = this.defGroupList, void(this.textData.groupIdex = 0);
                                    app.globalData.cmd.textData.refresh ? (app.globalData.cmd.textData.refresh = !1, this.textData.groupIdex = t.data.textData.groupIdex, this.textData.groupList = t.data.textData.groupList) : this.textData = t.data.textData;
                                    for (var h = 0; h < this.textData.groupList.length; h++) null == this.textData.groupList[h].fontIdex && (this.textData.groupList[h].fontIdex = this.fontIdex);
                                    this.textData.groupList.length > 0 && this.setFontIdex(this.textData.groupList[this.textData.groupIdex].fontIdex), this.sendColorTag = !0, this.needRefresh = !0
                                }
                            },
                            saveTextFileData: function(e) {
                                var t = this;
                                this.createXyByIdex(this.textData.groupIdex, (function() {
                                    if ("" == t.textData.groupList[t.textData.groupIdex].text.trim()) return app.globalData.showModalTips(t.$t("\u7b2c") + (t.textData.groupIdex + 1) + t.$t("\u7ec4\u6587\u672c\u4e3a\u7a7a\uff0c\u8bf7\u8f93\u5165\u518d\u4fdd\u5b58"), !0), !1;
                                    if (t.checkCurrentGroupOk()) {
                                        handwritingCanvasHelper.doDrawPicEx(t);
                                        var r = t.getSumSizeExclude(),
                                            n = {
                                                features: t.features,
                                                textData: t.textData
                                            };
                                        handDrawFileManager.saveTextFileData(e, n, r), t.currSelectedFile = e, app.globalData.showModalTips(t.$t("\u4fdd\u5b58\u6210\u529f"))
                                    }
                                }))
                            },
                            getFileDataByName: function(e) {
                                var t = handDrawFileManager.getTextFileData(e);
                                if (t) {
                                    this.textData = t.data.textData;
                                    var r = !1;
                                    this.textData.hasOwnProperty("runDir") || (this.textData["runDir"] = 0, r = this.features.arbPlay);
                                    for (var n = 0; n < this.textData.groupList.length; n++) null == this.textData.groupList[n].fontIdex && (this.textData.groupList[n].fontIdex = this.fontIdex), r && (this.textData.groupList[n].update = 1);
                                    this.textData.groupList.length > 0 && this.setFontIdex(this.textData.groupList[this.textData.groupIdex].fontIdex), this.sendColorTag = !0, this.needRefresh = !0, this.currSelectedFile = e
                                }
                            },

                        }
                    };
                t.default = b
            }).call(this, r("enhancedConsoleLogger")["default"])
  
			
	    },
    
        "handwritingCanvasHelper": function(e, t) {
            function getTxXySize(xysArray ) {
                for (var currentCharId  = -1, pointCount = 0, charCount = 0, index = 0; index < xysArray .length; index++) {
                    var currentItem = xysArray [index];
                    currentCharId  != currentItem[0] && (currentCharId  = currentItem[0], charCount++), pointCount += currentItem[1].length
                }
                return {
                    chCount: charCount,
                    ptCount: pointCount
                }
            }

            function doDrawPicEx(textComponenet) {
                var textCompnenet2 = textComponenet;
                (function(textCompnenet3) {
                    var textPlaybackPageComponent = textCompnenet3,
                        textData = textPlaybackPageComponent.textData,
                        xyData = 0 == textData.groupList.length ? null : textData.groupList[textData.groupIdex].xys;
                    if (null != xyData) {
                        for (var a = textData.txColor, i = textData.arrColor, c = textPlaybackPageComponent.cvWH.w / 2, o = textPlaybackPageComponent.cvWH.h / 2, s = textPlaybackPageComponent.ctx, l = textData.txSize / 100, p = textPlaybackPageComponent.cvWH.h / 800 * l, d = "red", b = -1, g = -1, charPointCount = getTxXySize(xyData), x = 0; x < xyData.length; x++)
                            if (!(xyData[x][1].length < 2)) {
                                b != xyData[x][0] && (g++, s.beginPath(), b = xyData[x][0], d = a <= 7 ? i[a - 1] : i[g % 7], s.strokeStyle = d);
                                for (var V = xyData[x][1], f = 0; f < V.length; f++) {
                                    var F = V[f],
                                        k = F.x * p + c,
                                        m = o - F.y * p;
                                    0 == f ? s.moveTo(k.toFixed(), m.toFixed()) : Math.abs(F.x - V[f - 1].x) < 1 && Math.abs(F.y - V[f - 1].y) < 1 ? (s.arc(k, m, 1, 0, 2 * Math.PI), s.moveTo(k.toFixed(), m.toFixed())) : s.lineTo(k.toFixed(), m.toFixed())
                                }
                                s.stroke()
                            } s.beginPath(), s.setFillStyle("white"), s.setFontSize(10);
                        var P = textPlaybackPageComponent.getSumSizeExclude(textData.groupIdex),
                            u = textPlaybackPageComponent.$t("Character count") + ": " + (charPointCount.chCount + P.chCount) + "/" + textPlaybackPageComponent.maxChar + "  " + textPlaybackPageComponent.$t("Point count") + ": " + (charPointCount.ptCount + P.ptCount) + "/" + textPlaybackPageComponent.maxPoints;
                        s.fillText(u, 4, textPlaybackPageComponent.cvWH.h - 4), s.draw()
                    }
                })(textCompnenet2)
            }
            e.exports = {
                getTxXySize: getTxXySize,
                doDrawPicEx: doDrawPicEx,
                onReady: function(e) {
                    var t = e,
                        r = uni.createSelectorQuery();
                    r.select("#myCanvas").fields({
                        node: !0,
                        size: !0
                    }).exec((function(e) {
                        e[0].node;
                        var r = uni.createCanvasContext("myCanvas", t);
                        t.ctx = r;
                        var h = uni.getSystemInfoSync(),
                            a = h.pixelRatio;
                        t.cvRatio = {
                            w: e[0].width * a,
                            h: e[0].height * a
                        }, t.cvWH = {
                            w: e[0].width,
                            h: e[0].height
                        }, t.$nextTick((function() {
                            doDrawPicEx(t)
                        }))
                    }))
                }
            }
        },

    
    "fontRegistryModule ": function(e, t, r) {
            var mergedDrawFontsUtils = r("mergedDrawFontsUtils"),
                h = [{
                    name: "Single Line Font",
                    file: mergedDrawFontsUtils.DrawFonts,
                    mode: 2,
                    sn: 1004,
                    note: "font_note_1004",
                    msg: "Minimal strokes, no flicker, recommended for use"
                }, {
                    name: "SimSun",
                    file: "simsun_0.woff",
                    mode: 1,
                    sn: 1003,
                    note: "font_note_1003"
                }, {
                    name: "Source Han Sans 1",
                    file: "latin.woff",
                    mode: 1,
                    sn: 1002,
                    note: "font_note_1002"
                }, {
                    name: "Source Han Sans 2",
                    file: "china.woff",
                    mode: 1,
                    sn: 1005,
                    note: "font_note_1005"
                }, {
                    name: "Source Han Sans 3",
                    file: "japan_korea.woff",
                    mode: 1,
                    sn: 1006,
                    note: "font_note_1006"
                }, {
                    name: "Source Han Sans 4",
                    file: "arabic.woff",
                    mode: 1,
                    sn: 1007,
                    note: "font_note_1007"
                }];
            e.exports = {
                getFontNameList: function(e) {
                    for (var t = [], r = 0; r < h.length; r++) {
                        var n = h[r].name,
                            a = h[r].sn;
                        t.push({
                            name: n,
                            sn: a
                        })
                    }
                    return t
                },
                getFontList: function(e) {
                    return h
                }
            }
        },

        "fontGeometryUtils ": function(e, t) {
            function calculateAngleBetweenPoints (pointA , vertex , pointB) {
                var vectorA = {
                        x: pointA [0] - vertex [0],
                        y: pointA [1] - vertex [1]
                    },
                    vectorB = {
                        x: pointB[0] - vertex [0],
                        y: pointB[1] - vertex [1]
                    },
                    dotProduct = vectorA.x * vectorB.x + vectorA.y * vectorB.y,
                    magnitudeA = Math.sqrt(Math.pow(vectorA.x, 2) + Math.pow(vectorA.y, 2)),
                    magnitudeB = Math.sqrt(Math.pow(vectorB.x, 2) + Math.pow(vectorB.y, 2));
                if (0 == magnitudeA || 0 == magnitudeB) return 0;
                var o = Math.acos(dotProduct / (magnitudeA * magnitudeB)),
                    s = 180 * o / Math.PI;
                return s
            }

            function getLineRectangleIntersection (pointA, pointB, box) {
                var rectangleBoundary = {
                        w: box.w,
                        h: -box.h
                    },
                    slope = (pointA[1] - pointB[1]) / (pointB[0] - pointA[0]),
                    yIntercept = -pointB[1] - slope * pointB[0],
                    intersectionPoint = [];
                yIntercept <= 0 && yIntercept >= rectangleBoundary.h && (pointB[0] < 0 || pointA[0] < 0) && (intersectionPoint = [0, -yIntercept]);
                var c = slope * rectangleBoundary.w + yIntercept;
                if (c <= 0 && c >= rectangleBoundary.h && (pointB[0] > rectangleBoundary.w || pointA[0] > rectangleBoundary.w) && (intersectionPoint = [rectangleBoundary.w, -c]), 0 != slope) {
                    var o = -yIntercept / slope;
                    o >= 0 && o <= rectangleBoundary.w && (pointB[1] < 0 || pointA[1] < 0) && (intersectionPoint = [o, 0]);
                    var s = (rectangleBoundary.h - yIntercept) / slope;
                    s >= 0 && s <= rectangleBoundary.w && (pointB[1] > -rectangleBoundary.h || pointA[1] > -rectangleBoundary.h) && (intersectionPoint = [s, -rectangleBoundary.h])
                }
                return intersectionPoint.length > 0 && intersectionPoint.push(1), intersectionPoint
            }

            function isChineese(e) {
                return /[\u4E00-\u9FA5]/.test(e)
            }
            e.exports = {
                fontData: null,
                ifHasChinese: function(e) {
                    if (null == e) return !1;
                    for (var t = 0; t < e.length; t++)
                        if (isChineese(e[t])) return !0;
                    return !1
                },
                parseLines: function(points) {
                    for (var rextangleSize = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : {
                            w: 800,
                            h: 800
                        }, r = [], h = [], a = 0; a < points.length; a++) {
                        var i = points[a];
                        if (i[0] < 0 || i[0] > rextangleSize.w || i[1] < 0 || i[1] > rextangleSize.h) {
                            if (0 != h.length) {
                                var c = getLineRectangleIntersection (h[h.length - 1], i, rextangleSize);
                                c.length > 0 && h.push(c), r.push(h), h = []
                            }
                        } else {
                            if (a > 0 && 0 == h.length) {
                                var o = points[a - 1];
                                if (o[0] < 0 || o[0] > rextangleSize.w || o[1] < 0 || o[1] > rextangleSize.h) {
                                    var s = getLineRectangleIntersection (o, i, rextangleSize);
                                    s.length > 0 && h.push(s)
                                }
                            }
                            h.push(i)
                        }
                    }
                    return h.length > 0 && r.push(h), r
                },
                readTTF: function(fontName, mode, callback) {
                    2 != mode ? plus.io.resolveLocalFileSystemURL("_www/static/app-plus/font/" + fontName, (function(e) {
                        e.file((function(e) {
                            var fileReader = new plus.io.FileReader;
                            fileReader.onloadend = function(e) {
                                var dataURl = e.target.result,
                                    fontData = dataURl.split(",")[1];
                                callback(fontData, mode)
                            }, fileReader.onerror = function() {
                                uni.hideLoading(), uni.showToast({
                                    title: "\u8bfb\u53d6\u5b57\u4f53\u5931\u8d25",
                                    icon: "none"
                                })
                            }, fileReader.readAsDataURL(e)
                        }))
                    }), (function(e) {
                        uni.hideLoading(), uni.showToast({
                            title: "\u5b57\u4f53\u6587\u4ef6\u89e3\u6790\u5931\u8d25\uff1a" + JSON.stringify(e),
                            icon: "none"
                        })
                    })) : callback(fontName, mode)
                },
                dealLine: function(points) {
                    var t = !(arguments.length > 1 && void 0 !== arguments[1]) || arguments[1],
                        output = [],
                        previousPoint = points[0];
                    output.push([previousPoint[0], previousPoint[1], 0, 1]);
                    for (var index = 1; index < points.length - 1; index++) {
                        var currentPoint = points[index],
                            nextPoint = points[index + 1],
                            angle = calculateAngleBetweenPoints (previousPoint, currentPoint, nextPoint);
                        if (0 != angle && 180 != angle) {
                            var s = angle <= 135 ? 1 : 0;
                            t || (s = 0), output.push([currentPoint[0], currentPoint[1], 1, s]), previousPoint = currentPoint
                        }
                    }
                    var l = points[points.length - 1];
                    return output.push([l[0], l[1], 1, 1]), output
                }
            }
     
        },

        "textLineVectorizer ": function(e, t, r) {
            (function(t) {
                var spreadToArrayHelper = r("spreadToArrayHelper"),
                    arrayConversionHelper = r("arrayConversionHelper"),
                    arabicHelper = r("arabicPresentationFormsConverter");

                function sampleQuadraticBezier (start, controlPoint, endPoint, n) {
                    for (var h = [], a = 0; a <= 1; a += 1 / n) {
                        var i = Math.pow(1 - a, 2) * start.x + 2 * (1 - a) * a * controlPoint.x + Math.pow(a, 2) * endPoint.x,
                            c = Math.pow(1 - a, 2) * start.y + 2 * (1 - a) * a * controlPoint.y + Math.pow(a, 2) * endPoint.y;
                        h.push({
                            x: i,
                            y: c,
                            z: 0
                        })
                    }
                    return h
                }

                function appendToArrayOr(e, t) {
                    return e.length, e.push(t), !0
                }

                function parsePathCommands (pathCommands ) {
                    for (var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 5, r = [], n = [], h = 0, index = 0; index < pathCommands .length; index++) {
                        var cmd = pathCommands [index];
                        // MOVE
                        if ("M" == cmd.type) {
                            var s = {
                                x: cmd.x,
                                y: cmd.y,
                                z: 1
                            };
                            h = appendToArrayOr(n, s) ? h : h + 1
                        }
                        //LINETO
                        if ("L" == cmd.type) {
                            var l = {
                                x: cmd.x,
                                y: cmd.y,
                                z: 1
                            };
                            h = appendToArrayOr(n, l) ? h : h + 1
                        }
                        //QUADRATICBEZIER
                        if ("Q" == cmd.type)
                            for (var p = sampleQuadraticBezier (n[n.length - 1], {
                                    x: cmd.x1,
                                    y: cmd.y1
                                }, {
                                    x: cmd.x,
                                    y: cmd.y
                                }, t), d = 0; d < p.length; d++) h = appendToArrayOr(n, p[d]) ? h : h + 1;
                        //CLOSEPATH 
                        if ("Z" == cmd.type) {
                            var b = n[0],
                                g = n[n.length - 1];
                            b.z = 0, 999 == g.z && n.pop(), n.length - h > 2 && n.push(b), r.push(n), n = [], h = 0
                        }
                    }
                    return r
                }

                function markCornerPoints (points) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 145,
                        r = arguments.length > 2 ? arguments[2] : void 0;
                    point1 = {
                        x: points[0].x,
                        y: points[0].y,
                        z: 1
                    };
                    for (var n = 1; n < points.length - 1; n++) {
                        var h = {
                                x: points[n].x,
                                y: points[n].y,
                                z: points[n].z
                            },
                            a = {
                                x: points[n + 1].x,
                                y: points[n + 1].y,
                                z: points[n + 1].z
                            },
                            i = calculateAngleBetweenPoints_B([point1.x, point1.y], [h.x, h.y], [a.x, a.y]);
                        (r || 1 == points[n].z) && (points[n].z = i <= t && i > 0 ? 1 : 0), point1 = h
                    }
                    return points
                }

                function normalizeAndCenterLines (linesContainer, isHorizontalAdjustment, flipHorizontal) {
                    var n = linesContainer.lines,
                        h = linesContainer.w,
                        a = linesContainer.h,
                        i = {
                            left: 99999,
                            top: 99999,
                            right: -99999,
                            bottom: -99999,
                            width: 0,
                            height: 0,
                            x0: 0,
                            y0: 0
                        };
                    if (0 == n.length) i = {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0,
                        width: 200,
                        height: 200,
                        x0: 0,
                        y0: 0
                    };
                    else {
                        for (var c = 0; c < n.length; c++)
                            for (var o = n[c], s = 0; s < o.length; s++) {
                                var l = [o[s].x, o[s].y];
                                i.left = Math.min(i.left, l[0]), i.top = Math.min(i.top, l[1]), i.right = Math.max(i.right, l[0]), i.bottom = Math.max(i.bottom, l[1])
                            }
                        i.width = i.right - i.left, i.height = i.bottom - i.top, i.x0 = i.left + i.width / 2, i.y0 = i.top + i.height / 2
                    }
                    for (var p = [], d = 0; d < n.length; d++) {
                        for (var b = n[d], g = [], j = 0; j < b.length; j++) {
                            var x = {
                                x: b[j].x,
                                y: b[j].y,
                                z: b[j].z
                            };
                            isHorizontalAdjustment ? x.x = flipHorizontal ? -x.x + 2 * i.x0 - i.left + 20 : x.x - i.left + 20 : x.y = x.y - i.top + 20, g.push(x)
                        }
                        p.push(g)
                    }
                    isHorizontalAdjustment ? h = i.width + 40 : a = i.height + 40;
                    var V = {
                        lines: p,
                        w: h,
                        h: a
                    };
                    return V
                }

                function layoutAndSimplifyShapes (shapes, markCorners , isHorizontalLayout , simplify , flipHorizontal) {
                    for (var a = 0, i = 0, index = 0; index < shapes.length; index++) {
                        shapes[index] = normalizeAndCenterLines (shapes[index], isHorizontalLayout , flipHorizontal);
                        var o = shapes[index];
                        isHorizontalLayout  ? (a += o.w, i = o.h) : (a = o.w, i += o.h)
                    }
                    for (var p = [], b = -a / 2, g = i / 2, x = 0, V = 0, f = 0; f < shapes.length; f++) {
                        var F = shapes[f],
                            k = F.lines;
                        isHorizontalLayout  || (x = -F.w / 2, b = 0);
                        for (var m = 0; m < k.length; m++) {
                            var P = k[m],
                                u = [],
                                X = {
                                    x: b + P[0].x + x,
                                    y: g - P[0].y + V,
                                    z: 1
                                };
                            if (simplify )
                                if (markCorners ) P = markCornerPoints (P, 135, !1);
                                else {
                                    var N = 1;
                                    while (N < P.length) {
                                        var H = {
                                            x: b + P[N].x + x,
                                            y: g - P[N].y + V,
                                            z: P[N].z
                                        };
                                        distanceBetweenPoints(X, H) < 2 ? P.splice(N, 1) : (N++, X = H)
                                    }
                                    P = markCornerPoints (P, 145, !0)
                                } X = {
                                x: b + P[0].x + x,
                                y: g - P[0].y + V,
                                z: 1
                            }, u.push(X);
                            var z = 1;
                            while (z < P.length - 1) {
                                var Q = {
                                        x: b + P[z].x + x,
                                        y: g - P[z].y + V,
                                        z: P[z].z
                                    },
                                    R = {
                                        x: b + P[z + 1].x + x,
                                        y: g - P[z + 1].y + V,
                                        z: P[z + 1].z
                                    };
                                if (simplify ) {
                                    var v = calculateAngleBetweenPoints_B([X.x, X.y], [Q.x, Q.y], [R.x, R.y]);
                                    if ((0 == v || v > 174) && 0 == Q.z) {
                                        P.splice(z, 1), z > 1 && (z--, u.pop(), X = u[u.length - 1]);
                                        continue
                                    }
                                    if (0 == Q.z && distanceBetweenPoints(u[u.length - 1], Q) < 20) {
                                        P.splice(z, 1), z > 1 && (z--, u.pop(), X = u[u.length - 1]);
                                        continue
                                    }
                                }
                                u.push(Q), X = Q, z++
                            }
                            var I = {
                                x: b + P[P.length - 1].x + x,
                                y: g - P[P.length - 1].y + V,
                                z: 1
                            };
                            u.push(I), p.push([f, u, F.w, F.h])
                        }
                        if (0 == k.length) {
                            var w = [{
                                x: b + F.w / 2 + x,
                                y: 0,
                                z: 0
                            }];
                            p.push([f, w, F.w, F.h])
                        }
                        isHorizontalLayout  ? x += F.w : V -= F.h
                    }
                    return simplify  && !markCorners  && (p = function(e) {
                        for (var t = 0; t < e.length; t++) {
                            var r = e[t][1];
                            if (!(r.length < 4)) {
                                var n = calculateAngleBetweenPoints_B([r[r.length - 2].x, r[r.length - 2].y], [r[0].x, r[0].y], [r[1].x, r[1].y]);
                                if (n > 145 || 0 == n)
                                    for (var h = 1; h < r.length - 1; h++) {
                                        var a = [];
                                        if (1 == r[h].z) {
                                            for (var i = h; i < r.length - 1; i++) a.push(r[i]);
                                            for (var c = 0; c <= h; c++) 0 == c && (r[c].z = 0), a.push(r[c]);
                                            0 != a.length && (e[t][1] = a);
                                            break
                                        }
                                    }
                            }
                        }
                        return e
                    }(p)), p
                }

                function distanceBetweenPoints(pointA, pointB) {
                    var r = Math.pow(pointA.x - pointB.x, 2),
                        n = Math.pow(pointA.y - pointB.y, 2),
                        h = Math.sqrt(r + n);
                    return h
                }

                function markPolylineCorners(polylinePoints) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 145,
                        r = arguments.length > 2 ? arguments[2] : void 0;
                    point1 = {
                        x: polylinePoints[0][0],
                        y: polylinePoints[0][1],
                        z: 1
                    };
                    for (var n = 1; n < polylinePoints.length - 1; n++) {
                        var h = {
                                x: polylinePoints[n][0],
                                y: polylinePoints[n][1],
                                z: polylinePoints[n][3]
                            },
                            a = {
                                x: polylinePoints[n + 1][0],
                                y: polylinePoints[n + 1][1],
                                z: polylinePoints[n + 1][3]
                            },
                            i = calculateAngleBetweenPoints_B([point1.x, point1.y], [h.x, h.y], [a.x, a.y]);
                        (r || 1 == polylinePoints[n][3]) && (polylinePoints[n][3] = i <= t && i > 0 ? 1 : 0), point1 = h
                    }
                    return polylinePoints
                }

                function rotatePolylineToCornerStart(polyLinePoints) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 145;
                    if (polyLinePoints.length < 4) return polyLinePoints;
                    if (polyLinePoints[0][0] != polyLinePoints[polyLinePoints.length - 1][0] || polyLinePoints[0][1] != polyLinePoints[polyLinePoints.length - 1][1]) return polyLinePoints;
                    var r = calculateAngleBetweenPoints_B([polyLinePoints[polyLinePoints.length - 2][0], polyLinePoints[polyLinePoints.length - 2][1]], [polyLinePoints[0][0], polyLinePoints[0][1]], [polyLinePoints[1][0], polyLinePoints[1][1]]);
                    if (r > t || 0 == r)
                        for (var n = 1; n < polyLinePoints.length - 1; n++) {
                            var h = [];
                            if (1 == polyLinePoints[n][3]) {
                                for (var a = n; a < polyLinePoints.length - 1; a++) a == n ? h.push([polyLinePoints[a][0], polyLinePoints[a][1], 0, polyLinePoints[a][3]]) : h.push(polyLinePoints[a]);
                                for (var i = 0; i <= n; i++) 0 == i && (polyLinePoints[i][3] = 0, polyLinePoints[i][2] = polyLinePoints[i + 1][2]), h.push(polyLinePoints[i]);
                                if (0 != h.length) return h;
                                break
                            }
                        }
                    return polyLinePoints
                }

                function calculateAngleBetweenPoints_B(e, point, r) {
                    var n = {
                            x: e[0] - point[0],
                            y: e[1] - point[1]
                        },
                        h = {
                            x: r[0] - point[0],
                            y: r[1] - point[1]
                        },
                        dotProduct = n.x * h.x + n.y * h.y,
                        i = Math.sqrt(Math.pow(n.x, 2) + Math.pow(n.y, 2)),
                        c = Math.sqrt(Math.pow(h.x, 2) + Math.pow(h.y, 2));
                    if (0 == i || 0 == c) return 0;
                    var o = Math.acos(dotProduct / (i * c)),
                        s = 180 * o / Math.PI;
                    return s
                }

                function isArabic(e) {
                    return /[\u0600-\u06FF\uFE80-\uFEFF]/.test(e)
                }

                function searchArabic(e) {
                    if ("" == e) return !1;
                    for (var t = 0; t < e.length; t++)
                        if (isArabic(e[t])) return !0;
                    return !1
                }

                function reverseWithArabicSupport(input) {
                    for (var t = "", r = "", n = 0, h = 0; h < input.length; h++) {
                        var a = input[h];
                        isArabic(a) ? (0 == n && (t = r + t, r = ""), n = 1, r += a) : " " == a ? (t = t + r + a, r = "", n = 0) : (1 == n && (t = r + t, r = ""), n = 0, r = a + r)
                    }
                    return "" != r && (t += r), t = t.split("").reverse().join(""), t
                }

                function transformPolylinesForVerticalMirroring(inputPolylines, unused , width, height) {
                    for (var h = [], a = [], i = 0; i < inputPolylines.length; i++) {
                        for (var c = [], o = [], s = 0; s < inputPolylines[i].length; s++) {
                            var l = inputPolylines[i][s];
                            c.push({
                                x: l.y,
                                y: -l.x + width / 2 + .4 * height,
                                z: l.z
                            }), o.push({
                                x: -l.y,
                                y: -l.x + width / 2 + .4 * height,
                                z: l.z
                            })
                        }
                        h.push(c), a.push(o)
                    }
                    return {
                        newLinesUp: h,
                        newLinesDown: a
                    }
                }

                function getTextLines(fontLoader, fontDataBase64, text) {
                    var numberOfSegements = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : 5,
                        generateMirrorLines = arguments.length > 4 && void 0 !== arguments[4] && arguments[4];
                    try {
                        var c = 400,
                            inputText = text,
                            l = searchArabic(inputText);
                        l && (inputText = arabicHelper.convertArabic(inputText), inputText = reverseWithArabicSupport(inputText));
                        var fontBinaryData  = new Uint8Array(uni.base64ToArrayBuffer(fontDataBase64)),
                            d = [],
                            b = [],
                            g = [],
                            j = "";
                            //https://github.com/opentypejs/opentype.js/tree/master/test
                        return fontLoader.load(fontBinaryData , (function(e, loadedFontOpentype) {
                            if (e) t("log", "\u52a0\u8f7d\u5b57\u4f53\u5f02\u5e38: " + e, " at utils/TextLine.js:496");
                            else
                                for (var index = 0; index < inputText.length; index++) {
                                    var letter = inputText[index],
                                        gylph = loadedFontOpentype.charToGlyph(letter),
                                        p = c * loadedFontOpentype.ascender / (loadedFontOpentype.ascender - loadedFontOpentype.descender),
                                        gylphPath = gylph.getPath(0, p, c),
                                        V = gylphPath.getBoundingBox(),
                                        f = Math.abs(V.y1) + Math.abs(V.y2),
                                        k = Math.abs(V.x1) + Math.abs(V.x2);
                                    k = 0 == k ? c / 2 : k, f = 0 == f ? c : 1.1 * f;
                                    var m = [];
                                    if (" " != letter && (0 != gylph.index || 0 != gylph.unicodes.length)) {
                                        var gylphCommands = gylphPath.commands;
                                        m = parsePathCommands (gylphCommands, numberOfSegements)
                                    }
                                    if (0 == m.length && (j += letter), generateMirrorLines) {
                                        var u = transformPolylinesForVerticalMirroring(m, 0, k, c);
                                        b.push({
                                            lines: u.newLinesUp,
                                            w: k,
                                            h: f
                                        }), g.push({
                                            lines: u.newLinesDown,
                                            w: k,
                                            h: f
                                        })
                                    }
                                    d.push({
                                        lines: m,
                                        w: k,
                                        h: f
                                    })
                                }
                        }), {
                            isUrl: !1
                        }), {
                            linesArr: d,
                            linesArrUp: b,
                            linesArrDown: g,
                            notRec: j,
                            hasArb: l
                        }
                    } catch (x) {
                        t("log", "\u5f02\u5e38:" + x.message, " at utils/TextLine.js:528")
                    }
                }

                function m(e, t, r) {
                    for (var n = t / 800, h = [], a = [], i = 0, c = 0, o = 99999, s = 99999, l = 0; l < e.length; l++) {
                        var p = e[l],
                            d = [p[0] * n, p[1] * n, p[2], p[3]];
                        i < d[0] && (i = d[0]), c < d[1] && (c = d[1]), o > d[0] && (o = d[0]), s > d[1] && (s = d[1])
                    }
                    var b = -o,
                        g = i - o + .1 * t,
                        j = t;
                    r || (g = t, b = t / 2 - ((i - o) / 2 + o), j = c - s + .1 * t);
                    for (var x = 0; x < e.length; x++) {
                        var V = e[x],
                            f = [V[0] * n, V[1] * n, V[2], V[3]];
                        0 == f[2] && a.length > 0 && (h.push(a), a = []), a.push({
                            x: f[0] + b,
                            y: j / 2 - f[1],
                            z: f[3]
                        })
                    }
                    return a.length > 0 && h.push(a), {
                        lines: h,
                        w: g,
                        h: j
                    }
                }

                function P(e) {
                    var t = e[0],
                        r = t.charCodeAt(0),
                        n = r.toString(16);
                    return n.toLowerCase()
                }

                function u(e) {
                    var t = e % 10,
                        r = function(e) {
                            var t, r, n;
                            return e < 4 ? (t = e, r = 0, n = 1) : e < 7 ? (t = e - 3, r = 1, n = 0) : (t = e - 6, r = 1, n = 1), [t, r, n]
                        }(t),
                        h = spreadToArrayHelper(r, 3),
                        a = h[0],
                        i = h[1],
                        c = h[2],
                        o = Math.pow(10, a + 1),
                        s = e % o,
                        l = Math.floor((e - s) / o - 400);
                    return s = Math.floor((s - t) / 10 - 400), [s, l, i, c]
                }
                var X = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+/";

                function N(e) {
                    for (var t = 0, r = 0; r < e.length; r++) t = 64 * t + X.indexOf(e.charAt(r));
                    return t
                }

                function H(e) {
                    for (var t = [], r = function(e) {
                            for (var t = e.split(","), r = [], n = 0; n < t.length; n++) {
                                var h = N(t[n]);
                                r.push(h)
                            }
                            return r
                        }(e), n = 0; n < r.length; n++) {
                        var h = u(r[n]);
                        t.push(h)
                    }
                    return t
                }

                function z(e) {
                    for (var t = [], r = [], h = 0; h < e.length; h++) {
                        var a = spreadToArrayHelper(e[h], 4),
                            i = a[0],
                            c = a[1],
                            o = a[2],
                            s = a[3];
                        t.push([-c, i, o, s]), r.push([c, i, o, s])
                    }
                    return {
                        xysUp: t,
                        xysDown: r
                    }
                }

                function Q(e, r) {
                    for (var n = arguments.length > 2 && void 0 !== arguments[2] && arguments[2], h = !(arguments.length > 3 && void 0 !== arguments[3]) || arguments[3], a = arguments.length > 4 && void 0 !== arguments[4] && arguments[4], i = 400, c = [], o = [], s = [], l = "", p = 0; p < r.length; p++) {
                        var d = P(r[p]),
                            b = [],
                            g = [],
                            j = [],
                            x = i / 3,
                            V = i,
                            f = i,
                            F = i / 3;
                        if (d in e) {
                            var k = e[d];
                            if (n && (k = H(k)), a) {
                                t("log", "xysVer", JSON.stringify(k), " at utils/TextLine.js:680");
                                var u = JSON.parse(JSON.stringify(k));
                                u = z(u);
                                var X = m(u.xysUp, i, h);
                                g = X.lines, f = X.w, F = X.h;
                                var N = m(u.xysDown, i, h);
                                j = N.lines
                            }
                            var Q = m(k, i, h);
                            b = Q.lines, x = Q.w, V = Q.h
                        } else l += r[p];
                        c.push({
                            lines: b,
                            w: x,
                            h: V
                        }), o.push({
                            lines: g,
                            w: f,
                            h: F
                        }), s.push({
                            lines: j,
                            w: f,
                            h: F
                        })
                    }
                    return {
                        linesArr: c,
                        linesArrUp: o,
                        linesArrDown: s,
                        notRec: l
                    }
                }
                e.exports = {
                    getTextLines: getTextLines,
                    getXXYY: function(e, t, r, n) {
                        var h = !(arguments.length > 4 && void 0 !== arguments[4]) || arguments[4],
                            a = arguments.length > 5 && void 0 !== arguments[5] ? arguments[5] : 5,
                            i = {},
                            c = [],
                            o = [],
                            s = [],
                            l = [],
                            d = [];
                        if (1 == t.mode) i = getTextLines(e, t.data, r, a, n), c = layoutAndSimplifyShapes (i.linesArr, !1, h, !0, !1), s = layoutAndSimplifyShapes (i.linesArrUp, !1, h, !0, !1), l = layoutAndSimplifyShapes (i.linesArrDown, !1, h, !0, !1), d = JSON.parse(JSON.stringify(i.linesArr)), d.reverse(), o = layoutAndSimplifyShapes (d, !1, h, !0, !0);
                        else {
                            if (2 != t.mode) return {
                                xxyy: [],
                                notRec: "",
                                XxyyRight: [],
                                xxyyUp: [],
                                xxyyDown: l
                            };
                            i = Q(t.data, r, !0, h, n), c = layoutAndSimplifyShapes (i.linesArr, !0, h, !0, !1), s = layoutAndSimplifyShapes (i.linesArrUp, !0, h, !0, !1), l = layoutAndSimplifyShapes (i.linesArrDown, !0, h, !0, !1), d = JSON.parse(JSON.stringify(i.linesArr)), d.reverse(), o = layoutAndSimplifyShapes (d, !0, h, !0, !0)
                        }
                        return {
                            xxyy: c,
                            notRec: i.notRec,
                            xxyyRight: o,
                            xxyyUp: s,
                            xxyyDown: l
                        }
                    },
                    dealObjLines: function(e) {
                        for (var t = !(arguments.length > 1 && void 0 !== arguments[1]) || arguments[1], r = 20, n = [], h = [], a = {
                                left: 99999,
                                top: -99999,
                                right: -99999,
                                bottom: 99999
                            }, i = e, c = 0; c < i.length; c++) {
                            var o = [i[c][0], i[c][1]];
                            a.left = Math.min(a.left, o[0]), a.top = Math.max(a.top, o[1]), a.right = Math.max(a.right, o[0]), a.bottom = Math.min(a.bottom, o[1])
                        }
                        for (var s = (a.right - a.left) / 2 + a.left, l = (a.top - a.bottom) / 2 - a.top, p = 0; p < e.length; p++) {
                            var d = e[p],
                                b = d[3];
                            if (t && 0 != h.length && 0 != d[2] && p < e.length - 1) {
                                var g = e[p + 1];
                                if (0 != g[2]) {
                                    var x = calculateAngleBetweenPoints_B(h, d, g);
                                    if (0 == x || x >= 166) continue;
                                    if (b = x <= 145 ? 1 : 0, 0 == b && Math.abs(d[d.length - 1][0] - d[0]) + Math.abs(d[d.length - 1][1] - d[1]) < r) continue
                                }
                            }
                            n.push([d[0] + s, d[1] + l, d[2], b]), h = d
                        }
                        return n
                    },
                    dealImgLines: function(e) {
                        for (var t = [], r = 0; r < e.length; r++) {
                            var n = markPolylineCorners(e[r], 135, !1);
                            n = rotatePolylineToCornerStart(n, 135), t.push.apply(t, arrayConversionHelper(n))
                        }
                        return t
                    },
                    printXXYY: function(e, r) {
                        for (var n = "", h = -1, a = 0; a < e.length; a++) {
                            var i = e[a][1];
                            h != e[a][0] && ("" != n && t("log", h, "printXXYY", n, " at utils/TextLine.js:713"), h = e[a][0], n = "");
                            for (var c = 0; c < i.length; c++) {
                                var o = i[c],
                                    s = (2 * o.x).toFixed(0) + " * 1",
                                    l = (2 * o.y).toFixed(0) + " * 1",
                                    p = r,
                                    d = o.z;
                                0 == c && (p = 0, d = 1), n = n + "\n{" + s.padStart(8, " ") + "," + l.padStart(8, " ") + ", " + p + ", " + d + "},"
                            }
                        }
                        t("log", h, "printXXYY", n, " at utils/TextLine.js:731")
                    }
                }
            }).call(this, r("enhancedConsoleLogger")["default"])
        },
  

        "spreadToArrayHelper": function(e, t, r) {
            var n = r("arrayIfArrayHelper"),
                h = r("iterableToArrayLimitHelper"),
                a = r("toConsumableArrayHelper"),
                i = r("nonIterableDestructuringErrorHelper");
            e.exports = function(e, t) {
                return n(e) || h(e, t) || a(e, t) || i()
            }, e.exports.__esModule = !0, e.exports["default"] = e.exports
        },
        


        arrayConversionHelper: function(e, t, r) {
            var n = r("arrayToArrayLikeHelper"),
                h = r("b893"),
                a = r("toConsumableArrayHelper"),
                i = r("nonIterableSpreadErrorHelper");
            e.exports = function(e) {
                return n(e) || h(e) || a(e) || i()
            }, e.exports.__esModule = !0, e.exports["default"] = e.exports
        },
	[
        ["mainAppEntry", "app-config"]
    ]
]);