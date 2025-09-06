       c4ce: function(e, t, r) {
            (function(t) {
                var n = getApp(),
                    h = r("3b77");

                function a(e, t, r) {
                    var n = 2 * (e - 1),
                        h = n + 2 * t,
                        a = r.slice(n, h),
                        i = parseInt(a, 16);
                    return i
                }

                function i(e, t, r, n) {
                    return isNaN(e) || e < t || e > r ? n : e
                }

                function c(e) {
                    var t = n.globalData.blu_rec_content;
                    if (null == t ? e.startsWith("E0E1E2E3") && (t = e) : t += e, "" != t) {
                        var r = t.lastIndexOf("E0E1E2E3"),
                            h = t.lastIndexOf("E4E5E6E7"),
                            a = t;
                        h > 0 && (h == t.length - 8 ? (a = t.slice(r, h + 8), n.globalData.setRecCallBack(a), a = null) : a = t.slice(r)), n.globalData.blu_rec_content = a
                    }
                }

                function o(e, r, h) {
                    var a = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : null;
                    if (n.globalData.blu_connect_stop) a && a(!1);
                    else {
                        var i = !1,
                            c = "",
                            s = -1;
                        uni.getBLEDeviceCharacteristics({
                            deviceId: e,
                            serviceId: r,
                            success: function(h) {
                                s = 0, t("log", "getBLEDeviceCharacteristics success", h.characteristics, " at utils/bluCtrl.js:173");
                                for (var o = function(o) {
                                        var l = h.characteristics[o];
                                        l.properties.read && (c = l.uuid, i && uni.readBLECharacteristicValue({
                                            deviceId: e,
                                            serviceId: r,
                                            characteristicId: l.uuid,
                                            success: function(e) {
                                                t("log", "readBLECharacteristicValue1:", e, " at utils/bluCtrl.js:184")
                                            },
                                            fail: function(e) {
                                                t("log", "readBLECharacteristicValue1-fail:", e, " at utils/bluCtrl.js:187")
                                            }
                                        })), l.properties.write && -1 != n.globalData.mtxduuids.indexOf(l.uuid) && (n.globalData.ble_device.characteristicId = l.uuid, n.globalData.ble_device.serviceId = r, s++), (l.properties.notify || l.properties.indicate) && -1 != n.globalData.mrxduuids.indexOf(l.uuid) && uni.notifyBLECharacteristicValueChange({
                                            deviceId: e,
                                            serviceId: r,
                                            characteristicId: l.uuid,
                                            state: !0,
                                            success: function(h) {
                                                n.globalData.blu_readyRec = !0, i = !0, "" != c && uni.readBLECharacteristicValue({
                                                    deviceId: e,
                                                    serviceId: r,
                                                    characteristicId: l.uuid,
                                                    success: function(e) {
                                                        t("log", "readBLECharacteristicValue2:", e, " at utils/bluCtrl.js:217")
                                                    },
                                                    fail: function(e) {
                                                        t("log", "readBLECharacteristicValue2-fail:", e, " at utils/bluCtrl.js:220")
                                                    }
                                                }), n.globalData.setBluCnnState(2, !1), a && a(!0)
                                            },
                                            fail: function(e) {
                                                s > 0 && (n.globalData.blu_readyRec = !0, i = !0, n.globalData.setBluCnnState(2, !1), a && a(!0))
                                            }
                                        })
                                    }, l = 0; l < h.characteristics.length; l++) o(l)
                            },
                            fail: function(e) {
                                0 == h && n.globalData.showModalTips(g("\u8fde\u63a5\u5931\u8d25") + "-1002"), s = -2
                            },
                            complete: function() {
                                s <= 0 && (h > 0 ? setTimeout((function() {
                                    o(e, r, --h, a)
                                }), 1500) : (uni.hideLoading(), a && a(!1), n.globalData.showModalTips(g("\u8fde\u63a5\u5931\u8d25") + "-1001")))
                            }
                        })
                    }
                }

                function s(e, r) {
                    var a = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null;
                    n.globalData.blu_connect_stop ? a && a(!1) : (uni.onBLECharacteristicValueChange((function(e) {
                        var r = new Uint8Array(e.value),
                            a = h.ab2hex(e.value);
                        h.ab2Str(e.value); - 1 != n.globalData.mrxduuids.indexOf(e.characteristicId) ? n.globalData.blu_readyRec && r.length > 0 && c(a) : t("error", "no same characteristicId: ", n.globalData.mrxduuids, e.characteristicId, " at utils/bluCtrl.js:270")
                    })), o(e, r, 1, a))
                }

                function l(e, r) {
                    var h = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : 3,
                        a = r.callback;
                    if (n.globalData.blu_connect_stop) a && a(!1);
                    else if (h <= 0) a && a(!1);
                    else {
                        n.globalData.blu_readyRec = !1;
                        var i = e,
                            c = !1;
                        uni.getBLEDeviceServices({
                            deviceId: i,
                            success: function(e) {
                                t("log", "services: ", e, " at utils/bluCtrl.js:301");
                                for (var r = 0; r < e.services.length; r++)
                                    if (-1 != n.globalData.mserviceuuids.indexOf(e.services[r].uuid)) {
                                        c = !0, s(i, e.services[r].uuid, a);
                                        break
                                    }
                            },
                            fail: function(e) {
                                t("log", "getBLEDeviceServices fail:", JSON.stringify(e), " at utils/bluCtrl.js:311")
                            },
                            complete: function() {
                                c || setTimeout((function() {
                                    l(i, r, --h)
                                }), 1e3)
                            }
                        })
                    }
                }

                function p(e) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] && arguments[1],
                        r = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null;
                    if (n.globalData.blu_connect_stop) r && r(!1);
                    else if (void 0 != e && "" != e && null != e) {
                        n.globalData.readSetting(), n.globalData.blu_readyRec = !1;
                        var h = e.deviceId;
                        n.globalData.createBLEConnection(h, (function(e) {
                            e ? (n.globalData.setBluCnnState(1, !1), l(h, {
                                showMsg: t,
                                callback: r
                            })) : (uni.hideLoading(), t && n.globalData.showModalTips(g("\u8fde\u63a5\u5931\u8d25"), !0), r && r(!1))
                        }))
                    } else r && r(!1)
                }

                function d(e) {
                    var r = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 0,
                        h = 20,
                        a = n.globalData.blu_data_send_interval;
                    if (n.globalData.platform.app && "android" == n.globalData.platform.system && (h = 40), e.showMsg) {
                        e.count;
                        var i = Math.floor((e.count - e.sendBufs.length) / e.count * 100),
                            c = (new Date).getTime();
                        (100 == i || c - n.globalData.blu_data_lastShowTime > 200) && (n.globalData.blu_data_lastShowTime = c, e.callBack ? (uni.hideLoading(), e.callBack(0, i)) : uni.showLoading({
                            mask: !0
                        }))
                    }
                    if (0 != e.sendBufs.length) {
                        var o = (new Date).getTime();
                        r = 0 == r ? o : r;
                        var s = h - (o - r),
                            l = s > 0 ? s : 1;
                        setTimeout((function() {
                            var n = e.sendBufs.shift();
                            "split" != n ? (t("log", "send date---", (new Date).getTime() / 1e3, " at utils/bluCtrl.js:441"), uni.writeBLECharacteristicValue({
                                deviceId: e.device.deviceId,
                                serviceId: e.device.serviceId,
                                characteristicId: e.device.characteristicId,
                                value: n,
                                success: function(t) {
                                    d(e, o)
                                },
                                fail: function(r) {
                                    t("log", "writeBLECharacteristicValue fail", r, " at utils/bluCtrl.js:454"), setTimeout((function() {
                                        e.fail(r)
                                    }), h)
                                },
                                complete: function(e) {}
                            })) : setTimeout((function() {
                                t("log", "sleep---", a, h, " at utils/bluCtrl.js:436"), d(e, o)
                            }), a - (o - r))
                        }), l)
                    } else setTimeout((function() {
                        e.success({})
                    }), h)
                }

                function b(e, t, r) {
                    var n = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : null;
                    return new Promise((function(h, a) {
                        d({
                            device: t,
                            sendBufs: e,
                            count: e.length,
                            showMsg: r,
                            callBack: n,
                            success: function(e) {
                                h(e)
                            },
                            fail: function(e) {
                                a(e)
                            }
                        })
                    }))
                }

                function g(e) {
                    return n.globalData.t(e)
                }

                function j(e) {
                    for (var r = "", n = 0; n < e.length; n++) n % 2 == 0 ? ("" != r && (r += ", "), r = r + "0x" + e[n]) : r += e[n];
                    t("log", r, " at utils/bluCtrl.js:494")
                }

                function x(e) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 20;
                    if ("" == e) return [];
                    var r = new Uint8Array(e.match(/[\da-f]{2}/gi).map((function(e) {
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

                function V(e) {
                    var r = e.toUpperCase().split("Z");
                    t("log", r, " at utils/bluCtrl.js:530");
                    for (var n = [], h = 0; h < r.length; h++) {
                        t("log", h, r[h], " at utils/bluCtrl.js:533");
                        var a = x(r[h]);
                        a.length > 0 && (n.length > 0 && n.push("split"), n = n.concat(a))
                    }
                    return n
                }

                function f() {
                    return n.globalData.blu_data_canSend
                }
                e.exports = {
                    cnnPreBlu: function() {
                        if (t("log", "cnnPreBlu", n.globalData.blu_state, " at utils/bluCtrl.js:350"), 0 == n.globalData.blu_state) {
                            n.globalData.blu_state = 1, n.globalData.blu_connect_stop = !1, n.globalData.readDevice();
                            var e = n.globalData.ble_device;
                            void 0 != e && "" != e && null != e ? n.globalData.openBluetoothAdapter((function(r) {
                                r && p(e, !1, (function(e) {
                                    1 == n.globalData.blu_state && (n.globalData.blu_state = 0), t("log", "cnnTheBlu", e, " at utils/bluCtrl.js:365")
                                }))
                            })) : n.globalData.blu_state = 0
                        }
                    },
                    cnnLaser: function() {
                        uni.navigateTo({
                            url: "/pages/cnn/cnn",
                            events: {
                                acceptDataFromOpenedPage: function(e) {
                                    setTimeout((function() {
                                        uni.showLoading({
                                            title: g("\u6b63\u5728\u8fde\u63a5..."),
                                            mask: !0
                                        }), n.globalData.blu_state = 1, n.globalData.blu_connect_stop = !1;
                                        var e = n.globalData.ble_device;
                                        p(e, !0, (function(e) {
                                            e ? n.globalData.blu_state = 0 : (uni.hideLoading(), n.globalData.blu_state = 2), t("log", "cnnTheBlu", e, " at utils/bluCtrl.js:391")
                                        }))
                                    }), 1)
                                }
                            }
                        })
                    },
                    setCanSend: function(e) {
                        n.globalData.blu_data_canSend = e
                    },
                    getCanSend: f,
                    gosend: function(e, r) {
                        var h = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null;
                        if (f() ? j(r) : t("log", "len:" + r.length, r, " at utils/bluCtrl.js:548"), 0 == r.length || !f() && !r.startsWith("E0E1E2E3")) return 0 == r.length || (t("log", "Simulate sending ------- 20ms", n.globalData.blu_data_cmdSending, " at utils/bluCtrl.js:552"), !n.globalData.blu_data_cmdSending && (n.globalData.blu_data_cmdSending = !0, setTimeout((function() {
                            n.globalData.blu_data_cmdSending = !1, h && h(1, 100)
                        }), 20), !0));
                        if (n.globalData.blu_data_cmdSending) return t("error", "last cmd is sending", " at utils/bluCtrl.js:563"), !1;
                        if (2 != n.globalData.blu_connected) return n.globalData.showModalTips(g("\u84dd\u7259\u672a\u8fde\u63a5")), !0;
                        e && (n.globalData.blu_data_lastShowTime = (new Date).getTime(), h ? h(0, 0) : uni.showLoading({
                            mask: !0
                        }));
                        var a = V(r);
                        if (0 == a.length) return !1;
                        if (n.globalData.blu_data_cmdSending) return !1;
                        n.globalData.blu_data_cmdSending = !0;
                        var i = a,
                            c = n.globalData.ble_device;
                        return b(i, c, e, h).then((function(r) {
                            e && uni.hideLoading(), n.globalData.blu_data_cmdSending = !1, t("log", "bluSend succ", " at utils/bluCtrl.js:592"), h && h(1, 100)
                        })).catch((function(r) {
                            e && uni.hideLoading(), t("log", "\u53d1\u9001\u5931\u8d25", r, " at utils/bluCtrl.js:596"), n.globalData.blu_data_cmdSending = !1, h && h(-1, 0)
                        })), !0
                    },
                    drawProgress: function(e, t, r) {
                        e.beginPath(), e.setFillStyle("#4C4C4C");
                        var n = t - 0,
                            h = n;
                        e.moveTo(20, 0), e.lineTo(0 + n - 20, 0), e.arcTo(0 + n, 0, 0 + n, 20, 20), e.lineTo(0 + n, 0 + h - 20), e.arcTo(0 + n, 0 + h, 0 + n - 20, 0 + h, 20), e.lineTo(20, 0 + h), e.arcTo(0, 0 + h, 0, 0 + h - 20, 20), e.lineTo(0, 20), e.arcTo(0, 0, 20, 0, 20), e.fill();
                        var a = t / 2,
                            i = a,
                            c = t / 3,
                            o = -Math.PI / 2,
                            s = 2 * Math.PI * r / 100 + o;
                        e.setLineWidth(t / 30), e.beginPath(), e.arc(a, i, c, 0, 2 * Math.PI), e.setStrokeStyle("#616161"), e.stroke(), e.beginPath(), e.arc(a, i, c, o, s), e.setStrokeStyle("#ECECEC"), e.stroke(), e.beginPath();
                        var l = r + "%",
                            p = t / 5;
                        e.setFillStyle("#ECECEC"), e.setFontSize(p);
                        var d = e.measureText(l).width;
                        e.fillText(r + "%", a - d / 2, i + p / 3), e.fill(), e.draw()
                    },
                    setCmdData: function(e) {
                        t("log", "\u8bbe\u5907\u8fd4\u56de\u6570\u636e", e, " at utils/bluCtrl.js:21"), h.getCmdValue("B0B1B2B3", "B4B5B6B7", e);
                        var r = h.getCmdValue("C0C1C2C3", "C4C5C6C7", e);
                        n.globalData.cmd.curMode = i(a(1, 1, r), 0, 12, 0), n.globalData.cmd.prjData.prjIndex = i(a(1, 1, r), 0, 12, 0), n.globalData.cmd.prjData.public.txColor = i(a(3, 1, r), 0, 9, 0), n.globalData.cmd.textData.txColor = n.globalData.cmd.prjData.public.txColor, n.globalData.cmd.textData.txSize = i(Math.round(a(4, 1, r) / 255 * 100), 10, 100, 60), n.globalData.cmd.textData.runSpeed = i(Math.round(a(6, 1, r) / 255 * 100), 0, 255, 128), n.globalData.cmd.prjData.public.runSpeed = n.globalData.cmd.textData.runSpeed, n.globalData.cmd.textData.txDist = i(Math.round(a(8, 1, r) / 255 * 100), 10, 100, 60), n.globalData.cmd.prjData.public.rdMode = i(a(9, 1, r), 0, 255, 0), n.globalData.cmd.prjData.public.soundVal = i(Math.round(a(10, 1, r) / 255 * 100), 0, 255, 0), n.globalData.cmd.textData.txPointTime = i(a(15, 1, r), 0, 100, 50), n.globalData.cmd.drawData.pisObj.txPointTime = i(a(16, 1, r), 0, 100, 50), n.globalData.cmd.textData.refresh = !0;
                        var c = n.globalData.cmd.prjData.prjItem,
                            o = 17;
                        for (var s in c) {
                            var l = c[s];
                            l.pyMode = i(a(o, 1, r), 0, 255, 0), l.prjSelected[3] = a(o + 1, 2, r), l.prjSelected[2] = a(o + 3, 2, r), l.prjSelected[1] = a(o + 5, 2, r), l.prjSelected[0] = a(o + 7, 2, r), o += 9
                        }
                        n.globalData.cmd.textData.runDir = i(a(o, 1, r), 0, 255, 0), o += 1;
                        for (var p = n.globalData.cmd.subsetData, d = 0; d < 6; d++) 0 == d ? p.xyCnf.auto = p.xyCnf.autoValue == i(a(o + d, 1, r), 0, 255, 0) : 1 == d ? p.xyCnf.phase = i(a(o + d, 1, r), 0, 255, 0) : p.xyCnf.xy[d - 2].value = i(a(o + d, 1, r), 0, 255, 0);
                        var b = h.getCmdValue("00010203", "04050607", e);
                        n.globalData.cmd.settingData.valArr[0] = i(a(1, 2, b), 1, 512, 1), n.globalData.cmd.settingData.ch = a(3, 1, b), n.globalData.cmd.settingData.valArr[1] = i(a(4, 1, b), 10, 100, 10), n.globalData.cmd.settingData.xy = i(a(5, 1, b), 0, 7, 0), n.globalData.cmd.settingData.valArr[2] = i(a(6, 1, b), 0, 255, 255), n.globalData.cmd.settingData.valArr[3] = i(a(7, 1, b), 0, 255, 255), n.globalData.cmd.settingData.valArr[4] = i(a(8, 1, b), 0, 255, 255), n.globalData.cmd.settingData.light = i(a(9, 1, b), 1, 3, 3), n.globalData.cmd.settingData.cfg = i(a(10, 1, b), 0, 255, 0);
                        var g = h.getCmdValue("D0D1D2D3", "D4D5D6D7", e);
                        if ("" != g) {
                            var j = n.globalData.getDeviceFeatures(),
                                x = 16;
                            t("log", "features", JSON.stringify(j), " at utils/bluCtrl.js:96"), h.getFeaturesValue({
                                features: j
                            }, "xyCnf") && (x = 22);
                            for (var V = [], f = i(a(1, 1, g), 0, 255, 0), F = 127 & f, k = 0; k < F; k++) {
                                for (var m = {
                                        playTime: 0,
                                        cnfValus: []
                                    }, P = 0; P < x; P++) {
                                    var u = i(a(3 + k * x + P, 1, g), 0, 255, 0);
                                    m.cnfValus.push(u), 13 == P && (m.playTime = (u / 10).toFixed())
                                }
                                t("log", "pis.cnfValus", JSON.stringify(m.cnfValus), " at utils/bluCtrl.js:111"), V.push(m)
                            }
                            n.globalData.cmd.pgsData.pisList = V
                        }
                        var X = h.getCmdValue("F0F1F2F3", "F4F5F6F7", e);
                        if ("" != X)
                            for (var N = n.globalData.cmd.drawData.pisObj, H = 0; H < 15; H++) {
                                var z = i(a(H + 1, 1, X), 0, 255, 0);
                                H < N.cnfValus.length && (N.cnfValus[H] = z), 14 == H && (N.txPointTime = z)
                            }
                    }
                }
            }).call(this, r("f3b9")["default"]),

            
