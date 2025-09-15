(this["webpackJsonp"] = this["webpackJsonp"] || []).push([

    ["app-service"], {
		
		
		"mainPageComponent": function(e, t, r) {
        "use strict";
            (function(logger) {

                var app = r("appStateManager")["default"],
                geometryUtil = (r("codePointAt"), r("geometryAndUuidUtils")),
                deviceCommandUtil = r("deviceCommandUtils "),
                bleDeviceController = (r("handDrawFileManager"), r("bleDeviceControlUtils ")),
                pages = ["pages/cnn/cnn", "pages/main/main", "pages/lang/lang", "pages/setting/setting"],
                 module = {
                        data: function() {
                            var deviceFeatures = app.globalData.getDeviceFeatures();
                            return {
                                screen_width: app.globalData.screen_width_str,
                                rtl: app.globalData.rtl,
                                debugTag: !1,
                                ledDevTag: !1,
                                warnTop: 20,
                                modeCmdSend: "",
                                functions: [{
                                    tag: 8,
                                    name: "Hand-drawn doodle",
                                    img: "2.png",
                                    show: !0
                                }, {
                                    tag: 4,
                                    name: "Text playback",
                                    img: "3.png",
                                    show: !0
                                }, {
                                    tag: 7,
                                    name: "Personalized programming",
                                    img: "4.png",
                                    show: !0
                                }, {
                                    tag: 1,
                                    name: "Random playback",
                                    img: "5.png",
                                    show: !0
                                }, {
                                    tag: 3,
                                    name: "Animation playback",
                                    img: "6.png",
                                    show: !0
                                }, {
                                    tag: 2,
                                    name: "Timeline playback",
                                    img: "7.png",
                                    show: !0
                                }, {
                                    tag: 5,
                                    name: "Christmas broadcast",
                                    img: "8.png",
                                    show: !0
                                }, {
                                    tag: 6,
                                    name: "Outdoor playback",
                                    img: "9.png",
                                    show: !0
                                }, {
                                    tag: 0,
                                    name: "DMX",
                                    img: "10.png",
                                    show: !0
                                }, {
                                    tag: 5,
                                    name: "ILDA",
                                    img: "11.png",
                                    show: !1
                                }, {
                                    tag: 9,
                                    name: "Playlist",
                                    img: "12.png",
                                    show: !0
                                }],
                                features: deviceFeatures,
                                deviceOn: !1,
                                prjIndex: -1,
                                cnnDevice: "Not connected",
                                cnnState: !1,
                                bluTimer: null,
                                randomCheck: [],
                                initShow: !1,
                                ctx: "null",
                                sendTimer: null,
                                lastSendTime: 0,
                                lastCmdTime: 0,

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
                                },
                                lastRefresh: 0,
                                chPer: 1,
                                chBeginPoint: {
                                    x: 0,
                                    y: 0
                                },
                                chEndPoint: {
                                    x: 0,
                                    y: 0
                                },
                                chDraw: {
                                    w: 0,
                                    h: 0,
                                    max: 255
                                },
                                chCanvas: {
                                    w: 0,
                                    h: 0
                                },
                                cnfIdx: 0
                            }
                        },
                        created: function() {
                            uni.setKeepScreenOn({
                                keepScreenOn: !0
                            }), app.globalData.setMainPage(this);
                            var e = uni.getSystemInfoSync();
                        },
                        onLoad: function() {
                            this.genRandomCheck(), app.globalData.platform.app ? this.warnTop = 12 : this.warnTop = 40
                        },
                        onShow: function() {
                            if (this.rtl = app.globalData.rtl, logger("log", "onShow rtl", this.rtl, " at pages/main/main.js:85"), this.features = app.globalData.getDeviceFeatures(), this.bluTimer && uni.showLoading({
                                    title: this.$t("Reading device parameters..."),
                                    mask: !0
                                }), !this.initShow) {
                                this.initShow = !0;
                                var t = this;
                                this.$nextTick((function() {
                                    t.testShow((function() {
                                        t.bluInitPro()
                                    }))
                                }))
                            }
                        },
                        //When the page is ready, this function logs a message and, if LED device mode is enabled, initializes the channel drawing UI.
                        onReady: function() {
                            logger("log", "main onready", " at pages/main/main.js:99"), this.ledDevTag && this.chDrawInit()
                        },
                        computed: {
                            functionsShow: function() {
                                for (var t = [], r = 0; r < this.functions.length; r++) {
                                    var n = this.functions[r];
                                    if (this.features.ilda) {
                                        if (["Christmas broadcast"].includes(n.name)) continue
                                    } else if (["ILDA"].includes(n.name)) continue;
                                    !this.features.picsPlay && ["Playlist"].includes(n.name) || t.push(n)
                                }
                                return logger("log", "functionsShow", JSON.stringify(t), " at pages/main/main.js:121"), t
                            }
                        },
                        methods: {
                            bluInitPro: function() {
                                app.globalData.blu_cnn_call_back = this.blu_cnn_call_back, app.globalData.blu_rec_call_back = this.blu_rec_call_back, setTimeout((function() {
                                    bleDeviceController.cnnPreBlu()
                                }), 10)
                            },
                            clearBluTimer: function() {
                                null != this.bluTimer && (clearTimeout(this.bluTimer), this.bluTimer = null)
                            },
                            goQueryCmd: function() {
                                var t = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : 3,
                                    r = this;
                                if (t > 0) "pages/main/main" == this.getCurPage() && uni.showLoading({
                                    title: r.$t("Reading device parameters..."),
                                    mask: !0
                                }), bleDeviceController.gosend(!1, deviceCommandUtil.getQueryCmd(this.randomCheck)), r.clearBluTimer(), r.bluTimer = setTimeout((function() {
                                    r.bluTimer = null, r.goQueryCmd(--t)
                                }), 3e3);
                                else {
                                    if (logger("log", "this.debugTag", this.debugTag, " at pages/main/main.js:162"), r.clearBluTimer(), this.debugTag) return void bleDeviceController.setCanSend(!0);
                                    uni.showToast({
                                        title: r.$t("Failed to read device parameters"),
                                        icon: "none",
                                        duration: 3e3
                                    })
                                }
                            },
                            blu_cnn_call_back: function(t, r) {
                                if (1 != t) {
                                    var n = app.globalData.ble_device;
                                    if (t && n && "characteristicId" in n) logger("log", "blu_cnn_call_back1", t, r, " at pages/main/main.js:191"), logger("log", "Connected", n.name, " at pages/main/main.js:192"), this.cnnDevice = n.name, this.cnnState = !0, app.globalData.blu_cnn_from_page && "pages/cnn/cnn" == this.getCurPage() && uni.navigateBack(), app.globalData.blu_cnn_from_page = !1, this.goQueryCmd();
                                    else {
                                        if (logger("log", "blu_cnn_call_back2", t, r, " at pages/main/main.js:202"), this.clearBluTimer(), uni.hideLoading(), this.cnnState = !1, this.deviceOn = !1, this.prjIndex = -1, app.globalData.blu_cnn_from_test) return;
                                        var h = this.getCurPage(); - 1 == pages.indexOf(h) && (n && app.globalData.showModalTips(n.name + this.$t("Disconnected")), this.gotoMain())
                                    }
                                }
                            },
                            gotoMain: function() {
                                var t = arguments.length > 0 && void 0 !== arguments[0] && arguments[0];
                                if (!this.debugTag && (logger("log", "gotoMain cnnBlu", t, this.debugTag, app.globalData.img_selecting, " at pages/main/main.js:223"), !app.globalData.appHide)) {
                                    var r = this.getCurPage();
                                    "pages/main/main" != r ? uni.reLaunch({
                                        url: "/pages/main/main"
                                    }) : t && (this.initShow = !1)
                                }
                            },
                            getCurPage: function() {
                                var e = getCurrentPages();
                                return e[e.length - 1].route
                            },

                            // This function handles the response after receiving data from a Bluetooth device, 
                            // validates it, updates the app state if valid, and notifies the user if there’s an error.
                            blu_rec_call_back: function(data) {
                                logger("log", "blu_rec_call_back", " at pages/main/main.js:270");
                                this.clearBluTimer(), uni.hideLoading(), this.checkRcvData(data, this.randomCheck) ? (bleDeviceController.setCanSend(!0), bleDeviceController.setCmdData(data), this.prjIndex = app.globalData.cmd.curMode) : uni.showToast({
                                    title: this.$t("Abnormality in reading device parameters"),
                                    icon: "none",
                                    duration: 3e3
                                })
                            },
                            
                            // fill the this.randomCheck array with 4 random integers between 0 and 255 (inclusive). 
                            // This is typically used to generate a random check value, possibly for device communication or session validation.
                            genRandomCheck: function() {
                                for (var e = 0; e < 4; e++) this.randomCheck[e] = Math.floor(256 * Math.random())
                            },
                        
                            // Validates the received data string and random check array.
                            // Decodes and checks a checksum validation code.
                            // Updates device status and features if valid.
                            // Returns true if the data is valid and processed, otherwise false.
                            checkRcvData: function(e, t) {
                                if (4 != t.length || e.length < 24) return !1;
                                for (var r = e.substr(e.length - 24, 8), n = [], h = 0; h < 4; h++) {
                                    var i = 0,
                                        c = t[h];
                                    0 == h && (i = (c + 55 >> 1) - 10 & 255), 1 == h && (i = 7 + (c - 68 << 1) & 255), 2 == h && (i = 15 + (c + 97 >> 1) & 255), 3 == h && (i = 87 + (c - 127 >> 1) & 255), n.push(i)
                                }
                                for (var o = [], s = 0; s < 4; s++) {
                                    var l = r[2 * s] + r[2 * s + 1],
                                        p = parseInt(l, 16);
                                    o.push(p)
                                }
                                for (var d = 0; d < 4; d++)
                                    if (o[d] != n[d]) return !1;
                                var b = e.substr(e.length - 16, 2);
                                this.deviceOn = 0 != parseInt(b, 16);
                                var g = e.substr(e.length - 14, 2),
                                    j = e.substr(e.length - 12, 2),
                                    x = e.substr(e.length - 10, 2);
                                return this.debugTag || (app.globalData.setDeviceInfo(g, j, x), this.features = app.globalData.getDeviceFeatures()), this.features = app.globalData.getDeviceFeatures(), !0
                            },

                            voteTitle: function(e) {
                                this.sendText = e.detail.value
                            },

                            t: function(e) {
                                var t = this.$t(e);
                                return t
                            },
                            cnnLaser: function() {
                                bleDeviceController.cnnLaser()
                            },

                            // decides whether to switch to DMX mode and send a command, 
                            // navigate to the settings page, or prompt the user to turn on the device, 
                            // based on the clicked tag and device/debug state.
                            settingClick: function(e) {
                                var t = e.currentTarget.dataset.tag;
                                if (0 != t || this.deviceOn || this.debugTag) return this.prjIndex != t && 0 == t ? (this.prjIndex = t, app.globalData.setCmdMode(t), void this.sendCmd()) : void uni.navigateTo({
                                    url: "/pages/setting/setting?dmx=" + t
                                });
                                app.globalData.showModalTips(this.$t("Please turn on the device first"), !0)
                            },
                            
                            // This function handles toggling the device's power state and 
                            // sending the appropriate command, with user feedback if the action cannot be performed.
                            onOffChange: function(t) {
                                if (bleDeviceController.getCanSend()) {
                                    this.deviceOn = !this.deviceOn, logger("log", "this.deviceOn", this.deviceOn, " at pages/main/main.js:379");
                                    var r = "B0B1B2B300B4B5B6B7";
                                    this.deviceOn && (r = "B0B1B2B3FFB4B5B6B7"), bleDeviceController.gosend(!1, r)
                                } else this.cnnState ? app.globalData.showModalTips(this.$t("The current device cannot be identified"), !0) : app.globalData.showModalTips(this.$t("\u8bf7\u5148\u8fde\u63a5Bluetooth"), !0)
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
                                var e = deviceCommandUtil.getCmdStr(app.globalData.cmd, {
                                    features: this.features
                                });
                                this.modeCmdSend = e, this.doSendCmd()
                            },

                            // doSendCmd tries to send the current command. If it fails, it retries every 100ms until successful. 
                            // When successful, it clears the command buffer.
                            doSendCmd: function() {
                                if ("" != this.modeCmdSend) {
                                    var e = bleDeviceController.gosend(!1, this.modeCmdSend),
                                        t = this;
                                    e ? this.modeCmdSend = "" : setTimeout((function() {
                                        t.doSendCmd()
                                    }), 100)
                                }
                            },
                            //  routes the user to the correct function or project page, 
                            // sending the necessary command to the device, 
                            // and handles device state checks and navigation logic.
                            prjClick: function(e) {
                                
                                var t = e.currentTarget.dataset.tag;
                                
                                if (0 != t)
                                    if (this.deviceOn || this.debugTag) {
                                        
                                        if (this.prjIndex != t || 5 == t && this.features.ilda) return this.prjIndex = t, 
                                            app.globalData.setCmdMode(t), void this.sendCmd();
                                        
                                        this.sendCmd(), 4 == t && uni.navigateTo({
                                            url: "/sub/pages/text/text"
                                        }), 7 == t && uni.navigateTo({
                                            url: "/sub2/pages/pgs/pgs"
                                        }), 8 == t && uni.navigateTo({
                                            url: "/sub/pages/draw/draw"
                                        }), 9 == t && uni.navigateTo({
                                            url: "/sub/pages/listMaster/listMaster"
                                        }), t >= 1 && t <= 6 && 4 != t && uni.navigateTo({
                                            url: "/pages/prj/prj?tag=" + t
                                        })
                                    } else app.globalData.showModalTips(this.$t("Please turn on the device first"), !0);
                                else this.settingClick(e)
                            },

                            //  ensures that shake commands are sent only when the device is ready, 
                            // and retries if a previous command is still being processed. It prevents overlapping sends and manages timing for reliable communication.
                            sendCmd2: function(t) {
                                if (logger("log", "app.globalData.blu_data_cmdSending", app.globalData.blu_data_cmdSending, " at pages/main/main.js:489"), app.globalData.blu_data_cmdSending) {
                                    if (null == this.sendTimer) {
                                        var r = this;
                                        this.sendTimer = setTimeout((function() {
                                            r.sendTimer = null, r.sendCmd2(t)
                                        }), 100)
                                    }
                                } else if (!(this.lastCmdTime < this.lastSendTime)) {
                                    var n = app.globalData.getDeviceFeatures(),
                                        h = deviceCommandUtil.getShakeCmdStr(app.globalData.cmd, {
                                            features: n,
                                            xyCnfSave: t
                                        }),
                                        i = bleDeviceController.gosend(!1, h);
                                    i && (this.lastSendTime = (new Date).getTime())
                                }
                            },
                            sendLastCmd: function(e) {
                                this.lastCmdTime = (new Date).getTime(), this.sendCmd2(e)
                            },
                            chDrawInit: function() {
                                var e = this,
                                    t = uni.createSelectorQuery().in(e);
                                setTimeout((function() {
                                    t.select("#cvView").boundingClientRect((function(t) {
                                        e.chCanvas.w = t.width / 4, e.chCanvas.h = t.height, e.chDraw.w = .6 * e.chCanvas.w;
                                        var r = .95 * e.chCanvas.h;
                                        e.chPer = r / 255, e.chDraw.h = r, e.refreshAllChDraw()
                                    })).exec()
                                }), 200)
                            },                
                            getIteratorHelper: function(e, t) {
                            // This function provides an iterator for the given object 'e'.
                            },
                            refreshAllChDraw: function() {
                                var e, t = getIteratorHelper(this.xyCnf.xy);
                                try {
                                    for (t.s(); !(e = t.n()).done;) {
                                        var r = e.value,
                                            h = r["value"],
                                            a = "chCanvas" + r["name"],
                                            i = !this.xyCnf.auto;
                                        this.drawChCanvas(a, this.chDraw.w, this.chDraw.h, this.chDraw.max, h, i, this.callBackCh)
                                    }
                                } catch (c) {
                                    t.e(c)
                                } finally {
                                    t.f()
                                }
                            },
                            refreshChDraw: function() {
                                var e = this.xyCnf.xy[this.cnfIdx],
                                    t = e["value"],
                                    r = "chCanvas" + e["name"],
                                    n = !this.xyCnf.auto;
                                this.drawChCanvas(r, this.chDraw.w, this.chDraw.h, this.chDraw.max, t, n, this.callBackCh)
                            },
                            radioPhaseChange: function(e) {
                                var t = e.detail.value,
                                    r = parseInt(t);
                                this.$set(this.xyCnf, "phase", r), this.sendLastCmd(!0)
                            },
                            radioChange: function(e) {
                                var t = e.detail.value,
                                    r = this.xyCnf.auto;
                                "auto" == t && (r = !0), "manual" == t && (r = !1), this.$set(this.xyCnf, "auto", r), this.refreshAllChDraw(), this.sendLastCmd(!0)
                            },
                            drawChCanvas: function(e, t, r, n, h, a) {
                                var c = arguments.length > 6 && void 0 !== arguments[6] ? arguments[6] : null,
                                    o = uni.createCanvasContext(e, this),
                                    s = t / 2,
                                    l = (this.chCanvas.w - t) / 2,
                                    p = (this.chCanvas.h - r) / 2 + s,
                                    d = l + t,
                                    b = p + r - t,
                                    g = l + s,
                                    j = p,
                                    x = d - s,
                                    V = b,
                                    f = 2 * this.scUnit,
                                    F = "#24292E",
                                    k = o.createLinearGradient(x, V + s, g, j - s);
                                k.addColorStop(0, "#112233"), k.addColorStop(1, "#1E374C"), a ? o.setFillStyle(k) : o.setFillStyle(F), o.beginPath(), o.moveTo(d, b), o.arc(x, V, s, 0, 1 * Math.PI);
                                var m = r - 2 * s;
                                o.rect(d - t, b - m, t, m), o.moveTo(l, p), o.arc(g, j, s, Math.PI, 2 * Math.PI), o.fill();
                                var P = o.createLinearGradient(x, V + s, g, j - s);
                                P.addColorStop(0, "#008BD1"), P.addColorStop(1, "white"), a ? o.setFillStyle(P) : o.setFillStyle(F), o.beginPath(), o.moveTo(l, p), o.arc(g, j, s, Math.PI, 2 * Math.PI), o.moveTo(d, b), o.arc(x, V, s, 0, 1 * Math.PI), o.beginPath();
                                var u = r / n,
                                    X = u * h;
                                if (X < s) {
                                    var N = s - X,
                                        H = s - Math.sqrt(Math.pow(s, 2) - Math.pow(N, 2)),
                                        z = geometryUtil.lineTheta([d, b], [x, V], [d - H, b + N]);
                                    o.moveTo(d - H, b + N), o.arc(x, V, s, z, Math.PI - z), o.fill()
                                } else if (X <= r - s) {
                                    o.moveTo(d, b), o.arc(x, V, s, 0, 1 * Math.PI);
                                    var Q = X - s;
                                    o.rect(d - t, b - Q, t, Q), o.fill()
                                } else {
                                    o.moveTo(d, b), o.arc(x, V, s, 0, 1 * Math.PI);
                                    var R = r - 2 * s;
                                    if (o.rect(d - t, b - R, t, R), h == n) o.moveTo(l, p), o.arc(g, j, s, Math.PI, 2 * Math.PI);
                                    else {
                                        var v = X - (r - s),
                                            I = s - Math.sqrt(Math.pow(s, 2) - Math.pow(v, 2)),
                                            w = geometryUtil.lineTheta([l, p], [g, j], [l + I, p - v]);
                                        o.moveTo(l + I, p - v), o.arc(g, j, s, 2 * Math.PI - w, Math.PI + w)
                                    }
                                    o.fill()
                                }
                                o.beginPath();
                                var y = .5 * t;
                                if (o.setFontSize(y), a ? o.setFillStyle("white") : o.setFillStyle(F), o.setShadow(5 * f, 5 * f, 5 * f, "rgba(0, 0, 0, 0.5)"), o.fillText(h + "", x - o.measureText(h + "").width / 2, j - s + r / 2 + y / 2), o.beginPath(), y = .8 * t, o.setFontSize(y), o.fillText("+", g - o.measureText("+").width / 2, j + y / 3), o.fillText("-", x - o.measureText("-").width / 2, V + y / 3), null != c) {
                                    var C = l,
                                        A = V + s;
                                    c(o, C, A, t, r, s, n, h)
                                }
                                o.draw()
                            },

                            //  changes a channel value by a given amount, ensures it stays within limits,
                            //  updates the UI, and sends the new value to the device if it changed.
                            addCnfValusAndSend: function(e) {
                                if (!this.xyCnf.auto) {
                                    var t = this.xyCnf.xy[this.cnfIdx].value,
                                        r = t + Math.floor(e);
                                    r = r < 0 ? 0 : r, r = r > this.chDraw.max ? this.chDraw.max : r, t != r && (this.$set(this.xyCnf.xy[this.cnfIdx], "value", r), this.refreshChDraw(), this.lastCmdTime = (new Date).getTime(), this.sendLastCmd(!1))
                                }
                            },

                            // chTouchstart initializes the state for a channel adjustment gesture, recording which channel is being adjusted and where the touch started.
                            chTouchstart: function(e) {
                                this.cnfIdx = e.currentTarget.dataset.idx;
                                var t = e.touches[0];
                                this.chBeginPoint = {
                                    x: t.x,
                                    y: t.y
                                }, this.chEndPoint = null, this.lastRefresh = 0
                            },
                            // chTouchmove tracks finger movement, throttles updates to every 100ms, and adjusts the channel value in response to vertical drag gestures.
                            chTouchmove: function(e) {
                                var t = e.touches[0];
                                this.chEndPoint = {
                                    x: t.x,
                                    y: t.y
                                };
                                var r = (new Date).getTime();
                                if (r - this.lastRefresh > 100) {
                                    var n = Math.floor((this.chBeginPoint.y - this.chEndPoint.y) / this.chPer);
                                    Math.abs(n) >= 1 && (this.chBeginPoint = {
                                        x: this.chEndPoint.x,
                                        y: this.chEndPoint.y
                                    }, this.addCnfValusAndSend(n)), this.lastRefresh = r
                                }
                            },
                            //
                            chTouchend: function(e) {
                                if (null == this.chEndPoint) {
                                    var t = this.chBeginPoint.y > this.chCanvas.h / 2 ? -1 : 1;
                                    this.addCnfValusAndSend(t)
                                }
                                this.chEndPoint = null
                            }
                        }
                    };
                t.default = module
            }).call(this, r("enhancedConsoleLogger")["default"])
        },

        "appStateManager": function(e, t, r) {
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

                        // List of Bluetooth service UUIDs to connect to
                        mserviceuuids: [],
                        // List of Bluetooth characteristic UUIDs for transmitting (TX) data
                        mtxduuids: [],
                        // List of Bluetooth characteristic UUIDs for receiving (RX) data
                        mrxduuids: [],
                        // 0,1,2
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
                        //  global "stop BLE operations" flag
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
                        setbluDataSendInterval: function(e) {
                            this.blu_data_send_interval = e
                        },

                        // invokes the registered Bluetooth receive callback with the provided data, 
                        //  if a callback is set.
                        setRecCallBack: function(e) {
                            var t = this.blu_rec_call_back;
                            null != t && t(e)
                        },

                        // updates the Bluetooth connection state, saves the device if connected, 
                        // and notifies any registered callback.
                        setBluCnnState: function(connectionState , isManualChange) {
                            this.blu_connected = connectionState , 2 == this.blu_connected && this.saveDevice();
                            var connectionCallback  = this.blu_cnn_call_back;
                            null != connectionCallback  && connectionCallback (connectionState , isManualChange)
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
                            if ("prjData" == key) return this.cmd[key].public = data.public, 1 != data.prjIndex && (this.cmd[key].prjItem[data.prjIndex + ""] = data.item), this.cmd.textData.runSpeed = data.public.runSpeed, void(this.cmd.textData.txColor = data.public.txColor);
                            this.cmd[key] = data, "textData" == key && (this.cmd.prjData.public.runSpeed = data.runSpeed, this.cmd.prjData.public.txColor = data.txColor)
                        },

                        //restores the last used Bluetooth UUID configuration by reading a saved index and updating the relevant UUID arrays for device communication.
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

                        // getDeviceFeatures() returns an object indicating which features are supported by the current device, 
                        // based on its type and version.
                        getDeviceFeatures: function() {
                            var e = {
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
                            return (1 == deviceType && version >= 1 || 0 == deviceType && version >= 2 || deviceType >= 2) && (e.textStopTime = !0, e.textDecimalTime = !0), (1 == deviceType && version >= 2 || deviceType > 1) && (e.showOutDoorTips = !0), 1 == deviceType && 1 == version && (e.textModeFix01 = !0), 2 == deviceType && (e.xyCnf = !0), 1 != deviceType && 2 != deviceType || (e.ilda = !0), 1 != deviceType && 2 != deviceType || (e.ttlAn = !0), (deviceType >= 2 || version >= 3) && (e.arbPlay = !0), (deviceType >= 3 || version >= 4) && (e.textUpDown = !0), (deviceType >= 3 || version >= 5) && (e.picsPlay = !0), 1 == deviceType && (e.animationFix = !0), e.displayType = deviceType, e
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
                                        e("log", "openBluetoothAdapter2", h, " at App.vue:450"), t.doCloseBluetoothAdapter(), 10001 === h.errCode && t.showModalTips(n.$t("\u8bf7\u68c0\u67e5\u624b\u673aBluetooth\u662f\u5426\u542f\u7528"), !0), 103 == h.errno ? t.showModalTips(n.$t("\u8bf7Settings\u5c0f\u7a0b\u5e8fBluetooth\u6743\u9650"), !0) : t.showModalTips("Open Bluetooth Adapter Fail"), r && r(!1)
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
            }).call(this, r("enhancedConsoleLogger")["default"])
        },

        "appStateManager": function(e, t, r) {
            "use strict";
            (function(e) {

                var r = {
                    globalData: {
                        $i18n: {
                            locale: "en-US"
                        },
                        $t: {},
                        MaxSaveFileCount: 50,
                        MaxListCount: 200,
                        // reference to the main page component
                        mainPage: null,
                        cloudApi: null,
                        appHide: !1,

                        // List of Bluetooth service UUIDs to connect to
                        mserviceuuids: [],
                        // List of Bluetooth characteristic UUIDs for transmitting (TX) data
                        mtxduuids: [],
                        // List of Bluetooth characteristic UUIDs for receiving (RX) data
                        mrxduuids: [],
                        // 0,1,2
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
                        //  global "stop BLE operations" flag
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
                        setbluDataSendInterval: function(e) {
                            this.blu_data_send_interval = e
                        },

                        // invokes the registered Bluetooth receive callback with the provided data, 
                        //  if a callback is set.
                        setRecCallBack: function(e) {
                            var t = this.blu_rec_call_back;
                            null != t && t(e)
                        },

                        // updates the Bluetooth connection state, saves the device if connected, 
                        // and notifies any registered callback.
                        setBluCnnState: function(connectionState , isManualChange) {
                            this.blu_connected = connectionState , 2 == this.blu_connected && this.saveDevice();
                            var connectionCallback  = this.blu_cnn_call_back;
                            null != connectionCallback  && connectionCallback (connectionState , isManualChange)
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
                            if ("prjData" == key) return this.cmd[key].public = data.public, 1 != data.prjIndex && (this.cmd[key].prjItem[data.prjIndex + ""] = data.item), this.cmd.textData.runSpeed = data.public.runSpeed, void(this.cmd.textData.txColor = data.public.txColor);
                            this.cmd[key] = data, "textData" == key && (this.cmd.prjData.public.runSpeed = data.runSpeed, this.cmd.prjData.public.txColor = data.txColor)
                        },

                        //restores the last used Bluetooth UUID configuration by reading a saved index and updating the relevant UUID arrays for device communication.
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

                        // getDeviceFeatures() returns an object indicating which features are supported by the current device, 
                        // based on its type and version.
                        getDeviceFeatures: function() {
                            var e = {
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
                            return (1 == deviceType && version >= 1 || 0 == deviceType && version >= 2 || deviceType >= 2) && (e.textStopTime = !0, e.textDecimalTime = !0), (1 == deviceType && version >= 2 || deviceType > 1) && (e.showOutDoorTips = !0), 1 == deviceType && 1 == version && (e.textModeFix01 = !0), 2 == deviceType && (e.xyCnf = !0), 1 != deviceType && 2 != deviceType || (e.ilda = !0), 1 != deviceType && 2 != deviceType || (e.ttlAn = !0), (deviceType >= 2 || version >= 3) && (e.arbPlay = !0), (deviceType >= 3 || version >= 4) && (e.textUpDown = !0), (deviceType >= 3 || version >= 5) && (e.picsPlay = !0), 1 == deviceType && (e.animationFix = !0), e.displayType = deviceType, e
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
                                        e("log", "openBluetoothAdapter2", h, " at App.vue:450"), t.doCloseBluetoothAdapter(), 10001 === h.errCode && t.showModalTips(n.$t("\u8bf7\u68c0\u67e5\u624b\u673aBluetooth\u662f\u5426\u542f\u7528"), !0), 103 == h.errno ? t.showModalTips(n.$t("\u8bf7Settings\u5c0f\u7a0b\u5e8fBluetooth\u6743\u9650"), !0) : t.showModalTips("Open Bluetooth Adapter Fail"), r && r(!1)
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
        }
 
			
	},
	[
        ["mainAppEntry", "app-config"]
    ]
]);