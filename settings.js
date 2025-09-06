        "072a": function(e, t, r) {
            "use strict";
            r.d(t, "b", (function() {
                return h
            })), r.d(t, "c", (function() {
                return a
            })), r.d(t, "a", (function() {
                return n
            }));
            var n = {
                    pageMeta: r("7854").default,
                    navigationBar: r("7943").default
                },
                h = function() {
                    var e = this,
                        t = e.$createElement,
                        r = e._self._c || t;
                    return r("view", [r("page-meta", {
                        attrs: {
                            "page-style": "overflow: hidden",
                            "root-font-size": e.screen_width,
                            _i: 1
                        }
                    }, [r("navigation-bar", {
                        attrs: {
                            title: e.ntitle,
                            "background-color": "#10161C",
                            "color-animation-timing-func": "easeIn",
                            _i: 2
                        }
                    })], 1), r("view", {
                        class: e._$s(3, "c", e.rtl ? "rtl" : ""),
                        attrs: {
                            _i: 3
                        }
                    }, [r("view", [r("view", [r("view", [e._$s(7, "i", -1 != e.dmx) ? r("view", {
                        staticClass: e._$s(7, "sc", "display-btn-group"),
                        attrs: {
                            _i: 7
                        }
                    }, [r("view", [r("image", {
                        staticClass: e._$s(9, "sc", "ico-size"),
                        attrs: {
                            _i: 9
                        }
                    }), r("label", {
                        staticClass: e._$s(10, "sc", "display-btn-lable"),
                        attrs: {
                            _i: 10
                        }
                    }, [e._v(e._$s(10, "t0-0", e._s(e.$t("DMX\u5730\u5740\u7801"))))])]), r("view", [r("view", {
                        staticClass: e._$s(12, "sc", "btn-view-style2"),
                        attrs: {
                            _i: 12
                        },
                        on: {
                            longpress: e.subNumPress,
                            touchend: e.subNumPressEnd,
                            click: e.subNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(13, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 13
                        }
                    }, [r("label", {
                        staticClass: e._$s(14, "sc", "btn-color-text"),
                        attrs: {
                            _i: 14
                        }
                    })])]), r("input", {
                        directives: [{
                            name: "model",
                            rawName: "v-model",
                            value: e.valArr[0],
                            expression: "valArr[0]"
                        }],
                        staticClass: e._$s(15, "sc", "display-input-num"),
                        attrs: {
                            _i: 15
                        },
                        domProps: {
                            value: e._$s(15, "v-model", e.valArr[0])
                        },
                        on: {
                            input: [function(t) {
                                t.target.composing || e.$set(e.valArr, 0, t.target.value)
                            }, e.inputEvent],
                            blur: e.onInputBlur
                        }
                    }), r("view", {
                        staticClass: e._$s(16, "sc", "btn-view-style2"),
                        attrs: {
                            _i: 16
                        },
                        on: {
                            longpress: e.addNumPress,
                            touchend: e.addNumPressEnd,
                            click: e.addNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(17, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 17
                        }
                    }, [r("label", {
                        staticClass: e._$s(18, "sc", "btn-color-text"),
                        attrs: {
                            _i: 18
                        }
                    })])])])]) : e._e(), e._$s(19, "i", -1 == e.dmx) ? r("view", {
                        attrs: {
                            _i: 19
                        }
                    }, [r("view", {
                        staticClass: e._$s(20, "sc", "display-btn-group"),
                        attrs: {
                            _i: 20
                        }
                    }, [r("view", {
                        staticClass: e._$s(21, "sc", "view-row-class"),
                        attrs: {
                            _i: 21
                        }
                    }, [r("label", {
                        staticClass: e._$s(22, "sc", "btn-color-text2"),
                        attrs: {
                            _i: 22
                        }
                    }, [e._v(e._$s(22, "t0-0", e._s(e.$t("\u5730\u5740\u7801"))))]), r("view", {
                        staticClass: e._$s(23, "sc", "btn-view-style"),
                        attrs: {
                            _i: 23
                        },
                        on: {
                            longpress: e.subNumPress,
                            touchend: e.subNumPressEnd,
                            click: e.subNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(24, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 24
                        }
                    }, [r("label", {
                        staticClass: e._$s(25, "sc", "btn-color-text"),
                        attrs: {
                            _i: 25
                        }
                    })])]), r("input", {
                        directives: [{
                            name: "model",
                            rawName: "v-model",
                            value: e.valArr[0],
                            expression: "valArr[0]"
                        }],
                        staticClass: e._$s(26, "sc", "display-input-num"),
                        attrs: {
                            _i: 26
                        },
                        domProps: {
                            value: e._$s(26, "v-model", e.valArr[0])
                        },
                        on: {
                            input: [function(t) {
                                t.target.composing || e.$set(e.valArr, 0, t.target.value)
                            }, e.inputEvent],
                            blur: e.onInputBlur
                        }
                    }), r("view", {
                        staticClass: e._$s(27, "sc", "btn-view-style"),
                        attrs: {
                            _i: 27
                        },
                        on: {
                            longpress: e.addNumPress,
                            touchend: e.addNumPressEnd,
                            click: e.addNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(28, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 28
                        }
                    }, [r("label", {
                        staticClass: e._$s(29, "sc", "btn-color-text"),
                        attrs: {
                            _i: 29
                        }
                    })])])]), r("view", {
                        staticClass: e._$s(30, "sc", "view-row-class"),
                        attrs: {
                            _i: 30
                        }
                    }, [r("label", {
                        staticClass: e._$s(31, "sc", "btn-color-text2"),
                        attrs: {
                            _i: 31
                        }
                    }, [e._v(e._$s(31, "t0-0", e._s(e.$t("\u663e\u793a\u8303\u56f4"))))]), r("view", {
                        staticClass: e._$s(32, "sc", "btn-view-style"),
                        attrs: {
                            _i: 32
                        },
                        on: {
                            longpress: e.subNumPress,
                            touchend: e.subNumPressEnd,
                            click: e.subNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(33, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 33
                        }
                    }, [r("label", {
                        staticClass: e._$s(34, "sc", "btn-color-text"),
                        attrs: {
                            _i: 34
                        }
                    })])]), r("input", {
                        directives: [{
                            name: "model",
                            rawName: "v-model",
                            value: e.valArr1,
                            expression: "valArr1"
                        }],
                        staticClass: e._$s(35, "sc", "display-input-num"),
                        attrs: {
                            _i: 35
                        },
                        domProps: {
                            value: e._$s(35, "v-model", e.valArr1)
                        },
                        on: {
                            input: [function(t) {
                                t.target.composing || (e.valArr1 = t.target.value)
                            }, e.inputEvent],
                            blur: e.onInputBlur
                        }
                    }), r("view", {
                        staticClass: e._$s(36, "sc", "btn-view-style"),
                        attrs: {
                            _i: 36
                        },
                        on: {
                            longpress: e.addNumPress,
                            touchend: e.addNumPressEnd,
                            click: e.addNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(37, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 37
                        }
                    }, [r("label", {
                        staticClass: e._$s(38, "sc", "btn-color-text"),
                        attrs: {
                            _i: 38
                        }
                    })])])])]), r("view", [r("radio-group", {
                        attrs: {
                            _i: 40
                        },
                        on: {
                            change: e.radioChange
                        }
                    }, [r("view", {
                        staticClass: e._$s(41, "sc", "display-xy-group"),
                        attrs: {
                            _i: 41
                        }
                    }, [r("view", {
                        staticClass: e._$s(42, "sc", "display-xy-border"),
                        style: e._$s(42, "s", e.rtl ? "margin-left: 20rem;" : "margin-right: 20rem;"),
                        attrs: {
                            _i: 42
                        }
                    }, [r("label", [e._v(e._$s(43, "t0-0", e._s(e.$t("\u6b63\u5e38\u663e\u793a"))))]), r("view", [r("view", {
                        staticClass: e._$s(45, "sc", "display-xy-radio"),
                        style: e._$s(45, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 45
                        }
                    }, [r("radio", {
                        attrs: {
                            checked: e._$s(46, "a-checked", 0 == e.xy),
                            _i: 46
                        }
                    }), r("text", {
                        staticClass: e._$s(47, "sc", "text-xy-class"),
                        style: e._$s(47, "s", 0 == e.xy ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 47
                        }
                    })]), r("view", {
                        staticClass: e._$s(48, "sc", "display-xy-radio"),
                        style: e._$s(48, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 48
                        }
                    }, [r("radio", {
                        attrs: {
                            checked: e._$s(49, "a-checked", 1 == e.xy),
                            _i: 49
                        }
                    }), r("text", {
                        staticClass: e._$s(50, "sc", "text-xy-class"),
                        style: e._$s(50, "s", 1 == e.xy ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 50
                        }
                    })]), r("view", {
                        staticClass: e._$s(51, "sc", "display-xy-radio"),
                        style: e._$s(51, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 51
                        }
                    }, [r("radio", {
                        attrs: {
                            checked: e._$s(52, "a-checked", 2 == e.xy),
                            _i: 52
                        }
                    }), r("text", {
                        staticClass: e._$s(53, "sc", "text-xy-class"),
                        style: e._$s(53, "s", 2 == e.xy ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 53
                        }
                    })]), r("view", {
                        staticClass: e._$s(54, "sc", "display-xy-radio"),
                        style: e._$s(54, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 54
                        }
                    }, [r("radio", {
                        attrs: {
                            checked: e._$s(55, "a-checked", 3 == e.xy),
                            _i: 55
                        }
                    }), r("text", {
                        staticClass: e._$s(56, "sc", "text-xy-class"),
                        style: e._$s(56, "s", 3 == e.xy ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 56
                        }
                    })])])]), r("view", {
                        staticClass: e._$s(57, "sc", "display-xy-border"),
                        style: e._$s(57, "s", e.rtl ? "margin-right: 20rem;" : "margin-left: 20rem;"),
                        attrs: {
                            _i: 57
                        }
                    }, [r("label", [e._v(e._$s(58, "t0-0", e._s(e.$t("XY\u4e92\u6362"))))]), r("view", [r("view", {
                        staticClass: e._$s(60, "sc", "display-xy-radio"),
                        style: e._$s(60, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 60
                        }
                    }, [r("radio", {
                        attrs: {
                            checked: e._$s(61, "a-checked", 4 == e.xy),
                            _i: 61
                        }
                    }), r("text", {
                        staticClass: e._$s(62, "sc", "text-xy-class"),
                        style: e._$s(62, "s", 4 == e.xy ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 62
                        }
                    })]), r("view", {
                        staticClass: e._$s(63, "sc", "display-xy-radio"),
                        style: e._$s(63, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 63
                        }
                    }, [r("radio", {
                        attrs: {
                            checked: e._$s(64, "a-checked", 5 == e.xy),
                            _i: 64
                        }
                    }), r("text", {
                        staticClass: e._$s(65, "sc", "text-xy-class"),
                        style: e._$s(65, "s", 5 == e.xy ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 65
                        }
                    })]), r("view", {
                        staticClass: e._$s(66, "sc", "display-xy-radio"),
                        style: e._$s(66, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 66
                        }
                    }, [r("radio", {
                        attrs: {
                            checked: e._$s(67, "a-checked", 6 == e.xy),
                            _i: 67
                        }
                    }), r("text", {
                        staticClass: e._$s(68, "sc", "text-xy-class"),
                        style: e._$s(68, "s", 6 == e.xy ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 68
                        }
                    })]), r("view", {
                        staticClass: e._$s(69, "sc", "display-xy-radio"),
                        style: e._$s(69, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 69
                        }
                    }, [r("radio", {
                        attrs: {
                            checked: e._$s(70, "a-checked", 7 == e.xy),
                            _i: 70
                        }
                    }), r("text", {
                        staticClass: e._$s(71, "sc", "text-xy-class"),
                        style: e._$s(71, "s", 7 == e.xy ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 71
                        }
                    })])])])])])]), r("view", {
                        staticClass: e._$s(72, "sc", "view-rgb-class"),
                        attrs: {
                            _i: 72
                        }
                    }, [r("view", {
                        staticClass: e._$s(73, "sc", "view-row-class"),
                        attrs: {
                            _i: 73
                        }
                    }, [r("view", {
                        staticClass: e._$s(74, "sc", "btn-view-color-class"),
                        attrs: {
                            _i: 74
                        }
                    }, [r("view"), r("label", {
                        staticClass: e._$s(76, "sc", "btn-view-text"),
                        attrs: {
                            _i: 76
                        }
                    }, [e._v(e._$s(76, "t0-0", e._s(e.$t("\u7ea2\u5149\u8c03\u5149"))))])]), r("view", {
                        staticClass: e._$s(77, "sc", "view-rgb-inut"),
                        style: e._$s(77, "s", "opacity: " + (0 != e.cfg ? "1;" : "0.5; pointer-events: none;")),
                        attrs: {
                            _i: 77
                        }
                    }, [r("view", {
                        staticClass: e._$s(78, "sc", "btn-view-style"),
                        attrs: {
                            _i: 78
                        },
                        on: {
                            longpress: e.subNumPress,
                            touchend: e.subNumPressEnd,
                            click: e.subNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(79, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 79
                        }
                    }, [r("label", {
                        staticClass: e._$s(80, "sc", "btn-color-text"),
                        attrs: {
                            _i: 80
                        }
                    })])]), r("input", {
                        directives: [{
                            name: "model",
                            rawName: "v-model",
                            value: e.valArr2,
                            expression: "valArr2"
                        }],
                        staticClass: e._$s(81, "sc", "display-input-num"),
                        attrs: {
                            _i: 81
                        },
                        domProps: {
                            value: e._$s(81, "v-model", e.valArr2)
                        },
                        on: {
                            input: function(t) {
                                t.target.composing || (e.valArr2 = t.target.value)
                            }
                        }
                    }), r("view", {
                        staticClass: e._$s(82, "sc", "btn-view-style"),
                        attrs: {
                            _i: 82
                        },
                        on: {
                            longpress: e.addNumPress,
                            touchend: e.addNumPressEnd,
                            click: e.addNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(83, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 83
                        }
                    }, [r("label", {
                        staticClass: e._$s(84, "sc", "btn-color-text"),
                        attrs: {
                            _i: 84
                        }
                    })])])])]), r("view", {
                        staticClass: e._$s(85, "sc", "view-row-class"),
                        attrs: {
                            _i: 85
                        }
                    }, [r("view", {
                        staticClass: e._$s(86, "sc", "btn-view-color-class"),
                        attrs: {
                            _i: 86
                        }
                    }, [r("view"), r("label", {
                        staticClass: e._$s(88, "sc", "btn-view-text"),
                        attrs: {
                            _i: 88
                        }
                    }, [e._v(e._$s(88, "t0-0", e._s(e.$t("\u7eff\u5149\u8c03\u5149"))))])]), r("view", {
                        staticClass: e._$s(89, "sc", "view-rgb-inut"),
                        style: e._$s(89, "s", "opacity: " + (0 != e.cfg ? "1;" : "0.5; pointer-events: none;")),
                        attrs: {
                            _i: 89
                        }
                    }, [r("view", {
                        staticClass: e._$s(90, "sc", "btn-view-style"),
                        attrs: {
                            _i: 90
                        },
                        on: {
                            longpress: e.subNumPress,
                            touchend: e.subNumPressEnd,
                            click: e.subNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(91, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 91
                        }
                    }, [r("label", {
                        staticClass: e._$s(92, "sc", "btn-color-text"),
                        attrs: {
                            _i: 92
                        }
                    })])]), r("input", {
                        directives: [{
                            name: "model",
                            rawName: "v-model",
                            value: e.valArr3,
                            expression: "valArr3"
                        }],
                        staticClass: e._$s(93, "sc", "display-input-num"),
                        attrs: {
                            _i: 93
                        },
                        domProps: {
                            value: e._$s(93, "v-model", e.valArr3)
                        },
                        on: {
                            input: function(t) {
                                t.target.composing || (e.valArr3 = t.target.value)
                            }
                        }
                    }), r("view", {
                        staticClass: e._$s(94, "sc", "btn-view-style"),
                        attrs: {
                            _i: 94
                        },
                        on: {
                            longpress: e.addNumPress,
                            touchend: e.addNumPressEnd,
                            click: e.addNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(95, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 95
                        }
                    }, [r("label", {
                        staticClass: e._$s(96, "sc", "btn-color-text"),
                        attrs: {
                            _i: 96
                        }
                    })])])])]), r("view", {
                        staticClass: e._$s(97, "sc", "view-row-class"),
                        attrs: {
                            _i: 97
                        }
                    }, [r("view", {
                        staticClass: e._$s(98, "sc", "btn-view-color-class"),
                        attrs: {
                            _i: 98
                        }
                    }, [r("view"), r("label", {
                        staticClass: e._$s(100, "sc", "btn-view-text"),
                        attrs: {
                            _i: 100
                        }
                    }, [e._v(e._$s(100, "t0-0", e._s(e.$t("\u84dd\u5149\u8c03\u5149"))))])]), r("view", {
                        staticClass: e._$s(101, "sc", "view-rgb-inut"),
                        style: e._$s(101, "s", "opacity: " + (0 != e.cfg ? "1;" : "0.5; pointer-events: none;")),
                        attrs: {
                            _i: 101
                        }
                    }, [r("view", {
                        staticClass: e._$s(102, "sc", "btn-view-style"),
                        attrs: {
                            _i: 102
                        },
                        on: {
                            longpress: e.subNumPress,
                            touchend: e.subNumPressEnd,
                            click: e.subNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(103, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 103
                        }
                    }, [r("label", {
                        staticClass: e._$s(104, "sc", "btn-color-text"),
                        attrs: {
                            _i: 104
                        }
                    })])]), r("input", {
                        directives: [{
                            name: "model",
                            rawName: "v-model",
                            value: e.valArr4,
                            expression: "valArr4"
                        }],
                        staticClass: e._$s(105, "sc", "display-input-num"),
                        attrs: {
                            _i: 105
                        },
                        domProps: {
                            value: e._$s(105, "v-model", e.valArr4)
                        },
                        on: {
                            input: function(t) {
                                t.target.composing || (e.valArr4 = t.target.value)
                            }
                        }
                    }), r("view", {
                        staticClass: e._$s(106, "sc", "btn-view-style"),
                        attrs: {
                            _i: 106
                        },
                        on: {
                            longpress: e.addNumPress,
                            touchend: e.addNumPressEnd,
                            click: e.addNum
                        }
                    }, [r("view", {
                        staticClass: e._$s(107, "sc", "btn-view-sub"),
                        attrs: {
                            _i: 107
                        }
                    }, [r("label", {
                        staticClass: e._$s(108, "sc", "btn-color-text"),
                        attrs: {
                            _i: 108
                        }
                    })])])])])]), r("view", [r("radio-group", {
                        staticClass: e._$s(110, "sc", "radio-group-class"),
                        attrs: {
                            _i: 110
                        },
                        on: {
                            change: e.radioChange
                        }
                    }, [r("label", {
                        staticClass: e._$s(111, "sc", "btn-color-text2"),
                        style: e._$s(111, "s", e.rtl ? "margin-left: 20rem;width: 140rem;" : "margin-right: 20rem;"),
                        attrs: {
                            _i: 111
                        }
                    }, [e._v(e._$s(111, "t0-0", e._s(e.$t("\u6fc0\u5149\u5149\u6e90"))))]), r("view", [r("label", {
                        staticClass: e._$s(113, "sc", "fun-radio"),
                        style: e._$s(113, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 113
                        }
                    }, [r("radio", {
                        attrs: {
                            disabled: e._$s(114, "a-disabled", e.showCtr.light1),
                            checked: e._$s(114, "a-checked", 1 == e.light),
                            _i: 114
                        }
                    }), r("text", {
                        staticClass: e._$s(115, "sc", "text-xy-class"),
                        style: e._$s(115, "s", 1 == e.light ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 115
                        }
                    }, [e._v(e._$s(115, "t0-0", e._s(e.$t("\u5355\u8272"))))])]), e._$s(116, "i", e.showCtr.light2) ? r("label", {
                        staticClass: e._$s(116, "sc", "fun-radio"),
                        style: e._$s(116, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 116
                        }
                    }, [r("radio", {
                        attrs: {
                            disabled: e._$s(117, "a-disabled", e.rdDisabled),
                            checked: e._$s(117, "a-checked", 2 == e.light),
                            _i: 117
                        }
                    }), r("text", {
                        staticClass: e._$s(118, "sc", "text-xy-class"),
                        style: e._$s(118, "s", 2 == e.light ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 118
                        }
                    }, [e._v(e._$s(118, "t0-0", e._s(e.$t("\u53cc\u8272"))))])]) : e._e(), r("label", {
                        staticClass: e._$s(119, "sc", "fun-radio"),
                        style: e._$s(119, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 119
                        }
                    }, [r("radio", {
                        attrs: {
                            disabled: e._$s(120, "a-disabled", e.showCtr.light3),
                            checked: e._$s(120, "a-checked", 3 == e.light),
                            _i: 120
                        }
                    }), r("text", {
                        staticClass: e._$s(121, "sc", "text-xy-class"),
                        style: e._$s(121, "s", 3 == e.light ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 121
                        }
                    }, [e._v(e._$s(121, "t0-0", e._s(e.$t("\u5168\u5f69"))))])]), e._$s(122, "i", e.showCtr.lightExt) ? r("label", {
                        staticClass: e._$s(122, "sc", "fun-radio"),
                        attrs: {
                            _i: 122
                        }
                    }) : e._e()])]), r("radio-group", {
                        staticClass: e._$s(123, "sc", "radio-group-class"),
                        attrs: {
                            _i: 123
                        },
                        on: {
                            change: e.radioChange
                        }
                    }, [r("label", {
                        staticClass: e._$s(124, "sc", "btn-color-text2"),
                        style: e._$s(124, "s", e.rtl ? "margin-left: 20rem;width: 140rem;" : "margin-right: 20rem;"),
                        attrs: {
                            _i: 124
                        }
                    }, [e._v(e._$s(124, "t0-0", e._s(e.$t("\u8c03\u5236\u6a21\u5f0f"))))]), r("view", [r("label", {
                        staticClass: e._$s(126, "sc", "fun-radio"),
                        style: e._$s(126, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 126
                        }
                    }, [r("radio", {
                        attrs: {
                            disabled: e._$s(127, "a-disabled", !e.features.ttlAn),
                            checked: e._$s(127, "a-checked", 0 == e.cfg),
                            _i: 127
                        }
                    }), r("text", {
                        staticClass: e._$s(128, "sc", "text-xy-class"),
                        style: e._$s(128, "s", 0 == e.cfg ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 128
                        }
                    })]), r("label", {
                        staticClass: e._$s(129, "sc", "fun-radio"),
                        style: e._$s(129, "s", e.rtl ? "justify-content: right;" : ""),
                        attrs: {
                            _i: 129
                        }
                    }, [r("radio", {
                        attrs: {
                            disabled: e._$s(130, "a-disabled", !e.features.ttlAn),
                            checked: e._$s(130, "a-checked", 255 == e.cfg),
                            _i: 130
                        }
                    }), r("text", {
                        staticClass: e._$s(131, "sc", "text-xy-class"),
                        style: e._$s(131, "s", 255 == e.cfg ? "color: #76CEE7;" : ""),
                        attrs: {
                            _i: 131
                        }
                    })]), r("label", {
                        staticClass: e._$s(132, "sc", "fun-radio"),
                        attrs: {
                            _i: 132
                        }
                    })])]), r("radio-group", {
                        staticClass: e._$s(133, "sc", "radio-group-class"),
                        attrs: {
                            _i: 133
                        },
                        on: {
                            change: e.radioChange
                        }
                    }, [r("label", {
                        staticClass: e._$s(134, "sc", "btn-color-text2"),
                        style: e._$s(134, "s", e.rtl ? "margin-left: 20rem;width: 140rem;" : "margin-right: 20rem;"),
                        attrs: {
                            _i: 134
                        }
                    }, [e._v(e._$s(134, "t0-0", e._s(e.$t("\u7cfb\u7edf\u8bed\u8a00"))))]), r("label", {
                        staticClass: e._$s(135, "sc", "fun-radio"),
                        style: e._$s(135, "s", "flex: 1; " + (e.rtl ? "justify-content: right;" : "")),
                        attrs: {
                            _i: 135
                        },
                        on: {
                            click: e.selectLang
                        }
                    }, [r("text", {
                        staticClass: e._$s(136, "sc", "text-xy-class"),
                        style: e._$s(136, "s", (e.rtl ? "padding-right: 36rem;" : "padding-left: 36rem;") + " color: #76CEE7; text-decoration: underline; text-underline-offset: 12rem;"),
                        attrs: {
                            _i: 136
                        }
                    }, [e._v(e._$s(136, "t0-0", e._s(e.langName)))])])]), r("view", {
                        staticClass: e._$s(137, "sc", "radio-group-class"),
                        attrs: {
                            _i: 137
                        }
                    }, [r("label", {
                        staticClass: e._$s(138, "sc", "btn-color-text2"),
                        style: e._$s(138, "s", e.rtl ? "margin-left: 20rem;width: 140rem;" : "margin-right: 20rem;"),
                        attrs: {
                            _i: 138
                        }
                    }, [e._v(e._$s(138, "t0-0", e._s(e.$t("\u8f6f\u4ef6\u7248\u672c"))))]), r("label", {
                        staticClass: e._$s(139, "sc", "fun-radio"),
                        style: e._$s(139, "s", "flex: 1; " + (e.rtl ? "justify-content: right;" : "")),
                        attrs: {
                            _i: 139
                        }
                    }, [r("text", {
                        staticClass: e._$s(140, "sc", "text-xy-class"),
                        style: e._$s(140, "s", (e.rtl ? "padding-right: 36rem;" : "padding-left: 36rem;") + " color: #76CEE7;"),
                        attrs: {
                            _i: 140
                        }
                    }, [e._v(e._$s(140, "t0-0", e._s(e.version ? e.version : "1.1.1")))])])]), r("view", {
                        staticClass: e._$s(141, "sc", "radio-group-class"),
                        attrs: {
                            _i: 141
                        }
                    }, [r("label", {
                        staticClass: e._$s(142, "sc", "btn-color-text2"),
                        style: e._$s(142, "s", e.rtl ? "margin-left: 20rem;width: 140rem;" : "margin-right: 20rem;"),
                        attrs: {
                            _i: 142
                        }
                    }, [e._v(e._$s(142, "t0-0", e._s(e.$t("\u5f53\u524d\u673a\u578b"))))]), r("label", {
                        staticClass: e._$s(143, "sc", "fun-radio"),
                        style: e._$s(143, "s", "flex: 1; align-content: center; " + (e.rtl ? "justify-content: right;" : "")),
                        attrs: {
                            _i: 143
                        }
                    }, [r("text", {
                        staticClass: e._$s(144, "sc", "text-xy-class"),
                        style: e._$s(144, "s", (e.rtl ? "padding-right: 36rem;" : "padding-left: 36rem;") + " color: #76CEE7;"),
                        attrs: {
                            _i: 144
                        }
                    }, [e._v(e._$s(144, "t0-0", e._s(e.machine)))])])])])]) : e._e()])])])])], 1)
                },
                a = []
