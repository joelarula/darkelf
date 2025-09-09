        "97f5": function(e, t, r) {
            "use strict";
            (function(e) {
                var n = r("47a9");
                Object.defineProperty(t, "__esModule", {
                    value: !0
                }), t.default = void 0;
                var h = n(r("762b")),
                    a = getApp();
                var i = {
                    data: function() {
                        return {
                            screen_width: a.globalData.screen_width_str,
                            nbTitle: this.$t("\u84dd\u7259\u8fde\u63a5"),
                            rtl: a.globalData.rtl,
                            showUUID: a.globalData.mainPage.debugTag,
                            devices: null,
                            stopTimer: null,
                            startTimer: null,
                            openTimer: null,
                            tryCnt: 12,
                            InsCnns: [],
                            connected: !1,
                            chs: [],
                            misScanding: 0,
                            name: "",
                            deviceId: "",
                            testDevTimer: null,
                            testDevCnt: 0,
                            findInterval: 3e3,
                            canWrite: !1,
                            findtimes: 0
                        }
                    },
                    onHide: function() {
                        e("log", "cnn hide", " at pages/cnn/cnn.js:64")
                    },
                    onUnload: function() {
                        this.stopBluetoothDevicesDiscovery(), this.openTimer && clearTimeout(this.openTimer), this.openTimer = null, a.globalData.blu_state = 0
                    },
                    onLoad: function() {
                        this.initInsCnns(), "android" == a.globalData.platform.system && (this.findInterval = 7e3), this.showUUID || 0 == a.globalData.muuidSel || (a.globalData.savelastsel(0), a.globalData.readSetting())
                    },
                    onReady: function() {
                        a.globalData.blu_connect_stop = !0;
                        var e = this;
                        setTimeout((function() {
                            e.closeConnectedDev((function(t) {
                                a.globalData.blu_state = 2, e.readyToFindDevices()
                            }))
                        }), 200)
                    },
                    components: {
                        uniPopup: h.default
                    },
                    methods: {
                        testTap: function(t) {
                            var r = this;
                            null == this.testDevTimer ? (r.testDevCnt = 1, this.testDevTimer = setTimeout((function() {
                                r.testDevTimer = null
                            }), 2e3)) : r.testDevCnt = 1 + r.testDevCnt, e("log", "that.testDevCnt", r.testDevCnt, " at pages/cnn/cnn.js:115")
                        },
                        initInsCnns: function() {
                            this.InsCnns = [], this.InsCnns.push({
                                title: "Ins_OpenBlu_Title",
                                text: "Ins_OpenBlu_Text"
                            }), a.globalData.platform.app ? "android" == a.globalData.platform.system ? (this.InsCnns.push({
                                title: "Ins_PerBlu_Title",
                                text: "Ins_PerBlu_Text_android"
                            }), this.InsCnns.push({
                                title: "Ins_OpenLoc_Title",
                                text: "Ins_OpenLoc_Text"
                            }), this.InsCnns.push({
                                title: "Ins_PerLoc_Title",
                                text: "Ins_PerLoc_Text"
                            })) : this.InsCnns.push({
                                title: "Ins_PerBlu_Title",
                                text: "Ins_PerBlu_Text_IOS"
                            }) : ("android" == a.globalData.platform.system && this.InsCnns.push({
                                title: "Ins_OpenLoc_Title",
                                text: "Ins_OpenLoc_Text"
                            }), this.InsCnns.push({
                                title: "Ins_PerBlu_Title",
                                text: "Ins_PerBlu_Text_WX"
                            })), this.InsCnns.push({
                                title: "Ins_Distance_Title",
                                text: "Ins_Distance_Text"
                            }), this.InsCnns.push({
                                title: "Ins_Reroot_Title",
                                text: "Ins_Reroot_Text"
                            })
                        },
                        showCnnDeal: function(e) {
                            this.$refs.cnnDealPopup.open("center")
                        },
                        closeCnnDealPopup: function(e) {
                            this.$refs.cnnDealPopup.close()
                        },
                        testLongtap: function(t) {
                            e("log", "testLongtap", this.testDevCnt, " at pages/cnn/cnn.js:149"), this.testDevCnt >= 5 && (this.showUUID = !0, a.globalData.mainPage.debugTag = !0)
                        },
                        testDev: function() {
                            uni.getBluetoothAdapterState({
                                success: function(e) {
                                    e.discovering ? uni.stopBluetoothDevicesDiscovery({
                                        success: function(e) {
                                            uni.navigateTo({
                                                url: "/pages/test/test"
                                            })
                                        }
                                    }) : uni.navigateTo({
                                        url: "/pages/test/test"
                                    })
                                }
                            })
                        },
                        readyToFindDevices: function() {
                            var t = this;
                            uni.hideLoading(), a.globalData.blu_state = 2, a.globalData.openBluetoothAdapter((function(r) {
                                e("log", "openBluetoothAdapter", r, " at pages/cnn/cnn.js:175"), r && (a.globalData.clearDevice(), a.globalData.blu_cnn_from_page = !1, t.startBluetoothDevicesDiscovery())
                            }))
                        },
                        closeConnectedDev: function() {
                            var e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            a.globalData.closeBLEConnection((function(t) {
                                e && e(t)
                            }))
                        },
                        dostartBluetoothDevicesDiscovery: function() {
                            var e = this;
                            uni.startBluetoothDevicesDiscovery({
                                services: a.globalData.mserviceuuids,
                                allowDuplicatesKey: !0,
                                success: function(t) {
                                    a.globalData.blu_Discovery_lastTime = (new Date).getTime(), e.findtimes = (new Date).getTime(), e.stopTimer = setTimeout((function() {
                                        e.stopTimer = null, e.stopBluetoothDevicesDiscovery()
                                    }), 1e4), e.misScanding = 2, e.onBluetoothDeviceFoundPro()
                                },
                                fail: function(t) {
                                    1509008 === t.errno && a.globalData.showModalTips(e.$t("\u8bf7\u8bbe\u7f6e\u5e94\u7528\u5b9a\u4f4d\u6743\u9650"), !0)
                                }
                            })
                        },
                        startBluetoothDevicesDiscovery: function() {
                            var e = this;
                            if (e.misScanding) this.stopBluetoothDevicesDiscovery();
                            else {
                                var t = (new Date).getTime();
                                t = this.findInterval - (t - a.globalData.blu_Discovery_lastTime), t = t < 0 ? 0 : t, e.devices = [], e.chs = [], e.misScanding = 1, e.startTimer = setTimeout((function() {
                                    e.startTimer = null, e.dostartBluetoothDevicesDiscovery()
                                }), t)
                            }
                        },
                        stopBluetoothDevicesDiscovery: function() {
                            var e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null,
                                t = this;
                            t.misScanding ? (t.stopTimer && (clearTimeout(t.stopTimer), t.stopTimer = null), t.startTimer && (clearTimeout(t.startTimer), t.startTimer = null), 2 == t.misScanding ? uni.stopBluetoothDevicesDiscovery({
                                success: function(r) {
                                    (new Date).getTime();
                                    t.misScanding = 0, e && e()
                                }
                            }) : t.misScanding = 0) : e && e()
                        },
                        ab2hex: function(e) {
                            var t = Array.prototype.map.call(new Uint8Array(e), (function(e) {
                                    return ("00" + e.toString(16)).slice(-2) + ""
                                })),
                                r = t.join("").toUpperCase().slice(0, 4);
                            return r
                        },
                        onBluetoothDeviceFoundPro: function() {
                            var t = this;
                            uni.onBluetoothDeviceFound((function(r) {
                                r.devices.forEach((function(r) {
                                    if (r.name || r.localName) {
                                        var n = t.devices,
                                            h = function(e, t, r) {
                                                for (var n = 0; n < e.length; n++)
                                                    if (e[n][t] === r) return n;
                                                return -1
                                            }(n, "deviceId", r.deviceId);
                                        if (-1 !== h) {
                                            if (!r.localName || !r.name || n[h].localName == r.localName || r.name == r.localName) return;
                                            n.splice(h, 1)
                                        }
                                        var i = r.localName;
                                        if (i || (i = r.name), 0 == a.globalData.muuidSel) {
                                            if (!i || !i.startsWith("TD5322A_")) return;
                                            var c = i.substring("TD5322A_".length, i.length);
                                            e("log", "\u84dd\u7259\u8bbe\u5907\u540d\uff1a", i, "\u957f\u5ea6\uff1a", i.length, "\u540e\u7f00\uff1a", c, " at pages/cnn/cnn.js:298");
                                            var o = t.tripCh(r.deviceId, ":"),
                                                s = o.slice(0, 4).toUpperCase();
                                            i = "Laser_Light_" + s;
                                            var l = 0;
                                            /\b[0-9]\d{2}\b/.test(c) && (l = parseInt(c)), l > 0 && (c = "000" + l, i = i + "_V" + c.slice(-3))
                                        }
                                        r["showName"] = i, t.devices.push(r)
                                    }
                                }))
                            }))
                        },
                        tripCh: function(e, t) {
                            for (var r = "", n = 0; n < e.length; n++) ":" != e[n] && (r += e[n]);
                            return r
                        },
                        goto_Comm: function(e) {
                            var t = this;
                            a.globalData.ble_device = e.currentTarget.dataset, this.stopBluetoothDevicesDiscovery((function() {
                                var e = t.getOpenerEventChannel();
                                a.globalData.blu_cnn_from_page = !0, e.emit("acceptDataFromOpenedPage", {
                                    data: !0
                                })
                            }))
                        },
                        gotosetuuid: function() {
                            uni.navigateTo({
                                url: "/pages/setuuid/setuuid"
                            })
                        }
                    }
                };
                t.default = i
            }).call(this, r("f3b9")["default"])
        },