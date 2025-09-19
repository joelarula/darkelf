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
                                        prjSelected: [0, 0, 0, 0],
                                        ckValues: [] // selected items
                                    },
                                    3: {
                                        pyMode: 0,
                                        prjSelected: [0, 0, 0, 0],
                                        ckValues: []
                                    },
                                    5: {
                                        pyMode: 0,
                                        prjSelected: [0, 0, 0, 0],
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
                                    rdMode: 0,
                                    runSpeed: 10,
                                    soundVal: 20
                                },
                                item: {
                                    pyMode: 0,
                                    prjSelected: [0, 0, 0, 0],
                                    ckValues: []
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
                var n = r("arrayConversionHelper");

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
                    var r, a = n(t),
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
                        var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null,
                            r = toFixedWidthHex(commandConfig.curMode, 2),
                            h = toFixedWidthHex(0, 2),
                            a = toFixedWidthHex(commandConfig.textData.txColor, 2),
                            c = toFixedWidthHex(commandConfig.textData.txSize / 100 * 255, 2),
                            o = toFixedWidthHex(commandConfig.textData.txSize / 100 * 255, 2),
                            s = toFixedWidthHex(commandConfig.textData.runSpeed / 100 * 255, 2),
                            l = "00",
                            p = toFixedWidthHex(commandConfig.textData.txDist / 100 * 255, 2),
                            d = toFixedWidthHex(commandConfig.prjData.public.rdMode, 2),
                            j = toFixedWidthHex(commandConfig.prjData.public.soundVal / 100 * 255, 2),
                            x = "ffffffff0000";
                        if (null != t) {
                            if (x = "", t.hasOwnProperty("groupList"))
                                for (var V = 0; V < t.groupList.length; V++) x += toFixedWidthHex(t.groupList[V].color, 2);
                            x += "ffffffff", x = x.substring(0, 8), getFeatureValue(t, "textStopTime") && (x += toFixedWidthHex(commandConfig.textData.txPointTime, 2)), x += "0000", x = x.substring(0, 12)
                        }
                        var f = "",
                            F = commandConfig.prjData.prjItem;
                        for (var k in F) {
                            var m = F[k],
                                P = 0 == m.pyMode ? 0 : 128;
                            0 != P && null != t && t.hasOwnProperty("prjParm") && t.prjParm.prjIndex == k && (3 == k && getFeatureValue(t, "animationFix") && [2, 4, 11, 13, 19].includes(t.prjParm.selIndex) ? P |= 50 - t.prjParm.selIndex : P |= t.prjParm.selIndex);
                            var u = toFixedWidthHex(P, 2),
                                X = "",
                                N = n(m.prjSelected);
                            3 == k && getFeatureValue(t, "animationFix") && (N = applyBitmaskUpdates([2, 4, 11, 13, 19], N));
                            for (var H = 0; H < N.length; H++) X = toFixedWidthHex(N[H]) + X;
                            f = f + u + X
                        }
                        var z = "";
                        getFeatureValue(t, "arbPlay") && (z += toFixedWidthHex(commandConfig.textData.runDir, 2));
                        for (var Q = "", R = Math.floor(z.length / 2), v = R; v < 44; v++) Q += "00";
                        var I = "c0c1c2c3" + r + h + a + c + o + s + l + p + d + j + x + f + z + Q + "c4c5c6c7";
                        return I.toUpperCase()
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
                        var t = toFixedWidthHex(settingData.valArr[0]),
                            r = toFixedWidthHex(settingData.ch, 2),
                            n = toFixedWidthHex(settingData.valArr[1], 2),
                            h = toFixedWidthHex(settingData.xy, 2),
                            a = toFixedWidthHex(settingData.valArr[2], 2),
                            c = toFixedWidthHex(settingData.valArr[3], 2),
                            o = toFixedWidthHex(settingData.valArr[4], 2),
                            s = toFixedWidthHex(settingData.light, 2),
                            l = toFixedWidthHex(settingData.cfg, 2);
                        0 == settingData.cfg && (a = "FF", c = "FF", o = "FF");
                        var p = toFixedWidthHex(settingData.lang, 2),
                            d = "00010203" + t + r + n + h + a + c + o + s + l + p + "000000000004050607";
                        return d.toUpperCase()
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
                                    return 0 == hexData.length || (t("log", "Simulate sending ------- 20ms", appStateManager.globalData.blu_data_cmdSending, " at utils/bluCtrl.js:552"), !appStateManager.globalData.blu_data_cmdSending && (appStateManager.globalData.blu_data_cmdSending = !0, setTimeout((function() {
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
                        t("log", "Device Returned data", deviceResponseData , " at utils/bluCtrl.js:21"), deviceCommandUtils.getCmdValue("B0B1B2B3", "B4B5B6B7", deviceResponseData );
                        var mainCommandData  = deviceCommandUtils.getCmdValue("C0C1C2C3", "C4C5C6C7", deviceResponseData );
                        appStateManager.globalData.cmd.curMode = clampOrDefault(extractHexValue (1, 1, mainCommandData ), 0, 12, 0), appStateManager.globalData.cmd.prjData.prjIndex = clampOrDefault(extractHexValue (1, 1, mainCommandData ), 0, 12, 0), appStateManager.globalData.cmd.prjData.public.txColor = clampOrDefault(extractHexValue (3, 1, mainCommandData ), 0, 9, 0), appStateManager.globalData.cmd.textData.txColor = appStateManager.globalData.cmd.prjData.public.txColor, appStateManager.globalData.cmd.textData.txSize = clampOrDefault(Math.round(extractHexValue (4, 1, mainCommandData ) / 255 * 100), 10, 100, 60), appStateManager.globalData.cmd.textData.runSpeed = clampOrDefault(Math.round(extractHexValue (6, 1, mainCommandData ) / 255 * 100), 0, 255, 128), appStateManager.globalData.cmd.prjData.public.runSpeed = appStateManager.globalData.cmd.textData.runSpeed, appStateManager.globalData.cmd.textData.txDist = clampOrDefault(Math.round(extractHexValue (8, 1, mainCommandData ) / 255 * 100), 10, 100, 60), appStateManager.globalData.cmd.prjData.public.rdMode = clampOrDefault(extractHexValue (9, 1, mainCommandData ), 0, 255, 0), appStateManager.globalData.cmd.prjData.public.soundVal = clampOrDefault(Math.round(extractHexValue (10, 1, mainCommandData ) / 255 * 100), 0, 255, 0), appStateManager.globalData.cmd.textData.txPointTime = clampOrDefault(extractHexValue (15, 1, mainCommandData ), 0, 100, 50), appStateManager.globalData.cmd.drawData.pisObj.txPointTime = clampOrDefault(extractHexValue (16, 1, mainCommandData ), 0, 100, 50), appStateManager.globalData.cmd.textData.refresh = !0;
                        var projectItems = appStateManager.globalData.cmd.prjData.prjItem,
                            projectItemStartIndex  = 17;
                        for (var itemKey in projectItems) {
                            var projectItem = projectItems[itemKey];
                            projectItem.pyMode = clampOrDefault(extractHexValue (projectItemStartIndex , 1, mainCommandData ), 0, 255, 0), projectItem.prjSelected[3] = extractHexValue (projectItemStartIndex  + 1, 2, mainCommandData ), projectItem.prjSelected[2] = extractHexValue (projectItemStartIndex  + 3, 2, mainCommandData ), projectItem.prjSelected[1] = extractHexValue (projectItemStartIndex  + 5, 2, mainCommandData ), projectItem.prjSelected[0] = extractHexValue (projectItemStartIndex  + 7, 2, mainCommandData ), projectItemStartIndex  += 9
                        }
                        appStateManager.globalData.cmd.textData.runDir = clampOrDefault(extractHexValue (projectItemStartIndex , 1, mainCommandData ), 0, 255, 0), projectItemStartIndex  += 1;
                        for (var p = appStateManager.globalData.cmd.subsetData, d = 0; d < 6; d++) 0 == d ? p.xyCnf.auto = p.xyCnf.autoValue == clampOrDefault(extractHexValue (projectItemStartIndex  + d, 1, mainCommandData ), 0, 255, 0) : 1 == d ? p.xyCnf.phase = clampOrDefault(extractHexValue (projectItemStartIndex  + d, 1, mainCommandData ), 0, 255, 0) : p.xyCnf.xy[d - 2].value = clampOrDefault(extractHexValue (projectItemStartIndex  + d, 1, mainCommandData ), 0, 255, 0);
                        var settingCommandData = deviceCommandUtils.getCmdValue("00010203", "04050607", deviceResponseData );
                        appStateManager.globalData.cmd.settingData.valArr[0] = clampOrDefault(extractHexValue (1, 2, settingCommandData), 1, 512, 1), appStateManager.globalData.cmd.settingData.ch = extractHexValue (3, 1, settingCommandData), appStateManager.globalData.cmd.settingData.valArr[1] = clampOrDefault(extractHexValue (4, 1, settingCommandData), 10, 100, 10), appStateManager.globalData.cmd.settingData.xy = clampOrDefault(extractHexValue (5, 1, settingCommandData), 0, 7, 0), appStateManager.globalData.cmd.settingData.valArr[2] = clampOrDefault(extractHexValue (6, 1, settingCommandData), 0, 255, 255), appStateManager.globalData.cmd.settingData.valArr[3] = clampOrDefault(extractHexValue (7, 1, settingCommandData), 0, 255, 255), appStateManager.globalData.cmd.settingData.valArr[4] = clampOrDefault(extractHexValue (8, 1, settingCommandData), 0, 255, 255), appStateManager.globalData.cmd.settingData.light = clampOrDefault(extractHexValue (9, 1, settingCommandData), 1, 3, 3), appStateManager.globalData.cmd.settingData.cfg = clampOrDefault(extractHexValue (10, 1, settingCommandData), 0, 255, 0);
                        var featureCommandData = deviceCommandUtils.getCmdValue("D0D1D2D3", "D4D5D6D7", deviceResponseData );
                        if ("" != featureCommandData) {
                            var j = appStateManager.globalData.getDeviceFeatures(),
                                x = 16;
                            t("log", "features", JSON.stringify(j), " at utils/bluCtrl.js:96"), deviceCommandUtils.getFeaturesValue({
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
 
  

        "enhancedConsoleLogger": function(t, r, n) {
            "use strict";
            // Stub logger: provides a default logger and a log method, but does nothing
            function noop() {}
            function defaultLogger() { return noop; }
            n.r && n.r(r);
            n.d && n.d(r, "log", function() { return noop; });
            n.d && n.d(r, "default", function() { return defaultLogger; });
        },
 

        "uni": function (t, r, n) {
            // Stub implementation of the uni module for testing/build purposes
            const uni = {
                getSystemInfoSync: () => ({ statusBarHeight: 20 }),
                getStorageSync: () => undefined,
                setStorageSync: () => { },
                removeStorageSync: () => { },
                reLaunch: () => { },
                navigateTo: () => { },
                getCurrentPages: () => [{ route: 'pages/main/main' }],
            };
            r.default = uni;
        }
			
	},
	[
        ["mainAppEntry", "app-config"]
    ]
]);