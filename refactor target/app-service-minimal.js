(this["webpackJsonp"] = this["webpackJsonp"] || []).push([

    ["app-service"], {
		
		
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
                                ].sort((a, b) => a.tag - b.tag),
                                features: deviceFeatures,
                                deviceOn: false,
                                prjIndex: -1,
                                cnnDevice: "Not connected",
                                cnnState: false,
                                randomCheck: [],
                                initShow: false,
                                // represents the configuration for X and Y axis adjustments, 
                                // likely for a device or UI component that allows fine-tuning of two-dimensional positioning.
                                // This configuration is used throughout the code to manage, display, 
                                //  and update the state of X and Y axis controls, including rendering UI elements and sending commands to the device.
                                xyCnf: {
                                    // boolean indicating whether automatic adjustment is enabled.
                                    auto: !0,
                                    // a numeric value related to automatic adjustment.
                                    autoValue: 0,
                                    // numeric value, possibly representing a phase or mode for the adjustment.
                                    phase: 0,
                                    xy: [
                                        {
                                            title: "X-axis coarse tuning",
                                            name: "xBig",
                                            value: 0
                                        },
                                        {
                                            title: "X-axis fine tuning",
                                            name: "xSmall",
                                            value: 0
                                        },
                                        {
                                            title: "Y-axis coarse tuning",
                                            name: "yBig",
                                            value: 0
                                        },
                                        {
                                            title: "Y-axis fine tuning",
                                            name: "ySmall",
                                            value: 0
                                        }
                                    ]
                                }
                            }
                        },
                        created: function() {
                            app.globalData.setMainPage(this);
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
                                bleDeviceController.setCmdData("E0E1E2E3B0B1B2B300B4B5B6B7C0C1C2C30400098080800080003309FFFFFF320000000000000000000000000000000000000000000000000000000000000000000000000000FF035393C06600000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001000A00FFFFFF020000000000000004050607D0D1D2D3820000FF28000000000000000000003200FF00FF28000000000000000000FF3200FFD4D5D6D7F0F1F2F300000000070102030405060700004466F4F5F6F743E3A317F0000000E4E5E6E7"), this.prjIndex = app.globalData.cmd.curMode
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

                            //  ensures that shake commands are sent only when the device is ready, 
                            // and retries if a previous command is still being processed. 
                            // It prevents overlapping sends and manages timing for reliable communication.
                            sendCmd2: function(xyConf) {
                                if (app.globalData.blu_data_cmdSending) {
                                    if (null == this.sendTimer) {
                                        var r = this;
                                        this.sendTimer = setTimeout((function() {
                                            r.sendTimer = null, r.sendCmd2(xyConf)
                                        }), 100)
                                    }
                                } else if (!(this.lastCmdTime < this.lastSendTime)) {
                                    var n = app.globalData.getDeviceFeatures(),
                                        command = deviceCommandUtil.getShakeCmdStr(app.globalData.cmd, {
                                            features: n,
                                            xyCnfSave: xyConf
                                        }),
                                        i = bleDeviceController.gosend(!1, command);
                                    i && (this.lastSendTime = (new Date).getTime())
                                }
                            },
                            sendLastCmd: function(e) {
                                this.lastCmdTime = (new Date).getTime(), this.sendCmd2(e)
                            },
             
                        }
                    };
                t.default = module
            }).call(this, r("enhancedConsoleLogger")["default"])
        },

        "appStateManager": function(e, t, r) {
            "use strict";
            (function(logger) {

                var r = {
                    globalData: {
                        $i18n: {
                            locale: "en-US"
                        },
                        $t: {},
                        MaxSaveFileCount: 50,
                        MaxListCount: 200,
                        mainPage: null,
                        cloudApi: null,

                        // List of Bluetooth service UUIDs to connect to
                        mserviceuuids: [],
                        // List of Bluetooth characteristic UUIDs for transmitting (TX) data
                        mtxduuids: [],
                        // List of Bluetooth characteristic UUIDs for receiving (RX) data
                        mrxduuids: [],
                        // 0,1,2
                        muuidSel: 0,
                        img_selecting: !1,
                        bleOpenCloseCount: 0,
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
                        screen_width_str: "0px",
                        screen_width_float: 0,
                        screen_width_page: 0,
                        screen_height_page: 0,
                        blu_data_canSend: !1,
                        blu_data_cmdSending: !1,
                        blu_data_lastShowTime: 0,
                        blu_Discovery_lastTime: 0,
                        blu_data_send_interval: 100,
                        deviceInfo: {},
                        rtl: !1,
                        langs: {
                            "zh-Hans": "Chinese",
                            en: "English"
                        },
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
                            prjData: {
                                prjIndex: 0,
                                public: {
                                    rdMode: 0,
                                    runSpeed: 50,
                                    txColor: 9,
                                    soundVal: 20
                                },
                                prjItem: {
                                    2: {
                                        pyMode: 0,
                                        prjSelected: [0, 0, 0, 0],
                                        ckValues: []
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
                            textData: {
                                refresh: !1,
                                verTag: 0,
                                runDir: 0,
                                arrColor: ["red", "green", "blue", "yellow", "#00FFFF", "purple", "white"],
                                txPointTime: 50,
                                txColor: 9,
                                txSize: 50,
                                txDist: 50,
                                runSpeed: 50,
                                groupIdex: 0,
                                groupList: [{
                                    text: "",
                                    update: 0,
                                    color: 9,
                                    fontIdex: null,
                                    time: 5,
                                    xys: [],
                                    XysRight: [],
                                    XysUp: [],
                                    XysDown: []
                                }]
                            },
                            drawData: {
                                pisObj: {
                                    txPointTime: 50,
                                    cnfValus: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                                }
                            },
                            pgsData: {
                                pisList: []
                            },
                            subsetData: {
                                xyCnf: {
                                    auto: !0,
                                    autoValue: 0,
                                    phase: 0,
                                    xy: [{
                                        title: "X-axis coarse tuning",
                                        name: "xBig",
                                        value: 0
                                    }, {
                                        title: "X-axis fine tuning",
                                        name: "xSmall",
                                        value: 0
                                    }, {
                                        title: "Y-axis coarse tuning",
                                        name: "yBig",
                                        value: 0
                                    }, {
                                        title: "Y-axis fine tuning",
                                        name: "ySmall",
                                        value: 0
                                    }]
                                }
                            }
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

                        // restores the last used Bluetooth UUID configuration by reading a saved index and updating the relevant UUID arrays for device communication.
                        readSetting: function() {
                            switch (this.muuidSel = this.readData("lastsel") || 0, this.muuidSel) {
                                case 0:
                                    this.mserviceuuids = ["0000FF00-0000-1000-8000-00805F9B34FB"], this.mtxduuids = ["0000FF02-0000-1000-8000-00805F9B34FB"], this.mrxduuids = "0000FF01-0000-1000-8000-00805F9B34FB";
                                    break;
                                case 1:
                                    this.mserviceuuids = ["0000FFE0-0000-1000-8000-00805F9B34FB"], this.mtxduuids = ["0000FFE1-0000-1000-8000-00805F9B34FB"], this.mrxduuids = ["0000FFE1-0000-1000-8000-00805F9B34FB"];
                                    break;
                                case 2:
                                    this.mserviceuuids = ["0000FF00-0000-1000-8000-00805F9B34FB"], this.mtxduuids = ["0000FF02-0000-1000-8000-00805F9B34FB"], this.mrxduuids = "0000FF01-0000-1000-8000-00805F9B34FB";
                                    break
                            }
                        },

                        saveTipsParm: function(e) {
                            var t = e ? "0" : "1";
                            this.saveData("tips", t)
                        },
                        getTipsParm: function() {
                            var e = this.readData("tips");
                            return "1" != e
                        },
                        setDeviceInfo: function(deviceType, version, userType) {
                            this.saveData("deviceType", deviceType), 
                            this.saveData("version", version), 
                            this.saveData("userType", userType), 
                            this.deviceInfo["deviceType"] = deviceType, 
                            this.deviceInfo["version"] = version, 
                            this.deviceInfo["userType"] = userType
                        },
                        getDeviceInfo: function() {
                            var e = this.readData("deviceType");
                            "" == e && (e = 0);
                            var t = this.readData("version");
                            "" == t && (t = 0);
                            var r = this.readData("userType");
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
                                    textStopTime: !1,
                                    textDecimalTime: !1,
                                    displayType: 0,
                                    showOutDoorTips: !1,
                                    xyCnf: !1,
                                    arbPlay: !1,
                                    ilda: !1,
                                    // device supports TTL analog
                                    ttlAn: !1,
                                    picsPlay: !1,
                                    textUpDown: !1,
                                    animationFix: !1
                                },
                                deviceType = this.deviceInfo["deviceType"],
                                version = this.deviceInfo["version"];
                            // Determine device features based on deviceType and version
                            // Each feature is enabled according to specific deviceType/version rules
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
                        saveData: function(e, t) {
                            uni.setStorageSync(e, t)
                        },
                        readData: function(e) {
                            var t = uni.getStorageSync(e);
                            return t
                        },
                        deleteData: function(e) {
                            uni.removeStorageSync(e)
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

                        setMainPage: function(mainPageComponent) {
                            this.mainPage = mainPageComponent
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
                                logger("log", "this.bleOpenCloseCount", this.bleOpenCloseCount, " at App.vue:440"), this.bleOpenCloseCount++;
                                var n = this;
                                uni.openBluetoothAdapter({
                                    success: function(e) {
                                        t.BluetoothAdapterOpen = !0, t.setBLEConnectionStateChange(), callbackFunc && callbackFunc(!0)
                                    },
                                    fail: function(h) {
                                        logger("log", "openBluetoothAdapter2", h, " at App.vue:450"), t.doCloseBluetoothAdapter(), 10001 === h.errCode && t.showModalTips(n.$t("\u8bf7\u68c0\u67e5\u624b\u673aBluetooth\u662f\u5426\u542f\u7528"), !0), 103 == h.errno ? t.showModalTips(n.$t("\u8bf7Settings\u5c0f\u7a0b\u5e8fBluetooth\u6743\u9650"), !0) : t.showModalTips("Open Bluetooth Adapter Fail"), callbackFunc && callbackFunc(!1)
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
                        getSysinfo: function() {
                            var sysinfo = uni.getSystemInfoSync();
                            this.screen_width_page = sysinfo.screenWidth;
                            var n = Math.min(9 * sysinfo.screenHeight / 16, sysinfo.screenWidth);
                            sysinfo.devicePixelRatio;
                            this.screen_width_float = n / 750, 
                            this.screen_width_str = this.screen_width_float + "px", 
                            this.screen_height_page = sysinfo.safeArea.height
                        },
                        t: function(t) {
                            return logger("log", "app vue $t", t, this.$t(t), " at App.vue:505"), this.$t(t)
                        }
                    },
                    onLaunch: function() {
                        this.globalData.getDeviceInfo(),
                        this.globalData.getSysinfo();
                    },
                    onShow: function() {
                        this.globalData.blu_connected || null != this.globalData.mainPage && this.globalData.mainPage.gotoMain(true)
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

        enhancedConsoleLogger: function(t, r, n) {
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