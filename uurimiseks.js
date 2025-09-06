       "095b": function(e, t, r) {
            (function(t) {
                var n = r("34cf"),
                    h = r("af34"),
                    a = r("35ce");

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
