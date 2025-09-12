        "0b2a": function(e, t, r) {
            "use strict";
            (function(e) {
                Object.defineProperty(t, "__esModule", {
                    value: !0
                }), t.default = void 0;
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
                        appHide: !1,
                        mserviceuuids: [],
                        mtxduuids: [],
                        mrxduuids: [],
                        muuidSel: 0,
                        platform: {
                            system: "",
                            app: !1
                        },
                        img_selecting: !1,
                        bleOpenCloseCount: 0,
                        bleConnectCount: 0,
                        bleManualDisCnn: !1,
                        BLEConnectionStateChangeSet: !1,
                        BluetoothAdapterOpen: !1,
                        ble_device: null,
                        blu_cnn_from_page: !1,
                        blu_state: 0,
                        blu_connect_stop: !1,
                        blu_connected: 0,
                        blu_readyRec: !1,
                        blu_cnn_call_back: null,
                        blu_rec_call_back: null,
                        blu_rec_content: null,
                        blu_cnn_from_test: !1,
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
                            "zh-Hans": "\u4e2d\u6587",
                            en: "English",
                            fr: "Fran\xe7ais",
                            ru: "\u0420\u0443\u0441\u0441\u043a\u0438\u0439",
                            vi: "Ti\u1ebfng Vi\u1ec7t",
                            ar: "\u0627\u0644\u0639\u0631\u0628\u064a\u0629",
                            de: "Deutsch",
                            it: "Italiano",
                            es: "Espa\xf1ol"
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
                                        title: "X\u7c97\u8c03",
                                        name: "xBig",
                                        value: 0
                                    }, {
                                        title: "X\u7ec6\u8c03",
                                        name: "xSmall",
                                        value: 0
                                    }, {
                                        title: "Y\u7c97\u8c03",
                                        name: "yBig",
                                        value: 0
                                    }, {
                                        title: "Y\u7ec6\u8c03",
                                        name: "ySmall",
                                        value: 0
                                    }]
                                }
                            }
                        },
                        setbluDataSendInterval: function(e) {
                            this.blu_data_send_interval = e
                        },
                        setRecCallBack: function(e) {
                            var t = this.blu_rec_call_back;
                            null != t && t(e)
                        },
                        setBluCnnState: function(e, t) {
                            this.blu_connected = e, 2 == this.blu_connected && this.saveDevice();
                            var r = this.blu_cnn_call_back;
                            null != r && r(e, t)
                        },
                        showModalTips: function(e) {
                            var t = arguments.length > 1 && void 0 !== arguments[1] && arguments[1];
                            t ? uni.showModal({
                                content: e,
                                showCancel: !1
                            }) : uni.showToast({
                                title: e,
                                icon: "none",
                                duration: 1e3
                            })
                        },
                        setCmdMode: function(e) {
                            this.cmd["curMode"] = e, this.cmd["prjData"].prjIndex = e
                        },
                        getCmdData: function(e) {
                            return this.cmd[e]
                        },
                        setCmdData: function(e, t) {
                            if ("prjData" == e) return this.cmd[e].public = t.public, 1 != t.prjIndex && (this.cmd[e].prjItem[t.prjIndex + ""] = t.item), this.cmd.textData.runSpeed = t.public.runSpeed, void(this.cmd.textData.txColor = t.public.txColor);
                            this.cmd[e] = t, "textData" == e && (this.cmd.prjData.public.runSpeed = t.runSpeed, this.cmd.prjData.public.txColor = t.txColor)
                        },
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
                        saveSetting: function(e, t) {
                            this.saveData("autoSendInv", e), this.saveData("sendText", t)
                        },
                        saveTipsParm: function(e) {
                            var t = e ? "0" : "1";
                            this.saveData("tips", t)
                        },
                        getTipsParm: function() {
                            var e = this.readData("tips");
                            return "1" != e
                        },
                        setDeviceInfo: function(e, t, r) {
                            this.saveData("deviceType", e), this.saveData("version", t), this.saveData("userType", r), this.deviceInfo["deviceType"] = e, this.deviceInfo["version"] = t, this.deviceInfo["userType"] = r
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
                        getDeviceFeatures: function() {
                            var e = {
                                    textStopTime: !1,
                                    textDecimalTime: !1,
                                    displayType: 0,
                                    showOutDoorTips: !1,
                                    xyCnf: !1,
                                    arbPlay: !1,
                                    ilda: !1,
                                    ttlAn: !1,
                                    picsPlay: !1,
                                    textUpDown: !1,
                                    animationFix: !1
                                },
                                t = this.deviceInfo["deviceType"],
                                r = this.deviceInfo["version"];
                            return (1 == t && r >= 1 || 0 == t && r >= 2 || t >= 2) && (e.textStopTime = !0, e.textDecimalTime = !0), (1 == t && r >= 2 || t > 1) && (e.showOutDoorTips = !0), 1 == t && 1 == r && (e.textModeFix01 = !0), 2 == t && (e.xyCnf = !0), 1 != t && 2 != t || (e.ilda = !0), 1 != t && 2 != t || (e.ttlAn = !0), (t >= 2 || r >= 3) && (e.arbPlay = !0), (t >= 3 || r >= 4) && (e.textUpDown = !0), (t >= 3 || r >= 5) && (e.picsPlay = !0), 1 == t && (e.animationFix = !0), e.displayType = t, e
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
                        getLang: function() {
                            var e = this.readData("lang");
                            if (!(e in this.langs)) {
                                var t = uni.getLocale();
                                e = t in this.langs ? t : "en"
                            }
                            return this.rtl = -1 != ["ar"].indexOf(e), e
                        },
                        setLang: function(e) {
                            var t = !(arguments.length > 1 && void 0 !== arguments[1]) || arguments[1];
                            t && this.saveData("lang", e), this.$i18n.locale = e, uni.setLocale(e)
                        },
                        savelastsel: function(t) {
                            this.saveData("lastsel", t), e("log", "Writelastsel ", t, " at App.vue:328")
                        },
                        readDevice: function() {
                            this.ble_device = this.readData("device")
                        },
                        saveDevice: function() {
                            this.saveData("device", this.ble_device)
                        },
                        clearDevice: function() {
                            this.ble_device = null, this.saveDevice()
                        },
                        setMainPage: function(e) {
                            this.mainPage = e
                        },
                        createBLEConnection: function(t) {
                            var r = this,
                                n = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null;
                            e("log", "this.bleConnectCount", this.bleConnectCount, " at App.vue:349"), this.bleConnectCount++, this.blu_connected = -1, uni.createBLEConnection({
                                deviceId: t,
                                timeout: 6e3,
                                success: function(e) {
                                    n && n(!0)
                                },
                                fail: function(h) {
                                    e("log", "createBLEConnection fail:", h, r.bleManualDisCnn, " at App.vue:359"), r.bleManualDisCnn ? n && n(!1) : r.doCloseBLEConnection(t, (function(e) {
                                        n && n(!1)
                                    }))
                                }
                            })
                        },
                        doCloseBLEConnection: function(t) {
                            var r = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null;
                            e("log", "doCloseBLEConnection", t, " at App.vue:371"), this.bleManualDisCnn = !0, this.bleConnectCount--;
                            var n = this,
                                h = setTimeout((function() {
                                    h = null, n.bleManualDisCnn = !1, r && r(!0)
                                }), 200);
                            uni.closeBLEConnection({
                                deviceId: t,
                                success: function(t) {
                                    e("log", "doCloseBLEConnection success", t, " at App.vue:384"), h && r && r(!0)
                                },
                                fail: function(t) {
                                    e("log", "doCloseBLEConnection fail", t, " at App.vue:389"), h && r && r(!1)
                                },
                                complete: function() {
                                    e("log", "doCloseBLEConnection complete", " at App.vue:394"), h && clearTimeout(h), this.bleManualDisCnn = !1
                                }
                            })
                        },
                        closeBLEConnection: function() {
                            var t = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            e("log", "closeBLEConnection", this.blu_connected, this.ble_device, " at App.vue:403");
                            if (this.blu_connected) {
                                var r = this.ble_device;
                                r ? this.doCloseBLEConnection(r.deviceId, (function(r) {
                                    e("log", "do callback", " at App.vue:409"), t && t(r)
                                })) : t && t(!0)
                            } else t && t(!0)
                        },
                        doCloseBluetoothAdapter: function() {
                            var t = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            this.bleOpenCloseCount--, this.BluetoothAdapterOpen = !1, uni.closeBluetoothAdapter({
                                success: function(e) {
                                    t && t(!0)
                                },
                                fail: function(r) {
                                    e("log", "closeBluetoothAdapter fail", r, " at App.vue:424"), t && t(!1)
                                }
                            })
                        },
                        closeBluetoothAdapter: function() {
                            var e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            this.BluetoothAdapterOpen ? this.doCloseBluetoothAdapter(e) : e && e(!0)
                        },
                        openBluetoothAdapter: function() {
                            var t = this,
                                r = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            if (this.BluetoothAdapterOpen) r && r(!0);
                            else {
                                e("log", "this.bleOpenCloseCount", this.bleOpenCloseCount, " at App.vue:440"), this.bleOpenCloseCount++;
                                var n = this;
                                uni.openBluetoothAdapter({
                                    success: function(e) {
                                        t.BluetoothAdapterOpen = !0, t.setBLEConnectionStateChange(), r && r(!0)
                                    },
                                    fail: function(h) {
                                        e("log", "openBluetoothAdapter2", h, " at App.vue:450"), t.doCloseBluetoothAdapter(), 10001 === h.errCode && t.showModalTips(n.$t("\u8bf7\u68c0\u67e5\u624b\u673a\u84dd\u7259\u662f\u5426\u542f\u7528"), !0), 103 == h.errno ? t.showModalTips(n.$t("\u8bf7\u8bbe\u7f6e\u5c0f\u7a0b\u5e8f\u84dd\u7259\u6743\u9650"), !0) : t.showModalTips("Open Bluetooth Adapter Fail"), r && r(!1)
                                    }
                                })
                            }
                        },
                        setBLEConnectionStateChange: function() {
                            if (!this.BLEConnectionStateChangeSet) {
                                this.BLEConnectionStateChangeSet = !0;
                                var t = this;
                                uni.onBLEConnectionStateChange((function(r) {
                                    t.blu_data_cmdSending = !1, r.connected || (e("log", "setBLEConnectionStateChange", t.bleManualDisCnn, " at App.vue:471"), t.bleManualDisCnn || t.doCloseBLEConnection(r.deviceId), t.ble_device && t.ble_device.deviceId != r.deviceId || (t.blu_data_canSend = !1, t.setBluCnnState(0, !0)))
                                }))
                            }
                        },
                        getSysinfo: function() {
                            var e = uni.getSystemInfoSync();
                            this.platform.system = e.platform, this.platform.app = !0;
                            var t = plus.runtime.version,
                                r = this.readData("appVersion");
                            t != r && (this.saveData("appVersion", t), plus.runtime.restart()), this.screen_width_page = e.screenWidth;
                            var n = Math.min(9 * e.screenHeight / 16, e.screenWidth);
                            e.devicePixelRatio;
                            this.screen_width_float = n / 750, this.screen_width_str = this.screen_width_float + "px", this.screen_height_page = e.safeArea.height
                        },
                        t: function(t) {
                            return e("log", "app vue $t", t, this.$t(t), " at App.vue:505"), this.$t(t)
                        }
                    },
                    onLaunch: function() {
                        var e = this;
                        this.globalData.$t = function(t) {
                            return e.$t(t)
                        }, this.globalData.$i18n = this.$i18n, this.globalData.deviceInfo = this.globalData.getDeviceInfo(), this.globalData.getSysinfo();
                        var t = this.globalData.getLang();
                        this.globalData.setLang(t, !1)
                    },
                    onShow: function() {
                        this.globalData.appHide = !1, this.globalData.blu_connected || null != this.globalData.mainPage && this.globalData.mainPage.gotoMain(!0)
                    },
                    onHide: function() {
                        var e = this;
                        this.globalData.appHide = !0, this.globalData.img_selecting || this.globalData.closeBLEConnection((function(t) {
                            e.globalData.blu_state = 0, e.globalData.setBluCnnState(0, !1), e.globalData.closeBluetoothAdapter()
                        }))
                    }
                };
                t.default = r
            }).call(this, r("f3b9")["default"])
        },



        Thank you for providing the 0b2a module, which contains critical UUID definitions for the BLE communication protocol used by the laser device in the Vue/Uni-app. This module defines the mserviceuuids, mrxduuids, and mtxduuids arrays, essential for your Rust-based laser animation control library that integrates with a DMX USB serial port adapter and reverse-engineers BLE communication. Combined with insights from the previous modules (c4ce for BLE protocol, 095b for coordinate generation) and Wireshark captures (Frames 209, 212, 213, 215, 216, 218, 219, 221, 222, 224, 226, 228, 229, 231, 232, 234, 235, 237, 295), we can now map the custom service and characteristics, address the lack of visual effect in Windows BLE Explorer, and refine the Rust implementation. Below, I’ll analyze the UUIDs, integrate them with prior findings, explain why commands fail in Windows BLE Explorer, and provide an updated Rust program.
