        "5a85": function(e, t, r) {
            "use strict";
            (function(e) {
                Object.defineProperty(t, "__esModule", {
                    value: !0
                }), t.default = void 0;
                var r = getApp(),
                    n = {
                        data: function() {
                            var e = r.globalData.getDeviceInfo();
                            return {
                                items: [{
                                    name: "0",
                                    value: "\u6b63\u5f0f\u6a21\u7ec4",
                                    checked: "true"
                                }, {
                                    name: "1",
                                    value: "\u6d4b\u8bd5\u6a21\u7ec4"
                                }, {
                                    name: "2",
                                    value: "\u4e0d\u6307\u5b9a"
                                }],
                                nowsel: 0,
                                devVers: [{
                                    dev: 0,
                                    ver: 0
                                }, {
                                    dev: 0,
                                    ver: 1
                                }, {
                                    dev: 0,
                                    ver: 2
                                }, {
                                    dev: 0,
                                    ver: 3
                                }, {
                                    dev: 0,
                                    ver: 4
                                }, {
                                    dev: 0,
                                    ver: 5
                                }, {
                                    dev: 1,
                                    ver: 0
                                }, {
                                    dev: 1,
                                    ver: 1
                                }, {
                                    dev: 1,
                                    ver: 2
                                }, {
                                    dev: 1,
                                    ver: 3
                                }, {
                                    dev: 1,
                                    ver: 4
                                }, {
                                    dev: 1,
                                    ver: 5
                                }, {
                                    dev: 2,
                                    ver: 0
                                }, {
                                    dev: 2,
                                    ver: 1
                                }, {
                                    dev: 2,
                                    ver: 2
                                }, {
                                    dev: 2,
                                    ver: 3
                                }, {
                                    dev: 2,
                                    ver: 4
                                }, {
                                    dev: 2,
                                    ver: 5
                                }],
                                devInf: e
                            }
                        },
                        onLoad: function(e) {
                            this.nowsel = r.globalData.muuidSel
                        },
                        methods: {
                            radioVerChange: function(e) {
                                var t = parseInt(e.detail.value),
                                    n = this.devVers[t];
                                this.devInf.deviceType = n.dev, this.devInf.version = n.ver, r.globalData.setDeviceInfo(this.devInf.deviceType, this.devInf.version, this.devInf.userType), this.backok()
                            },
                            radioChange: function(t) {
                                e("log", "radio\u53d1\u751fchange\u4e8b\u4ef6\uff0c\u643a\u5e26value\u503c\u4e3a\uff1a", t.detail.value, " at pages/setuuid/setuuid.vue:76"), this.nowsel = parseInt(t.detail.value)
                            },
                            backok: function() {
                                r.globalData.savelastsel(this.nowsel), r.globalData.readSetting(), uni.navigateBack({
                                    delta: 1
                                })
                            }
                        }
                    };
                t.default = n
            }).call(this, r("f3b9")["default"])
        },