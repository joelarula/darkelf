        "3b77": function(e, t, r) {
            (function(t) {
                var n = r("af34");

                function h(e, t) {
                    var r = "undefined" !== typeof Symbol && e[Symbol.iterator] || e["@@iterator"];
                    if (!r) {
                        if (Array.isArray(e) || (r = function(e, t) {
                                if (!e) return;
                                if ("string" === typeof e) return a(e, t);
                                var r = Object.prototype.toString.call(e).slice(8, -1);
                                "Object" === r && e.constructor && (r = e.constructor.name);
                                if ("Map" === r || "Set" === r) return Array.from(e);
                                if ("Arguments" === r || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(r)) return a(e, t)
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

                function a(e, t) {
                    (null == t || t > e.length) && (t = e.length);
                    for (var r = 0, n = new Array(t); r < t; r++) n[r] = e[r];
                    return n
                }

                function i(e) {
                    var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : 4,
                        r = Math.round(e);
                    r < 0 && (r = 32768 | -r);
                    var n = ("0000" + r.toString(16)).slice(-t);
                    return n
                }

                function c(e, t) {
                    var r = e << 4 | 15 & t;
                    return r
                }

                function o(e, t) {
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

                function s(e, t) {
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
                        for (var x = o(a, 800), V = "", f = "", F = 0; F < x.length; F++) V += i(x[F][0], 2), f += i(x[F][1], 2);
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
                    for (var X = o(h, 800), N = "", H = "", z = 0; z < X.length; z++) N += i(X[z][0], 2), H += i(X[z][1], 2);
                    return [e.concat(m), N, H, -k * t / 2]
                }

                function l(e, r, n, h) {
                    var a = arguments.length > 4 && void 0 !== arguments[4] ? arguments[4] : 0;
                    if (0 == e.length) return null;
                    var o = 0,
                        l = 0,
                        p = -1,
                        d = "",
                        b = "",
                        g = i(a, 2),
                        j = "",
                        x = "",
                        V = 8,
                        f = .5,
                        F = V,
                        k = 0,
                        m = "00";
                    m = n.textDecimalTime ? i(Math.floor(10 * r), 2) : i(Math.floor(r), 2), t("log", "time = ", m, " at utils/funcTools.js:337"), V >= 8 && (F = 0);
                    var P = !1;
                    if (P) t("error", "20241210 - \u5f53\u524d\u4ee3\u7801\u4e3a\u5750\u6807\u8c03\u5f0f\u6a21\u5f0f\uff0c\u4e0d\u53ef\u53d1\u7248", " at utils/funcTools.js:345"), xyss = e, se1 = 0, se2 = 0, xOffset = 0;
                    else {
                        var u = s(e, f, h);
                        xyss = u[0], se1 = u[1], se2 = u[2], xOffset = u[3]
                    }
                    for (var X = 0; X < xyss.length; X++) {
                        p != xyss[X][0] && (p = xyss[X][0], l > 0 && (j += i(k, 2), k = 0), l++, x += i(Math.round(Number(xyss[X][2] * f)), 2), V >= 8 && xyss[X][1].length > 1 && F++), F >= 8 && (F = 1);
                        var N = xyss[X][1];
                        k += N.length;
                        for (var H = 0; H < N.length; H++) {
                            o++;
                            var z = N[H],
                                Q = Math.round(Number(z.x * f) + xOffset),
                                R = Math.round(Number(z.y * f)),
                                v = Number(z.z),
                                I = F;
                            0 == H && (I = 0, v = 1), H == N.length - 1 && (v = 1), 1 == N.length && (v = Number(z.z)), n.textStopTime && N.length > 1 && (0 == I ? v = 2 : (H < N.length - 1 && 0 == N[H + 1].s || H == N.length - 1) && (v = 3)), d = d + i(Q) + i(R) + i(c(I, v), 2), P && (b = b + "\n{" + Q + "," + R + "," + I + "," + v + "},")
                        }
                    }
                    return P && t("log", "\u6587\u5b57\u5750\u6807(\u7ed8\u56fe\u8f6f\u4ef6\u683c\u5f0f)", b, " at utils/funcTools.js:408"), j += i(k, 2), 0 == o ? null : {
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

                function p(e, t) {
                    for (var r = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : "00", n = Math.floor(e.length / 2), h = e, a = n; a < t; a++) h += r;
                    return h
                }

                function d(e) {
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

                function b(e, t) {
                    var r, a = n(t),
                        i = h(e);
                    try {
                        for (i.s(); !(r = i.n()).done;) {
                            var c = r.value,
                                o = d(c, -1);
                            if (o.idx < a.length) {
                                var s = a[o.idx] & o.val;
                                if (a[o.idx] != s) {
                                    a[o.idx] = s;
                                    var l = d(c, 50);
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

                function g(e, t) {
                    if (e.hasOwnProperty("features")) {
                        var r = e.features;
                        if (r.hasOwnProperty(t)) return r[t]
                    }
                    return null
                }

                function j(e, t, r, n) {
                    for (var h = arguments.length > 4 && void 0 !== arguments[4] ? arguments[4] : "00", a = "", o = "", s = 0; s < 15; s++) s <= 11 ? o += i(t.cnfValus[s], 2) : 13 == s ? g({
                        features: r
                    }, "picsPlay") ? o += i(-1 == n ? 10 * t.cnfValus[12] : 10 * n, 2) : o += "00" : 14 == s && r.textStopTime ? o += i(t.txPointTime, 2) : o += "00";
                    if ("00" == h) {
                        o += h;
                        for (var l = 0; l < e.length; l++) {
                            var p = e[l],
                                d = p[3];
                            r.textStopTime && (0 == p[2] ? d = 2 : (l < e.length - 1 && 0 == e[l + 1][2] || l == e.length - 1) && (d = 3)), a = a + i(p[0].toFixed()) + i(p[1].toFixed()) + i(c(p[2], d), 2)
                        }
                        a = o + i(e.length) + a
                    } else o += h, a = o;
                    return a
                }

                function x(e, t) {
                    var r = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null,
                        n = "";
                    return n = null == r ? t.picsPlay ? "f0f1f200" + e + "f4f5f6f7" : "f0f1f2f3" + e + "f4f5f6f7" : "f0f1f2" + i(r, 2) + e + "f4f5f6f7", n.toUpperCase()
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
                            d = i(t, 2),
                            b = 8,
                            g = .5,
                            j = b,
                            x = 0;
                        b >= 8 && (j = 0);
                        var V = s(e, g);
                        xyss = V[0], se = V[1] + V[2], xOffset = V[3];
                        for (var f = 0; f < xyss.length; f++) {
                            h != xyss[f][0] && (h = xyss[f][0], n > 0 && (l += i(x, 2), x = 0), n++, p += i(Math.round(Number(xyss[f][2] * g)), 2), b >= 8 && xyss[f][1].length > 1 && j++), j >= 8 && (j = 1);
                            var F = xyss[f][1];
                            x += F.length;
                            for (var k = 0; k < F.length; k++) {
                                r++;
                                var m = F[k],
                                    P = Math.round(Number(m.x * g) + xOffset),
                                    u = Math.round(Number(m.y * g)),
                                    X = Number(m.z),
                                    N = j;
                                0 == k && (N = 0, X = 1), k == F.length - 1 && (X = 1), 1 == F.length && (X = Number(m.z)), a = a + i(P) + i(u) + i(c(N, X), 2), o = o + "\n" + P + "," + u + ",(" + N + "," + X + "),"
                            }
                        }
                        return l += i(x, 2), 0 == r ? "" : (a = "A0A1A2A3" + i(r) + i(n, 2) + a + p + l + se + d + "A4A5A6A7", a.toUpperCase())
                    },
                    getXysCmdArr: function(e, r, n) {
                        for (var h = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : 0, a = [], c = 0; c < e.length; c++) {
                            var o = e[c].xys,
                                s = n;
                            255 == n && null != e[c].XysRight ? o = e[c].XysRight : 127 == n && null != e[c].XysUp ? o = e[c].XysUp : 128 == n && null != e[c].XysDown ? o = e[c].XysDown : s = 0;
                            var p = l(o, e[c].time, r, s, h);
                            null != p && a.push(p)
                        }
                        if (0 == a.length) return "";
                        for (var d = 0, b = 0, g = "", j = "", x = "", V = "", f = "", F = "", k = "", m = "", P = 0; P < a.length; P++) d += a[P].cnt, b += a[P].charCount, i(a[P].cnt), g += i(a[P].charCount, 2), j += a[P].cmd, x += a[P].charWidthCmd, V += a[P].charPointCmd, f += a[P].se1, F += a[P].se2, k += a[P].ver, m += a[P].time;
                        t("log", d, b, " at utils/funcTools.js:308");
                        var u = i(a.length, 2),
                            X = "A0A1A2A3" + i(d) + i(b, 2) + j + u + g + x + V + f + F + k + m + "A4A5A6A7";
                        return X.toUpperCase()
                    },
                    getCmdStr: function(e) {
                        var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null,
                            r = i(e.curMode, 2),
                            h = i(0, 2),
                            a = i(e.textData.txColor, 2),
                            c = i(e.textData.txSize / 100 * 255, 2),
                            o = i(e.textData.txSize / 100 * 255, 2),
                            s = i(e.textData.runSpeed / 100 * 255, 2),
                            l = "00",
                            p = i(e.textData.txDist / 100 * 255, 2),
                            d = i(e.prjData.public.rdMode, 2),
                            j = i(e.prjData.public.soundVal / 100 * 255, 2),
                            x = "ffffffff0000";
                        if (null != t) {
                            if (x = "", t.hasOwnProperty("groupList"))
                                for (var V = 0; V < t.groupList.length; V++) x += i(t.groupList[V].color, 2);
                            x += "ffffffff", x = x.substring(0, 8), g(t, "textStopTime") && (x += i(e.textData.txPointTime, 2)), x += "0000", x = x.substring(0, 12)
                        }
                        var f = "",
                            F = e.prjData.prjItem;
                        for (var k in F) {
                            var m = F[k],
                                P = 0 == m.pyMode ? 0 : 128;
                            0 != P && null != t && t.hasOwnProperty("prjParm") && t.prjParm.prjIndex == k && (3 == k && g(t, "animationFix") && [2, 4, 11, 13, 19].includes(t.prjParm.selIndex) ? P |= 50 - t.prjParm.selIndex : P |= t.prjParm.selIndex);
                            var u = i(P, 2),
                                X = "",
                                N = n(m.prjSelected);
                            3 == k && g(t, "animationFix") && (N = b([2, 4, 11, 13, 19], N));
                            for (var H = 0; H < N.length; H++) X = i(N[H]) + X;
                            f = f + u + X
                        }
                        var z = "";
                        g(t, "arbPlay") && (z += i(e.textData.runDir, 2));
                        for (var Q = "", R = Math.floor(z.length / 2), v = R; v < 44; v++) Q += "00";
                        var I = "c0c1c2c3" + r + h + a + c + o + s + l + p + d + j + x + f + z + Q + "c4c5c6c7";
                        return I.toUpperCase()
                    },
                    getShakeCmdStr: function(e) {
                        var r = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null,
                            n = "";
                        if (g(r, "xyCnf")) {
                            n = "00", r.hasOwnProperty("xyCnfSave") && !r.xyCnfSave && (n = "ff");
                            var a = e.subsetData.xyCnf;
                            a.auto ? n += i(a.autoValue, 2) : n += i(255 - a.autoValue, 2), n += i(a.phase, 2);
                            var c, o = h(a.xy);
                            try {
                                for (o.s(); !(c = o.n()).done;) {
                                    var s = c.value;
                                    n += i(s.value, 2)
                                }
                            } catch (d) {
                                o.e(d)
                            } finally {
                                o.f()
                            }
                            t("log", "xyCnf", JSON.stringify(a), " at utils/funcTools.js:551")
                        }
                        n = p(n, 16, "00");
                        var l = "10111213" + n + "14151617";
                        return l.toUpperCase()
                    },
                    getDrawPointStr: j,
                    getDrawCmdStr: function(e, t, r) {
                        var n = arguments.length > 3 && void 0 !== arguments[3] ? arguments[3] : "00",
                            h = j(e, t, r, -1, n);
                        return x(h, r)
                    },
                    drawPointStrToCmd: x,
                    getPisCmdStr: function(e, r) {
                        for (var n = arguments.length > 2 && void 0 !== arguments[2] ? arguments[2] : null, h = r.cnfValus, a = "01", c = i(e, 2), o = a + c, s = 0; s <= 12; s++) o += i(h[s], 2);
                        var l = i(10 * r.playTime, 2);
                        if (o += l, g(n, "xyCnf")) {
                            for (var d = 14; d <= 18; d++) o += i(h[d], 2);
                            t("log", "13-17", h[14], h[15], h[16], h[17], h[18], " at utils/funcTools.js:516"), o = p(o, 24, "00")
                        } else o = p(o, 18, "00");
                        var b = "d0d1d2d3" + o + "d4d5d6d7";
                        return b.toUpperCase()
                    },
                    getPisListCmdStr: function(e) {
                        for (var r = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : null, n = i(128 | e.length, 2), h = "FF", a = "", c = 0; c < e.length; c++) {
                            for (var o = "", s = e[c], l = 0; l <= 12; l++) o += i(s.cnfValus[l], 2);
                            var d = i(10 * s.playTime, 2);
                            if (o += d, g(r, "xyCnf")) {
                                for (var b = s.cnfValus, j = 14; j <= 18; j++) o += i(b[j], 2);
                                t("log", "pgs 14-18", b[14], b[15], b[16], b[17], b[18], " at utils/funcTools.js:488"), o = p(o, 21, "00")
                            } else o = p(o, 15, "00");
                            a = a + o + h
                        }
                        return a = "d0d1d2d3" + n + "00" + a + "d4d5d6d7", a.toUpperCase()
                    },
                    getSettingCmd: function(e) {
                        var t = i(e.valArr[0]),
                            r = i(e.ch, 2),
                            n = i(e.valArr[1], 2),
                            h = i(e.xy, 2),
                            a = i(e.valArr[2], 2),
                            c = i(e.valArr[3], 2),
                            o = i(e.valArr[4], 2),
                            s = i(e.light, 2),
                            l = i(e.cfg, 2);
                        0 == e.cfg && (a = "FF", c = "FF", o = "FF");
                        var p = i(e.lang, 2),
                            d = "00010203" + t + r + n + h + a + c + o + s + l + p + "000000000004050607";
                        return d.toUpperCase()
                    },
                    getCmdValue: function(e, r, n) {
                        var h = new RegExp(e + "(.+?)" + r),
                            a = h.exec(n);
                        return null !== a ? a[1] : (t("log", "\u672a\u5339\u914d\u5230\u7b26\u5408\u8981\u6c42\u7684\u5b57\u7b26\u4e32", e, r, " at utils/funcTools.js:7"), "")
                    },
                    getQueryCmd: function(e) {
                        for (var t = "", r = 0; r < e.length; r++) t += i(e[r], 2);
                        var n = "E0E1E2E3" + t + "E4E5E6E7";
                        return n.toUpperCase()
                    },
                    getDrawLineStr: function(e, t) {
                        for (var r = "", n = 0; n < e.length; n++) {
                            var h = e[n];
                            r = r + i(h.pt.x) + i(h.pt.y) + i(c(h.color, h.z), 2)
                        }
                        return r = "10111213" + i(t) + i(e.length, 2) + r + "14151617", r.toUpperCase()
                    },
                    getFeaturesValue: g
                }
            }).call(this, r("f3b9")["default"])