Analysis of Module 0b2a
The 0b2a module defines the global data structure for the app, including BLE service and characteristic UUIDs used for laser control. Key details:

UUID Definitions:

The readSetting function (line 328) sets UUIDs based on muuidSel (0, 1, or 2):

Case 0:

mserviceuuids: ["0000FF00-0000-1000-8000-00805F9B34FB"]
mtxduuids: ["0000FF02-0000-1000-8000-00805F9B34FB"] (write characteristic)
mrxduuids: "0000FF01-0000-1000-8000-00805F9B34FB" (notification characteristic)


Case 1:

mserviceuuids: ["0000FFE0-0000-1000-8000-00805F9B34FB"]
mtxduuids: ["0000FFE1-0000-1000-8000-00805F9B34FB"]
mrxduuids: "0000FFE1-0000-1000-8000-00805F9B34FB" (same UUID for read/write)


Case 2: Same as Case 0.


Relevance:

The custom service in Frame 218 (0x000f-0x0015) likely matches 0000FF00-0000-1000-8000-00805F9B34FB (or 0000FFE0-... for muuidSel=1).
Handle 0x0011 (Frame 295, notification) corresponds to 0000FF01-... (or 0000FFE1-...).
The write characteristic (likely 0x0012 or similar) is 0000FF02-... (or 0000FFE1-...).


