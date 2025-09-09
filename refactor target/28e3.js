        "28e3": function(t, r, n) {
            "use strict";
            (function(t) {
                Object.defineProperty(r, "__esModule", {
                    value: !0
                }), r.default = void 0;
                var n = getApp();

                function h(e, t, r) {
                    for (var n = 0; n < e.length; n++)
                        if (e[n][t] === r) return n;
                    return -1
                }
                var a = {
                    data: function() {
                        return {
                            nbTitle: "\u81ea\u52a8\u5316\u6d4b\u8bd5",
                            devices: [],
                            forTest: !1,
                            deviceTested: [],
                            deviceTesting: null,
                            testStatus: "",
                            blu_rec_content: null,
                            bluOpen: !1,
                            discoveryStarted: !1,
                            recDeviceMsgTimer: null,
                            testingIdx: -1,
                            total: 0,
                            notPass: 0,
                            passCount: 0,
                            scrollTop: 0,
                            chs: [],
                            misScanding: !1,
                            scandbutName: "\u5f00\u59cb\u81ea\u52a8\u6d4b\u8bd5",
                            name: "",
                            deviceId: "",
                            canWrite: !1
                        }
                    },
                    onHide: function() {},
                    onLoad: function() {
                        this.forTest = 0 != n.globalData.muuidSel
                    },
                    onunLoad: function() {
                        this.stopBluetoothDevicesDiscovery(), this.closeCnnAndRun(), uni.setKeepScreenOn({
                            keepScreenOn: !1
                        }), n.globalData.blu_cnn_from_test = !1
                    },
                    onReady: function() {
                        n.globalData.blu_cnn_from_test = !0, this.checkAndOpenBluetoothAdapter(), uni.setKeepScreenOn({
                            keepScreenOn: !0
                        })
                    },
                    methods: {
                        checkAndOpenBluetoothAdapter: function() {
                            var e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            if (this.bluOpen) e();
                            else {
                                var r = this;
                                uni.getBluetoothAdapterState({
                                    success: function(n) {
                                        t("log", "getBluetoothAdapterState", n, " at pages/test/test.js:79"), r.bluOpen = !0, uni.onBLEConnectionStateChange((function(e) {
                                            r.bleConnectionStateChange(e)
                                        })), e && e()
                                    },
                                    fail: function(t) {
                                        1e4 == t.errCode && r.openBluetoothAdapter(e)
                                    }
                                })
                            }
                        },
                        showMsg: function(e) {
                            n.globalData.showModalTips(e, !0)
                        },
                        openBluetoothAdapter: function() {
                            var e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null,
                                r = this;
                            uni.openBluetoothAdapter({
                                success: function(e) {
                                    t("log", "openBluetoothAdapter success", e, " at pages/test/test.js:102"), r.bluOpen = !0, uni.onBLEConnectionStateChange((function(e) {
                                        r.bleConnectionStateChange(e)
                                    }))
                                },
                                fail: function(e) {
                                    r.bluOpen = !1, t("log", "openBluetoothAdapter", e, " at pages/test/test.js:110"), 10001 === e.errCode && r.showMsg("\u8bf7\u68c0\u67e5\u624b\u673a\u84dd\u7259\u662f\u5426\u542f\u7528"), 103 == e.errno && r.showMsg("\u8bf7\u8bbe\u7f6e\u5c0f\u7a0b\u5e8f\u84dd\u7259\u6743\u9650")
                                },
                                complete: function() {
                                    e && e()
                                }
                            })
                        },
                        bleConnectionStateChange: function(e) {
                            var t = h(this.deviceTested, "deviceId", e.deviceId);
                            if (-1 != t) {
                                var r = this.deviceTested[t].conn;
                                this.$set(this.deviceTested[t], "conn", e.connected), !e.connected && r && (this.deviceTesting = null, this.discoveryStarted && this.doStart(this.testingIdx + 1))
                            }
                        },
                        getBluetoothAdapterState: function() {
                            var e = this;
                            uni.getBluetoothAdapterState({
                                success: function(r) {
                                    t("log", "getBluetoothAdapterState", r, " at pages/test/test.js:141"), r.discovering ? e.onBluetoothDeviceFound() : r.available && e.startBluetoothDevicesDiscovery()
                                }
                            })
                        },
                        startBluetoothDevicesDiscovery: function() {
                            var e = this;
                            this.discoveryStarted ? this.stopBluetoothDevicesDiscovery() : (this.misScanding = !0, this.scandbutName = "\u6b63\u5728\u81ea\u52a8\u6d4b\u8bd5", this.devices = [], this.chs = [], this.discoveryStarted = !0, uni.startBluetoothDevicesDiscovery({
                                allowDuplicatesKey: !0,
                                interval: 1e3,
                                success: function(r) {
                                    t("log", "startBluetoothDevicesDiscovery success", r, " at pages/test/test.js:173"), e.onBluetoothDeviceFound()
                                },
                                fail: function(r) {
                                    t("log", "startBluetoothDevicesDiscovery fail", r, " at pages/test/test.js:177"), 1509008 === r.errno && n.globalData.showModalTips(e.$t("\u8bf7\u8bbe\u7f6e\u5e94\u7528\u5b9a\u4f4d\u6743\u9650"), !0)
                                }
                            }))
                        },
                        stopBluetoothDevicesDiscovery: function() {
                            var e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null;
                            this.discoveryStarted = !1;
                            var r = this;
                            uni.stopBluetoothDevicesDiscovery({
                                success: function(n) {
                                    t("log", n, " at pages/test/test.js:189"), r.misScanding = !1, r.scandbutName = "\u5f00\u59cb\u81ea\u52a8\u6d4b\u8bd5", t("log", r.misScanding, r.scandbutName, " at pages/test/test.js:192"), null != e && e()
                                }
                            })
                        },
                        ab2hex: function(e) {
                            var t = Array.prototype.map.call(new Uint8Array(e), (function(e) {
                                    return ("00" + e.toString(16)).slice(-2) + ""
                                })),
                                r = t.join("").toUpperCase().slice(0, 4);
                            return r
                        },
                        onBluetoothDeviceFound: function() {
                            var e = this;
                            uni.onBluetoothDeviceFound((function(t) {
                                t.devices.forEach((function(t) {
                                    if (t.name || t.localName) {
                                        var r = t.name || t.localName;
                                        if (r) {
                                            if (!e.forTest) {
                                                if (!r.startsWith("TD5322A_")) return;
                                                var n = e.tripCh(t.deviceId, ":"),
                                                    a = n.slice(0, 4).toUpperCase();
                                                r = "Laser_Light_" + a
                                            }
                                            t["showName"] = r;
                                            var i = e.devices,
                                                c = h(i, "deviceId", t.deviceId); - 1 === c ? (e.devices.push(t), e.addToTest(t)) : e.devices[c] = t
                                        }
                                    }
                                }))
                            }))
                        },
                        addToTest: function(e) {
                            t("log", "addtotest begin", e, " at pages/test/test.js:237");
                            var r = h(this.deviceTested, "deviceId", e.deviceId); - 1 == r && (e["testTimeBegin"] = new Date, e["testStatus"] = "\u672a\u5f00\u59cb", e["testResult"] = "\u672a\u6d4b\u8bd5", e["testTimeEnd"] = "", e["testMsg"] = "", e["canSend"] = !1, this.deviceTested.push(e), this.scrollToBottom(), this.total = this.total + 1, this.notPass = this.notPass + 1, this.deviceTesting || -1 != this.testingIdx || this.doStart(this.testingIdx + 1))
                        },
                        scrollToBottom: function() {
                            var e = this;
                            uni.createSelectorQuery().in(this).select("#scroll_view").fields({
                                size: !0,
                                scrollOffset: !0,
                                scrollHeight: !0
                            }).exec((function(t) {
                                if (t[0]) {
                                    var r = t[0].scrollHeight;
                                    e.$nextTick((function() {
                                        e.scrollTop = r
                                    }))
                                }
                            }))
                        },
                        tripCh: function(e, t) {
                            for (var r = "", n = 0; n < e.length; n++) ":" != e[n] && (r += e[n]);
                            return r
                        },
                        doLog: function() {
                            this.testStatus = this.deviceTesting.testStatus, t("log", "testing dev", this.deviceTesting, " at pages/test/test.js:286")
                        },
                        createBLEConnection: function(e) {
                            var r = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null;
                            e.canSend = !1, t("log", "\u5f00\u59cb\u8fde\u63a5createBLEConnection", " at pages/test/test.js:291"), e.testStatus = "\u6b63\u5728\u8fde\u63a5", this.doLog();
                            var n = e.deviceId;
                            uni.createBLEConnection({
                                deviceId: n,
                                timeout: 4e3,
                                success: function(e) {
                                    r && r(null)
                                },
                                fail: function(e) {
                                    r && r(e)
                                }
                            })
                        },
                        closeBLEConnection: function(e) {
                            uni.closeBLEConnection({
                                deviceId: e.deviceId,
                                complete: function() {}
                            })
                        },
                        getBLEDeviceServices: function(e) {
                            var r = this,
                                n = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null,
                                h = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : 4;
                            e.testStatus = "\u6b63\u5728\u83b7\u53d6\u670d\u52a1", this.doLog(), h < 0 && n && n("\u83b7\u53d6\u670d\u52a1\u8d85\u65f6");
                            var a = e.deviceId;
                            uni.getBLEDeviceServices({
                                deviceId: a,
                                success: function(a) {
                                    if (0 == a.services.length) return t("log", "service length: ", a.services.length, " at pages/test/test.js:327"), void setTimeout((function() {
                                        r.getBLEDeviceServices(e, n, h - 1)
                                    }), 1e3);
                                    t("log", "getBLEDeviceServices----", a, " at pages/test/test.js:331");
                                    for (var i = !1, c = 0; c < a.services.length; c++) "0000FF00-0000-1000-8000-00805F9B34FB" != a.services[c].uuid && "0000FFE0-0000-1000-8000-00805F9B34FB1" != a.services[c].uuid || (e.serviceId = a.services[c].uuid, i = !0, r.getBLEDeviceCharacteristics(e));
                                    !i && n && n("\u6ca1\u6709\u627e\u5230\u6307\u5b9a\u670d\u52a1")
                                },
                                fail: function(e) {
                                    n && n(e)
                                }
                            })
                        },
                        getBLEDeviceCharacteristics: function(r) {
                            r.testStatus = "\u6b63\u5728\u83b7\u53d6\u7279\u5f81\u503c", this.doLog(), uni.onBLECharacteristicValueChange((function(e) {
                                new Uint8Array(e.value);
                                var t = function(e) {
                                    var t = Array.prototype.map.call(new Uint8Array(e), (function(e) {
                                        return ("00" + e.toString(16)).slice(-2)
                                    }));
                                    return t.join("")
                                }(e.value);
                                n.addContent(e.characteristicId, t.toUpperCase())
                            }));
                            var n = this,
                                h = r.deviceId,
                                a = r.serviceId;
                            uni.getBLEDeviceCharacteristics({
                                deviceId: h,
                                serviceId: a,
                                success: function(e) {
                                    t("log", "getBLEDeviceCharacteristics success", e.characteristics, " at pages/test/test.js:366");
                                    for (var i = function(i) {
                                            var c = e.characteristics[i];
                                            c.properties.read && uni.readBLECharacteristicValue({
                                                deviceId: h,
                                                serviceId: a,
                                                characteristicId: c.uuid
                                            }), c.properties.write && ("0000FFE2-0000-1000-8000-00805F9B34FB" != c.uuid && "0000FF02-0000-1000-8000-00805F9B34FB" != c.uuid || (r.writeUUID = c.uuid, n.startTest(r))), (c.properties.notify || c.properties.indicate) && ("0000FFE1-0000-1000-8000-00805F9B34FB" != c.uuid && "0000FF01-0000-1000-8000-00805F9B34FB" != c.uuid || (r.readUUID = c.uuid, uni.notifyBLECharacteristicValueChange({
                                                deviceId: h,
                                                serviceId: a,
                                                characteristicId: c.uuid,
                                                state: !0,
                                                success: function(e) {
                                                    r.canSend = !0, uni.readBLECharacteristicValue({
                                                        deviceId: h,
                                                        serviceId: a,
                                                        characteristicId: c.uuid,
                                                        success: function(e) {
                                                            t("log", "readBLECharacteristicValue2:", e, " at pages/test/test.js:399")
                                                        },
                                                        fail: function(e) {
                                                            t("log", "readBLECharacteristicValue2-fail:", e, " at pages/test/test.js:402")
                                                        }
                                                    })
                                                }
                                            })))
                                        }, c = 0; c < e.characteristics.length; c++) i(c)
                                },
                                fail: function(n) {
                                    r.testStatus = "\u6d4b\u8bd5\u7ed3\u675f", r.testResult = "\u672a\u901a\u8fc7", r.testMsg = "\u83b7\u53d6\u7279\u5f81\u503c\u5931\u8d25:" + JSON.stringify(e), t("error", "getBLEDeviceCharacteristics", n, " at pages/test/test.js:415"), this.doLog()
                                }
                            })
                        },
                        addContent: function(e, r) {
                            t("log", "addContent", r, " at pages/test/test.js:422");
                            var n = this.blu_rec_content;
                            if (null == n ? r.startsWith("E0E1E2E3") && (n = r) : n += r, "" != n && null != n) {
                                var h = n.lastIndexOf("E0E1E2E3"),
                                    a = n.lastIndexOf("E4E5E6E7"),
                                    i = n;
                                a > 0 && (a == n.length - 8 ? (i = n.slice(h, a + 8), this.dataReced(e, i), i = null) : i = n.slice(h)), this.blu_rec_content = i
                            }
                        },
                        getCmdValue: function(e, r, n) {
                            var h = new RegExp(e + "(.+?)" + r),
                                a = h.exec(n);
                            return null !== a ? a[1] : (t("log", "\u672a\u5339\u914d\u5230\u7b26\u5408\u8981\u6c42\u7684\u5b57\u7b26\u4e32", " at pages/test/test.js:455"), "")
                        },
                        dataReced: function(e, r) {
                            t("log", "data ", r, " at pages/test/test.js:461"), this.recDeviceMsgTimer && (clearTimeout(this.recDeviceMsgTimer), this.recDeviceMsgTimer = null);
                            var n = this.getCmdValue("C0C1C2C3", "C4C5C6C7", r),
                                h = "C0C1C2C304" + n.substring(2, 20) + "FFFFFFFF" + n.substring(28, n.length) + "C4C5C6C7";
                            if (t("log", "sTxt", h, " at pages/test/test.js:469"), this.deviceTesting && this.deviceTesting.readUUID == e) {
                                var a = this.deviceTesting;
                                a.testStatus = "\u6b63\u5728\u53d1\u9001\u8282\u76ee\u547d\u4ee4", this.doLog();
                                var i = this;
                                this.sendData(a, h, (function(e) {
                                    e ? (a.testStatus = "\u6d4b\u8bd5\u7ed3\u675f", a.testResult = "\u672a\u901a\u8fc7", t("log", "\u53d1\u9001\u8282\u76ee\u547d\u4ee4\u5931\u8d25" + JSON.stringify(e), " at pages/test/test.js:479"), a.testMsg = "\u53d1\u9001\u8282\u76ee\u547d\u4ee4\u5931\u8d25", i.doLog()) : (a.testStatus = "\u6d4b\u8bd5\u7ed3\u675f", a.testResult = "\u5df2\u901a\u8fc7", a.testMsg = "\u53d1\u9001\u8282\u76ee\u547d\u4ee4\u6210\u529f", i.notPass = i.notPass - 1, i.passCount = i.passCount + 1, i.doLog(), i.closeCnnAndRun())
                                }))
                            } else t("log", "\u7279\u5f81\u503c\u4e0d\u5339\u914d", " at pages/test/test.js:492")
                        },
                        startTest: function(e) {
                            var r = this,
                                n = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 10;
                            if (n < 0) return e.testStatus = "\u6d4b\u8bd5\u7ed3\u675f", e.testResult = "\u672a\u901a\u8fc7", t("log", "\u53d1\u9001\u547d\u4ee4\u5931\u8d25:" + JSON.stringify(res), " at pages/test/test.js:499"), e.testMsg = "\u53d1\u9001\u547d\u4ee4\u5931\u8d25-\u53d1\u9001\u8d85\u65f6", void this.doLog();
                            e.canSend ? setTimeout((function() {
                                r.doStartTest(e)
                            }), 4e3) : setTimeout((function() {
                                r.startTest(e, n - 1)
                            }), 1e3)
                        },
                        doStartTest: function(e) {
                            var r = this,
                                n = this;
                            e.testStatus = "\u6b63\u5728\u53d1\u9001\u67e5\u8be2\u547d\u4ee4", this.doLog(), this.sendData(e, "E0E1E2E3F776CD3AE4E5E6E7", (function(h) {
                                h ? (e.testStatus = "\u6d4b\u8bd5\u7ed3\u675f", e.testResult = "\u672a\u901a\u8fc7", t("log", "\u53d1\u9001\u67e5\u8be2\u547d\u4ee4\u5931\u8d25" + JSON.stringify(h), " at pages/test/test.js:519"), e.testMsg = "\u53d1\u9001\u67e5\u8be2\u547d\u4ee4\u5931\u8d25", n.doLog()) : (e.testStatus = "\u53d1\u9001\u67e5\u8be2\u547d\u4ee4\u6210\u529f\uff0c\u7b49\u5f85\u8bbe\u5907\u54cd\u5e94", n.doLog(), r.recDeviceMsgTimer && clearTimeout(r.recDeviceMsgTimer), r.recDeviceMsgTimer = setTimeout((function() {
                                    r.recDeviceMsgTimer = null, e.conn && (e.testStatus = "\u6d4b\u8bd5\u7ed3\u675f", e.testResult = "\u672a\u901a\u8fc7", t("log", "\u8bfb\u53d6\u8bbe\u5907\u8d85\u65f6", " at pages/test/test.js:531"), e.testMsg = "\u8bfb\u53d6\u8bbe\u5907\u8d85\u65f6", n.doLog(), n.closeCnnAndRun((function() {
                                        t("log", "that.closeCnnAndRun", " at pages/test/test.js:535")
                                    })))
                                }), 1e4))
                            }))
                        },
                        reTest: function(e) {
                            this.discoveryStarted ? this.showMsg("\u8bf7\u5148\u505c\u6b62\u81ea\u52a8\u6d4b\u8bd5") : ("\u5df2\u901a\u8fc7" == this.deviceTested[e].testResult && (this.notPass = this.notPass + 1, this.passCount = this.passCount - 1), this.doStart(e))
                        },
                        startTest1: function(e) {
                            var r = this;
                            uni.openBluetoothAdapter({
                                success: function(e) {
                                    r.doStart()
                                },
                                fail: function(e) {
                                    t("log", "openBluetoothAdapter\u5931\u8d25", e, " at pages/test/test.js:563")
                                }
                            })
                        },
                        closeCnnAndRun: function() {
                            for (var e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null, r = null, n = 0; n < this.deviceTested.length; n++)
                                if (this.deviceTested[n].conn) {
                                    r = this.deviceTested[n];
                                    break
                                } t("log", "dev====", r, " at pages/test/test.js:577"), null != r && r.conn ? uni.closeBLEConnection({
                                deviceId: r.deviceId,
                                fail: function(e) {
                                    t("log", "closeBLEConnection \u5931\u8d25:", e, " at pages/test/test.js:582")
                                },
                                complete: function() {
                                    e && e()
                                }
                            }) : e && e()
                        },
                        doStart: function() {
                            var e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : 0,
                                r = this;
                            if (e > this.deviceTested.length - 1) this.discoveryStarted && setTimeout((function() {
                                r.doStart(e)
                            }), 1e3);
                            else {
                                this.testingIdx = e, this.deviceTesting = this.deviceTested[e], this.deviceTesting.testResult = "\u6b63\u5728\u6d4b\u8bd5", this.deviceTesting.testStatus = "\u6b63\u5728\u6d4b\u8bd5", this.deviceTesting.testMsg = "\u6b63\u5728\u6d4b\u8bd5", t("log", "startTest  =========", this.testingIdx, this.deviceTesting, " at pages/test/test.js:604");
                                var n = this.deviceTesting;
                                this.closeCnnAndRun((function() {
                                    r.createBLEConnection(r.deviceTesting, (function(t) {
                                        t ? (n.testStatus = "\u6d4b\u8bd5\u7ed3\u675f", n.testResult = "\u672a\u901a\u8fc7", n.testMsg = "\u8fde\u63a5\u5931\u8d25:" + JSON.stringify(t), r.$set(r.deviceTested[e], "testStatus", "\u6d4b\u8bd5\u7ed3\u675f"), r.doLog(), r.deviceTesting = null, r.discoveryStarted && r.doStart(r.testingIdx + 1)) : (n.testStatus = "\u5df2\u8fde\u63a5", n.conn = !0, r.doLog(), r.getBLEDeviceServices(n, (function(e) {
                                            n.testStatus = "\u6d4b\u8bd5\u7ed3\u675f", n.testResult = "\u672a\u901a\u8fc7", n.testMsg = "\u83b7\u53d6\u670d\u52a1\u5931\u8d25:" + JSON.stringify(e), r.doLog(), r.closeCnnAndRun()
                                        })))
                                    }))
                                }))
                            }
                        },
                        testClick: function() {
                            this.sendData(this.deviceTesting, "E0E1E2E3F776CD3AE4E5E6E7")
                        },
                        covert2SendData: function(e) {
                            t("log", "covert2SendData", e, " at pages/test/test.js:639");
                            var r = new Uint8Array(e.match(/[\da-f]{2}/gi).map((function(e) {
                                return parseInt(e, 16)
                            })));
                            if (null == r) return [];
                            var n = r.buffer.byteLength,
                                h = [],
                                a = 0;
                            while (n > 0) {
                                var i = n % 20,
                                    c = null;
                                n >= 20 ? (c = new Uint8Array(r.subarray(a, a + 20)).buffer, n -= 20, a += 20) : (c = new Uint8Array(r.subarray(a, a + i)).buffer, n -= i, a += i), h.push(c)
                            }
                            return h
                        },
                        sendData: function(e, t) {
                            var r = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null,
                                n = this.covert2SendData(t);
                            this.doSendData(e, n, 0, r)
                        },
                        doSendData: function(e, t) {
                            var r = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : 0,
                                n = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : null,
                                h = t[r],
                                a = this;
                            this.writeBLECharacteristicValue(e, h, (function(h) {
                                null != h ? n && n(h) : r < t.length - 1 ? setTimeout((function() {
                                    a.doSendData(e, t, r + 1, n)
                                }), 20) : n && n(null)
                            }))
                        },
                        writeBLECharacteristicValue: function(e, t) {
                            var r = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null;
                            uni.writeBLECharacteristicValue({
                                deviceId: e.deviceId,
                                serviceId: e.serviceId,
                                characteristicId: e.writeUUID,
                                value: t,
                                success: function(e) {
                                    r && r(null)
                                },
                                fail: function(e) {
                                    r && r(e)
                                }
                            })
                        },
                        closeBluetoothAdapter: function() {
                            uni.closeBluetoothAdapter()
                        }
                    }
                };
                r.default = a
            }).call(this, n("f3b9")["default"])
