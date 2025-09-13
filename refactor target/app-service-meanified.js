(this["webpackJsonp"] = this["webpackJsonp"] || []).push([
    ["app-service"], {
        "mainLayoutComponent": function(e, t, r) {
            "use strict";
            r.d(t, "b", (function() {
                return h
            })), r.d(t, "c", (function() {
                return a
            })), r.d(t, "a", (function() {
                return n
            }));
            var n = {
                    pageMeta: r("pageMetaComponentExportWrapper").default,
                    navigationBar: r("navigationBarComponentExportWrapper").default,
                    uniPopup: r("uniPopupComponentExportWrapper").default
                },
                h = function() {
                    var e = this,
                        t = e.$createElement,
                        n = e._self._c || t;
                    return n("view", [n("page-meta", {
                        attrs: {
                            "page-style": "overflow: hidden",
                            "root-font-size": e.screen_width,
                            _i: 1
                        }
                    }, [n("navigation-bar", {
                        attrs: {
                            "background-color": "#10161C",
                            "color-animation-timing-func": "easeIn",
                            title: e.ntitle,
                            _i: 2
                        }
                    })], 1), n("view", {
                        class: e._$s(3, "c", e.rtl ? "rtl" : ""),
                        attrs: {
                            _i: 3
                        }
                    }, [n("view", [n("view", [n("view", [n("view", {
                        staticClass: e._$s(7, "sc", "display-color-group"),
                        attrs: {
                            _i: 7
                        }
                    }, [n("view", [n("view", e._l(2, (function(t, r, h, a) {
                        return n("view", {
                            key: r,
                            staticClass: e._$s("10-" + a, "sc", "btn-color-group"),
                            attrs: {
                                _i: "10-" + a
                            }
                        }, e._l(5, (function(t, h, i, c) {
                            return n("view", {
                                key: h
                            }, [n("view", {
                                staticClass: e._$s("12-" + a + "-" + c, "sc", "btn-view-style"),
                                attrs: {
                                    "data-tag": e._$s("12-" + a + "-" + c, "a-data-tag", e.colorDisplayOrder[5 * r + h].idx),
                                    _i: "12-" + a + "-" + c
                                },
                                on: {
                                    click: e.btnColorChange
                                }
                            }, [n("view", {
                                staticClass: e._$s("13-" + a + "-" + c, "sc", "btn-view-sub"),
                                attrs: {
                                    _i: "13-" + a + "-" + c
                                }
                            }, [e._$s("14-" + a + "-" + c, "i", 5 * r + h < 7) ? n("view", {
                                staticClass: e._$s("14-" + a + "-" + c, "sc", "btn-view-color"),
                                style: e._$s("14-" + a + "-" + c, "s", "background-color: " + e.colorDisplayOrder[5 * r + h].color + ";"),
                                attrs: {
                                    _i: "14-" + a + "-" + c
                                }
                            }) : e._e(), n("label", {
                                staticClass: e._$s("15-" + a + "-" + c, "sc", "btn-color-text"),
                                attrs: {
                                    _i: "15-" + a + "-" + c
                                }
                            }, [e._v(e._$s("15-" + a + "-" + c, "t0-0", e._s(e.$t(e.colorDisplayOrder[5 * r + h].name))))])])]), n("view", {
                                directives: [{
                                    name: "show",
                                    rawName: "v-show",
                                    value: e._$s("16-" + a + "-" + c, "v-show", e.lineColor == e.colorDisplayOrder[5 * r + h].idx),
                                    expression: "_$s((\"16-\"+$30+'-'+$31),'v-show',lineColor == colorDisplayOrder[i * 5 + n].idx)"
                                }],
                                staticClass: e._$s("16-" + a + "-" + c, "sc", "btn-color-title"),
                                attrs: {
                                    _i: "16-" + a + "-" + c
                                }
                            })])
                        })), 0)
                    })), 0), n("view", e._l(2, (function(t, r, h, a) {
                        return n("view", {
                            key: r,
                            staticClass: e._$s("18-" + a, "sc", "btn-color-group"),
                            attrs: {
                                _i: "18-" + a
                            }
                        }, e._l(5, (function(t, h, i, c) {
                            return n("view", {
                                key: h
                            }, [n("view", {
                                staticClass: e._$s("20-" + a + "-" + c, "sc", "btn-view-style"),
                                attrs: {
                                    "data-tag": e._$s("20-" + a + "-" + c, "a-data-tag", e.segDisplayOrder[5 * r + h].idx),
                                    _i: "20-" + a + "-" + c
                                },
                                on: {
                                    click: e.btnColorChange
                                }
                            }, [n("view", {
                                class: e._$s("21-" + a + "-" + c, "c", "btn-view-sub " + e.segDisplayOrder[5 * r + h].color),
                                attrs: {
                                    _i: "21-" + a + "-" + c
                                }
                            })]), n("view", {
                                directives: [{
                                    name: "show",
                                    rawName: "v-show",
                                    value: e._$s("22-" + a + "-" + c, "v-show", e.lineColor == e.segDisplayOrder[5 * r + h].idx),
                                    expression: "_$s((\"22-\"+$32+'-'+$33),'v-show',lineColor == segDisplayOrder[i2 * 5 + n2].idx)"
                                }],
                                staticClass: e._$s("22-" + a + "-" + c, "sc", "btn-color-title"),
                                attrs: {
                                    _i: "22-" + a + "-" + c
                                }
                            })])
                        })), 0)
                    })), 0)])]), n("view", {
                        attrs: {
                            id: "drawCanvasContainer1",
                            _i: 23
                        }
                    }, [n("view", [n("view", {
                        staticClass: e._$s(25, "sc", "display-btn-group"),
                        attrs: {
                            _i: 25
                        }
                    }, [n("view", {
                        attrs: {
                            _i: 26
                        },
                        on: {
                            click: e.fontSelect
                        }
                    }, [n("view", {
                        staticClass: e._$s(27, "sc", "fun-radio"),
                        style: e._$s(27, "s", e.rtl ? "justify-content: right;margin-right: 20rem;" : ""),
                        attrs: {
                            _i: 27
                        }
                    }, [n("image", {
                        staticClass: e._$s(28, "sc", "font-input-img"),
                        attrs: {
                            _i: 28
                        }
                    }), n("text", {
                        staticClass: e._$s(29, "sc", "font-input-font"),
                        style: e._$s(29, "s", e.rtl ? "margin-right: 10rem; " : "margin-left: 10rem; "),
                        attrs: {
                            _i: 29
                        }
                    }, [e._v(e._$s(29, "t0-0", e._s(e.fontNameList[e.fontIdex] ? e.$t(e.fontNameList[e.fontIdex].name) : "")))])])]), n("image", {
                        staticClass: e._$s(30, "sc", "draw-img-select"),
                        attrs: {
                            _i: 30
                        },
                        on: {
                            click: e.drawAddClick
                        }
                    }), n("input", {
                        directives: [{
                            name: "model",
                            rawName: "v-model",
                            value: e.inputTextX,
                            expression: "inputTextX"
                        }],
                        staticClass: e._$s(31, "sc", "text-input"),
                        style: e._$s(31, "s", e.rtl ? "text-align: right;" : ""),
                        attrs: {
                            placeholder: e._$s(31, "a-placeholder", e.inputNote),
                            _i: 31
                        },
                        domProps: {
                            value: e._$s(31, "v-model", e.inputTextX)
                        },
                        on: {
                            blur: e.inputBlur,
                            input: function(t) {
                                t.target.composing || (e.inputTextX = t.target.value)
                            }
                        }
                    }), n("image", {
                        staticClass: e._$s(32, "sc", "draw-img-select"),
                        style: e._$s(32, "s", e.rtl ? "margin-left: 20rem;" : "margin-right: 20rem;"),
                        attrs: {
                            _i: 32
                        },
                        on: {
                            click: e.chooseImag
                        }
                    })]), e._$s(33, "i", e.features.textStopTime) ? n("view", {
                        staticClass: e._$s(33, "sc", "display-btn-group"),
                        attrs: {
                            _i: 33
                        }
                    }, [n("label", {
                        staticClass: e._$s(34, "sc", "display-btn-lable2"),
                        attrs: {
                            _i: 34
                        }
                    }, [e._v(e._$s(34, "t0-0", e._s(e.$t("\u6587\u672c\u56fe\u5f62\u7cbe\u5ea6"))))]), n("slider", {
                        staticClass: e._$s(35, "sc", "fun-slider"),
                        attrs: {
                            value: e._$s(35, "a-value", e.pisObj.txPointTime),
                            _i: 35
                        },
                        on: {
                            change: e.slPointTimeChange
                        }
                    })]) : e._e(), n("view", {
                        staticClass: e._$s(36, "sc", "drawCanvasContainer0"),
                        attrs: {
                            id: "drawCanvasContainer0",
                            _i: 36
                        }
                    }, [n("view", {
                        directives: [{
                            name: "show",
                            rawName: "v-show",
                            value: e._$s(37, "v-show", e.showCanvas),
                            expression: "_$s(37,'v-show',showCanvas)"
                        }],
                        staticClass: e._$s(37, "sc", "draw-Canvas-Container"),
                        style: e._$s(37, "s", "width: " + e.drawCanvas.w + "px;direction: ltr;"),
                        attrs: {
                            _i: 37
                        }
                    }, [n("canvas", {
                        style: e._$s(38, "s", "height: " + e.drawCanvas.h + "px; width:100%; background-color: #0D1B22; border-radius: 4rem;"),
                        attrs: {
                            id: "imgCanvas",
                            _i: 38
                        }
                    }), n("canvas", {
                        staticClass: e._$s(39, "sc", "draw-Canvas-Sub0"),
                        style: e._$s(39, "s", "height: " + e.drawCanvas.h + "px; width:100%; border-radius: 4rem;"),
                        attrs: {
                            id: "drawCanvas",
                            _i: 39
                        }
                    }), n("canvas", {
                        staticClass: e._$s(40, "sc", "draw-Canvas-Sub"),
                        style: e._$s(40, "s", "height: " + e.drawCanvas.h + "px; width: " + e.drawCanvas.w + "px;"),
                        attrs: {
                            id: "drawCanvasSub",
                            _i: 40
                        },
                        on: {
                            touchcancel: e.touchEndSub,
                            touchend: e.touchEndSub,
                            touchmove: e.touchMoveSub,
                            touchstart: e.touchStartSub
                        }
                    })])]), n("view", {
                        attrs: {
                            id: "btn_draw_group",
                            _i: 41
                        }
                    }, [n("view", {
                        staticClass: e._$s(42, "sc", "btn-draw-group"),
                        style: e._$s(42, "s", "flex-wrap: " + e.btnDrawGroup.wrap + "; overflow-x: " + e.btnDrawGroup.x + ";overflow-y: " + e.btnDrawGroup.y + " width: " + e.btnDrawGroup.w + "px; height: " + e.btnDrawGroup.h + "px;"),
                        attrs: {
                            _i: 42
                        }
                    }, [n("view", {
                        staticClass: e._$s(43, "sc", "button-container"),
                        attrs: {
                            _i: 43
                        }
                    }, [n("button", {
                        staticClass: e._$s(44, "sc", "btn-draw-item"),
                        style: e._$s(44, "s", "padding: 10rem; " + (-1 == e.drawMode ? "border-color: #CCEEFA; background-color: #4FB8EF;" : "border-color: transparent;")),
                        attrs: {
                            _i: 44
                        },
                        on: {
                            click: e.btnDrawChange
                        }
                    }, [n("image", {
                        attrs: {
                            _i: 45
                        }
                    })])]), n("view", {
                        staticClass: e._$s(46, "sc", "button-container"),
                        attrs: {
                            _i: 46
                        }
                    }, [n("button", {
                        staticClass: e._$s(47, "sc", "btn-draw-item"),
                        style: e._$s(47, "s", "padding: 2rem; " + (9999 == e.drawMode && e.textToLeft ? "border-color: #CCEEFA; background-color: #4FB8EF;" : "border-color: transparent;")),
                        attrs: {
                            _i: 47
                        },
                        on: {
                            click: e.btnDrawChange
                        }
                    }, [n("image", {
                        attrs: {
                            _i: 48
                        }
                    })])]), n("view", {
                        staticClass: e._$s(49, "sc", "button-container"),
                        attrs: {
                            _i: 49
                        }
                    }, [n("button", {
                        staticClass: e._$s(50, "sc", "btn-draw-item"),
                        style: e._$s(50, "s", "padding: 2rem; " + (9999 != e.drawMode || e.textToLeft ? "border-color: transparent;" : "border-color: #CCEEFA; background-color: #4FB8EF;")),
                        attrs: {
                            _i: 50
                        },
                        on: {
                            click: e.btnDrawChange
                        }
                    }, [n("image", {
                        attrs: {
                            _i: 51
                        }
                    })])]), e._l(e._$s(52, "f", {
                        forItems: e.objCount
                    }), (function(t, r, h, a) {
                        return n("view", {
                            key: e._$s(52, "f", {
                                forIndex: h,
                                key: r
                            }),
                            staticClass: e._$s("52-" + a, "sc", "button-container"),
                            attrs: {
                                _i: "52-" + a
                            }
                        }, [n("button", {
                            staticClass: e._$s("53-" + a, "sc", "btn-draw-item"),
                            style: e._$s("53-" + a, "s", e.drawMode == r ? "border-color: #CCEEFA; background-color: #4FB8EF;" : "border-color: transparent;"),
                            attrs: {
                                "data-tag": e._$s("53-" + a, "a-data-tag", r),
                                _i: "53-" + a
                            },
                            on: {
                                click: e.btnDrawChange
                            }
                        }, [n("image", {
                            attrs: {
                                src: e._$s("54-" + a, "a-src", "/static/imgs/drawMode" + r + ".png"),
                                _i: "54-" + a
                            }
                        })])])
                    }))], 2)])])]), n("view", [n("view", [n("view", {
                        staticClass: e._$s(57, "sc", "opr-btn"),
                        style: e._$s(57, "s", (e.rtl ? "border-bottom-right-radius: 40rem; border-top-right-radius: 40rem; " : "border-bottom-left-radius: 40rem; border-top-left-radius: 40rem;") + "color: red; background-color: #51D1EA;"),
                        attrs: {
                            _i: 57
                        },
                        on: {
                            click: e.operateAciton
                        }
                    }, [e._v(e._$s(57, "t0-0", e._s(e.$t(e.selectLines.length > 0 ? "\u5220\u9664" : "\u6e05\u7a7a"))))]), n("view", {
                        staticClass: e._$s(58, "sc", "opr-btn btn-linear btn-back"),
                        style: e._$s(58, "s", e.rtl ? "border-bottom-left-radius: 40rem; border-top-left-radius: 40rem; " : "border-bottom-right-radius: 40rem; border-top-right-radius: 40rem;"),
                        attrs: {
                            _i: 58
                        },
                        on: {
                            click: e.backDraw
                        }
                    }, [e._v(e._$s(58, "t0-0", e._s(e.$t("\u56de\u9000"))))])]), n("view", [n("view", {
                        staticClass: e._$s(60, "sc", "opr-btn"),
                        style: e._$s(60, "s", (e.rtl ? "border-bottom-right-radius: 40rem; border-top-right-radius: 40rem; " : "border-bottom-left-radius: 40rem; border-top-left-radius: 40rem;") + "color: blue; background-color: #51D1EA;"),
                        attrs: {
                            _i: 60
                        },
                        on: {
                            click: e.parmSet
                        }
                    }, [e._v(e._$s(60, "t0-0", e._s(e.$t("\u914d\u7f6e"))))]), n("view", {
                        staticClass: e._$s(61, "sc", "opr-btn btn-linear"),
                        style: e._$s(61, "s", e.rtl ? "border-bottom-left-radius: 40rem; border-top-left-radius: 40rem;" : "border-bottom-right-radius: 40rem; border-top-right-radius: 40rem; color: white;"),
                        attrs: {
                            _i: 61
                        },
                        on: {
                            click: e.drawDone
                        }
                    }, [e._v(e._$s(61, "t0-0", e._s(e.$t("\u53d1\u9001"))))])])]), e._$s(62, "i", e.showSending) ? [n("view"), n("canvas", {
                        attrs: {
                            id: "progressCanvas",
                            _i: 64
                        }
                    })] : e._e(), n("uni-popup", {
                        ref: "tips",
                        attrs: {
                            "mask-click": !1,
                            animation: !1,
                            _i: 65
                        }
                    }, [n("view", [n("image", {
                        attrs: {
                            src: e._$s(67, "a-src", r("ffb9")),
                            _i: 67
                        }
                    }), n("view", [n("text", [e._v(e._$s(69, "t0-0", e._s(e.$t("hand_draw_tips"))))])]), n("view", [n("checkbox-group", {
                        staticClass: e._$s(71, "sc", "view-gen-line"),
                        attrs: {
                            _i: 71
                        },
                        on: {
                            change: e.tipsCheckboxChange
                        }
                    }, [n("label", {
                        staticClass: e._$s(72, "sc", "label-parm"),
                        attrs: {
                            _i: 72
                        }
                    }, [n("checkbox", {
                        attrs: {
                            _i: 73
                        }
                    }), e._v(e._$s(72, "t1-0", e._s(e.$t("\u4e0d\u518d\u63d0\u793a"))))])]), n("view", {
                        staticClass: e._$s(74, "sc", "opr-btn btn-linear"),
                        attrs: {
                            _i: 74
                        },
                        on: {
                            click: e.tipsClose
                        }
                    }, [e._v(e._$s(74, "t0-0", e._s(e.$t("\u786e\u5b9a"))))])])])]), n("uni-popup", {
                        ref: "popup",
                        attrs: {
                            "mask-click": !1,
                            animation: !1,
                            _i: 75
                        }
                    }, [n("view", [n("view", {
                        attrs: {
                            id: "parent-component",
                            _i: 77
                        }
                    }, [n("view", [n("view", [e._l(e._$s(80, "f", {
                        forItems: e.cnfList,
                        fill: !0
                    }), (function(t, r, h, a) {
                        return [e._$s("81-" + a, "i", t.idx <= 11) ? [n("view", {
                            key: e._$s("82-" + a, "a-key", r + "_0_0"),
                            staticClass: e._$s("82-" + a, "sc", "display-ch-lable"),
                            style: e._$s("82-" + a, "s", (e.cnfIdx == t.idx ? "background-color: #25333D;" : "background-color: #1C2B39;") + "; " + (4 == t.idx ? "height: 60rem; border-top-left-radius: 40rem; border-top-right-radius: 40rem;" : "flex: 1;") + (11 == t.idx ? "height: 60rem; border-bottom-left-radius: 40rem; border-bottom-right-radius: 40rem;" : "flex: 1;")),
                            attrs: {
                                _i: "82-" + a
                            },
                            on: {
                                click: function(r) {
                                    return e.chClick(t.idx)
                                }
                            }
                        }, [n("label", {
                            style: e._$s("83-" + a, "s", (e.cnfIdx == t.idx ? "color: #59E2FF;" : "color: #687C8E;") + (e.rtl ? "text-align: right;" : "text-align: left;") + " flex: 1; font-size: 28rem;"),
                            attrs: {
                                _i: "83-" + a
                            }
                        }, [e._v(e._$s("83-" + a, "t0-0", e._s(t.name)))]), n("text", {
                            style: e._$s("84-" + a, "s", (e.cnfIdx == t.idx ? "color: #59E2FF;" : "color: #687C8E;") + (e.rtl ? "text-align: left;" : "text-align: right;") + " width: 50rem; font-size: 22rem; padding-left: 10rem;"),
                            attrs: {
                                _i: "84-" + a
                            }
                        }, [e._v(e._$s("84-" + a, "t0-0", e._s(e.pisObj.cnfValus[t.idx])))])])] : e._e()]
                    }))], 2), n("view", {
                        style: e._$s(85, "s", (e.rtl ? "margin-right: 20rem; " : "margin-left: 20rem; ") + "flex: 1; background-color: #1C2B39; border-radius: 40rem; direction: ltr;"),
                        attrs: {
                            _i: 85
                        }
                    }, [n("canvas", {
                        attrs: {
                            id: "chCanvas",
                            _i: 86
                        },
                        on: {
                            touchstart: e.chTouchstart,
                            touchmove: e.chTouchmove,
                            touchend: e.chTouchend,
                            touchcancel: e.chTouchend
                        }
                    })])])]), n("view", [n("view", {
                        staticClass: e._$s(88, "sc", "opr-btn"),
                        attrs: {
                            _i: 88
                        },
                        on: {
                            click: e.parmReset
                        }
                    }, [e._v(e._$s(88, "t0-0", e._s(e.$t("\u91cd\u7f6e"))))]), n("view", {
                        staticClass: e._$s(89, "sc", "opr-btn btn-linear"),
                        attrs: {
                            _i: 89
                        },
                        on: {
                            click: e.parmClose
                        }
                    }, [e._v(e._$s(89, "t0-0", e._s(e.$t("\u786e\u5b9a"))))])])])]), n("uni-popup", {
                        ref: "classNamePopup",
                        attrs: {
                            "mask-click": !1,
                            animation: !1,
                            _i: 90
                        }
                    }, [n("view", [n("view", [n("view"), n("view", [e._v(e._$s(94, "t0-0", e._s(e.$t("\u4fdd\u5b58\u81f3\u6587\u4ef6\u5939"))))]), n("picker", {
                        attrs: {
                            value: e._$s(95, "a-value", e.handDrawClassIdx),
                            range: e._$s(95, "a-range", e.handDrawClassName),
                            _i: 95
                        },
                        on: {
                            change: e.handDrawClassPickerChange
                        }
                    }, [n("view", [e._v(e._$s(96, "t0-0", e._s(e.handDrawClassName[e.handDrawClassIdx])))])]), n("view", [e._v(e._$s(97, "t0-0", e._s(e.$t("\u8bf7\u8f93\u5165\u6587\u4ef6\u540d"))))]), n("input", {
                        attrs: {
                            value: e._$s(98, "a-value", e.drawAddFileName),
                            _i: 98
                        },
                        on: {
                            input: e.picNameNewInput
                        }
                    }), n("view", [n("view", {
                        attrs: {
                            _i: 100
                        },
                        on: {
                            click: e.picNameInputCancelClick
                        }
                    }, [e._v(e._$s(100, "t0-0", e._s(e.$t("\u53d6\u6d88"))))]), n("view"), n("view", {
                        attrs: {
                            _i: 102
                        },
                        on: {
                            click: e.picNameInputOkClick
                        }
                    }, [e._v(e._$s(102, "t0-0", e._s(e.$t("\u786e\u5b9a"))))])])]), n("view", {
                        attrs: {
                            _i: 103
                        },
                        on: {
                            click: e.picNameInputCancelClick
                        }
                    })])])], 2)])]), e._$s(104, "i", e.features.xyCnf) ? n("button", {
                        staticClass: e._$s(104, "sc", "floating-button"),
                        style: e._$s(104, "s", {
                            left: e.position.x + "px",
                            top: e.position.y + "px"
                        }),
                        attrs: {
                            _i: 104
                        },
                        on: {
                            touchstart: e.onBtnSetTouchStart,
                            touchmove: e.onBtnSetTouchMove,
                            touchend: e.onBtnSetTouchEnd,
                            click: e.onBtnSetClick
                        }
                    }, [n("image", {
                        attrs: {
                            src: e._$s(105, "a-src", r("barsPng")),
                            _i: 105
                        }
                    })]) : e._e()])], 1)
                },
                a = []
        },
        "interopExportModule": function(e, t, r) {
            "use strict";
            r.r(t);
            var n = r("uniTransitionComponent "),
                h = r.n(n);
            for (var a in n)["default"].indexOf(a) < 0 && function(e) {
                r.d(t, e, (function() {
                    return n[e]
                }))
            }(a);
            t["default"] = h.a
        },
        "fontRegistryModule ": function(e, t, r) {
            var n = r("4e7c"),
                h = [{
                    name: "Single Line Font",
                    file: n.DrawFonts,
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
        "deviceConfigPageComponent ": function(e, t, r) {
            "use strict";
            r.d(t, "b", (function() {
                return h
            })), r.d(t, "c", (function() {
                return a
            })), r.d(t, "a", (function() {
                return n
            }));
            var n = {
                    pageMeta: r("pageMetaComponentExportWrapper").default,
                    navigationBar: r("navigationBarComponentExportWrapper").default
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
        },
        "fontGeometryUtils ": function(e, t) {
            function r(e, t, r) {
                var n = {
                        x: e[0] - t[0],
                        y: e[1] - t[1]
                    },
                    h = {
                        x: r[0] - t[0],
                        y: r[1] - t[1]
                    },
                    a = n.x * h.x + n.y * h.y,
                    i = Math.sqrt(Math.pow(n.x, 2) + Math.pow(n.y, 2)),
                    c = Math.sqrt(Math.pow(h.x, 2) + Math.pow(h.y, 2));
                if (0 == i || 0 == c) return 0;
                var o = Math.acos(a / (i * c)),
                    s = 180 * o / Math.PI;
                return s
            }

            function n(e, t, r) {
                var n = {
                        w: r.w,
                        h: -r.h
                    },
                    h = (e[1] - t[1]) / (t[0] - e[0]),
                    a = -t[1] - h * t[0],
                    i = [];
                a <= 0 && a >= n.h && (t[0] < 0 || e[0] < 0) && (i = [0, -a]);
                var c = h * n.w + a;
                if (c <= 0 && c >= n.h && (t[0] > n.w || e[0] > n.w) && (i = [n.w, -c]), 0 != h) {
                    var o = -a / h;
                    o >= 0 && o <= n.w && (t[1] < 0 || e[1] < 0) && (i = [o, 0]);
                    var s = (n.h - a) / h;
                    s >= 0 && s <= n.w && (t[1] > -n.h || e[1] > -n.h) && (i = [s, -n.h])
                }
                return i.length > 0 && i.push(1), i
            }

            function h(e) {
                return /[\u4E00-\u9FA5]/.test(e)
            }
            e.exports = {
                fontData: null,
                ifHasChinese: function(e) {
                    if (null == e) return !1;
                    for (var t = 0; t < e.length; t++)
                        if (h(e[t])) return !0;
                    return !1
                },
                parseLines: function(e) {
                    for (var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : {
                            w: 800,
                            h: 800
                        }, r = [], h = [], a = 0; a < e.length; a++) {
                        var i = e[a];
                        if (i[0] < 0 || i[0] > t.w || i[1] < 0 || i[1] > t.h) {
                            if (0 != h.length) {
                                var c = n(h[h.length - 1], i, t);
                                c.length > 0 && h.push(c), r.push(h), h = []
                            }
                        } else {
                            if (a > 0 && 0 == h.length) {
                                var o = e[a - 1];
                                if (o[0] < 0 || o[0] > t.w || o[1] < 0 || o[1] > t.h) {
                                    var s = n(o, i, t);
                                    s.length > 0 && h.push(s)
                                }
                            }
                            h.push(i)
                        }
                    }
                    return h.length > 0 && r.push(h), r
                },
                readTTF: function(e, t, r) {
                    2 != t ? plus.io.resolveLocalFileSystemURL("_www/static/app-plus/font/" + e, (function(e) {
                        e.file((function(e) {
                            var n = new plus.io.FileReader;
                            n.onloadend = function(e) {
                                var n = e.target.result,
                                    h = n.split(",")[1];
                                r(h, t)
                            }, n.onerror = function() {
                                uni.hideLoading(), uni.showToast({
                                    title: "\u8bfb\u53d6\u5b57\u4f53\u5931\u8d25",
                                    icon: "none"
                                })
                            }, n.readAsDataURL(e)
                        }))
                    }), (function(e) {
                        uni.hideLoading(), uni.showToast({
                            title: "\u5b57\u4f53\u6587\u4ef6\u89e3\u6790\u5931\u8d25\uff1a" + JSON.stringify(e),
                            icon: "none"
                        })
                    })) : r(e, t)
                },
                dealLine: function(e) {
                    var t = !(arguments.length > 1 && void 0 !== arguments[1]) || arguments[1],
                        n = [],
                        h = e[0];
                    n.push([h[0], h[1], 0, 1]);
                    for (var a = 1; a < e.length - 1; a++) {
                        var i = e[a],
                            c = e[a + 1],
                            o = r(h, i, c);
                        if (0 != o && 180 != o) {
                            var s = o <= 135 ? 1 : 0;
                            t || (s = 0), n.push([i[0], i[1], 1, s]), h = i
                        }
                    }
                    var l = e[e.length - 1];
                    return n.push([l[0], l[1], 1, 1]), n
                }
            }
        },
        "textLineVectorizer ": function(e, t, r) {
            (function(t) {
                var n = r("spreadToArrayHelper"),
                    h = r("arrayConversionHelper"),
                    a = r("arabicPresentationFormsConverter");

                function i(e, t, r, n) {
                    for (var h = [], a = 0; a <= 1; a += 1 / n) {
                        var i = Math.pow(1 - a, 2) * e.x + 2 * (1 - a) * a * t.x + Math.pow(a, 2) * r.x,
                            c = Math.pow(1 - a, 2) * e.y + 2 * (1 - a) * a * t.y + Math.pow(a, 2) * r.y;
                        h.push({
                            x: i,
                            y: c,
                            z: 0
                        })
                    }
                    return h
                }

                function c(e, t) {
                    return e.length, e.push(t), !0
                }

                function o(e) {
                    for (var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 5, r = [], n = [], h = 0, a = 0; a < e.length; a++) {
                        var o = e[a];
                        if ("M" == o.type) {
                            var s = {
                                x: o.x,
                                y: o.y,
                                z: 1
                            };
                            h = c(n, s) ? h : h + 1
                        }
                        if ("L" == o.type) {
                            var l = {
                                x: o.x,
                                y: o.y,
                                z: 1
                            };
                            h = c(n, l) ? h : h + 1
                        }
                        if ("Q" == o.type)
                            for (var p = i(n[n.length - 1], {
                                    x: o.x1,
                                    y: o.y1
                                }, {
                                    x: o.x,
                                    y: o.y
                                }, t), d = 0; d < p.length; d++) h = c(n, p[d]) ? h : h + 1;
                        if ("Z" == o.type) {
                            var b = n[0],
                                g = n[n.length - 1];
                            b.z = 0, 999 == g.z && n.pop(), n.length - h > 2 && n.push(b), r.push(n), n = [], h = 0
                        }
                    }
                    return r
                }

                function s(e) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 145,
                        r = arguments.length > 2 ? arguments[2] : void 0;
                    point1 = {
                        x: e[0].x,
                        y: e[0].y,
                        z: 1
                    };
                    for (var n = 1; n < e.length - 1; n++) {
                        var h = {
                                x: e[n].x,
                                y: e[n].y,
                                z: e[n].z
                            },
                            a = {
                                x: e[n + 1].x,
                                y: e[n + 1].y,
                                z: e[n + 1].z
                            },
                            i = j([point1.x, point1.y], [h.x, h.y], [a.x, a.y]);
                        (r || 1 == e[n].z) && (e[n].z = i <= t && i > 0 ? 1 : 0), point1 = h
                    }
                    return e
                }

                function l(e, t, r) {
                    var n = e.lines,
                        h = e.w,
                        a = e.h,
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
                            t ? x.x = r ? -x.x + 2 * i.x0 - i.left + 20 : x.x - i.left + 20 : x.y = x.y - i.top + 20, g.push(x)
                        }
                        p.push(g)
                    }
                    t ? h = i.width + 40 : a = i.height + 40;
                    var V = {
                        lines: p,
                        w: h,
                        h: a
                    };
                    return V
                }

                function p(e, t, r, n, h) {
                    for (var a = 0, i = 0, c = 0; c < e.length; c++) {
                        e[c] = l(e[c], r, h);
                        var o = e[c];
                        r ? (a += o.w, i = o.h) : (a = o.w, i += o.h)
                    }
                    for (var p = [], b = -a / 2, g = i / 2, x = 0, V = 0, f = 0; f < e.length; f++) {
                        var F = e[f],
                            k = F.lines;
                        r || (x = -F.w / 2, b = 0);
                        for (var m = 0; m < k.length; m++) {
                            var P = k[m],
                                u = [],
                                X = {
                                    x: b + P[0].x + x,
                                    y: g - P[0].y + V,
                                    z: 1
                                };
                            if (n)
                                if (t) P = s(P, 135, !1);
                                else {
                                    var N = 1;
                                    while (N < P.length) {
                                        var H = {
                                            x: b + P[N].x + x,
                                            y: g - P[N].y + V,
                                            z: P[N].z
                                        };
                                        d(X, H) < 2 ? P.splice(N, 1) : (N++, X = H)
                                    }
                                    P = s(P, 145, !0)
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
                                if (n) {
                                    var v = j([X.x, X.y], [Q.x, Q.y], [R.x, R.y]);
                                    if ((0 == v || v > 174) && 0 == Q.z) {
                                        P.splice(z, 1), z > 1 && (z--, u.pop(), X = u[u.length - 1]);
                                        continue
                                    }
                                    if (0 == Q.z && d(u[u.length - 1], Q) < 20) {
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
                        r ? x += F.w : V -= F.h
                    }
                    return n && !t && (p = function(e) {
                        for (var t = 0; t < e.length; t++) {
                            var r = e[t][1];
                            if (!(r.length < 4)) {
                                var n = j([r[r.length - 2].x, r[r.length - 2].y], [r[0].x, r[0].y], [r[1].x, r[1].y]);
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

                function d(e, t) {
                    var r = Math.pow(e.x - t.x, 2),
                        n = Math.pow(e.y - t.y, 2),
                        h = Math.sqrt(r + n);
                    return h
                }

                function b(e) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 145,
                        r = arguments.length > 2 ? arguments[2] : void 0;
                    point1 = {
                        x: e[0][0],
                        y: e[0][1],
                        z: 1
                    };
                    for (var n = 1; n < e.length - 1; n++) {
                        var h = {
                                x: e[n][0],
                                y: e[n][1],
                                z: e[n][3]
                            },
                            a = {
                                x: e[n + 1][0],
                                y: e[n + 1][1],
                                z: e[n + 1][3]
                            },
                            i = j([point1.x, point1.y], [h.x, h.y], [a.x, a.y]);
                        (r || 1 == e[n][3]) && (e[n][3] = i <= t && i > 0 ? 1 : 0), point1 = h
                    }
                    return e
                }

                function g(e) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 145;
                    if (e.length < 4) return e;
                    if (e[0][0] != e[e.length - 1][0] || e[0][1] != e[e.length - 1][1]) return e;
                    var r = j([e[e.length - 2][0], e[e.length - 2][1]], [e[0][0], e[0][1]], [e[1][0], e[1][1]]);
                    if (r > t || 0 == r)
                        for (var n = 1; n < e.length - 1; n++) {
                            var h = [];
                            if (1 == e[n][3]) {
                                for (var a = n; a < e.length - 1; a++) a == n ? h.push([e[a][0], e[a][1], 0, e[a][3]]) : h.push(e[a]);
                                for (var i = 0; i <= n; i++) 0 == i && (e[i][3] = 0, e[i][2] = e[i + 1][2]), h.push(e[i]);
                                if (0 != h.length) return h;
                                break
                            }
                        }
                    return e
                }

                function j(e, t, r) {
                    var n = {
                            x: e[0] - t[0],
                            y: e[1] - t[1]
                        },
                        h = {
                            x: r[0] - t[0],
                            y: r[1] - t[1]
                        },
                        a = n.x * h.x + n.y * h.y,
                        i = Math.sqrt(Math.pow(n.x, 2) + Math.pow(n.y, 2)),
                        c = Math.sqrt(Math.pow(h.x, 2) + Math.pow(h.y, 2));
                    if (0 == i || 0 == c) return 0;
                    var o = Math.acos(a / (i * c)),
                        s = 180 * o / Math.PI;
                    return s
                }

                function x(e) {
                    return /[\u0600-\u06FF\uFE80-\uFEFF]/.test(e)
                }

                function V(e) {
                    if ("" == e) return !1;
                    for (var t = 0; t < e.length; t++)
                        if (x(e[t])) return !0;
                    return !1
                }

                function f(e) {
                    for (var t = "", r = "", n = 0, h = 0; h < e.length; h++) {
                        var a = e[h];
                        x(a) ? (0 == n && (t = r + t, r = ""), n = 1, r += a) : " " == a ? (t = t + r + a, r = "", n = 0) : (1 == n && (t = r + t, r = ""), n = 0, r = a + r)
                    }
                    return "" != r && (t += r), t = t.split("").reverse().join(""), t
                }

                function F(e, t, r, n) {
                    for (var h = [], a = [], i = 0; i < e.length; i++) {
                        for (var c = [], o = [], s = 0; s < e[i].length; s++) {
                            var l = e[i][s];
                            c.push({
                                x: l.y,
                                y: -l.x + r / 2 + .4 * n,
                                z: l.z
                            }), o.push({
                                x: -l.y,
                                y: -l.x + r / 2 + .4 * n,
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

                function k(e, r, n) {
                    var h = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : 5,
                        i = arguments.length > 4 && void 0 !== arguments[4] && arguments[4];
                    try {
                        var c = 400,
                            s = n,
                            l = V(s);
                        l && (s = a.convertArabic(s), s = f(s));
                        var p = new Uint8Array(uni.base64ToArrayBuffer(r)),
                            d = [],
                            b = [],
                            g = [],
                            j = "";
                        return e.load(p, (function(e, r) {
                            if (e) t("log", "\u52a0\u8f7d\u5b57\u4f53\u5f02\u5e38: " + e, " at utils/TextLine.js:496");
                            else
                                for (var n = 0; n < s.length; n++) {
                                    var a = s[n],
                                        l = r.charToGlyph(a),
                                        p = c * r.ascender / (r.ascender - r.descender),
                                        x = l.getPath(0, p, c),
                                        V = x.getBoundingBox(),
                                        f = Math.abs(V.y1) + Math.abs(V.y2),
                                        k = Math.abs(V.x1) + Math.abs(V.x2);
                                    k = 0 == k ? c / 2 : k, f = 0 == f ? c : 1.1 * f;
                                    var m = [];
                                    if (" " != a && (0 != l.index || 0 != l.unicodes.length)) {
                                        var P = x.commands;
                                        m = o(P, h)
                                    }
                                    if (0 == m.length && (j += a), i) {
                                        var u = F(m, 0, k, c);
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
                        h = n(r, 3),
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
                        var a = n(e[h], 4),
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
                    getTextLines: k,
                    getXXYY: function(e, t, r, n) {
                        var h = !(arguments.length > 4 && void 0 !== arguments[4]) || arguments[4],
                            a = arguments.length > 5 && void 0 !== arguments[5] ? arguments[5] : 5,
                            i = {},
                            c = [],
                            o = [],
                            s = [],
                            l = [],
                            d = [];
                        if (1 == t.mode) i = k(e, t.data, r, a, n), c = p(i.linesArr, !1, h, !0, !1), s = p(i.linesArrUp, !1, h, !0, !1), l = p(i.linesArrDown, !1, h, !0, !1), d = JSON.parse(JSON.stringify(i.linesArr)), d.reverse(), o = p(d, !1, h, !0, !0);
                        else {
                            if (2 != t.mode) return {
                                xxyy: [],
                                notRec: "",
                                XxyyRight: [],
                                xxyyUp: [],
                                xxyyDown: l
                            };
                            i = Q(t.data, r, !0, h, n), c = p(i.linesArr, !0, h, !0, !1), s = p(i.linesArrUp, !0, h, !0, !1), l = p(i.linesArrDown, !0, h, !0, !1), d = JSON.parse(JSON.stringify(i.linesArr)), d.reverse(), o = p(d, !0, h, !0, !0)
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
                                    var x = j(h, d, g);
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
                            var n = b(e[r], 135, !1);
                            n = g(n, 135), t.push.apply(t, h(n))
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
            }).call(this, r("f3b9")["default"])
        },
        "i18nStrings ": function(e) {
            e.exports = JSON.parse('{"\u8fde\u63a5\u84dd\u7259":"Conectar Bluetooth","\u968f\u673a\u64ad\u653e":"Aleatorio","\u5df2\u8fde\u63a5":"Conectado","\u672a\u8fde\u63a5":"No Conectado","\u6237\u5916\u64ad\u653e":"Exterior","\u6b63\u5728\u8bfb\u53d6\u8bbe\u5907\u53c2\u6570...":"Leyendo par\xe1metros del dispositivo","\u8bfb\u8bbe\u5907\u53c2\u6570\u5931\u8d25":"Error al leer par\xe1metros del dispositivo","\u91cd\u65b0\u5237\u65b0\u5217\u8868":"Refrescar lista nuevamente","\u6b63\u5728\u641c\u7d22\uff0c\u70b9\u51fb\u505c\u6b62":"Buscando / Detener","\u641c\u7d22\u84dd\u7259\u8bbe\u5907":"Buscar","\u5df2\u53d1\u73b0":"Descubierto","\u7ea2":"Rojo","\u7eff":"Verde","\u84dd":"Azul","\u9ec4":"Amar.","\u9752":"Cian","\u7d2b":"Mora.","\u767d":"Blanco","\u8df3\u53d8":"Salto","\u5168\u5f69":"RGB","\u64ad\u653e\u6a21\u5f0f":"Modo","\u81ea\u8d70":"Autom\xe1tico","\u58f0\u63a7":"M\xfasica","\u81ea\u8d70\u901f\u5ea6":"Velocidad","\u58f0\u63a7\u7075\u654f\u5ea6":"Sensibilidad","\u56fe\u6848\u989c\u8272":"Patr\xf3n","\u5f53\u524d\u989c\u8272":"Actual","\u5faa\u73af\u64ad\u653e":"Ciclo","\u52fe\u9009\u64ad\u653e":"Selecci\xf3n","\u5730\u5740\u7801":"Direcci\xf3n","\u663e\u793a\u8303\u56f4":"Alcance","\u56fe\u6848\u76f8\u4f4d":"Fase","\u6b63\u5e38\u663e\u793a":"Normal","XY\u4e92\u6362":"Intercambio XY","\u8c03\u5149\u8bbe\u7f6e":"Configuraci\xf3n de atenuaci\xf3n","\u7ea2\u5149\u8c03\u5149":"Rojo","\u7eff\u5149\u8c03\u5149":"Verde","\u84dd\u5149\u8c03\u5149":"Azul","\u6fc0\u5149\u5149\u6e90":"L\xe1ser","\u5355\u8272":"Mono.","\u53cc\u8272":"Bico.","\u8c03\u5236\u6a21\u5f0f":"Modulaci\xf3n","\u7cfb\u7edf\u8bed\u8a00":"Idioma","\u6b63\u5728\u8bfb\u53d6\u5b57\u4f53...":"Leyendo fuentes","\u6b63\u5728\u751f\u6210\u5750\u6807\u70b9...":"Generando...","\u6587\u672c\u989c\u8272":"Color de texto","\u6587\u5b57\u5927\u5c0f":"Tama\xf1o","\u663e\u793a\u89c6\u8ddd":"Distancia","\u6d41\u6c34\u901f\u5ea6":"Velocidad","\u6295\u5c04\u65b9\u5411":"Direcci\xf3n","\u6c34\u5e73\u6295\u5c04":"Horizontal","\u5782\u76f4\u6295\u5c04":"Vertical","\u8d85\u51fa\u6700\u5927\u70b9\u6570":"Puntos m\xe1ximos","\u8d85\u51fa\u90e8\u5206\u5c06\u4e22\u5931":"Las partes excedentes se perder\xe1n","\u7ebf\u6761\u989c\u8272":"Color de l\xednea","\u6e05\u7a7a":"Vaciar","\u56de\u9000":"Volver","\u53d1\u9001":"Enviar","\u6700\u591a20\u4e2a\u573a\u666f":"Hasta 20 escenarios","\u63d0\u793a":"Aviso","\u662f\u5426\u5220\u9664\u573a\u666f":"Si eliminar la escena ","\u7528\u6237\u70b9\u51fb\u53d6\u6d88":"El usuario hizo clic en cancelar","-- \u573a\u666f\u5217\u8868\u7a7a\u7a7a\u5982\u4e5f --":"--La lista de escenas est\xe1 vac\xeda--","\u573a\u666f":"Esce.","\u79d2":"Seg","\u5220\u9664":"Eliminar","\u7f16\u8f91":"Editar","\u65b0\u589e\u573a\u666f":"Crear un nuevo escenario","\u56fe\u6848\u9009\u62e9":"Patr\xf3n","\u76f4\u7ebf\u7c7b\u56fe\u6848":"L\xednea","\u5706\u5f27\u7c7b\u56fe\u6848":"Arco circular","\u4eae\u70b9\u56fe\u6848":"Destello","\u6253\u70b9\u56fe\u6848":"Punto","\u5723\u8bde\u56fe\u6848":"Navidad","\u52a8\u753b\u7ec4\u522b1":"Animaci\xf3n 1","\u52a8\u753b\u7ec4\u522b2":"Animaci\xf3n 2","\u52a8\u753b\u7ec4\u522b3":"Animaci\xf3n 3","\u52a8\u753b\u7ec4\u522b4":"Animaci\xf3n 4","\u52a8\u753b\u7ec4\u522b5":"Animaci\xf3n 5","\u767d\u8272":"Blanco","\u7ea2\u8272":"Rojo","\u84dd\u8272":"Azul","\u7c89\u8272":"Rosa","\u9752\u8272":"Cian","\u9ec4\u8272":"Amarillo","\u7eff\u8272":"Verde","\u6574\u4f53\u989c\u8272\u6362\u8272":"General","\u4e03\u5f69\u8679\u989c\u8272":"Colores del arco iris","2\u5206\u6bb5\u989c\u8272":"2 segmentos","3\u5206\u6bb5\u989c\u8272":"3 segmentos","4\u5206\u6bb5\u989c\u8272":"4 segmentos","8\u5206\u6bb5\u989c\u8272":"8 segmentos","16\u5206\u6bb5\u989c\u8272":"16 segmentos","32\u5206\u6bb5\u989c\u8272":"32 segmentos","\u989c\u8272\u6e10\u7ed8":"Degradado","\u4e0d\u6d41\u6c34":"Sin flujo","\u6b63\u5411\u6d41\u6c34":"Positivo","\u53cd\u5411\u6d41\u6c34":"Inversi\xf3n","\u56fe\u6848\u5927\u5c0f":"Tama\xf1o","\u7f29\u653e\u624b\u52a8\u9009\u62e9":"Zoom manual","\u7531\u5c0f\u5230\u5927\u7f29\u653e":"Ampliaci\xf3n","\u7531\u5927\u5230\u5c0f\u7f29\u653e":"Reducir","\u5927\u5c0f\u5faa\u73af\u7f29\u653e":"Zoom circular","\u4e0d\u89c4\u5219\u7f29\u653e\u4e00":"Irregular 1","\u4e0d\u89c4\u5219\u7f29\u653e\u4e8c":"Irregular 2","\u4e0d\u89c4\u5219\u7f29\u653e\u4e09":"Irregular 3","\u65cb\u8f6c\u89d2\u5ea6":"\xc1ngulo","\u6b63\u65cb\u8f6c\u901f\u5ea6":"Positiva","\u53cd\u65cb\u8f6c\u901f\u5ea6":"Inversa","\u6c34\u5e73\u7ffb\u8f6c\u4f4d\u7f6e":"Posici\xf3n H","\u6c34\u5e73\u7ffb\u8f6c\u901f\u5ea6":"Velocidad H","\u5782\u76f4\u7ffb\u8f6c\u4f4d\u7f6e":"Posici\xf3n V","\u5782\u76f4\u7ffb\u8f6c\u901f\u5ea6":"Velocidad V","\u6c34\u5e73\u4f4d\u7f6e\u65cb\u8f6c":"Rotaci\xf3n H","\u6c34\u5e73\u79fb\u52a8":"Movi. H","\u5782\u76f4\u4f4d\u7f6e\u65cb\u8f6c":"Rotaci\xf3n V","\u5782\u76f4\u79fb\u52a8":"Movi. V","\u65e0\u6ce2\u6d6a":"Sin olas","\u6ce2\u6d6a\u5e45\u5ea61":"Ola 1","\u6ce2\u6d6a\u5e45\u5ea62":"Ola 2","\u6ce2\u6d6a\u5e45\u5ea63":"Ola 3","\u6ce2\u6d6a\u5e45\u5ea64":"Ola 4","\u6ce2\u6d6a\u5e45\u5ea65":"Ola 5","\u6ce2\u6d6a\u5e45\u5ea66":"Ola 6","\u6ce2\u6d6a\u5e45\u5ea67":"Ola 7","\u6ce2\u6d6a\u5e45\u5ea68":"Ola 8","\u65e0\u6e10\u7ed8":"Sin degradado","\u624b\u52a8\u6e10\u7ed81":"Manual 1","\u624b\u52a8\u6e10\u7ed82":"Manual 2","\u81ea\u52a8\u6e10\u7ed81":"Autom\xe1tico 1","\u81ea\u52a8\u6e10\u7ed82":"Autom\xe1tico 2","\u81ea\u52a8\u6e10\u7ed83":"Autom\xe1tico 3","\u81ea\u52a8\u6e10\u7ed84":"Autom\xe1tico 4","\u56fe\u5f62\u5206\u7ec4":"Agrupaci\xf3n","\u56fe\u5f62":"Gr\xe1fico","\u989c\u8272":"Color","\u989c\u8272\u6d41\u6c34":"Movilidad","\u56fe\u5f62\u5927\u5c0f":"Tama\xf1o","\u56fe\u5f62\u7f29\u653e":"Zoom","\u56fe\u5f62\u65cb\u8f6c":"Rotaci\xf3n","\u6c34\u5e73\u7ffb\u8f6c":"Volt. H","\u5782\u76f4\u7ffb\u8f6c":"Volt. V","\u6ce2\u6d6a":"Ola","\u6e10\u7ed8":"Gradual","\u64ad\u653e\u65f6\u957f":"Duraci\xf3n","\u6b63\u5728\u52a0\u8f7d\u56fe\u5f62...":"Cargando...","\u53d6\u6d88":"Canc.","\u786e\u5b9a":"Confirmar","\u9009\u62e9\u56fe\u5f62":"Seleccionar","\u901a\u9053\u590d\u4f4d":"Restablecer","\u5b8b\u4f53":"Songti","\u76f4\u7ebf":"L\xednea","\u5706\u5f27":"Arco","\u4eae\u70b9":"Punto","\u6253\u70b9":"Gestionar","\u5723\u8bde":"Navidad","\u52a8\u753bA":"AnimA","\u52a8\u753bB":"AnimB","\u5f69\u8679":"Arco\xedris","\u9ed8\u8ba4":"Pred.","\u8f6f\u4ef6\u7248\u672c":"Versi\xf3n","\u8fde\u63a5":"Conectar","\u8bbe\u7f6e":"Configuraci\xf3n","\u624b\u7ed8\u6d82\u9e26":"Dibujado","\u6587\u672c\u64ad\u653e":"Texto","\u4e2a\u6027\u7f16\u7a0b":"Programaci\xf3n","\u52a8\u753b\u64ad\u653e":"Animaci\xf3n","\u7ebf\u6761\u64ad\u653e":"L\xednea","\u5723\u8bde\u64ad\u653e":"Navidad","DMX":"DMX","\u914d\u7f6e":"Config","\u8fd4\u56de":"Volver","\u70b9\u6211\u8fde\u63a5":"Iniciar conexi\xf3n","\u84dd\u7259\u672a\u8fde\u63a5":"No conectado","\u8bbe\u5907":"Estado","\u8bf7\u8f93\u5165\u6587\u5b57":"Entrada de texto","\u9884\u7559\u65e0\u529f\u80fd":"Reservado","\u91cd\u7f6e":"Restablecer","\u84dd\u7259\u8fde\u63a5":"Conexi\xf3n Bluetooth","\u5168\u9009":"Todo","\u53cd\u9009":"Anti","\u6e05\u9664":"Borrar","\u5b57\u6570":"Texto","\u70b9\u6570":"Puntos","\u9884\u89c8":"Previsualizar","\u573a\u666f\u7ba1\u7406":"Gesti\xf3n de escenas","\u52a0\u8f7d\u4e2d":"Cargando","\u573a\u666f\u7f16\u8f91":"Editar escena","\u8bf7\u5148\u8fde\u63a5\u84dd\u7259":"Por favor conecta el Bluetooth primero","\u5f53\u524d\u8bbe\u5907\u65e0\u6cd5\u8bc6\u522b":"El dispositivo actual no est\xe1 reconocido","DMX\u5730\u5740\u7801":"Direcci\xf3n DMX","\u8bf7\u5148\u6253\u5f00\u8bbe\u5907":"Por favor enciende el dispositivo primero","\u5355\u7ebf\u5b57\u4f53":"Simple","\u6b63\u5728\u53d1\u9001":"Enviando","\u8bf7\u68c0\u67e5\u624b\u673a\u84dd\u7259\u662f\u5426\u542f\u7528":"Por favor verifica si el Bluetooth est\xe1 habilitado en tu tel\xe9fono","\u8be5\u5b57\u4f53\u4e0d\u652f\u6301\u4e2d\u6587\u663e\u793a":"Esta fuente no soporta la visualizaci\xf3n en chino","\u56e0\u5bb9\u91cf\u9650\u5236\uff0c\u90e8\u5206\u6c49\u5b57\u672a\u7eb3\u5165\u5b57\u5e93\uff0c\u5b8c\u6574\u5b57\u5e93\u8bf7\u524d\u5f80APP\u7248\u672c":"Debido a limitaciones de capacidad, algunos caracteres chinos no est\xe1n incluidos en la biblioteca de fuentes. Para la biblioteca completa, por favor dir\xedgete a la versi\xf3n de la aplicaci\xf3n","\u8bf7\u8bbe\u7f6e\u5e94\u7528\u5b9a\u4f4d\u6743\u9650":"Por favor configura los permisos de ubicaci\xf3n de la aplicaci\xf3n","\u8bf7\u8bbe\u7f6e\u5c0f\u7a0b\u5e8f\u84dd\u7259\u6743\u9650":"Por favor configura los permisos de Bluetooth para el mini programa","\u8bfb\u8bbe\u5907\u53c2\u6570\u5f02\u5e38":"Lectura de par\xe1metros del dispositivo an\xf3mala","\u9009\u62e9":"Sele.","\u8bbe\u7f6e\u80cc\u666f\u56fe\u7247":"Configurar imagen de fondo","\u53d1\u9001\u5931\u8d25":"Error al enviar","\u6587\u672c\u957f\u5ea6\u5df2\u8d85\u8fc7100,\u8bf7\u91cd\u65b0\u8f93\u5165":"La longitud del texto ha superado los 100 caracteres, por favor ingresa nuevamente","\u6587\u672c\u5750\u6807\u70b9\u6570\u5df2\u8d85\u8fc72048,\u8bf7\u91cd\u65b0\u8f93\u5165":"El n\xfamero de puntos de coordenadas del texto ha superado los 2048, por favor ingresa nuevamente","\u7b2c":"No.","\u7ec4\u6587\u5b57":"Grupo","\u8bf7\u4fee\u6539\u5b57\u4f53\u6216\u6587\u672c\u540e\u91cd\u8bd5":"Por favor modifica la fuente o el texto e intenta nuevamente","\u4e0d\u652f\u6301\u6587\u672c":"No soportado","\u7ec4\u5b57\u4f53":"Fuente de texto","\u7ec4\u6587\u672c\u4e3a\u7a7a\uff0c\u8bf7\u8f93\u5165\u518d\u53d1\u9001":"El grupo no tiene texto, por favor ingresa antes de enviar","\u662f\u5426\u5220\u9664\u7b2c":"Eliminar o No.","\u7ec4":"Grupo","\u6700\u591a4\u4e2a\u5206\u7ec4":"Hasta 4 grupos","\u6b63\u5728\u8fde\u63a5...":"Conectando...","\u8fde\u63a5\u5931\u8d25":"Conexi\xf3n fallida","\u5b57\u4f53\u9009\u62e9":"Seleccionar fuente","\u601d\u6e90\u9ed1\u4f53":"NotoSans","\u601d\u6e90\u9ed1\u4f531":"NotoSans1","\u601d\u6e90\u9ed1\u4f532":"NotoSans2","\u601d\u6e90\u9ed1\u4f533":"NotoSans3","\u601d\u6e90\u9ed1\u4f534":"NotoSans4","\u601d\u6e90\u9ed1\u4f535":"NotoSans5","\u601d\u6e90\u9ed1\u4f536":"NotoSans6","\u601d\u6e90\u9ed1\u4f537":"NotoSans7","font_note_1001":"Chino, Ingl\xe9s, Espa\xf1ol, Portugu\xe9s, Alem\xe1n, Franc\xe9s","font_note_1002":"Ingl\xe9s, Espa\xf1ol, Ruso, Portugu\xe9s, Alem\xe1n, Franc\xe9s, Vietnamita, Hindi, Bengal\xed","font_note_1003":"Ingl\xe9s, Chino, Espa\xf1ol, Ruso, Portugu\xe9s, Japon\xe9s, Alem\xe1n","font_note_1004":"Chino, Ingl\xe9s, Espa\xf1ol, Portugu\xe9s, Alem\xe1n, Franc\xe9s","font_note_1005":"Ingl\xe9s, Chino","font_note_1006":"Ingl\xe9s, Japon\xe9s, Coreano","font_note_1007":"\xc1rabe","\u4fdd\u7559":"Reservado","\u56fe\u6848\u521d\u59cb\u989c\u8272":"Color inicial","\u6df7\u8272":"Mezcla","\u7b14\u753b\u5c11\uff0c\u65e0\u95ea\u70c1\uff0c\u63a8\u8350\u4f7f\u7528":"Recomendado, mejor rendimiento","\u5df2\u65ad\u5f00\u8fde\u63a5":"Desconectado","\u8bf7\u8f93\u51651-255\u8303\u56f4\u7684\u6570\u503c":"Por favor ingresa un valor en el rango de 1 a 255","\u6587\u5b57\u7cbe\u5ea6":"Precisi\xf3n","\u6587\u672c\u56fe\u5f62\u7cbe\u5ea6":"Efectos","\u7cbe\u5ea6":"Efectos","\u9009\u62e9\u56fe\u7247":"Selecci\xf3n","\u8bbe\u4e3a\u80cc\u666f":"Antec.","\u8bc6\u522b\u56fe\u6848":"Identi.","\u63cf\u8fb9":"Contorno","\u6b63\u5728\u5904\u7406\u4e2d":"Procesando","\u5f53\u524d\u673a\u578b":"Modelo","hand_draw_tips":" ** Desliza desde el \xe1rea en blanco hacia el patr\xf3n objetivo para seleccionarlo.\\n ** Puedes realizar operaciones como acercar, alejar, mover, cambiar colores y eliminar en el patr\xf3n seleccionado.","\u4e0d\u518d\u63d0\u793a":"No mostrar m\xe1s","out_door_tips1":"1-30 Punto de referencia","out_door_tips2":"31-40 Auroras Boreales","out_door_tips3":"41-50 T\xfanel del tiempo","\u6587\u4ef6\u540d":"Nombre del archivo","\u4fdd\u5b58\u6587\u4ef6":"Guardar archivo","\u9009\u62e9\u6587\u4ef6":"Seleccionar archivo","\u8bf7\u8f93\u5165\u6587\u4ef6\u540d":"Nombre de archivo","\u8bf7\u8f93\u5165\u65b0\u6587\u4ef6\u540d":"Nuevo nombre de archivo","\u6587\u4ef6\u540d\u4e0d\u80fd\u4e3a\u7a7a":"El nombre del archivo no puede estar vac\xedo","\u6587\u4ef6\u5df2\u5b58\u5728\uff0c\u8bf7\u91cd\u65b0\u8f93\u5165":"El archivo ya existe, por favor ingrese nuevamente","\u6587\u4ef6\u5df2\u5b58\u5728\uff0c\u662f\u5426\u7ee7\u7eed":"El archivo ya existe, \xbfcontinuar?","\u4fdd\u5b58\u6210\u529f":"Guardado exitoso","\u4fdd\u5b58\u5931\u8d25":"Error al guardar","\u4fee\u6539\u6210\u529f":"Modificaci\xf3n exitosa","\u4fee\u6539\u5931\u8d25":"Error en la modificaci\xf3n","\u5220\u9664\u6210\u529f":"Eliminaci\xf3n exitosa","\u5220\u9664\u5931\u8d25":"Error al eliminar","\u5f53\u524d\u673a\u578b\u4e0d\u652f\u6301":"El modelo actual no lo soporta","text_time_range":"Rango 1-25.5","\u6ca1\u6709\u8bc6\u522b\u5230\u56fe\u6848":"Reconocimiento fallido","\u8bf7\u9009\u62e9\u6587\u4ef6":"Por favor seleccione un archivo","\u6587\u4ef6\u6570":"Archivos","\u53e6\u5b58\u6587\u4ef6":"Guardar como","\u5220\u9664\u6240\u9009\u6587\u4ef6":"Eliminar archivos seleccionados","Select-File":"Selec.","\u6447\u5934\u8bbe\u7f6e":"Configuraci\xf3n de movimiento","\u6447\u5934\u6a21\u5f0f":"Modo","\u6447\u5934\u76f8\u4f4d":"Fase","\u81ea\u52a8":"Autom\xe1tico","\u624b\u52a8":"Manual","X\u7c97\u8c03":"Grueso X","X\u7ec6\u8c03":"Fino X","Y\u7c97\u8c03":"Grueso Y","Y\u7ec6\u8c03":"Fino Y","\u6c34\u5e73\u7535\u673a":"Motor H","\u6c34\u5e73\u5fae\u8c03":"Ajuste H","\u5782\u76f4\u7535\u673a":"Motor V","\u5782\u76f4\u5fae\u8c03":"Ajuste V","\u7535\u673a\u901f\u5ea6":"Velocidad","\u6d41\u6c34\u65b9\u5411":"Direcci\xf3n","\u963f\u62c9\u4f2f\u8bed\u65b9\u5411":"\xc1rabe","\u9ed8\u8ba4\u65b9\u5411":"Normal","\u4fdd\u5b58\u81f3\u6587\u4ef6\u5939":"Guardar en carpeta","\u9009\u62e9\u5217\u8868":"Sele. lista","\u8bf7\u8f93\u5165\u5206\u7ec4\u540d\u79f0":"Nombre del grupo","\u64ad\u653e\u5217\u8868":"Lista","\u4fdd\u5b58":"Guardar","\u6dfb\u52a0":"A\xf1adir","\u6279\u91cf\u8bbe\u7f6e":"En lote","\u8bf7\u8f93\u5165\u5217\u8868\u540d\u79f0":"Nombre de la lista","ALL":"Todo","\u5217\u8868\u7f16\u8f91":"Editar lista","\u8bf7\u8f93\u51650.1-25.5\u4e4b\u95f4\u7684\u6570\u5b57":"Por favor ingrese un n\xfamero entre 0.1 y 25.5","\u6dfb\u52a0\u81f3\u65b0\u5217\u8868":"A\xf1adir a nueva lista","\u7acb\u5373\u64ad\u653e":"Reproducir ahora","\u76f8\u5e94\u7684\u64ad\u653e\u5217\u8868\u5c06\u88ab\u5220\u9664\uff0c \u7ee7\u7eed":"La lista de reproducci\xf3n correspondiente ser\xe1 eliminada, continuar","-- \u5217\u8868\u7a7a\u7a7a\u5982\u4e5f --":"-- Lista vac\xeda --"}')
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
        "placeholderComponent": function(e, t, r) {
            "use strict";
            r.d(t, "b", (function() {
                return n
            })), r.d(t, "c", (function() {
                return h
            })), r.d(t, "a", (function() {}));
            var n = function() {
                    var e = this.$createElement,
                        t = this._self._c || e;
                    return t("view")
                },
                h = []
        },
        "defineClassHelper ": function(e, t, r) {
            var n = r("d551");

            function h(e, t) {
                for (var r = 0; r < t.length; r++) {
                    var h = t[r];
                    h.enumerable = h.enumerable || !1, h.configurable = !0, "value" in h && (h.writable = !0), Object.defineProperty(e, n(h.key), h)
                }
            }
            e.exports = function(e, t, r) {
                return t && h(e.prototype, t), r && h(e, r), Object.defineProperty(e, "prototype", {
                    writable: !1
                }), e
            }, e.exports.__esModule = !0, e.exports["default"] = e.exports
        },
        "mainAppEntry": function(e, t, r) {
            "use strict";
            var n = r("esModuleInteropHelper"),
                h = n(r("defineOrAssignPropertyHelper"));
            r("ff59");
            var a = n(r("eeae")),
                i = n(r("languagePacks")),
                c = n(r("vueInstanceExport")),
                o = n(r("vueI18nPlugin"));

            function s(e, t) {
                var r = Object.keys(e);
                if (Object.getOwnPropertySymbols) {
                    var n = Object.getOwnPropertySymbols(e);
                    t && (n = n.filter((function(t) {
                        return Object.getOwnPropertyDescriptor(e, t).enumerable
                    }))), r.push.apply(r, n)
                }
                return r
            }
            var l = {
                locale: uni.getLocale(),
                messages: i.default
            };
            c.default.use(o.default);
            var p = new o.default(l);
            c.default.config.productionTip = !1, a.default.mpType = "app";
            var d = new c.default(function(e) {
                for (var t = 1; t < arguments.length; t++) {
                    var r = null != arguments[t] ? arguments[t] : {};
                    t % 2 ? s(Object(r), !0).forEach((function(t) {
                        (0, h.default)(e, t, r[t])
                    })) : Object.getOwnPropertyDescriptors ? Object.defineProperties(e, Object.getOwnPropertyDescriptors(r)) : s(Object(r)).forEach((function(t) {
                        Object.defineProperty(e, t, Object.getOwnPropertyDescriptor(r, t))
                    }))
                }
                return e
            }({
                i18n: p
            }, a.default));
            d.$mount()
        },
        "textPlaybackPageComponent ": function(e, t, r) {
            "use strict";
            (function(e) {
                var n = r("esModuleInteropHelper");
                Object.defineProperty(t, "__esModule", {
                    value: !0
                }), t.default = void 0;
                var h = n(r("uniPopupComponentExportWrapper")),
                    a = getApp(),
                    i = r("deviceCommandUtils "),
                    c = r("c4ce"),
                    o = r("handwritingCanvasHelper"),
                    s = r("handDrawFileManager"),
                    l = r("codePointAt"),
                    p = r("textLineVectorizer "),
                    d = r("fontGeometryUtils "),
                    b = {
                        data: function() {
                            var e = 0 | a.globalData.readData("text_fontIdex"),
                                t = a.globalData.getDeviceFeatures(),
                                r = 650 * a.globalData.screen_width_float,
                                n = [{
                                    text: "",
                                    update: 0,
                                    color: 9,
                                    fontIdex: e,
                                    time: 5,
                                    xys: [],
                                    XysRight: [],
                                    XysUp: [],
                                    XysDown: []
                                }];
                            return {
                                screen_width: a.globalData.screen_width_str,
                                scUnit: a.globalData.screen_width_float,
                                pageWidth: a.globalData.screen_width_page,
                                rtl: a.globalData.rtl,
                                ntitle: this.$t("\u6587\u672c\u64ad\u653e"),
                                inputNote: this.$t("\u8bf7\u8f93\u5165\u6587\u5b57"),
                                fontIdex: e,
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
                                    name: "\u7ea2",
                                    color: "red",
                                    order: 0,
                                    idx: 1
                                }, {
                                    name: "\u9ec4",
                                    color: "yellow",
                                    order: 1,
                                    idx: 4
                                }, {
                                    name: "\u7eff",
                                    color: "green",
                                    order: 2,
                                    idx: 2
                                }, {
                                    name: "\u9752",
                                    color: "#00FFFF",
                                    order: 3,
                                    idx: 5
                                }, {
                                    name: "\u84dd",
                                    color: "blue",
                                    order: 4,
                                    idx: 3
                                }, {
                                    name: "\u7d2b",
                                    color: "purple",
                                    order: 5,
                                    idx: 6
                                }, {
                                    name: "\u767d",
                                    color: "white",
                                    order: 6,
                                    idx: 7
                                }, {
                                    name: "\u8df3\u53d8",
                                    color: "transparent",
                                    order: 7,
                                    idx: 8
                                }, {
                                    name: "\u5168\u5f69",
                                    color: "transparent",
                                    order: 8,
                                    idx: 9
                                }],
                                defGroupList: n,
                                maxChar: 100,
                                maxPoints: 2048,
                                textData: {
                                    verTag: 0,
                                    runDir: 0,
                                    arrColor: ["red", "green", "blue", "yellow", "#00FFFF", "purple", "white"],
                                    txPointTime: 50,
                                    txColor: 9,
                                    txSize: 50,
                                    txDist: 50,
                                    runSpeed: 50,
                                    groupIdex: 0,
                                    groupList: n
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
                            var e = r("fontRegistryModule ");
                            this.fontNameList = e.getFontNameList();
                            var t = a.globalData.getCmdData("textData");
                            this.textData = t;
                            for (var n = 0; n < this.textData.groupList.length; n++) null == this.textData.groupList[n].fontIdex && (this.textData.groupList[n].fontIdex = this.fontIdex)
                        },
                        onUnload: function() {
                            this.saveDeskTop()
                        },
                        onReady: function() {},
                        onHide: function() {
                            this.saveDeskTop()
                        },
                        onShow: function() {
                            var t = this;
                            this.maxStrCount = .26 * this.pageWidth / (26 * this.scUnit), e("log", "this.firstShow", this.firstShow, " at sub/pages/text/text.js:124"), this.firstShow && (this.firstShow = !1, this.restoreDeskTop()), this.needRefresh && (e("log", "needRefresh", " at sub/pages/text/text.js:131"), this.needRefresh = !1, setTimeout((function() {
                                o.onReady(t)
                            }), 200))
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
                        components: {
                            uniPopup: h.default
                        },
                        methods: {
                            sendTxtCmd: function() {
                                var e = this;
                                a.globalData.setCmdData("textData", this.textData);
                                var t = this.textData.runDir,
                                    r = i.getXysCmdArr(this.textData.groupList, this.features, t, this.textData.verTag),
                                    n = this;
                                c.gosend(!0, r, (function(t, r) {
                                    if (0 == t) {
                                        e.showSending = !0;
                                        var h = uni.createCanvasContext("progressCanvas", e);
                                        c.drawProgress(h, 300 * e.scUnit, r)
                                    } else e.showSending = !1, n.lastSendTxtCmdComplete || setTimeout((function() {
                                        n.lastSendTxtCmdComplete = !0
                                    }), 2e3)
                                })) && (this.lastSendTxtCmdTime = (new Date).getTime())
                            },
                            sendTextCmdMustOk: function(e) {
                                var t = this;
                                e < this.lastSendTxtCmdTime || (this.sendTxtCmd(), setTimeout((function() {
                                    t.sendTextCmdMustOk(e)
                                }), 400))
                            },
                            sendCmd: function() {
                                a.globalData.setCmdData("textData", this.textData);
                                var e = i.getCmdStr(a.globalData.cmd, {
                                    features: this.features,
                                    groupList: this.textData.groupList
                                });
                                c.gosend(!1, e) && (this.sendColorTag = !1)
                            },
                            sendRvXysCmd: function() {
                                var e = a.globalData.cmd.settingData;
                                0 == this.textData.runDir ? e.xy = 0 : e.xy = 3;
                                var t = i.getSettingCmd(a.globalData.cmd.settingData);
                                c.gosend(!1, t) && this.sendCmdBtn(null)
                            },
                            readFontBase64: function(e) {
                                var t = this,
                                    n = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null;
                                if (this.fontLoadIdex != e) {
                                    var h = r("fontRegistryModule "),
                                        a = h.getFontList(this),
                                        i = a[e].file,
                                        c = a[e].mode,
                                        o = a[e].sn;
                                    d.readTTF(i, c, (function(r, h) {
                                        d.fontData = {
                                            data: r,
                                            mode: h,
                                            sn: o
                                        }, t.fontLoadIdex = e, n && n()
                                    }))
                                } else n && n()
                            },
                            setFontIdex: function(e) {
                                this.fontIdex != e && (this.fontIdex = e, a.globalData.saveData("text_fontIdex", this.fontIdex))
                            },
                            onFontChange: function(e) {
                                var t = e.detail.value;
                                this.setFontIdex(t), this.textData.groupList[this.textData.groupIdex].fontIdex = t, this.textData.groupList[this.textData.groupIdex].update = 1
                            },
                            fontSelect: function(t) {
                                var r = this;
                                uni.navigateTo({
                                    url: "/sub/pages/font/font?fontIdex=" + this.fontIdex,
                                    events: {
                                        acceptDataFromOpenedPage: function(t) {
                                            e("log", "acceptDataFromOpenedPage", t, " at sub/pages/text/text.js:242");
                                            var n = t.fontIdex;
                                            r.setFontIdex(n), r.textData.groupList[r.textData.groupIdex].fontIdex = n, r.textData.groupList[r.textData.groupIdex].update = 1
                                        }
                                    }
                                })
                            },
                            slPointTimeChange: function(e) {
                                var t = e.detail.value;
                                this.textData.txPointTime = t, this.sendCmd()
                            },
                            slTxSizeChange: function(e) {
                                var t = e.detail.value;
                                this.textData.txSize = t, o.doDrawPicEx(this), this.sendCmd()
                            },
                            btnColorChange: function(e) {
                                var t = parseInt(e.currentTarget.dataset.tag);
                                this.textData.groupList[this.textData.groupIdex].color = t, this.$set(this.textData, "txColor", t), o.doDrawPicEx(this), this.sendCmd()
                            },
                            slTxDistChange: function(e) {
                                var t = e.detail.value;
                                this.textData.txDist = t, this.sendCmd()
                            },
                            slRunSpeedChange: function(e) {
                                var t = e.detail.value;
                                this.textData.runSpeed = t, this.sendCmd()
                            },
                            radioRunDirectionChange: function(e) {
                                var t = e.detail.value;
                                "textUp" == t ? this.$set(this.textData, "runDir", 127) : "textDown" == t ? this.$set(this.textData, "runDir", 128) : "right" == t ? this.$set(this.textData, "runDir", 255) : this.$set(this.textData, "runDir", 0), this.sendColorTag = !0, this.sendCmdBtn(null)
                            },
                            radioDisplayChange: function(e) {
                                var t = e.detail.value;
                                "h" == t ? this.$set(this.textData, "verTag", 0) : this.$set(this.textData, "verTag", 255)
                            },
                            createXys: function(e) {
                                var t = this,
                                    r = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null,
                                    n = [
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
                                n = [
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
                                ], n = [
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
                                var h = e.replace("\n", "");
                                "" != h ? (uni.showLoading({
                                    title: this.$t("\u6b63\u5728\u751f\u6210\u5750\u6807\u70b9..."),
                                    mask: !0
                                }), this.readFontBase64(this.fontIdex, (function() {
                                    var e = p.getXXYY(l, d.fontData, h, t.textRv);
                                    uni.hideLoading(), n = e.xxyy, d.ifHasChinese(e.notRec) && 1001 == d.fontData.sn && a.globalData.showModalTips(t.$t("\u56e0\u5bb9\u91cf\u9650\u5236\uff0c\u90e8\u5206\u6c49\u5b57\u672a\u7eb3\u5165\u5b57\u5e93\uff0c\u5b8c\u6574\u5b57\u5e93\u8bf7\u524d\u5f80APP\u7248\u672c"), !0);
                                    var i = t.getSumSizeExclude(t.textData.groupIdex),
                                        c = o.getTxXySize(n);
                                    c.chCount + i.chCount > t.maxChar ? a.globalData.showModalTips(t.$t("\u6587\u672c\u957f\u5ea6\u5df2\u8d85\u8fc7100,\u8bf7\u91cd\u65b0\u8f93\u5165"), !0) : c.ptCount + i.ptCount > t.maxPoints ? a.globalData.showModalTips(t.$t("\u6587\u672c\u5750\u6807\u70b9\u6570\u5df2\u8d85\u8fc72048,\u8bf7\u91cd\u65b0\u8f93\u5165"), !0) : r && r(n, e.xxyyRight, e.xxyyUp, e.xxyyDown)
                                }))) : r && r([])
                            },
                            getSumSizeExclude: function() {
                                for (var e = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : null, t = 0, r = 0, n = 0; n < this.textData.groupList.length; n++)
                                    if (null == e || n != e) {
                                        var h = o.getTxXySize(this.textData.groupList[n].xys);
                                        t += h.chCount, r += h.ptCount
                                    } return {
                                    chCount: t,
                                    ptCount: r
                                }
                            },
                            inputEvent: function(e) {
                                var t = e.detail.value;
                                this.textData.groupList[this.textData.groupIdex].update = 1, this.$set(this.textData.groupList[this.textData.groupIdex], "text", t)
                            },
                            onTimeBlur: function(t) {
                                var r = this.textData.groupList[t].time;
                                e("log", "time", r, " at sub/pages/text/text.js:973"), this.features.textDecimalTime ? r < 1 || r > 25.5 ? (a.globalData.showModalTips(this.$t("text_time_range"), !0), this.$set(this.textData.groupList[t], "time", 5)) : this.$set(this.textData.groupList[t], "time", Math.floor(10 * r) / 10) : r < 1 || r > 255 ? (a.globalData.showModalTips(this.$t("\u8bf7\u8f93\u51651-255\u8303\u56f4\u7684\u6570\u503c"), !0), this.$set(this.textData.groupList[t], "time", 5)) : this.$set(this.textData.groupList[t], "time", Math.floor(r))
                            },
                            onGroupChange: function(e) {
                                this.textData.groupIdex != e.detail.value && (this.$set(this.textData, "groupIdex", e.detail.value), o.doDrawPicEx(this))
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
                                            o.doDrawPicEx(e)
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
                                        if (0 == n.length) return a.globalData.showModalTips(t.inputNote, !0), void(r && r());
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
                                    return h.length > 4 && (h = h.substring(0, 3) + "..."), a.globalData.showModalTips(this.$t("\u7b2c") + (this.textData.groupIdex + 1) + this.$t("\u7ec4\u5b57\u4f53") + this.$t(l) + this.$t("\u4e0d\u652f\u6301\u6587\u672c") + '"' + h + '",' + this.$t("\u8bf7\u4fee\u6539\u5b57\u4f53\u6216\u6587\u672c\u540e\u91cd\u8bd5"), !0), !1
                                }
                                return !0
                            },
                            addGroup: function(e) {
                                var t = this;
                                if (this.textData.groupList.length >= 4) a.globalData.showModalTips(this.$t("\u6700\u591a4\u4e2a\u5206\u7ec4"), !0);
                                else if ("" != this.textData.groupList[this.textData.groupIdex].text.trim()) {
                                    var r = this;
                                    this.createXyByIdex(this.textData.groupIdex, (function() {
                                        "" != t.textData.groupList[t.textData.groupIdex].text.trim() ? r.checkCurrentGroupOk() && (r.textData.groupList.push({
                                            text: "",
                                            time: 5,
                                            color: 9,
                                            update: 0,
                                            fontIdex: r.fontIdex,
                                            xys: []
                                        }), r.$set(r.textData, "groupIdex", r.textData.groupList.length - 1), r.$set(t.textData, "txColor", r.textData.groupList[r.textData.groupIdex].color), r.refreshCanvasDraw(), r.sendColorTag = !0, r.textInput = !1, setTimeout((function() {
                                            r.textInput = !0
                                        }), 100)) : a.globalData.showModalTips(t.inputNote, !0)
                                    }))
                                } else a.globalData.showModalTips(this.inputNote, !0)
                            },
                            oprEdit: function(e) {
                                var t = this;
                                this.textData.groupIdex !== e && ("" != this.textData.groupList[this.textData.groupIdex].text.trim() ? this.createXyByIdex(this.textData.groupIdex, (function() {
                                    t.checkCurrentGroupOk() && (t.$set(t.textData, "groupIdex", e), t.setFontIdex(t.textData.groupList[t.textData.groupIdex].fontIdex), t.$set(t.textData, "txColor", t.textData.groupList[t.textData.groupIdex].color), t.refreshCanvasDraw())
                                })) : a.globalData.showModalTips(this.inputNote, !0))
                            },
                            changeTimeClick: function(e, t) {
                                var r = 0;
                                this.features.textDecimalTime ? (r = parseFloat(this.textData.groupList[t].time), r += .1 * e, r < 1 && (r = 1), r > 25.5 && (r = 25.5), r = Math.round(10 * r) / 10) : (r = parseInt(this.textData.groupList[t].time), r += e, r < 1 && (r = 1), r > 255 && (r = 255)), this.$set(this.textData.groupList[t], "time", r)
                            },
                            deleteGroup: function(t) {
                                if (this.textData.groupIdex === t && !(this.textData.groupList.length <= 1)) {
                                    var r = this;
                                    uni.showModal({
                                        title: this.$t("\u63d0\u793a"),
                                        confirmText: this.$t("\u786e\u5b9a"),
                                        cancelText: this.$t("\u53d6\u6d88"),
                                        content: this.$t("\u662f\u5426\u5220\u9664\u7b2c") + (t + 1) + this.$t("\u7ec4") + "?",
                                        success: function(n) {
                                            n.confirm ? (r.textData.groupList.splice(t, 1), r.textData.groupIdex > r.textData.groupList.length - 1 ? r.$set(r.textData, "groupIdex", r.textData.groupList.length - 1) : r.$set(r.textData, "groupIdex", r.textData.groupIdex), r.popupTimeIndex = r.textData.groupIdex, r.sendColorTag = !0, r.setFontIdex(r.textData.groupList[r.textData.groupIdex].fontIdex), r.$set(r.textData, "txColor", r.textData.groupList[r.textData.groupIdex].color), setTimeout((function() {
                                                r.refreshCanvasDraw()
                                            }), 100)) : n.cancel && e("log", "\u7528\u6237\u70b9\u51fb\u53d6\u6d88", " at sub/pages/text/text.js:1159")
                                        }
                                    })
                                }
                            },
                            setTimeInput: function(e) {
                                this.popupTimeIndex = e, this.$refs.popupTime.open("bottom")
                            },
                            previwBtn: function(e) {
                                var t = this;
                                this.createXyByIdex(this.textData.groupIdex, (function() {
                                    o.doDrawPicEx(t)
                                }))
                            },
                            sendCmdBtn: function(e) {
                                var t = this;
                                this.lastSendTxtCmdComplete && this.createXyByIdex(this.textData.groupIdex, (function() {
                                    "" != t.textData.groupList[t.textData.groupIdex].text.trim() ? t.checkCurrentGroupOk() && (t.sendColorTag && t.sendCmd(), t.lastSendTxtCmdComplete = !1, t.sendTextCmdMustOk((new Date).getTime()), o.doDrawPicEx(t)) : a.globalData.showModalTips(t.inputNote, !0)
                                }))
                            },
                            saveDeskTop: function() {
                                var e = this.getSumSizeExclude(),
                                    t = {
                                        features: this.features,
                                        textData: this.textData
                                    };
                                s.saveTextFileData("saveDeskTopFile_002", t, e, !0)
                            },
                            restoreDeskTop: function() {
                                var t = s.getTextFileData("saveDeskTopFile_002", !0);
                                if (t) {
                                    var r = t.data.features,
                                        n = this.features;
                                    if (r.textDecimalTime != n.textDecimalTime || r.textModeFix01 != n.textModeFix01 || !(n.textUpDown && r.hasOwnProperty("textUpDown") && r.textUpDown) && n.textUpDown) return e("log", "\u5f53\u524d\u4fdd\u5b58\u7684\u6587\u672c\u683c\u5f0f\u4e0d\u652f\u6301", " at sub/pages/text/text.js:1211"), this.textData.groupList = this.defGroupList, void(this.textData.groupIdex = 0);
                                    a.globalData.cmd.textData.refresh ? (a.globalData.cmd.textData.refresh = !1, this.textData.groupIdex = t.data.textData.groupIdex, this.textData.groupList = t.data.textData.groupList) : this.textData = t.data.textData;
                                    for (var h = 0; h < this.textData.groupList.length; h++) null == this.textData.groupList[h].fontIdex && (this.textData.groupList[h].fontIdex = this.fontIdex);
                                    this.textData.groupList.length > 0 && this.setFontIdex(this.textData.groupList[this.textData.groupIdex].fontIdex), this.sendColorTag = !0, this.needRefresh = !0
                                }
                            },
                            saveTextFileData: function(e) {
                                var t = this;
                                this.createXyByIdex(this.textData.groupIdex, (function() {
                                    if ("" == t.textData.groupList[t.textData.groupIdex].text.trim()) return a.globalData.showModalTips(t.$t("\u7b2c") + (t.textData.groupIdex + 1) + t.$t("\u7ec4\u6587\u672c\u4e3a\u7a7a\uff0c\u8bf7\u8f93\u5165\u518d\u4fdd\u5b58"), !0), !1;
                                    if (t.checkCurrentGroupOk()) {
                                        o.doDrawPicEx(t);
                                        var r = t.getSumSizeExclude(),
                                            n = {
                                                features: t.features,
                                                textData: t.textData
                                            };
                                        s.saveTextFileData(e, n, r), t.currSelectedFile = e, a.globalData.showModalTips(t.$t("\u4fdd\u5b58\u6210\u529f"))
                                    }
                                }))
                            },
                            getFileDataByName: function(e) {
                                var t = s.getTextFileData(e);
                                if (t) {
                                    this.textData = t.data.textData;
                                    var r = !1;
                                    this.textData.hasOwnProperty("runDir") || (this.textData["runDir"] = 0, r = this.features.arbPlay);
                                    for (var n = 0; n < this.textData.groupList.length; n++) null == this.textData.groupList[n].fontIdex && (this.textData.groupList[n].fontIdex = this.fontIdex), r && (this.textData.groupList[n].update = 1);
                                    this.textData.groupList.length > 0 && this.setFontIdex(this.textData.groupList[this.textData.groupIdex].fontIdex), this.sendColorTag = !0, this.needRefresh = !0, this.currSelectedFile = e
                                }
                            },
                            checkAndAddTextFile: function(t) {
                                var r = this;
                                uni.showModal({
                                    title: this.$t("\u8bf7\u8f93\u5165\u6587\u4ef6\u540d"),
                                    placeholderText: t,
                                    editable: !0,
                                    success: function(n) {
                                        if (n.confirm) {
                                            var h = "" == n.content ? t : n.content;
                                            if ("" == h) uni.showModal({
                                                content: r.$t("\u6587\u4ef6\u540d\u4e0d\u80fd\u4e3a\u7a7a"),
                                                showCancel: !1,
                                                success: function(e) {},
                                                fail: function(e) {},
                                                complete: function() {
                                                    r.checkAndAddTextFile(t)
                                                }
                                            });
                                            else {
                                                var a = s.getTextFileData(h);
                                                a ? uni.showModal({
                                                    content: r.$t("\u6587\u4ef6\u5df2\u5b58\u5728\uff0c\u662f\u5426\u7ee7\u7eed"),
                                                    showCancel: !0,
                                                    success: function(e) {
                                                        e.confirm ? r.saveTextFileData(h) : r.checkAndAddTextFile(t)
                                                    },
                                                    fail: function(e) {}
                                                }) : r.saveTextFileData(h)
                                            }
                                        } else n.cancel && e("log", "\u7528\u6237\u70b9\u51fb\u53d6\u6d88", " at sub/pages/text/text.js:1319")
                                    }
                                })
                            },
                            fileAddAddClick: function() {
                                if ("" != this.textData.groupList[this.textData.groupIdex].text) {
                                    var e = s.getTextFileNames();
                                    if (e.count >= 50) a.globalData.showModalTips(this.$t("\u5df2\u8d85\u8fc7\u6700\u5927\u6587\u4ef6\u6570\u91cf ") + 50, !0);
                                    else if (e.noSpace) a.globalData.showModalTips(this.$t("\u5b58\u50a8\u7a7a\u95f4\u4e0d\u8db3"), !0);
                                    else {
                                        var t = s.getNewFileName(!1);
                                        this.checkAndAddTextFile(t)
                                    }
                                } else a.globalData.showModalTips(this.inputNote)
                            },
                            fileAddSaveClick: function() {
                                if ("" != this.textData.groupList[this.textData.groupIdex].text) {
                                    var e = this;
                                    uni.showModal({
                                        content: e.$t("\u4fdd\u5b58\u6587\u4ef6") + this.currSelectedFile + "?",
                                        showCancel: !0,
                                        success: function(t) {
                                            t.confirm && e.saveTextFileData(e.currSelectedFile)
                                        },
                                        fail: function(e) {}
                                    })
                                } else a.globalData.showModalTips(this.inputNote)
                            },
                            fileAddSelectClick: function() {
                                var t = this;
                                uni.navigateTo({
                                    url: "/sub/pages/files/files",
                                    events: {
                                        acceptDataFromOpenedPage: function(r) {
                                            e("log", "acceptDataFromOpenedPage", r, " at sub/pages/text/text.js:1365"), t.getFileDataByName(r.fileName)
                                        }
                                    },
                                    success: function(e) {
                                        e.eventChannel.emit("acceptDataFromOpenerPage", {
                                            callFrom: "text"
                                        })
                                    },
                                    fail: function(t) {
                                        e("log", t, " at sub/pages/text/text.js:1373")
                                    }
                                })
                            },
                            onBtnSetTouchStart: function(e) {
                                this.startPosition.x = e.touches[0].clientX - this.position.x, this.startPosition.y = e.touches[0].clientY - this.position.y
                            },
                            onBtnSetTouchMove: function(e) {
                                this.position.x = e.touches[0].clientX - this.startPosition.x, this.position.y = e.touches[0].clientY - this.startPosition.y
                            },
                            onBtnSetTouchEnd: function() {},
                            onBtnSetClick: function(e) {
                                uni.navigateTo({
                                    url: "/pages/subset/subset"
                                })
                            },
                            fileAddClick: function(t) {
                                var r = this,
                                    n = [this.$t("\u53e6\u5b58\u6587\u4ef6"), this.$t("\u9009\u62e9\u6587\u4ef6")];
                                this.currSelectedFile && n.push(this.$t("\u4fdd\u5b58\u6587\u4ef6")), uni.showActionSheet({
                                    itemList: n,
                                    success: function(e) {
                                        0 == e.tapIndex && r.fileAddAddClick(), 1 == e.tapIndex && r.fileAddSelectClick(), 2 == e.tapIndex && r.fileAddSaveClick()
                                    },
                                    fail: function(t) {
                                        e("log", t.errMsg, " at sub/pages/text/text.js:1410")
                                    }
                                })
                            }
                        }
                    };
                t.default = b
            }).call(this, r("f3b9")["default"])
        },
        "getGlobalObject": function(t, r) {
            var n;
            n = function() {
                return this
            }();
            try {
                n = n || new Function("return this")()
            } catch (e) {
                "object" === typeof window && (n = window)
            }
            t.exports = n
        },
        "base64js": function(e, t, r) {
            "use strict";
            t.byteLength = function(e) {
                var t = s(e),
                    r = t[0],
                    n = t[1];
                return 3 * (r + n) / 4 - n
            }, t.toByteArray = function(e) {
                var t, r, n = s(e),
                    i = n[0],
                    c = n[1],
                    o = new a(function(e, t, r) {
                        return 3 * (t + r) / 4 - r
                    }(0, i, c)),
                    l = 0,
                    p = c > 0 ? i - 4 : i;
                for (r = 0; r < p; r += 4) t = h[e.charCodeAt(r)] << 18 | h[e.charCodeAt(r + 1)] << 12 | h[e.charCodeAt(r + 2)] << 6 | h[e.charCodeAt(r + 3)], o[l++] = t >> 16 & 255, o[l++] = t >> 8 & 255, o[l++] = 255 & t;
                2 === c && (t = h[e.charCodeAt(r)] << 2 | h[e.charCodeAt(r + 1)] >> 4, o[l++] = 255 & t);
                1 === c && (t = h[e.charCodeAt(r)] << 10 | h[e.charCodeAt(r + 1)] << 4 | h[e.charCodeAt(r + 2)] >> 2, o[l++] = t >> 8 & 255, o[l++] = 255 & t);
                return o
            }, t.fromByteArray = function(e) {
                for (var t, r = e.length, h = r % 3, a = [], i = 0, c = r - h; i < c; i += 16383) a.push(p(e, i, i + 16383 > c ? c : i + 16383));
                1 === h ? (t = e[r - 1], a.push(n[t >> 2] + n[t << 4 & 63] + "==")) : 2 === h && (t = (e[r - 2] << 8) + e[r - 1], a.push(n[t >> 10] + n[t >> 4 & 63] + n[t << 2 & 63] + "="));
                return a.join("")
            };
            for (var n = [], h = [], a = "undefined" !== typeof Uint8Array ? Uint8Array : Array, i = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", c = 0, o = i.length; c < o; ++c) n[c] = i[c], h[i.charCodeAt(c)] = c;

            function s(e) {
                var t = e.length;
                if (t % 4 > 0) throw new Error("Invalid string. Length must be a multiple of 4");
                var r = e.indexOf("="); - 1 === r && (r = t);
                var n = r === t ? 0 : 4 - r % 4;
                return [r, n]
            }

            function l(e) {
                return n[e >> 18 & 63] + n[e >> 12 & 63] + n[e >> 6 & 63] + n[63 & e]
            }

            function p(e, t, r) {
                for (var n, h = [], a = t; a < r; a += 3) n = (e[a] << 16 & 16711680) + (e[a + 1] << 8 & 65280) + (255 & e[a + 2]), h.push(l(n));
                return h.join("")
            }
            h["-".charCodeAt(0)] = 62, h["_".charCodeAt(0)] = 63
        },
        "playListManagerComponent ": function(e, t, r) {
            "use strict";
            (function(e) {
                var n = r("esModuleInteropHelper");
                Object.defineProperty(t, "__esModule", {
                    value: !0
                }), t.default = void 0;
                var h = n(r("uniPopupComponentExportWrapper")),
                    a = getApp(),
                    i = r("handDrawFileManager"),
                    c = r("deviceCommandUtils "),
                    o = r("c4ce"),
                    s = r("handDrawGeometryUtils "),
                    l = a.globalData.MaxSaveFileCount,
                    p = a.globalData.MaxListCount,
                    d = {
                        data: function() {
                            var e = a.globalData.getDeviceFeatures(),
                                t = s.colorSeg;
                            return {
                                screen_width: a.globalData.screen_width_str,
                                scUnit: a.globalData.screen_width_float,
                                rtl: a.globalData.rtl,
                                ntitle: this.$t("\u64ad\u653e\u5217\u8868"),
                                playListName: "",
                                playListNamesAll: [],
                                playListNewName: "",
                                selectFiles: [],
                                drawCanvas: {
                                    w: s.defaultWith,
                                    h: s.defaultHeight
                                },
                                features: e,
                                colorSeg: t,
                                viewImgIdx: 0,
                                viewImgPath: "",
                                viewImgState: "play",
                                currFileData: [],
                                slviewImgTime: 5,
                                renamePlayFile: !1,
                                drawCanvasShow: !0
                            }
                        },
                        onShow: function() {
                            var e = i.getPlayListFileNames();
                            this.playListNamesAll = e.fileNames
                        },
                        components: {
                            uniPopup: h.default
                        },
                        methods: {
                            playListFileClick: function(e) {
                                var t = this,
                                    r = i.getPlayListFileData(e);
                                if (r) {
                                    var n = r.data;
                                    if (0 == n.length) return;
                                    uni.showLoading({
                                        mask: !0
                                    }), setTimeout((function() {
                                        for (var e = "", r = 0; r < n.length; r++) {
                                            "" != e && (e += "zz");
                                            var h = t.playDrawFile(n[r].fileName, n[r].playTime);
                                            e += c.drawPointStrToCmd(h, t.features, r)
                                        }
                                        uni.hideLoading(), e && o.gosend(!0, e, t.sendComplete)
                                    }), 100)
                                }
                            },
                            playDrawFile: function(e) {
                                var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : -1,
                                    r = "",
                                    n = i.getHandDrawImg(e);
                                if (n) {
                                    var h = uni.createCanvasContext("drawCanvas", this);
                                    h.setLineWidth(4);
                                    var a = {
                                            ctx: h,
                                            w: this.drawCanvas.w,
                                            h: this.drawCanvas.h,
                                            draw_line_type: [],
                                            colorSeg: this.colorSeg
                                        },
                                        o = s.drawPs(n.drawPoints, a, null);
                                    r = c.getDrawPointStr(o, n.pisObj, this.features, t)
                                }
                                return r
                            },
                            deleteListFileClick: function(t) {
                                var r = this;
                                uni.showModal({
                                    content: this.$t("\u5220\u9664") + " " + t + "?",
                                    success: function(n) {
                                        if (n.confirm) {
                                            var h = i.deletePlayList(t);
                                            if (h) {
                                                var c = r.playListNamesAll.indexOf(t); - 1 !== c && r.playListNamesAll.splice(c, 1), a.globalData.showModalTips(r.$t("\u5220\u9664\u6210\u529f"))
                                            } else a.globalData.showModalTips(r.$t("\u5220\u9664\u5931\u8d25"))
                                        } else n.cancel && e("log", "\u7528\u6237\u70b9\u51fb\u53d6\u6d88", " at sub/pages/listMaster/listMaster.js:103")
                                    }
                                })
                            },
                            listNameInputOkClick: function(e) {
                                this.$refs.listNameInput.close();
                                var t = this.playListNewName,
                                    r = this.playListName;
                                this.savePlayListName(t, r, this.renamePlayFile)
                            },
                            savePlayListName: function(e) {
                                var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : "",
                                    r = arguments.length > 2 && void 0 !== arguments[2] && arguments[2],
                                    n = this;
                                if ("" == e) uni.showModal({
                                    content: n.$t("\u5217\u8868\u540d\u79f0\u4e0d\u80fd\u4e3a\u7a7a"),
                                    showCancel: !1,
                                    success: function(e) {},
                                    fail: function(e) {},
                                    complete: function() {
                                        n.$refs.listNameInput.open("center")
                                    }
                                });
                                else {
                                    var h = n.playListNamesAll.indexOf(e);
                                    if (-1 != h) r && t == e || uni.showModal({
                                        content: n.$t("\u5217\u8868\u540d\u79f0\u5df2\u5b58\u5728\uff0c\u8bf7\u91cd\u65b0\u8f93\u5165"),
                                        showCancel: !1,
                                        success: function(e) {
                                            n.$refs.listNameInput.open("center")
                                        },
                                        fail: function(e) {}
                                    });
                                    else {
                                        var c = i.savePlayListFileData(e, n.selectFiles);
                                        if (c) {
                                            if (r) {
                                                if (i.deletePlayList(t)) {
                                                    var o = n.playListNamesAll.indexOf(t);
                                                    n.playListNamesAll.splice(o, 1)
                                                }
                                                a.globalData.showModalTips(n.$t("\u4fee\u6539\u6210\u529f"))
                                            } else a.globalData.showModalTips(n.$t("\u6dfb\u52a0\u6210\u529f"));
                                            n.playListNamesAll.unshift(e), n.playListName = e
                                        } else r ? a.globalData.showModalTips(n.$t("\u4fee\u6539\u5931\u8d25")) : a.globalData.showModalTips(n.$t("\u6dfb\u52a0\u5931\u8d25"))
                                    }
                                }
                            },
                            listNameNewInput: function(e) {
                                this.playListNewName = e.detail.value
                            },
                            listNameInputCancelClick: function(e) {
                                this.$refs.listNameInput.close()
                            },
                            playListEdit: function(e) {
                                "" != e && uni.navigateTo({
                                    url: "/sub/pages/playList/playList?playListName=" + e
                                })
                            },
                            playListAdd: function(t) {
                                var r = this,
                                    n = [];
                                uni.navigateTo({
                                    url: "/sub/pages/files/files",
                                    events: {
                                        acceptDataFromOpenedPage: function(t) {
                                            e("log", "acceptDataFromOpenedPage", JSON.stringify(t.selectFiles), " at sub/pages/listMaster/listMaster.js:219"), r.saveNewPlayList(t.selectFiles)
                                        }
                                    },
                                    success: function(e) {
                                        e.eventChannel.emit("acceptDataFromOpenerPage", {
                                            callFrom: "draw",
                                            mode: "add",
                                            listCount: n.length
                                        })
                                    },
                                    fail: function(t) {
                                        e("log", t, " at sub/pages/listMaster/listMaster.js:227")
                                    }
                                })
                            },
                            saveNewPlayList: function(e) {
                                if (!(e.length <= 0))
                                    if (e.length > p) a.globalData.showModalTips(this.$t("\u5df2\u8d85\u8fc7\u6700\u5927\u6587\u4ef6\u6570\u91cf ") + p, !0);
                                    else {
                                        var t = i.getNewPlayListName();
                                        t.count >= l ? a.globalData.showModalTips(this.$t("\u5df2\u8d85\u8fc7\u6700\u5927\u6587\u4ef6\u6570\u91cf ") + l, !0) : t.noSpace ? a.globalData.showModalTips(this.$t("\u5b58\u50a8\u7a7a\u95f4\u4e0d\u8db3"), !0) : (this.renamePlayFile = !1, this.playListNewName = i.getNewPlayListName(), this.$refs.listNameInput.open("center"), this.selectFiles = e)
                                    }
                            },
                            editListFileClick: function(t) {
                                this.renamePlayFile = !0, this.playListName = t, this.playListNewName = t;
                                var r = i.getPlayListFileData(t);
                                e("log", "editListFileClick", JSON.stringify(r), " at sub/pages/listMaster/listMaster.js:257"), this.selectFiles = r.data, this.$refs.listNameInput.open("center")
                            },
                            playListFileSelectClick: function(e) {
                                this.playListName = e
                            },
                            playListViewClick: function(e) {
                                this.viewImgIdx = 0;
                                var t = i.getPlayListFileData(e);
                                this.currFileData = t.data, this.$refs.playListView.open("bottom"), this.viewImgState = "play", this.autoView(0)
                            },
                            playListViewChange: function(e) {
                                e.show || (this.viewImgState = "stop")
                            },
                            playListViewImgClick: function(e) {
                                "stop2" == this.viewImgState ? (this.viewImgState = "play", this.autoView(0)) : "play" == this.viewImgState && (this.viewImgState = "stop")
                            },
                            autoView: function(t) {
                                if (e("log", "autoView ", this.viewImgState, " at sub/pages/listMaster/listMaster.js:288"), "stop" != this.viewImgState) {
                                    this.viewImgIdx >= this.currFileData.length - 1 && (this.viewImgIdx = 0, t = 0);
                                    var r = this;
                                    this.getPlayListItemImg(t, (function(e, t) {
                                        r.viewImgPath = t, r.currFileData.length > 1 ? setTimeout((function() {
                                            r.autoView(1)
                                        }), 150 * r.slviewImgTime) : r.viewImgState = "stop2"
                                    }))
                                } else this.viewImgState = "stop2"
                            },
                            getPlayListItemImg: function(e, t) {
                                var r = this.viewImgIdx + e;
                                if (!(r < 0 || r >= this.currFileData.length)) {
                                    this.viewImgIdx = r;
                                    var n = this.currFileData[r],
                                        h = i.getHandDrawImg(n.fileName);
                                    h && i.isImgFileExist(h.picPath, (function(e) {
                                        e ? t(n, h.picPath) : that.createImg(fileName, h, (function(e, r) {
                                            e && t(n, r)
                                        }))
                                    }))
                                }
                            },
                            createImg: function(e, t, r) {
                                var n = uni.createCanvasContext("drawCanvas", this);
                                n.setLineWidth(4);
                                var h = {
                                    ctx: n,
                                    w: this.drawCanvas.w,
                                    h: this.drawCanvas.h,
                                    draw_line_type: [],
                                    colorSeg: this.colorSeg
                                };
                                s.drawPs(t.drawPoints, h, null);
                                n.draw(!1, (function(n) {
                                    uni.canvasToTempFilePath({
                                        canvasId: "drawCanvas",
                                        destWidth: 100,
                                        destHeight: 100,
                                        success: function(n) {
                                            i.saveHandDrawImg(e, n.tempFilePath, t.drawPoints, t.pointCnt, t.pisObj, t.features), r(!0, n.tempFilePath)
                                        },
                                        fail: function(e) {
                                            r(!1, "")
                                        }
                                    })
                                }))
                            },
                            slViewTimeChange: function(e) {
                                var t = e.detail.value;
                                this.slviewImgTime = t
                            },
                            playListViewItemClick: function(e) {
                                var t = this;
                                "play" == this.viewImgState && (this.viewImgState = "stop"), this.getPlayListItemImg(e, (function(e, r) {
                                    t.viewImgPath = r
                                }))
                            }
                        }
                    };
                t.default = d
            }).call(this, r("f3b9")["default"])
        },
        "buffer": function(t, r, n) {
            "use strict";
            (function(t) {
                /*!
                 * The buffer module from node.js, for the browser.
                 *
                 * @author   Feross Aboukhadijeh <http://feross.org>
                 * @license  MIT
                 */
                var h = n("base64js"),
                    a = n("