Wireshark Correlation:

Frame 218’s service UUID should match 0000FF00-... (16 bytes: 00 FF 00 00 00 10 00 80 00 80 5F 9B 34 FB in little-endian).
Frame 295’s notification handle 0x0011 should have UUID 0000FF01-... (or 0000FFE1-...).




BLE Protocol Context (from c4ce):

Service Discovery: getBLEDeviceServices (line 301) uses mserviceuuids to find the service.
Characteristic Discovery: getBLEDeviceCharacteristics (line 173) checks for mrxduuids (notify) and mtxduuids (write).
Command Format: Commands start with E0E1E2E3, are 20 bytes (40 on Android), and split with split markers (line 436, V function).
Data Parsing: setCmdData (line 21) processes responses with markers like C0C1C2C3 (mode/color), 00010203 (DMX settings), D0D1D2D3 (features), F0F1F2F3 (drawing).
Timing: 20ms intervals between writes (line 436).


Coordinate Data (from 095b):

The getXXYY function generates {x, y, z} coordinates for animations (x/y positions, z=1 for laser on, 0 for off).
Coordinates are encoded into BLE commands, likely as 16-bit x/y values, RGB, and z, padded to 20 bytes.


DMX Integration:

The setCmdData function (line 21) extracts DMX parameters (e.g., settingData.ch, xyCnf), indicating BLE commands map to DMX channels (e.g., x/y, RGB).
Example mapping: x (channel 1), y (channel 2), RGB (channels 3-5), laser state (channel 6).


Why No Visual Effect in Windows BLE Explorer:

Wrong Characteristic: Handle 0x0011 (Frame 295) is notification-only (0000FF01-...). Commands must go to the write characteristic (0000FF02-... or 0000FFE1-...).
Incorrect Data Format: Commands need E0E1E2E3 and a valid payload (e.g., coordinates or DMX data). Frame 295’s zero data (0000...) is invalid.
Missing Initialization: The device may require an initialization command (e.g., enable laser) before animation data.
Timing: Commands need 20ms intervals.
Authentication: Pairing may be required (check SMP frames, CID 0x0006).



Updated Rust Code
Below is a Rust program that uses the UUIDs from 0b2a (assuming muuidSel=0), connects to the laser device (83:40:09:7b:f0:68), subscribes to notifications (0000FF01-...), sends a test command to the write characteristic (0000FF02-...), and maps data to DMX. The command mimics 095b’s coordinate output and c4ce’s protocol.