        "codePointAt": function(t, r, n) {
            (function(h, a) {
                var i, c, o, s = n("typeofHelper");
                (function(e, n) {
                    "object" === s(r) && "undefined" !== typeof t ? n(r) : (c = [r], i = n, o = "function" === typeof i ? i.apply(r, c) : i, void 0 === o || (t.exports = o))
                })(0, (function(t) {
                    "use strict";
                    /*! https://mths.be/codepointat v0.2.0 by @mathias */
                    String.prototype.codePointAt || function() {
                        var e = function() {
                                try {
                                    var e = {},
                                        t = Object.defineProperty,
                                        r = t(e, e, e) && t
                                } catch (n) {}
                                return r
                            }(),
                            t = function(e) {
                                if (null == this) throw TypeError();
                                var t = String(this),
                                    r = t.length,
                                    n = e ? Number(e) : 0;
                                if (n != n && (n = 0), !(n < 0 || n >= r)) {
                                    var h, a = t.charCodeAt(n);
                                    return a >= 55296 && a <= 56319 && r > n + 1 && (h = t.charCodeAt(n + 1), h >= 56320 && h <= 57343) ? 1024 * (a - 55296) + h - 56320 + 65536 : a
                                }
                            };
                        e ? e(String.prototype, "codePointAt", {
                            value: t,
                            configurable: !0,
                            writable: !0
                        }) : String.prototype.codePointAt = t
                    }();

                    function r() {
                        this.table = new Uint16Array(16), this.trans = new Uint16Array(288)
                    }

                    function i(e, t) {
                        this.source = e, this.sourceIndex = 0, this.tag = 0, this.bitcount = 0, this.dest = t, this.destLen = 0, this.ltree = new r, this.dtree = new r
                    }
                    var c = new r,
                        o = new r,
                        l = new Uint8Array(30),
                        p = new Uint16Array(30),
                        d = new Uint8Array(30),
                        b = new Uint16Array(30),
                        g = new Uint8Array([16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15]),
                        j = new r,
                        x = new Uint8Array(320);

                    function V(e, t, r, n) {
                        var h, a;
                        for (h = 0; h < r; ++h) e[h] = 0;
                        for (h = 0; h < 30 - r; ++h) e[h + r] = h / r | 0;
                        for (a = n, h = 0; h < 30; ++h) t[h] = a, a += 1 << e[h]
                    }
                    var f = new Uint16Array(16);

                    function F(e, t, r, n) {
                        var h, a;
                        for (h = 0; h < 16; ++h) e.table[h] = 0;
                        for (h = 0; h < n; ++h) e.table[t[r + h]]++;
                        for (e.table[0] = 0, a = 0, h = 0; h < 16; ++h) f[h] = a, a += e.table[h];
                        for (h = 0; h < n; ++h) t[r + h] && (e.trans[f[t[r + h]]++] = h)
                    }

                    function k(e) {
                        e.bitcount-- || (e.tag = e.source[e.sourceIndex++], e.bitcount = 7);
                        var t = 1 & e.tag;
                        return e.tag >>>= 1, t
                    }

                    function m(e, t, r) {
                        if (!t) return r;
                        while (e.bitcount < 24) e.tag |= e.source[e.sourceIndex++] << e.bitcount, e.bitcount += 8;
                        var n = e.tag & 65535 >>> 16 - t;
                        return e.tag >>>= t, e.bitcount -= t, n + r
                    }

                    function P(e, t) {
                        while (e.bitcount < 24) e.tag |= e.source[e.sourceIndex++] << e.bitcount, e.bitcount += 8;
                        var r = 0,
                            n = 0,
                            h = 0,
                            a = e.tag;
                        do {
                            n = 2 * n + (1 & a), a >>>= 1, ++h, r += t.table[h], n -= t.table[h]
                        } while (n >= 0);
                        return e.tag = a, e.bitcount -= h, t.trans[r + n]
                    }

                    function u(e, t, r) {
                        var n, h, a, i, c, o;
                        for (n = m(e, 5, 257), h = m(e, 5, 1), a = m(e, 4, 4), i = 0; i < 19; ++i) x[i] = 0;
                        for (i = 0; i < a; ++i) {
                            var s = m(e, 3, 0);
                            x[g[i]] = s
                        }
                        for (F(j, x, 0, 19), c = 0; c < n + h;) {
                            var l = P(e, j);
                            switch (l) {
                                case 16:
                                    var p = x[c - 1];
                                    for (o = m(e, 2, 3); o; --o) x[c++] = p;
                                    break;
                                case 17:
                                    for (o = m(e, 3, 3); o; --o) x[c++] = 0;
                                    break;
                                case 18:
                                    for (o = m(e, 7, 11); o; --o) x[c++] = 0;
                                    break;
                                default:
                                    x[c++] = l;
                                    break
                            }
                        }
                        F(t, x, 0, n), F(r, x, n, h)
                    }

                    function X(e, t, r) {
                        while (1) {
                            var n, h, a, i, c = P(e, t);
                            if (256 === c) return 0;
                            if (c < 256) e.dest[e.destLen++] = c;
                            else
                                for (c -= 257, n = m(e, l[c], p[c]), h = P(e, r), a = e.destLen - m(e, d[h], b[h]), i = a; i < a + n; ++i) e.dest[e.destLen++] = e.dest[i]
                        }
                    }

                    function N(e) {
                        var t, r, n;
                        while (e.bitcount > 8) e.sourceIndex--, e.bitcount -= 8;
                        if (t = e.source[e.sourceIndex + 1], t = 256 * t + e.source[e.sourceIndex], r = e.source[e.sourceIndex + 3], r = 256 * r + e.source[e.sourceIndex + 2], t !== (65535 & ~r)) return -3;
                        for (e.sourceIndex += 4, n = t; n; --n) e.dest[e.destLen++] = e.source[e.sourceIndex++];
                        return e.bitcount = 0, 0
                    }(function(e, t) {
                        var r;
                        for (r = 0; r < 7; ++r) e.table[r] = 0;
                        for (e.table[7] = 24, e.table[8] = 152, e.table[9] = 112, r = 0; r < 24; ++r) e.trans[r] = 256 + r;
                        for (r = 0; r < 144; ++r) e.trans[24 + r] = r;
                        for (r = 0; r < 8; ++r) e.trans[168 + r] = 280 + r;
                        for (r = 0; r < 112; ++r) e.trans[176 + r] = 144 + r;
                        for (r = 0; r < 5; ++r) t.table[r] = 0;
                        for (t.table[5] = 32, r = 0; r < 32; ++r) t.trans[r] = r
                    })(c, o), V(l, p, 4, 3), V(d, b, 2, 1), l[28] = 0, p[28] = 258;
                    var H = function(e, t) {
                        var r, n, h, a = new i(e, t);
                        do {
                            switch (r = k(a), n = m(a, 2, 0), n) {
                                case 0:
                                    h = N(a);
                                    break;
                                case 1:
                                    h = X(a, c, o);
                                    break;
                                case 2:
                                    u(a, a.ltree, a.dtree), h = X(a, a.ltree, a.dtree);
                                    break;
                                default:
                                    h = -3
                            }
                            if (0 !== h) throw new Error("Data error")
                        } while (!r);
                        return a.destLen < a.dest.length ? "function" === typeof a.dest.slice ? a.dest.slice(0, a.destLen) : a.dest.subarray(0, a.destLen) : a.dest
                    };

                    function z(e, t, r, n, h) {
                        return Math.pow(1 - h, 3) * e + 3 * Math.pow(1 - h, 2) * h * t + 3 * (1 - h) * Math.pow(h, 2) * r + Math.pow(h, 3) * n
                    }

                    function Q() {
                        this.x1 = Number.NaN, this.y1 = Number.NaN, this.x2 = Number.NaN, this.y2 = Number.NaN
                    }

                    function R() {
                        this.commands = [], this.fill = "black", this.stroke = null, this.strokeWidth = 1
                    }

                    function v(e) {
                        throw new Error(e)
                    }

                    function I(e, t) {
                        e || v(t)
                    }
                    Q.prototype.isEmpty = function() {
                        return isNaN(this.x1) || isNaN(this.y1) || isNaN(this.x2) || isNaN(this.y2)
                    }, Q.prototype.addPoint = function(e, t) {
                        "number" === typeof e && ((isNaN(this.x1) || isNaN(this.x2)) && (this.x1 = e, this.x2 = e), e < this.x1 && (this.x1 = e), e > this.x2 && (this.x2 = e)), "number" === typeof t && ((isNaN(this.y1) || isNaN(this.y2)) && (this.y1 = t, this.y2 = t), t < this.y1 && (this.y1 = t), t > this.y2 && (this.y2 = t))
                    }, Q.prototype.addX = function(e) {
                        this.addPoint(e, null)
                    }, Q.prototype.addY = function(e) {
                        this.addPoint(null, e)
                    }, Q.prototype.addBezier = function(e, t, r, n, h, a, i, c) {
                        var o = [e, t],
                            s = [r, n],
                            l = [h, a],
                            p = [i, c];
                        this.addPoint(e, t), this.addPoint(i, c);
                        for (var d = 0; d <= 1; d++) {
                            var b = 6 * o[d] - 12 * s[d] + 6 * l[d],
                                g = -3 * o[d] + 9 * s[d] - 9 * l[d] + 3 * p[d],
                                j = 3 * s[d] - 3 * o[d];
                            if (0 !== g) {
                                var x = Math.pow(b, 2) - 4 * j * g;
                                if (!(x < 0)) {
                                    var V = (-b + Math.sqrt(x)) / (2 * g);
                                    0 < V && V < 1 && (0 === d && this.addX(z(o[d], s[d], l[d], p[d], V)), 1 === d && this.addY(z(o[d], s[d], l[d], p[d], V)));
                                    var f = (-b - Math.sqrt(x)) / (2 * g);
                                    0 < f && f < 1 && (0 === d && this.addX(z(o[d], s[d], l[d], p[d], f)), 1 === d && this.addY(z(o[d], s[d], l[d], p[d], f)))
                                }
                            } else {
                                if (0 === b) continue;
                                var F = -j / b;
                                0 < F && F < 1 && (0 === d && this.addX(z(o[d], s[d], l[d], p[d], F)), 1 === d && this.addY(z(o[d], s[d], l[d], p[d], F)))
                            }
                        }
                    }, Q.prototype.addQuad = function(e, t, r, n, h, a) {
                        var i = e + 2 / 3 * (r - e),
                            c = t + 2 / 3 * (n - t),
                            o = i + 1 / 3 * (h - e),
                            s = c + 1 / 3 * (a - t);
                        this.addBezier(e, t, i, c, o, s, h, a)
                    }, R.prototype.moveTo = function(e, t) {
                        this.commands.push({
                            type: "M",
                            x: e,
                            y: t
                        })
                    }, R.prototype.lineTo = function(e, t) {
                        this.commands.push({
                            type: "L",
                            x: e,
                            y: t
                        })
                    }, R.prototype.curveTo = R.prototype.bezierCurveTo = function(e, t, r, n, h, a) {
                        this.commands.push({
                            type: "C",
                            x1: e,
                            y1: t,
                            x2: r,
                            y2: n,
                            x: h,
                            y: a
                        })
                    }, R.prototype.quadTo = R.prototype.quadraticCurveTo = function(e, t, r, n) {
                        this.commands.push({
                            type: "Q",
                            x1: e,
                            y1: t,
                            x: r,
                            y: n
                        })
                    }, R.prototype.close = R.prototype.closePath = function() {
                        this.commands.push({
                            type: "Z"
                        })
                    }, R.prototype.extend = function(e) {
                        if (e.commands) e = e.commands;
                        else if (e instanceof Q) {
                            var t = e;
                            return this.moveTo(t.x1, t.y1), this.lineTo(t.x2, t.y1), this.lineTo(t.x2, t.y2), this.lineTo(t.x1, t.y2), void this.close()
                        }
                        Array.prototype.push.apply(this.commands, e)
                    }, R.prototype.getBoundingBox = function() {
                        for (var e = new Q, t = 0, r = 0, n = 0, h = 0, a = 0; a < this.commands.length; a++) {
                            var i = this.commands[a];
                            switch (i.type) {
                                case "M":
                                    e.addPoint(i.x, i.y), t = n = i.x, r = h = i.y;
                                    break;
                                case "L":
                                    e.addPoint(i.x, i.y), n = i.x, h = i.y;
                                    break;
                                case "Q":
                                    e.addQuad(n, h, i.x1, i.y1, i.x, i.y), n = i.x, h = i.y;
                                    break;
                                case "C":
                                    e.addBezier(n, h, i.x1, i.y1, i.x2, i.y2, i.x, i.y), n = i.x, h = i.y;
                                    break;
                                case "Z":
                                    n = t, h = r;
                                    break;
                                default:
                                    throw new Error("Unexpected path command " + i.type)
                            }
                        }
                        return e.isEmpty() && e.addPoint(0, 0), e
                    }, R.prototype.draw = function(e) {
                        e.beginPath();
                        for (var t = 0; t < this.commands.length; t += 1) {
                            var r = this.commands[t];
                            "M" === r.type ? e.moveTo(r.x, r.y) : "L" === r.type ? e.lineTo(r.x, r.y) : "C" === r.type ? e.bezierCurveTo(r.x1, r.y1, r.x2, r.y2, r.x, r.y) : "Q" === r.type ? e.quadraticCurveTo(r.x1, r.y1, r.x, r.y) : "Z" === r.type && e.closePath()
                        }
                        this.fill && (e.fillStyle = this.fill, e.fill()), this.stroke && (e.strokeStyle = this.stroke, e.lineWidth = this.strokeWidth, e.stroke())
                    }, R.prototype.toPathData = function(e) {
                        function t(t) {
                            return Math.round(t) === t ? "" + Math.round(t) : t.toFixed(e)
                        }

                        function r() {
                            for (var e = arguments, r = "", n = 0; n < arguments.length; n += 1) {
                                var h = e[n];
                                h >= 0 && n > 0 && (r += " "), r += t(h)
                            }
                            return r
                        }
                        e = void 0 !== e ? e : 2;
                        for (var n = "", h = 0; h < this.commands.length; h += 1) {
                            var a = this.commands[h];
                            "M" === a.type ? n += "M" + r(a.x, a.y) : "L" === a.type ? n += "L" + r(a.x, a.y) : "C" === a.type ? n += "C" + r(a.x1, a.y1, a.x2, a.y2, a.x, a.y) : "Q" === a.type ? n += "Q" + r(a.x1, a.y1, a.x, a.y) : "Z" === a.type && (n += "Z")
                        }
                        return n
                    }, R.prototype.toSVG = function(e) {
                        var t = '<path d="';
                        return t += this.toPathData(e), t += '"', this.fill && "black" !== this.fill && (null === this.fill ? t += ' fill="none"' : t += ' fill="' + this.fill + '"'), this.stroke && (t += ' stroke="' + this.stroke + '" stroke-width="' + this.strokeWidth + '"'), t += "/>", t
                    }, R.prototype.toDOMElement = function(e) {
                        var t = this.toPathData(e),
                            r = document.createElementNS("http://www.w3.org/2000/svg", "path");
                        return r.setAttribute("d", t), r
                    };
                    var w = {
                            fail: v,
                            argument: I,
                            assert: I
                        },
                        y = {},
                        C = {},
                        A = {};

                    function D(e) {
                        return function() {
                            return e
                        }
                    }
                    C.BYTE = function(e) {
                        return w.argument(e >= 0 && e <= 255, "Byte value should be between 0 and 255."), [e]
                    }, A.BYTE = D(1), C.CHAR = function(e) {
                        return [e.charCodeAt(0)]
                    }, A.CHAR = D(1), C.CHARARRAY = function(e) {
                        "undefined" === typeof e && (e = "", h("warn", "Undefined CHARARRAY encountered and treated as an empty string. This is probably caused by a missing glyph name.", " at utils/opentype.js:999"));
                        for (var t = [], r = 0; r < e.length; r += 1) t[r] = e.charCodeAt(r);
                        return t
                    }, A.CHARARRAY = function(e) {
                        return "undefined" === typeof e ? 0 : e.length
                    }, C.USHORT = function(e) {
                        return [e >> 8 & 255, 255 & e]
                    }, A.USHORT = D(2), C.SHORT = function(e) {
                        return e >= 32768 && (e = -(65536 - e)), [e >> 8 & 255, 255 & e]
                    }, A.SHORT = D(2), C.UINT24 = function(e) {
                        return [e >> 16 & 255, e >> 8 & 255, 255 & e]
                    }, A.UINT24 = D(3), C.ULONG = function(e) {
                        return [e >> 24 & 255, e >> 16 & 255, e >> 8 & 255, 255 & e]
                    }, A.ULONG = D(4), C.LONG = function(e) {
                        return e >= 2147483648 && (e = -(4294967296 - e)), [e >> 24 & 255, e >> 16 & 255, e >> 8 & 255, 255 & e]
                    }, A.LONG = D(4), C.FIXED = C.ULONG, A.FIXED = A.ULONG, C.FWORD = C.SHORT, A.FWORD = A.SHORT, C.UFWORD = C.USHORT, A.UFWORD = A.USHORT, C.LONGDATETIME = function(e) {
                        return [0, 0, 0, 0, e >> 24 & 255, e >> 16 & 255, e >> 8 & 255, 255 & e]
                    }, A.LONGDATETIME = D(8), C.TAG = function(e) {
                        return w.argument(4 === e.length, "Tag should be exactly 4 ASCII characters."), [e.charCodeAt(0), e.charCodeAt(1), e.charCodeAt(2), e.charCodeAt(3)]
                    }, A.TAG = D(4), C.Card8 = C.BYTE, A.Card8 = A.BYTE, C.Card16 = C.USHORT, A.Card16 = A.USHORT, C.OffSize = C.BYTE, A.OffSize = A.BYTE, C.SID = C.USHORT, A.SID = A.USHORT, C.NUMBER = function(e) {
                        return e >= -107 && e <= 107 ? [e + 139] : e >= 108 && e <= 1131 ? (e -= 108, [247 + (e >> 8), 255 & e]) : e >= -1131 && e <= -108 ? (e = -e - 108, [251 + (e >> 8), 255 & e]) : e >= -32768 && e <= 32767 ? C.NUMBER16(e) : C.NUMBER32(e)
                    }, A.NUMBER = function(e) {
                        return C.NUMBER(e).length
                    }, C.NUMBER16 = function(e) {
                        return [28, e >> 8 & 255, 255 & e]
                    }, A.NUMBER16 = D(3), C.NUMBER32 = function(e) {
                        return [29, e >> 24 & 255, e >> 16 & 255, e >> 8 & 255, 255 & e]
                    }, A.NUMBER32 = D(5), C.REAL = function(e) {
                        var t = e.toString(),
                            r = /\.(\d*?)(?:9{5,20}|0{5,20})\d{0,2}(?:e(.+)|$)/.exec(t);
                        if (r) {
                            var n = parseFloat("1e" + ((r[2] ? +r[2] : 0) + r[1].length));
                            t = (Math.round(e * n) / n).toString()
                        }
                        for (var h = "", a = 0, i = t.length; a < i; a += 1) {
                            var c = t[a];
                            h += "e" === c ? "-" === t[++a] ? "c" : "b" : "." === c ? "a" : "-" === c ? "e" : c
                        }
                        h += 1 & h.length ? "f" : "ff";
                        for (var o = [30], s = 0, l = h.length; s < l; s += 2) o.push(parseInt(h.substr(s, 2), 16));
                        return o
                    }, A.REAL = function(e) {
                        return C.REAL(e).length
                    }, C.NAME = C.CHARARRAY, A.NAME = A.CHARARRAY, C.STRING = C.CHARARRAY, A.STRING = A.CHARARRAY, y.UTF8 = function(e, t, r) {
                        for (var n = [], h = r, a = 0; a < h; a++, t += 1) n[a] = e.getUint8(t);
                        return String.fromCharCode.apply(null, n)
                    }, y.UTF16 = function(e, t, r) {
                        for (var n = [], h = r / 2, a = 0; a < h; a++, t += 2) n[a] = e.getUint16(t);
                        return String.fromCharCode.apply(null, n)
                    }, C.UTF16 = function(e) {
                        for (var t = [], r = 0; r < e.length; r += 1) {
                            var n = e.charCodeAt(r);
                            t[t.length] = n >> 8 & 255, t[t.length] = 255 & n
                        }
                        return t
                    }, A.UTF16 = function(e) {
                        return 2 * e.length
                    };
                    var S = {
                        "x-mac-croatian": "\xc4\xc5\xc7\xc9\xd1\xd6\xdc\xe1\xe0\xe2\xe4\xe3\xe5\xe7\xe9\xe8\xea\xeb\xed\xec\xee\xef\xf1\xf3\xf2\xf4\xf6\xf5\xfa\xf9\xfb\xfc\u2020\xb0\xa2\xa3\xa7\u2022\xb6\xdf\xae\u0160\u2122\xb4\xa8\u2260\u017d\xd8\u221e\xb1\u2264\u2265\u2206\xb5\u2202\u2211\u220f\u0161\u222b\xaa\xba\u03a9\u017e\xf8\xbf\xa1\xac\u221a\u0192\u2248\u0106\xab\u010c\u2026\xa0\xc0\xc3\xd5\u0152\u0153\u0110\u2014\u201c\u201d\u2018\u2019\xf7\u25ca\uf8ff\xa9\u2044\u20ac\u2039\u203a\xc6\xbb\u2013\xb7\u201a\u201e\u2030\xc2\u0107\xc1\u010d\xc8\xcd\xce\xcf\xcc\xd3\xd4\u0111\xd2\xda\xdb\xd9\u0131\u02c6\u02dc\xaf\u03c0\xcb\u02da\xb8\xca\xe6\u02c7",
                        "x-mac-cyrillic": "\u0410\u0411\u0412\u0413\u0414\u0415\u0416\u0417\u0418\u0419\u041a\u041b\u041c\u041d\u041e\u041f\u0420\u0421\u0422\u0423\u0424\u0425\u0426\u0427\u0428\u0429\u042a\u042b\u042c\u042d\u042e\u042f\u2020\xb0\u0490\xa3\xa7\u2022\xb6\u0406\xae\xa9\u2122\u0402\u0452\u2260\u0403\u0453\u221e\xb1\u2264\u2265\u0456\xb5\u0491\u0408\u0404\u0454\u0407\u0457\u0409\u0459\u040a\u045a\u0458\u0405\xac\u221a\u0192\u2248\u2206\xab\xbb\u2026\xa0\u040b\u045b\u040c\u045c\u0455\u2013\u2014\u201c\u201d\u2018\u2019\xf7\u201e\u040e\u045e\u040f\u045f\u2116\u0401\u0451\u044f\u0430\u0431\u0432\u0433\u0434\u0435\u0436\u0437\u0438\u0439\u043a\u043b\u043c\u043d\u043e\u043f\u0440\u0441\u0442\u0443\u0444\u0445\u0446\u0447\u0448\u0449\u044a\u044b\u044c\u044d\u044e",
                        "x-mac-gaelic": "\xc4\xc5\xc7\xc9\xd1\xd6\xdc\xe1\xe0\xe2\xe4\xe3\xe5\xe7\xe9\xe8\xea\xeb\xed\xec\xee\xef\xf1\xf3\xf2\xf4\xf6\xf5\xfa\xf9\xfb\xfc\u2020\xb0\xa2\xa3\xa7\u2022\xb6\xdf\xae\xa9\u2122\xb4\xa8\u2260\xc6\xd8\u1e02\xb1\u2264\u2265\u1e03\u010a\u010b\u1e0a\u1e0b\u1e1e\u1e1f\u0120\u0121\u1e40\xe6\xf8\u1e41\u1e56\u1e57\u027c\u0192\u017f\u1e60\xab\xbb\u2026\xa0\xc0\xc3\xd5\u0152\u0153\u2013\u2014\u201c\u201d\u2018\u2019\u1e61\u1e9b\xff\u0178\u1e6a\u20ac\u2039\u203a\u0176\u0177\u1e6b\xb7\u1ef2\u1ef3\u204a\xc2\xca\xc1\xcb\xc8\xcd\xce\xcf\xcc\xd3\xd4\u2663\xd2\xda\xdb\xd9\u0131\xdd\xfd\u0174\u0175\u1e84\u1e85\u1e80\u1e81\u1e82\u1e83",
                        "x-mac-greek": "\xc4\xb9\xb2\xc9\xb3\xd6\xdc\u0385\xe0\xe2\xe4\u0384\xa8\xe7\xe9\xe8\xea\xeb\xa3\u2122\xee\xef\u2022\xbd\u2030\xf4\xf6\xa6\u20ac\xf9\xfb\xfc\u2020\u0393\u0394\u0398\u039b\u039e\u03a0\xdf\xae\xa9\u03a3\u03aa\xa7\u2260\xb0\xb7\u0391\xb1\u2264\u2265\xa5\u0392\u0395\u0396\u0397\u0399\u039a\u039c\u03a6\u03ab\u03a8\u03a9\u03ac\u039d\xac\u039f\u03a1\u2248\u03a4\xab\xbb\u2026\xa0\u03a5\u03a7\u0386\u0388\u0153\u2013\u2015\u201c\u201d\u2018\u2019\xf7\u0389\u038a\u038c\u038e\u03ad\u03ae\u03af\u03cc\u038f\u03cd\u03b1\u03b2\u03c8\u03b4\u03b5\u03c6\u03b3\u03b7\u03b9\u03be\u03ba\u03bb\u03bc\u03bd\u03bf\u03c0\u03ce\u03c1\u03c3\u03c4\u03b8\u03c9\u03c2\u03c7\u03c5\u03b6\u03ca\u03cb\u0390\u03b0\xad",
                        "x-mac-icelandic": "\xc4\xc5\xc7\xc9\xd1\xd6\xdc\xe1\xe0\xe2\xe4\xe3\xe5\xe7\xe9\xe8\xea\xeb\xed\xec\xee\xef\xf1\xf3\xf2\xf4\xf6\xf5\xfa\xf9\xfb\xfc\xdd\xb0\xa2\xa3\xa7\u2022\xb6\xdf\xae\xa9\u2122\xb4\xa8\u2260\xc6\xd8\u221e\xb1\u2264\u2265\xa5\xb5\u2202\u2211\u220f\u03c0\u222b\xaa\xba\u03a9\xe6\xf8\xbf\xa1\xac\u221a\u0192\u2248\u2206\xab\xbb\u2026\xa0\xc0\xc3\xd5\u0152\u0153\u2013\u2014\u201c\u201d\u2018\u2019\xf7\u25ca\xff\u0178\u2044\u20ac\xd0\xf0\xde\xfe\xfd\xb7\u201a\u201e\u2030\xc2\xca\xc1\xcb\xc8\xcd\xce\xcf\xcc\xd3\xd4\uf8ff\xd2\xda\xdb\xd9\u0131\u02c6\u02dc\xaf\u02d8\u02d9\u02da\xb8\u02dd\u02db\u02c7",
                        "x-mac-inuit": "\u1403\u1404\u1405\u1406\u140a\u140b\u1431\u1432\u1433\u1434\u1438\u1439\u1449\u144e\u144f\u1450\u1451\u1455\u1456\u1466\u146d\u146e\u146f\u1470\u1472\u1473\u1483\u148b\u148c\u148d\u148e\u1490\u1491\xb0\u14a1\u14a5\u14a6\u2022\xb6\u14a7\xae\xa9\u2122\u14a8\u14aa\u14ab\u14bb\u14c2\u14c3\u14c4\u14c5\u14c7\u14c8\u14d0\u14ef\u14f0\u14f1\u14f2\u14f4\u14f5\u1505\u14d5\u14d6\u14d7\u14d8\u14da\u14db\u14ea\u1528\u1529\u152a\u152b\u152d\u2026\xa0\u152e\u153e\u1555\u1556\u1557\u2013\u2014\u201c\u201d\u2018\u2019\u1558\u1559\u155a\u155d\u1546\u1547\u1548\u1549\u154b\u154c\u1550\u157f\u1580\u1581\u1582\u1583\u1584\u1585\u158f\u1590\u1591\u1592\u1593\u1594\u1595\u1671\u1672\u1673\u1674\u1675\u1676\u1596\u15a0\u15a1\u15a2\u15a3\u15a4\u15a5\u15a6\u157c\u0141\u0142",
                        "x-mac-ce": "\xc4\u0100\u0101\xc9\u0104\xd6\xdc\xe1\u0105\u010c\xe4\u010d\u0106\u0107\xe9\u0179\u017a\u010e\xed\u010f\u0112\u0113\u0116\xf3\u0117\xf4\xf6\xf5\xfa\u011a\u011b\xfc\u2020\xb0\u0118\xa3\xa7\u2022\xb6\xdf\xae\xa9\u2122\u0119\xa8\u2260\u0123\u012e\u012f\u012a\u2264\u2265\u012b\u0136\u2202\u2211\u0142\u013b\u013c\u013d\u013e\u0139\u013a\u0145\u0146\u0143\xac\u221a\u0144\u0147\u2206\xab\xbb\u2026\xa0\u0148\u0150\xd5\u0151\u014c\u2013\u2014\u201c\u201d\u2018\u2019\xf7\u25ca\u014d\u0154\u0155\u0158\u2039\u203a\u0159\u0156\u0157\u0160\u201a\u201e\u0161\u015a\u015b\xc1\u0164\u0165\xcd\u017d\u017e\u016a\xd3\xd4\u016b\u016e\xda\u016f\u0170\u0171\u0172\u0173\xdd\xfd\u0137\u017b\u0141\u017c\u0122\u02c7",
                        macintosh: "\xc4\xc5\xc7\xc9\xd1\xd6\xdc\xe1\xe0\xe2\xe4\xe3\xe5\xe7\xe9\xe8\xea\xeb\xed\xec\xee\xef\xf1\xf3\xf2\xf4\xf6\xf5\xfa\xf9\xfb\xfc\u2020\xb0\xa2\xa3\xa7\u2022\xb6\xdf\xae\xa9\u2122\xb4\xa8\u2260\xc6\xd8\u221e\xb1\u2264\u2265\xa5\xb5\u2202\u2211\u220f\u03c0\u222b\xaa\xba\u03a9\xe6\xf8\xbf\xa1\xac\u221a\u0192\u2248\u2206\xab\xbb\u2026\xa0\xc0\xc3\xd5\u0152\u0153\u2013\u2014\u201c\u201d\u2018\u2019\xf7\u25ca\xff\u0178\u2044\u20ac\u2039\u203a\ufb01\ufb02\u2021\xb7\u201a\u201e\u2030\xc2\xca\xc1\xcb\xc8\xcd\xce\xcf\xcc\xd3\xd4\uf8ff\xd2\xda\xdb\xd9\u0131\u02c6\u02dc\xaf\u02d8\u02d9\u02da\xb8\u02dd\u02db\u02c7",
                        "x-mac-romanian": "\xc4\xc5\xc7\xc9\xd1\xd6\xdc\xe1\xe0\xe2\xe4\xe3\xe5\xe7\xe9\xe8\xea\xeb\xed\xec\xee\xef\xf1\xf3\xf2\xf4\xf6\xf5\xfa\xf9\xfb\xfc\u2020\xb0\xa2\xa3\xa7\u2022\xb6\xdf\xae\xa9\u2122\xb4\xa8\u2260\u0102\u0218\u221e\xb1\u2264\u2265\xa5\xb5\u2202\u2211\u220f\u03c0\u222b\xaa\xba\u03a9\u0103\u0219\xbf\xa1\xac\u221a\u0192\u2248\u2206\xab\xbb\u2026\xa0\xc0\xc3\xd5\u0152\u0153\u2013\u2014\u201c\u201d\u2018\u2019\xf7\u25ca\xff\u0178\u2044\u20ac\u2039\u203a\u021a\u021b\u2021\xb7\u201a\u201e\u2030\xc2\xca\xc1\xcb\xc8\xcd\xce\xcf\xcc\xd3\xd4\uf8ff\xd2\xda\xdb\xd9\u0131\u02c6\u02dc\xaf\u02d8\u02d9\u02da\xb8\u02dd\u02db\u02c7",
                        "x-mac-turkish": "\xc4\xc5\xc7\xc9\xd1\xd6\xdc\xe1\xe0\xe2\xe4\xe3\xe5\xe7\xe9\xe8\xea\xeb\xed\xec\xee\xef\xf1\xf3\xf2\xf4\xf6\xf5\xfa\xf9\xfb\xfc\u2020\xb0\xa2\xa3\xa7\u2022\xb6\xdf\xae\xa9\u2122\xb4\xa8\u2260\xc6\xd8\u221e\xb1\u2264\u2265\xa5\xb5\u2202\u2211\u220f\u03c0\u222b\xaa\xba\u03a9\xe6\xf8\xbf\xa1\xac\u221a\u0192\u2248\u2206\xab\xbb\u2026\xa0\xc0\xc3\xd5\u0152\u0153\u2013\u2014\u201c\u201d\u2018\u2019\xf7\u25ca\xff\u0178\u011e\u011f\u0130\u0131\u015e\u015f\u2021\xb7\u201a\u201e\u2030\xc2\xca\xc1\xcb\xc8\xcd\xce\xcf\xcc\xd3\xd4\uf8ff\xd2\xda\xdb\xd9\uf8a0\u02c6\u02dc\xaf\u02d8\u02d9\u02da\xb8\u02dd\u02db\u02c7"
                    };
                    y.MACSTRING = function(e, t, r, n) {
                        var h = S[n];
                        if (void 0 !== h) {
                            for (var a = "", i = 0; i < r; i++) {
                                var c = e.getUint8(t + i);
                                a += c <= 127 ? String.fromCharCode(c) : h[127 & c]
                            }
                            return a
                        }
                    };
                    var Y, B = "function" === typeof WeakMap && new WeakMap;

                    function T(e) {
                        return e >= -128 && e <= 127
                    }

                    function L(e, t, r) {
                        var n = 0,
                            h = e.length;
                        while (t < h && n < 64 && 0 === e[t]) ++t, ++n;
                        return r.push(128 | n - 1), t
                    }

                    function M(e, t, r) {
                        var n = 0,
                            h = e.length,
                            a = t;
                        while (a < h && n < 64) {
                            var i = e[a];
                            if (!T(i)) break;
                            if (0 === i && a + 1 < h && 0 === e[a + 1]) break;
                            ++a, ++n
                        }
                        r.push(n - 1);
                        for (var c = t; c < a; ++c) r.push(e[c] + 256 & 255);
                        return a
                    }

                    function q(e, t, r) {
                        var n = 0,
                            h = e.length,
                            a = t;
                        while (a < h && n < 64) {
                            var i = e[a];
                            if (0 === i) break;
                            if (T(i) && a + 1 < h && T(e[a + 1])) break;
                            ++a, ++n
                        }
                        r.push(64 | n - 1);
                        for (var c = t; c < a; ++c) {
                            var o = e[c];
                            r.push(o + 65536 >> 8 & 255, o + 256 & 255)
                        }
                        return a
                    }
                    C.MACSTRING = function(e, t) {
                        var r = function(e) {
                            if (!Y)
                                for (var t in Y = {}, S) Y[t] = new String(t);
                            var r = Y[e];
                            if (void 0 !== r) {
                                if (B) {
                                    var n = B.get(r);
                                    if (void 0 !== n) return n
                                }
                                var h = S[e];
                                if (void 0 !== h) {
                                    for (var a = {}, i = 0; i < h.length; i++) a[h.charCodeAt(i)] = i + 128;
                                    return B && B.set(r, a), a
                                }
                            }
                        }(t);
                        if (void 0 !== r) {
                            for (var n = [], h = 0; h < e.length; h++) {
                                var a = e.charCodeAt(h);
                                if (a >= 128 && (a = r[a], void 0 === a)) return;
                                n[h] = a
                            }
                            return n
                        }
                    }, A.MACSTRING = function(e, t) {
                        var r = C.MACSTRING(e, t);
                        return void 0 !== r ? r.length : 0
                    }, C.VARDELTAS = function(e) {
                        var t = 0,
                            r = [];
                        while (t < e.length) {
                            var n = e[t];
                            t = 0 === n ? L(e, t, r) : n >= -128 && n <= 127 ? M(e, t, r) : q(e, t, r)
                        }
                        return r
                    }, C.INDEX = function(e) {
                        for (var t = 1, r = [t], n = [], h = 0; h < e.length; h += 1) {
                            var a = C.OBJECT(e[h]);
                            Array.prototype.push.apply(n, a), t += a.length, r.push(t)
                        }
                        if (0 === n.length) return [0, 0];
                        for (var i = [], c = 1 + Math.floor(Math.log(t) / Math.log(2)) / 8 | 0, o = [void 0, C.BYTE, C.USHORT, C.UINT24, C.ULONG][c], s = 0; s < r.length; s += 1) {
                            var l = o(r[s]);
                            Array.prototype.push.apply(i, l)
                        }
                        return Array.prototype.concat(C.Card16(e.length), C.OffSize(c), i, n)
                    }, A.INDEX = function(e) {
                        return C.INDEX(e).length
                    }, C.DICT = function(e) {
                        for (var t = [], r = Object.keys(e), n = r.length, h = 0; h < n; h += 1) {
                            var a = parseInt(r[h], 0),
                                i = e[a];
                            t = t.concat(C.OPERAND(i.value, i.type)), t = t.concat(C.OPERATOR(a))
                        }
                        return t
                    }, A.DICT = function(e) {
                        return C.DICT(e).length
                    }, C.OPERATOR = function(e) {
                        return e < 1200 ? [e] : [12, e - 1200]
                    }, C.OPERAND = function(e, t) {
                        var r = [];
                        if (Array.isArray(t))
                            for (var n = 0; n < t.length; n += 1) w.argument(e.length === t.length, "Not enough arguments given for type" + t), r = r.concat(C.OPERAND(e[n], t[n]));
                        else if ("SID" === t) r = r.concat(C.NUMBER(e));
                        else if ("offset" === t) r = r.concat(C.NUMBER32(e));
                        else if ("number" === t) r = r.concat(C.NUMBER(e));
                        else {
                            if ("real" !== t) throw new Error("Unknown operand type " + t);
                            r = r.concat(C.REAL(e))
                        }
                        return r
                    }, C.OP = C.BYTE, A.OP = A.BYTE;
                    var G = "function" === typeof WeakMap && new WeakMap;

                    function E(e, t, r) {
                        if (t.length && ("coverageFormat" !== t[0].name || 1 === t[0].value))
                            for (var n = 0; n < t.length; n += 1) {
                                var h = t[n];
                                this[h.name] = h.value
                            }
                        if (this.tableName = e, this.fields = t, r)
                            for (var a = Object.keys(r), i = 0; i < a.length; i += 1) {
                                var c = a[i],
                                    o = r[c];
                                void 0 !== this[c] && (this[c] = o)
                            }
                    }

                    function W(e, t, r) {
                        void 0 === r && (r = t.length);
                        var n = new Array(t.length + 1);
                        n[0] = {
                            name: e + "Count",
                            type: "USHORT",
                            value: r
                        };
                        for (var h = 0; h < t.length; h++) n[h + 1] = {
                            name: e + h,
                            type: "USHORT",
                            value: t[h]
                        };
                        return n
                    }

                    function _(e, t, r) {
                        var n = t.length,
                            h = new Array(n + 1);
                        h[0] = {
                            name: e + "Count",
                            type: "USHORT",
                            value: n
                        };
                        for (var a = 0; a < n; a++) h[a + 1] = {
                            name: e + a,
                            type: "TABLE",
                            value: r(t[a], a)
                        };
                        return h
                    }

                    function J(e, t, r) {
                        var n = t.length,
                            h = [];
                        h[0] = {
                            name: e + "Count",
                            type: "USHORT",
                            value: n
                        };
                        for (var a = 0; a < n; a++) h = h.concat(r(t[a], a));
                        return h
                    }

                    function U(e) {
                        1 === e.format ? E.call(this, "coverageTable", [{
                            name: "coverageFormat",
                            type: "USHORT",
                            value: 1
                        }].concat(W("glyph", e.glyphs))) : 2 === e.format ? E.call(this, "coverageTable", [{
                            name: "coverageFormat",
                            type: "USHORT",
                            value: 2
                        }].concat(J("rangeRecord", e.ranges, (function(e) {
                            return [{
                                name: "startGlyphID",
                                type: "USHORT",
                                value: e.start
                            }, {
                                name: "endGlyphID",
                                type: "USHORT",
                                value: e.end
                            }, {
                                name: "startCoverageIndex",
                                type: "USHORT",
                                value: e.index
                            }]
                        })))) : w.assert(!1, "Coverage format must be 1 or 2.")
                    }

                    function O(e) {
                        E.call(this, "scriptListTable", J("scriptRecord", e, (function(e, t) {
                            var r = e.script,
                                n = r.defaultLangSys;
                            return w.assert(!!n, "Unable to write GSUB: script " + e.tag + " has no default language system."), [{
                                name: "scriptTag" + t,
                                type: "TAG",
                                value: e.tag
                            }, {
                                name: "script" + t,
                                type: "TABLE",
                                value: new E("scriptTable", [{
                                    name: "defaultLangSys",
                                    type: "TABLE",
                                    value: new E("defaultLangSys", [{
                                        name: "lookupOrder",
                                        type: "USHORT",
                                        value: 0
                                    }, {
                                        name: "reqFeatureIndex",
                                        type: "USHORT",
                                        value: n.reqFeatureIndex
                                    }].concat(W("featureIndex", n.featureIndexes)))
                                }].concat(J("langSys", r.langSysRecords, (function(e, t) {
                                    var r = e.langSys;
                                    return [{
                                        name: "langSysTag" + t,
                                        type: "TAG",
                                        value: e.tag
                                    }, {
                                        name: "langSys" + t,
                                        type: "TABLE",
                                        value: new E("langSys", [{
                                            name: "lookupOrder",
                                            type: "USHORT",
                                            value: 0
                                        }, {
                                            name: "reqFeatureIndex",
                                            type: "USHORT",
                                            value: r.reqFeatureIndex
                                        }].concat(W("featureIndex", r.featureIndexes)))
                                    }]
                                }))))
                            }]
                        })))
                    }

                    function Z(e) {
                        E.call(this, "featureListTable", J("featureRecord", e, (function(e, t) {
                            var r = e.feature;
                            return [{
                                name: "featureTag" + t,
                                type: "TAG",
                                value: e.tag
                            }, {
                                name: "feature" + t,
                                type: "TABLE",
                                value: new E("featureTable", [{
                                    name: "featureParams",
                                    type: "USHORT",
                                    value: r.featureParams
                                }].concat(W("lookupListIndex", r.lookupListIndexes)))
                            }]
                        })))
                    }

                    function K(e, t) {
                        E.call(this, "lookupListTable", _("lookup", e, (function(e) {
                            var r = t[e.lookupType];
                            return w.assert(!!r, "Unable to write GSUB lookup type " + e.lookupType + " tables."), new E("lookupTable", [{
                                name: "lookupType",
                                type: "USHORT",
                                value: e.lookupType
                            }, {
                                name: "lookupFlag",
                                type: "USHORT",
                                value: e.lookupFlag
                            }].concat(_("subtable", e.subtables, r)))
                        })))
                    }
                    C.CHARSTRING = function(e) {
                        if (G) {
                            var t = G.get(e);
                            if (void 0 !== t) return t
                        }
                        for (var r = [], n = e.length, h = 0; h < n; h += 1) {
                            var a = e[h];
                            r = r.concat(C[a.type](a.value))
                        }
                        return G && G.set(e, r), r
                    }, A.CHARSTRING = function(e) {
                        return C.CHARSTRING(e).length
                    }, C.OBJECT = function(e) {
                        var t = C[e.type];
                        return w.argument(void 0 !== t, "No encoding function for type " + e.type), t(e.value)
                    }, A.OBJECT = function(e) {
                        var t = A[e.type];
                        return w.argument(void 0 !== t, "No sizeOf function for type " + e.type), t(e.value)
                    }, C.TABLE = function(e) {
                        for (var t = [], r = e.fields.length, n = [], h = [], a = 0; a < r; a += 1) {
                            var i = e.fields[a],
                                c = C[i.type];
                            w.argument(void 0 !== c, "No encoding function for field type " + i.type + " (" + i.name + ")");
                            var o = e[i.name];
                            void 0 === o && (o = i.value);
                            var s = c(o);
                            "TABLE" === i.type ? (h.push(t.length), t = t.concat([0, 0]), n.push(s)) : t = t.concat(s)
                        }
                        for (var l = 0; l < n.length; l += 1) {
                            var p = h[l],
                                d = t.length;
                            w.argument(d < 65536, "Table " + e.tableName + " too big."), t[p] = d >> 8, t[p + 1] = 255 & d, t = t.concat(n[l])
                        }
                        return t
                    }, A.TABLE = function(e) {
                        for (var t = 0, r = e.fields.length, n = 0; n < r; n += 1) {
                            var h = e.fields[n],
                                a = A[h.type];
                            w.argument(void 0 !== a, "No sizeOf function for field type " + h.type + " (" + h.name + ")");
                            var i = e[h.name];
                            void 0 === i && (i = h.value), t += a(i), "TABLE" === h.type && (t += 2)
                        }
                        return t
                    }, C.RECORD = C.TABLE, A.RECORD = A.TABLE, C.LITERAL = function(e) {
                        return e
                    }, A.LITERAL = function(e) {
                        return e.length
                    }, E.prototype.encode = function() {
                        return C.TABLE(this)
                    }, E.prototype.sizeOf = function() {
                        return A.TABLE(this)
                    }, U.prototype = Object.create(E.prototype), U.prototype.constructor = U, O.prototype = Object.create(E.prototype), O.prototype.constructor = O, Z.prototype = Object.create(E.prototype), Z.prototype.constructor = Z, K.prototype = Object.create(E.prototype), K.prototype.constructor = K;
                    var $ = {
                        Table: E,
                        Record: E,
                        Coverage: U,
                        ScriptList: O,
                        FeatureList: Z,
                        LookupList: K,
                        ushortList: W,
                        tableList: _,
                        recordList: J
                    };

                    function ee(e, t) {
                        return e.getUint8(t)
                    }

                    function te(e, t) {
                        return e.getUint16(t, !1)
                    }

                    function re(e, t) {
                        return e.getUint32(t, !1)
                    }

                    function ne(e, t) {
                        var r = e.getInt16(t, !1),
                            n = e.getUint16(t + 2, !1);
                        return r + n / 65535
                    }
                    var he = {
                        byte: 1,
                        uShort: 2,
                        short: 2,
                        uLong: 4,
                        fixed: 4,
                        longDateTime: 8,
                        tag: 4
                    };

                    function ae(e, t) {
                        this.data = e, this.offset = t, this.relativeOffset = 0
                    }
                    ae.prototype.parseByte = function() {
                        var e = this.data.getUint8(this.offset + this.relativeOffset);
                        return this.relativeOffset += 1, e
                    }, ae.prototype.parseChar = function() {
                        var e = this.data.getInt8(this.offset + this.relativeOffset);
                        return this.relativeOffset += 1, e
                    }, ae.prototype.parseCard8 = ae.prototype.parseByte, ae.prototype.parseUShort = function() {
                        var e = this.data.getUint16(this.offset + this.relativeOffset);
                        return this.relativeOffset += 2, e
                    }, ae.prototype.parseCard16 = ae.prototype.parseUShort, ae.prototype.parseSID = ae.prototype.parseUShort, ae.prototype.parseOffset16 = ae.prototype.parseUShort, ae.prototype.parseShort = function() {
                        var e = this.data.getInt16(this.offset + this.relativeOffset);
                        return this.relativeOffset += 2, e
                    }, ae.prototype.parseF2Dot14 = function() {
                        var e = this.data.getInt16(this.offset + this.relativeOffset) / 16384;
                        return this.relativeOffset += 2, e
                    }, ae.prototype.parseULong = function() {
                        var e = re(this.data, this.offset + this.relativeOffset);
                        return this.relativeOffset += 4, e
                    }, ae.prototype.parseOffset32 = ae.prototype.parseULong, ae.prototype.parseFixed = function() {
                        var e = ne(this.data, this.offset + this.relativeOffset);
                        return this.relativeOffset += 4, e
                    }, ae.prototype.parseString = function(e) {
                        var t = this.data,
                            r = this.offset + this.relativeOffset,
                            n = "";
                        this.relativeOffset += e;
                        for (var h = 0; h < e; h++) n += String.fromCharCode(t.getUint8(r + h));
                        return n
                    }, ae.prototype.parseTag = function() {
                        return this.parseString(4)
                    }, ae.prototype.parseLongDateTime = function() {
                        var e = re(this.data, this.offset + this.relativeOffset + 4);
                        return e -= 2082844800, this.relativeOffset += 8, e
                    }, ae.prototype.parseVersion = function(e) {
                        var t = te(this.data, this.offset + this.relativeOffset),
                            r = te(this.data, this.offset + this.relativeOffset + 2);
                        return this.relativeOffset += 4, void 0 === e && (e = 4096), t + r / e / 10
                    }, ae.prototype.skip = function(e, t) {
                        void 0 === t && (t = 1), this.relativeOffset += he[e] * t
                    }, ae.prototype.parseULongList = function(e) {
                        void 0 === e && (e = this.parseULong());
                        for (var t = new Array(e), r = this.data, n = this.offset + this.relativeOffset, h = 0; h < e; h++) t[h] = r.getUint32(n), n += 4;
                        return this.relativeOffset += 4 * e, t
                    }, ae.prototype.parseOffset16List = ae.prototype.parseUShortList = function(e) {
                        void 0 === e && (e = this.parseUShort());
                        for (var t = new Array(e), r = this.data, n = this.offset + this.relativeOffset, h = 0; h < e; h++) t[h] = r.getUint16(n), n += 2;
                        return this.relativeOffset += 2 * e, t
                    }, ae.prototype.parseShortList = function(e) {
                        for (var t = new Array(e), r = this.data, n = this.offset + this.relativeOffset, h = 0; h < e; h++) t[h] = r.getInt16(n), n += 2;
                        return this.relativeOffset += 2 * e, t
                    }, ae.prototype.parseByteList = function(e) {
                        for (var t = new Array(e), r = this.data, n = this.offset + this.relativeOffset, h = 0; h < e; h++) t[h] = r.getUint8(n++);
                        return this.relativeOffset += e, t
                    }, ae.prototype.parseList = function(e, t) {
                        t || (t = e, e = this.parseUShort());
                        for (var r = new Array(e), n = 0; n < e; n++) r[n] = t.call(this);
                        return r
                    }, ae.prototype.parseList32 = function(e, t) {
                        t || (t = e, e = this.parseULong());
                        for (var r = new Array(e), n = 0; n < e; n++) r[n] = t.call(this);
                        return r
                    }, ae.prototype.parseRecordList = function(e, t) {
                        t || (t = e, e = this.parseUShort());
                        for (var r = new Array(e), n = Object.keys(t), h = 0; h < e; h++) {
                            for (var a = {}, i = 0; i < n.length; i++) {
                                var c = n[i],
                                    o = t[c];
                                a[c] = o.call(this)
                            }
                            r[h] = a
                        }
                        return r
                    }, ae.prototype.parseRecordList32 = function(e, t) {
                        t || (t = e, e = this.parseULong());
                        for (var r = new Array(e), n = Object.keys(t), h = 0; h < e; h++) {
                            for (var a = {}, i = 0; i < n.length; i++) {
                                var c = n[i],
                                    o = t[c];
                                a[c] = o.call(this)
                            }
                            r[h] = a
                        }
                        return r
                    }, ae.prototype.parseStruct = function(e) {
                        if ("function" === typeof e) return e.call(this);
                        for (var t = Object.keys(e), r = {}, n = 0; n < t.length; n++) {
                            var h = t[n],
                                a = e[h];
                            r[h] = a.call(this)
                        }
                        return r
                    }, ae.prototype.parseValueRecord = function(e) {
                        if (void 0 === e && (e = this.parseUShort()), 0 !== e) {
                            var t = {};
                            return 1 & e && (t.xPlacement = this.parseShort()), 2 & e && (t.yPlacement = this.parseShort()), 4 & e && (t.xAdvance = this.parseShort()), 8 & e && (t.yAdvance = this.parseShort()), 16 & e && (t.xPlaDevice = void 0, this.parseShort()), 32 & e && (t.yPlaDevice = void 0, this.parseShort()), 64 & e && (t.xAdvDevice = void 0, this.parseShort()), 128 & e && (t.yAdvDevice = void 0, this.parseShort()), t
                        }
                    }, ae.prototype.parseValueRecordList = function() {
                        for (var e = this.parseUShort(), t = this.parseUShort(), r = new Array(t), n = 0; n < t; n++) r[n] = this.parseValueRecord(e);
                        return r
                    }, ae.prototype.parsePointer = function(e) {
                        var t = this.parseOffset16();
                        if (t > 0) return new ae(this.data, this.offset + t).parseStruct(e)
                    }, ae.prototype.parsePointer32 = function(e) {
                        var t = this.parseOffset32();
                        if (t > 0) return new ae(this.data, this.offset + t).parseStruct(e)
                    }, ae.prototype.parseListOfLists = function(e) {
                        for (var t = this.parseOffset16List(), r = t.length, n = this.relativeOffset, h = new Array(r), a = 0; a < r; a++) {
                            var i = t[a];
                            if (0 !== i)
                                if (this.relativeOffset = i, e) {
                                    for (var c = this.parseOffset16List(), o = new Array(c.length), s = 0; s < c.length; s++) this.relativeOffset = i + c[s], o[s] = e.call(this);
                                    h[a] = o
                                } else h[a] = this.parseUShortList();
                            else h[a] = void 0
                        }
                        return this.relativeOffset = n, h
                    }, ae.prototype.parseCoverage = function() {
                        var e = this.offset + this.relativeOffset,
                            t = this.parseUShort(),
                            r = this.parseUShort();
                        if (1 === t) return {
                            format: 1,
                            glyphs: this.parseUShortList(r)
                        };
                        if (2 === t) {
                            for (var n = new Array(r), h = 0; h < r; h++) n[h] = {
                                start: this.parseUShort(),
                                end: this.parseUShort(),
                                index: this.parseUShort()
                            };
                            return {
                                format: 2,
                                ranges: n
                            }
                        }
                        throw new Error("0x" + e.toString(16) + ": Coverage format must be 1 or 2.")
                    }, ae.prototype.parseClassDef = function() {
                        var e = this.offset + this.relativeOffset,
                            t = this.parseUShort();
                        if (1 === t) return {
                            format: 1,
                            startGlyph: this.parseUShort(),
                            classes: this.parseUShortList()
                        };
                        if (2 === t) return {
                            format: 2,
                            ranges: this.parseRecordList({
                                start: ae.uShort,
                                end: ae.uShort,
                                classId: ae.uShort
                            })
                        };
                        throw new Error("0x" + e.toString(16) + ": ClassDef format must be 1 or 2.")
                    }, ae.list = function(e, t) {
                        return function() {
                            return this.parseList(e, t)
                        }
                    }, ae.list32 = function(e, t) {
                        return function() {
                            return this.parseList32(e, t)
                        }
                    }, ae.recordList = function(e, t) {
                        return function() {
                            return this.parseRecordList(e, t)
                        }
                    }, ae.recordList32 = function(e, t) {
                        return function() {
                            return this.parseRecordList32(e, t)
                        }
                    }, ae.pointer = function(e) {
                        return function() {
                            return this.parsePointer(e)
                        }
                    }, ae.pointer32 = function(e) {
                        return function() {
                            return this.parsePointer32(e)
                        }
                    }, ae.tag = ae.prototype.parseTag, ae.byte = ae.prototype.parseByte, ae.uShort = ae.offset16 = ae.prototype.parseUShort, ae.uShortList = ae.prototype.parseUShortList, ae.uLong = ae.offset32 = ae.prototype.parseULong, ae.uLongList = ae.prototype.parseULongList, ae.struct = ae.prototype.parseStruct, ae.coverage = ae.prototype.parseCoverage, ae.classDef = ae.prototype.parseClassDef;
                    var ie = {
                        reserved: ae.uShort,
                        reqFeatureIndex: ae.uShort,
                        featureIndexes: ae.uShortList
                    };
                    ae.prototype.parseScriptList = function() {
                        return this.parsePointer(ae.recordList({
                            tag: ae.tag,
                            script: ae.pointer({
                                defaultLangSys: ae.pointer(ie),
                                langSysRecords: ae.recordList({
                                    tag: ae.tag,
                                    langSys: ae.pointer(ie)
                                })
                            })
                        })) || []
                    }, ae.prototype.parseFeatureList = function() {
                        return this.parsePointer(ae.recordList({
                            tag: ae.tag,
                            feature: ae.pointer({
                                featureParams: ae.offset16,
                                lookupListIndexes: ae.uShortList
                            })
                        })) || []
                    }, ae.prototype.parseLookupList = function(e) {
                        return this.parsePointer(ae.list(ae.pointer((function() {
                            var t = this.parseUShort();
                            w.argument(1 <= t && t <= 9, "GPOS/GSUB lookup type " + t + " unknown.");
                            var r = this.parseUShort(),
                                n = 16 & r;
                            return {
                                lookupType: t,
                                lookupFlag: r,
                                subtables: this.parseList(ae.pointer(e[t])),
                                markFilteringSet: n ? this.parseUShort() : void 0
                            }
                        })))) || []
                    }, ae.prototype.parseFeatureVariationsList = function() {
                        return this.parsePointer32((function() {
                            var e = this.parseUShort(),
                                t = this.parseUShort();
                            w.argument(1 === e && t < 1, "GPOS/GSUB feature variations table unknown.");
                            var r = this.parseRecordList32({
                                conditionSetOffset: ae.offset32,
                                featureTableSubstitutionOffset: ae.offset32
                            });
                            return r
                        })) || []
                    };
                    var ce = {
                        getByte: ee,
                        getCard8: ee,
                        getUShort: te,
                        getCard16: te,
                        getShort: function(e, t) {
                            return e.getInt16(t, !1)
                        },
                        getULong: re,
                        getFixed: ne,
                        getTag: function(e, t) {
                            for (var r = "", n = t; n < t + 4; n += 1) r += String.fromCharCode(e.getInt8(n));
                            return r
                        },
                        getOffset: function(e, t, r) {
                            for (var n = 0, h = 0; h < r; h += 1) n <<= 8, n += e.getUint8(t + h);
                            return n
                        },
                        getBytes: function(e, t, r) {
                            for (var n = [], h = t; h < r; h += 1) n.push(e.getUint8(h));
                            return n
                        },
                        bytesToString: function(e) {
                            for (var t = "", r = 0; r < e.length; r += 1) t += String.fromCharCode(e[r]);
                            return t
                        },
                        Parser: ae
                    };

                    function oe(e, t, r) {
                        e.segments.push({
                            end: t,
                            start: t,
                            delta: -(t - r),
                            offset: 0,
                            glyphIndex: r
                        })
                    }
                    var se = {
                            parse: function(e, t) {
                                var r = {};
                                r.version = ce.getUShort(e, t), w.argument(0 === r.version, "cmap table version should be 0."), r.numTables = ce.getUShort(e, t + 2);
                                for (var n = -1, h = r.numTables - 1; h >= 0; h -= 1) {
                                    var a = ce.getUShort(e, t + 4 + 8 * h),
                                        i = ce.getUShort(e, t + 4 + 8 * h + 2);
                                    if (3 === a && (0 === i || 1 === i || 10 === i) || 0 === a && (0 === i || 1 === i || 2 === i || 3 === i || 4 === i)) {
                                        n = ce.getULong(e, t + 4 + 8 * h + 4);
                                        break
                                    }
                                }
                                if (-1 === n) throw new Error("No valid cmap sub-tables found.");
                                var c = new ce.Parser(e, t + n);
                                if (r.format = c.parseUShort(), 12 === r.format)(function(e, t) {
                                    var r;
                                    t.parseUShort(), e.length = t.parseULong(), e.language = t.parseULong(), e.groupCount = r = t.parseULong(), e.glyphIndexMap = {};
                                    for (var n = 0; n < r; n += 1)
                                        for (var h = t.parseULong(), a = t.parseULong(), i = t.parseULong(), c = h; c <= a; c += 1) e.glyphIndexMap[c] = i, i++
                                })(r, c);
                                else {
                                    if (4 !== r.format) throw new Error("Only format 4 and 12 cmap tables are supported (found format " + r.format + ").");
                                    (function(e, t, r, n, h) {
                                        var a;
                                        e.length = t.parseUShort(), e.language = t.parseUShort(), e.segCount = a = t.parseUShort() >> 1, t.skip("uShort", 3), e.glyphIndexMap = {};
                                        for (var i = new ce.Parser(r, n + h + 14), c = new ce.Parser(r, n + h + 16 + 2 * a), o = new ce.Parser(r, n + h + 16 + 4 * a), s = new ce.Parser(r, n + h + 16 + 6 * a), l = n + h + 16 + 8 * a, p = 0; p < a - 1; p += 1)
                                            for (var d = void 0, b = i.parseUShort(), g = c.parseUShort(), j = o.parseShort(), x = s.parseUShort(), V = g; V <= b; V += 1) 0 !== x ? (l = s.offset + s.relativeOffset - 2, l += x, l += 2 * (V - g), d = ce.getUShort(r, l), 0 !== d && (d = d + j & 65535)) : d = V + j & 65535, e.glyphIndexMap[V] = d
                                    })(r, c, e, t, n)
                                }
                                return r
                            },
                            make: function(e) {
                                var t, r = !0;
                                for (t = e.length - 1; t > 0; t -= 1) {
                                    var n = e.get(t);
                                    if (n.unicode > 65535) {
                                        h("log", "Adding CMAP format 12 (needed!)", " at utils/opentype.js:2884"), r = !1;
                                        break
                                    }
                                }
                                var a = [{
                                    name: "version",
                                    type: "USHORT",
                                    value: 0
                                }, {
                                    name: "numTables",
                                    type: "USHORT",
                                    value: r ? 1 : 2
                                }, {
                                    name: "platformID",
                                    type: "USHORT",
                                    value: 3
                                }, {
                                    name: "encodingID",
                                    type: "USHORT",
                                    value: 1
                                }, {
                                    name: "offset",
                                    type: "ULONG",
                                    value: r ? 12 : 20
                                }];
                                r || (a = a.concat([{
                                    name: "cmap12PlatformID",
                                    type: "USHORT",
                                    value: 3
                                }, {
                                    name: "cmap12EncodingID",
                                    type: "USHORT",
                                    value: 10
                                }, {
                                    name: "cmap12Offset",
                                    type: "ULONG",
                                    value: 0
                                }])), a = a.concat([{
                                    name: "format",
                                    type: "USHORT",
                                    value: 4
                                }, {
                                    name: "cmap4Length",
                                    type: "USHORT",
                                    value: 0
                                }, {
                                    name: "language",
                                    type: "USHORT",
                                    value: 0
                                }, {
                                    name: "segCountX2",
                                    type: "USHORT",
                                    value: 0
                                }, {
                                    name: "searchRange",
                                    type: "USHORT",
                                    value: 0
                                }, {
                                    name: "entrySelector",
                                    type: "USHORT",
                                    value: 0
                                }, {
                                    name: "rangeShift",
                                    type: "USHORT",
                                    value: 0
                                }]);
                                var i = new $.Table("cmap", a);
                                for (i.segments = [], t = 0; t < e.length; t += 1) {
                                    for (var c = e.get(t), o = 0; o < c.unicodes.length; o += 1) oe(i, c.unicodes[o], t);
                                    i.segments = i.segments.sort((function(e, t) {
                                        return e.start - t.start
                                    }))
                                }(function(e) {
                                    e.segments.push({
                                        end: 65535,
                                        start: 65535,
                                        delta: 1,
                                        offset: 0
                                    })
                                })(i);
                                var s = i.segments.length,
                                    l = 0,
                                    p = [],
                                    d = [],
                                    b = [],
                                    g = [],
                                    j = [],
                                    x = [];
                                for (t = 0; t < s; t += 1) {
                                    var V = i.segments[t];
                                    V.end <= 65535 && V.start <= 65535 ? (p = p.concat({
                                        name: "end_" + t,
                                        type: "USHORT",
                                        value: V.end
                                    }), d = d.concat({
                                        name: "start_" + t,
                                        type: "USHORT",
                                        value: V.start
                                    }), b = b.concat({
                                        name: "idDelta_" + t,
                                        type: "SHORT",
                                        value: V.delta
                                    }), g = g.concat({
                                        name: "idRangeOffset_" + t,
                                        type: "USHORT",
                                        value: V.offset
                                    }), void 0 !== V.glyphId && (j = j.concat({
                                        name: "glyph_" + t,
                                        type: "USHORT",
                                        value: V.glyphId
                                    }))) : l += 1, r || void 0 === V.glyphIndex || (x = x.concat({
                                        name: "cmap12Start_" + t,
                                        type: "ULONG",
                                        value: V.start
                                    }), x = x.concat({
                                        name: "cmap12End_" + t,
                                        type: "ULONG",
                                        value: V.end
                                    }), x = x.concat({
                                        name: "cmap12Glyph_" + t,
                                        type: "ULONG",
                                        value: V.glyphIndex
                                    }))
                                }
                                if (i.segCountX2 = 2 * (s - l), i.searchRange = 2 * Math.pow(2, Math.floor(Math.log(s - l) / Math.log(2))), i.entrySelector = Math.log(i.searchRange / 2) / Math.log(2), i.rangeShift = i.segCountX2 - i.searchRange, i.fields = i.fields.concat(p), i.fields.push({
                                        name: "reservedPad",
                                        type: "USHORT",
                                        value: 0
                                    }), i.fields = i.fields.concat(d), i.fields = i.fields.concat(b), i.fields = i.fields.concat(g), i.fields = i.fields.concat(j), i.cmap4Length = 14 + 2 * p.length + 2 + 2 * d.length + 2 * b.length + 2 * g.length + 2 * j.length, !r) {
                                    var f = 16 + 4 * x.length;
                                    i.cmap12Offset = 20 + i.cmap4Length, i.fields = i.fields.concat([{
                                        name: "cmap12Format",
                                        type: "USHORT",
                                        value: 12
                                    }, {
                                        name: "cmap12Reserved",
                                        type: "USHORT",
                                        value: 0
                                    }, {
                                        name: "cmap12Length",
                                        type: "ULONG",
                                        value: f
                                    }, {
                                        name: "cmap12Language",
                                        type: "ULONG",
                                        value: 0
                                    }, {
                                        name: "cmap12nGroups",
                                        type: "ULONG",
                                        value: x.length / 3
                                    }]), i.fields = i.fields.concat(x)
                                }
                                return i
                            }
                        },
                        le = [".notdef", "space", "exclam", "quotedbl", "numbersign", "dollar", "percent", "ampersand", "quoteright", "parenleft", "parenright", "asterisk", "plus", "comma", "hyphen", "period", "slash", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "colon", "semicolon", "less", "equal", "greater", "question", "at", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "bracketleft", "backslash", "bracketright", "asciicircum", "underscore", "quoteleft", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "braceleft", "bar", "braceright", "asciitilde", "exclamdown", "cent", "sterling", "fraction", "yen", "florin", "section", "currency", "quotesingle", "quotedblleft", "guillemotleft", "guilsinglleft", "guilsinglright", "fi", "fl", "endash", "dagger", "daggerdbl", "periodcentered", "paragraph", "bullet", "quotesinglbase", "quotedblbase", "quotedblright", "guillemotright", "ellipsis", "perthousand", "questiondown", "grave", "acute", "circumflex", "tilde", "macron", "breve", "dotaccent", "dieresis", "ring", "cedilla", "hungarumlaut", "ogonek", "caron", "emdash", "AE", "ordfeminine", "Lslash", "Oslash", "OE", "ordmasculine", "ae", "dotlessi", "lslash", "oslash", "oe", "germandbls", "onesuperior", "logicalnot", "mu", "trademark", "Eth", "onehalf", "plusminus", "Thorn", "onequarter", "divide", "brokenbar", "degree", "thorn", "threequarters", "twosuperior", "registered", "minus", "eth", "multiply", "threesuperior", "copyright", "Aacute", "Acircumflex", "Adieresis", "Agrave", "Aring", "Atilde", "Ccedilla", "Eacute", "Ecircumflex", "Edieresis", "Egrave", "Iacute", "Icircumflex", "Idieresis", "Igrave", "Ntilde", "Oacute", "Ocircumflex", "Odieresis", "Ograve", "Otilde", "Scaron", "Uacute", "Ucircumflex", "Udieresis", "Ugrave", "Yacute", "Ydieresis", "Zcaron", "aacute", "acircumflex", "adieresis", "agrave", "aring", "atilde", "ccedilla", "eacute", "ecircumflex", "edieresis", "egrave", "iacute", "icircumflex", "idieresis", "igrave", "ntilde", "oacute", "ocircumflex", "odieresis", "ograve", "otilde", "scaron", "uacute", "ucircumflex", "udieresis", "ugrave", "yacute", "ydieresis", "zcaron", "exclamsmall", "Hungarumlautsmall", "dollaroldstyle", "dollarsuperior", "ampersandsmall", "Acutesmall", "parenleftsuperior", "parenrightsuperior", "266 ff", "onedotenleader", "zerooldstyle", "oneoldstyle", "twooldstyle", "threeoldstyle", "fouroldstyle", "fiveoldstyle", "sixoldstyle", "sevenoldstyle", "eightoldstyle", "nineoldstyle", "commasuperior", "threequartersemdash", "periodsuperior", "questionsmall", "asuperior", "bsuperior", "centsuperior", "dsuperior", "esuperior", "isuperior", "lsuperior", "msuperior", "nsuperior", "osuperior", "rsuperior", "ssuperior", "tsuperior", "ff", "ffi", "ffl", "parenleftinferior", "parenrightinferior", "Circumflexsmall", "hyphensuperior", "Gravesmall", "Asmall", "Bsmall", "Csmall", "Dsmall", "Esmall", "Fsmall", "Gsmall", "Hsmall", "Ismall", "Jsmall", "Ksmall", "Lsmall", "Msmall", "Nsmall", "Osmall", "Psmall", "Qsmall", "Rsmall", "Ssmall", "Tsmall", "Usmall", "Vsmall", "Wsmall", "Xsmall", "Ysmall", "Zsmall", "colonmonetary", "onefitted", "rupiah", "Tildesmall", "exclamdownsmall", "centoldstyle", "Lslashsmall", "Scaronsmall", "Zcaronsmall", "Dieresissmall", "Brevesmall", "Caronsmall", "Dotaccentsmall", "Macronsmall", "figuredash", "hypheninferior", "Ogoneksmall", "Ringsmall", "Cedillasmall", "questiondownsmall", "oneeighth", "threeeighths", "fiveeighths", "seveneighths", "onethird", "twothirds", "zerosuperior", "foursuperior", "fivesuperior", "sixsuperior", "sevensuperior", "eightsuperior", "ninesuperior", "zeroinferior", "oneinferior", "twoinferior", "threeinferior", "fourinferior", "fiveinferior", "sixinferior", "seveninferior", "eightinferior", "nineinferior", "centinferior", "dollarinferior", "periodinferior", "commainferior", "Agravesmall", "Aacutesmall", "Acircumflexsmall", "Atildesmall", "Adieresissmall", "Aringsmall", "AEsmall", "Ccedillasmall", "Egravesmall", "Eacutesmall", "Ecircumflexsmall", "Edieresissmall", "Igravesmall", "Iacutesmall", "Icircumflexsmall", "Idieresissmall", "Ethsmall", "Ntildesmall", "Ogravesmall", "Oacutesmall", "Ocircumflexsmall", "Otildesmall", "Odieresissmall", "OEsmall", "Oslashsmall", "Ugravesmall", "Uacutesmall", "Ucircumflexsmall", "Udieresissmall", "Yacutesmall", "Thornsmall", "Ydieresissmall", "001.000", "001.001", "001.002", "001.003", "Black", "Bold", "Book", "Light", "Medium", "Regular", "Roman", "Semibold"],
                        pe = ["", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "space", "exclam", "quotedbl", "numbersign", "dollar", "percent", "ampersand", "quoteright", "parenleft", "parenright", "asterisk", "plus", "comma", "hyphen", "period", "slash", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "colon", "semicolon", "less", "equal", "greater", "question", "at", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "bracketleft", "backslash", "bracketright", "asciicircum", "underscore", "quoteleft", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "braceleft", "bar", "braceright", "asciitilde", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "exclamdown", "cent", "sterling", "fraction", "yen", "florin", "section", "currency", "quotesingle", "quotedblleft", "guillemotleft", "guilsinglleft", "guilsinglright", "fi", "fl", "", "endash", "dagger", "daggerdbl", "periodcentered", "", "paragraph", "bullet", "quotesinglbase", "quotedblbase", "quotedblright", "guillemotright", "ellipsis", "perthousand", "", "questiondown", "", "grave", "acute", "circumflex", "tilde", "macron", "breve", "dotaccent", "dieresis", "", "ring", "cedilla", "", "hungarumlaut", "ogonek", "caron", "emdash", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "AE", "", "ordfeminine", "", "", "", "", "Lslash", "Oslash", "OE", "ordmasculine", "", "", "", "", "", "ae", "", "", "", "dotlessi", "", "", "lslash", "oslash", "oe", "germandbls"],
                        de = ["", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "space", "exclamsmall", "Hungarumlautsmall", "", "dollaroldstyle", "dollarsuperior", "ampersandsmall", "Acutesmall", "parenleftsuperior", "parenrightsuperior", "twodotenleader", "onedotenleader", "comma", "hyphen", "period", "fraction", "zerooldstyle", "oneoldstyle", "twooldstyle", "threeoldstyle", "fouroldstyle", "fiveoldstyle", "sixoldstyle", "sevenoldstyle", "eightoldstyle", "nineoldstyle", "colon", "semicolon", "commasuperior", "threequartersemdash", "periodsuperior", "questionsmall", "", "asuperior", "bsuperior", "centsuperior", "dsuperior", "esuperior", "", "", "isuperior", "", "", "lsuperior", "msuperior", "nsuperior", "osuperior", "", "", "rsuperior", "ssuperior", "tsuperior", "", "ff", "fi", "fl", "ffi", "ffl", "parenleftinferior", "", "parenrightinferior", "Circumflexsmall", "hyphensuperior", "Gravesmall", "Asmall", "Bsmall", "Csmall", "Dsmall", "Esmall", "Fsmall", "Gsmall", "Hsmall", "Ismall", "Jsmall", "Ksmall", "Lsmall", "Msmall", "Nsmall", "Osmall", "Psmall", "Qsmall", "Rsmall", "Ssmall", "Tsmall", "Usmall", "Vsmall", "Wsmall", "Xsmall", "Ysmall", "Zsmall", "colonmonetary", "onefitted", "rupiah", "Tildesmall", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "exclamdownsmall", "centoldstyle", "Lslashsmall", "", "", "Scaronsmall", "Zcaronsmall", "Dieresissmall", "Brevesmall", "Caronsmall", "", "Dotaccentsmall", "", "", "Macronsmall", "", "", "figuredash", "hypheninferior", "", "", "Ogoneksmall", "Ringsmall", "Cedillasmall", "", "", "", "onequarter", "onehalf", "threequarters", "questiondownsmall", "oneeighth", "threeeighths", "fiveeighths", "seveneighths", "onethird", "twothirds", "", "", "zerosuperior", "onesuperior", "twosuperior", "threesuperior", "foursuperior", "fivesuperior", "sixsuperior", "sevensuperior", "eightsuperior", "ninesuperior", "zeroinferior", "oneinferior", "twoinferior", "threeinferior", "fourinferior", "fiveinferior", "sixinferior", "seveninferior", "eightinferior", "nineinferior", "centinferior", "dollarinferior", "periodinferior", "commainferior", "Agravesmall", "Aacutesmall", "Acircumflexsmall", "Atildesmall", "Adieresissmall", "Aringsmall", "AEsmall", "Ccedillasmall", "Egravesmall", "Eacutesmall", "Ecircumflexsmall", "Edieresissmall", "Igravesmall", "Iacutesmall", "Icircumflexsmall", "Idieresissmall", "Ethsmall", "Ntildesmall", "Ogravesmall", "Oacutesmall", "Ocircumflexsmall", "Otildesmall", "Odieresissmall", "OEsmall", "Oslashsmall", "Ugravesmall", "Uacutesmall", "Ucircumflexsmall", "Udieresissmall", "Yacutesmall", "Thornsmall", "Ydieresissmall"],
                        be = [".notdef", ".null", "nonmarkingreturn", "space", "exclam", "quotedbl", "numbersign", "dollar", "percent", "ampersand", "quotesingle", "parenleft", "parenright", "asterisk", "plus", "comma", "hyphen", "period", "slash", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "colon", "semicolon", "less", "equal", "greater", "question", "at", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "bracketleft", "backslash", "bracketright", "asciicircum", "underscore", "grave", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "braceleft", "bar", "braceright", "asciitilde", "Adieresis", "Aring", "Ccedilla", "Eacute", "Ntilde", "Odieresis", "Udieresis", "aacute", "agrave", "acircumflex", "adieresis", "atilde", "aring", "ccedilla", "eacute", "egrave", "ecircumflex", "edieresis", "iacute", "igrave", "icircumflex", "idieresis", "ntilde", "oacute", "ograve", "ocircumflex", "odieresis", "otilde", "uacute", "ugrave", "ucircumflex", "udieresis", "dagger", "degree", "cent", "sterling", "section", "bullet", "paragraph", "germandbls", "registered", "copyright", "trademark", "acute", "dieresis", "notequal", "AE", "Oslash", "infinity", "plusminus", "lessequal", "greaterequal", "yen", "mu", "partialdiff", "summation", "product", "pi", "integral", "ordfeminine", "ordmasculine", "Omega", "ae", "oslash", "questiondown", "exclamdown", "logicalnot", "radical", "florin", "approxequal", "Delta", "guillemotleft", "guillemotright", "ellipsis", "nonbreakingspace", "Agrave", "Atilde", "Otilde", "OE", "oe", "endash", "emdash", "quotedblleft", "quotedblright", "quoteleft", "quoteright", "divide", "lozenge", "ydieresis", "Ydieresis", "fraction", "currency", "guilsinglleft", "guilsinglright", "fi", "fl", "daggerdbl", "periodcentered", "quotesinglbase", "quotedblbase", "perthousand", "Acircumflex", "Ecircumflex", "Aacute", "Edieresis", "Egrave", "Iacute", "Icircumflex", "Idieresis", "Igrave", "Oacute", "Ocircumflex", "apple", "Ograve", "Uacute", "Ucircumflex", "Ugrave", "dotlessi", "circumflex", "tilde", "macron", "breve", "dotaccent", "ring", "cedilla", "hungarumlaut", "ogonek", "caron", "Lslash", "lslash", "Scaron", "scaron", "Zcaron", "zcaron", "brokenbar", "Eth", "eth", "Yacute", "yacute", "Thorn", "thorn", "minus", "multiply", "onesuperior", "twosuperior", "threesuperior", "onehalf", "onequarter", "threequarters", "franc", "Gbreve", "gbreve", "Idotaccent", "Scedilla", "scedilla", "Cacute", "cacute", "Ccaron", "ccaron", "dcroat"];

                    function ge(e) {
                        this.font = e
                    }

                    function je(e) {
                        this.cmap = e
                    }

                    function xe(e, t) {
                        this.encoding = e, this.charset = t
                    }

                    function Ve(e) {
                        switch (e.version) {
                            case 1:
                                this.names = be.slice();
                                break;
                            case 2:
                                this.names = new Array(e.numberOfGlyphs);
                                for (var t = 0; t < e.numberOfGlyphs; t++) e.glyphNameIndex[t] < be.length ? this.names[t] = be[e.glyphNameIndex[t]] : this.names[t] = e.names[e.glyphNameIndex[t] - be.length];
                                break;
                            case 2.5:
                                this.names = new Array(e.numberOfGlyphs);
                                for (var r = 0; r < e.numberOfGlyphs; r++) this.names[r] = be[r + e.glyphNameIndex[r]];
                                break;
                            case 3:
                                this.names = [];
                                break;
                            default:
                                this.names = [];
                                break
                        }
                    }

                    function fe(e, t) {
                        t.lowMemory ? function(e) {
                            e._IndexToUnicodeMap = {};
                            for (var t = e.tables.cmap.glyphIndexMap, r = Object.keys(t), n = 0; n < r.length; n += 1) {
                                var h = r[n],
                                    a = t[h];
                                void 0 === e._IndexToUnicodeMap[a] ? e._IndexToUnicodeMap[a] = {
                                    unicodes: [parseInt(h)]
                                } : e._IndexToUnicodeMap[a].unicodes.push(parseInt(h))
                            }
                        }(e) : function(e) {
                            for (var t, r = e.tables.cmap.glyphIndexMap, n = Object.keys(r), h = 0; h < n.length; h += 1) {
                                var a = n[h],
                                    i = r[a];
                                t = e.glyphs.get(i), t.addUnicode(parseInt(a))
                            }
                            for (var c = 0; c < e.glyphs.length; c += 1) t = e.glyphs.get(c), e.cffEncoding ? e.isCIDFont ? t.name = "gid" + c : t.name = e.cffEncoding.charset[c] : e.glyphNames.names && (t.name = e.glyphNames.glyphIndexToName(c))
                        }(e)
                    }
                    ge.prototype.charToGlyphIndex = function(e) {
                        var t = e.codePointAt(0),
                            r = this.font.glyphs;
                        if (r)
                            for (var n = 0; n < r.length; n += 1)
                                for (var h = r.get(n), a = 0; a < h.unicodes.length; a += 1)
                                    if (h.unicodes[a] === t) return n;
                        return null
                    }, je.prototype.charToGlyphIndex = function(e) {
                        return this.cmap.glyphIndexMap[e.codePointAt(0)] || 0
                    }, xe.prototype.charToGlyphIndex = function(e) {
                        var t = e.codePointAt(0),
                            r = this.encoding[t];
                        return this.charset.indexOf(r)
                    }, Ve.prototype.nameToGlyphIndex = function(e) {
                        return this.names.indexOf(e)
                    }, Ve.prototype.glyphIndexToName = function(e) {
                        return this.names[e]
                    };
                    var Fe = {
                        line: function(e, t, r, n, h) {
                            e.beginPath(), e.moveTo(t, r), e.lineTo(n, h), e.stroke()
                        }
                    };

                    function ke(e) {
                        this.bindConstructorValues(e)
                    }

                    function me(e, t, r) {
                        Object.defineProperty(e, t, {
                            get: function() {
                                return e.path, e[r]
                            },
                            set: function(t) {
                                e[r] = t
                            },
                            enumerable: !0,
                            configurable: !0
                        })
                    }

                    function Pe(e, t) {
                        if (this.font = e, this.glyphs = {}, Array.isArray(t))
                            for (var r = 0; r < t.length; r++) {
                                var n = t[r];
                                n.path.unitsPerEm = e.unitsPerEm, this.glyphs[r] = n
                            }
                        this.length = t && t.length || 0
                    }
                    ke.prototype.bindConstructorValues = function(e) {
                        this.index = e.index || 0, this.name = e.name || null, this.unicode = e.unicode || void 0, this.unicodes = e.unicodes || void 0 !== e.unicode ? [e.unicode] : [], "xMin" in e && (this.xMin = e.xMin), "yMin" in e && (this.yMin = e.yMin), "xMax" in e && (this.xMax = e.xMax), "yMax" in e && (this.yMax = e.yMax), "advanceWidth" in e && (this.advanceWidth = e.advanceWidth), Object.defineProperty(this, "path", function(e, t) {
                            var r = t || new R;
                            return {
                                configurable: !0,
                                get: function() {
                                    return "function" === typeof r && (r = r()), r
                                },
                                set: function(e) {
                                    r = e
                                }
                            }
                        }(0, e.path))
                    }, ke.prototype.addUnicode = function(e) {
                        0 === this.unicodes.length && (this.unicode = e), this.unicodes.push(e)
                    }, ke.prototype.getBoundingBox = function() {
                        return this.path.getBoundingBox()
                    }, ke.prototype.getPath = function(e, t, r, n, h) {
                        var a, i;
                        e = void 0 !== e ? e : 0, t = void 0 !== t ? t : 0, r = void 0 !== r ? r : 72, n || (n = {});
                        var c = n.xScale,
                            o = n.yScale;
                        if (n.hinting && h && h.hinting && (i = this.path && h.hinting.exec(this, r)), i) a = h.hinting.getCommands(i), e = Math.round(e), t = Math.round(t), c = o = 1;
                        else {
                            a = this.path.commands;
                            var s = 1 / (this.path.unitsPerEm || 1e3) * r;
                            void 0 === c && (c = s), void 0 === o && (o = s)
                        }
                        for (var l = new R, p = 0; p < a.length; p += 1) {
                            var d = a[p];
                            "M" === d.type ? l.moveTo(e + d.x * c, t + -d.y * o) : "L" === d.type ? l.lineTo(e + d.x * c, t + -d.y * o) : "Q" === d.type ? l.quadraticCurveTo(e + d.x1 * c, t + -d.y1 * o, e + d.x * c, t + -d.y * o) : "C" === d.type ? l.curveTo(e + d.x1 * c, t + -d.y1 * o, e + d.x2 * c, t + -d.y2 * o, e + d.x * c, t + -d.y * o) : "Z" === d.type && l.closePath()
                        }
                        return l
                    }, ke.prototype.getContours = function() {
                        if (void 0 === this.points) return [];
                        for (var e = [], t = [], r = 0; r < this.points.length; r += 1) {
                            var n = this.points[r];
                            t.push(n), n.lastPointOfContour && (e.push(t), t = [])
                        }
                        return w.argument(0 === t.length, "There are still points left in the current contour."), e
                    }, ke.prototype.getMetrics = function() {
                        for (var e = this.path.commands, t = [], r = [], n = 0; n < e.length; n += 1) {
                            var h = e[n];
                            "Z" !== h.type && (t.push(h.x), r.push(h.y)), "Q" !== h.type && "C" !== h.type || (t.push(h.x1), r.push(h.y1)), "C" === h.type && (t.push(h.x2), r.push(h.y2))
                        }
                        var a = {
                            xMin: Math.min.apply(null, t),
                            yMin: Math.min.apply(null, r),
                            xMax: Math.max.apply(null, t),
                            yMax: Math.max.apply(null, r),
                            leftSideBearing: this.leftSideBearing
                        };
                        return isFinite(a.xMin) || (a.xMin = 0), isFinite(a.xMax) || (a.xMax = this.advanceWidth), isFinite(a.yMin) || (a.yMin = 0), isFinite(a.yMax) || (a.yMax = 0), a.rightSideBearing = this.advanceWidth - a.leftSideBearing - (a.xMax - a.xMin), a
                    }, ke.prototype.draw = function(e, t, r, n, h) {
                        this.getPath(t, r, n, h).draw(e)
                    }, ke.prototype.drawPoints = function(e, t, r, n) {
                        function h(t, r, n, h) {
                            e.beginPath();
                            for (var a = 0; a < t.length; a += 1) e.moveTo(r + t[a].x * h, n + t[a].y * h), e.arc(r + t[a].x * h, n + t[a].y * h, 2, 0, 2 * Math.PI, !1);
                            e.closePath(), e.fill()
                        }
                        t = void 0 !== t ? t : 0, r = void 0 !== r ? r : 0, n = void 0 !== n ? n : 24;
                        for (var a = 1 / this.path.unitsPerEm * n, i = [], c = [], o = this.path, s = 0; s < o.commands.length; s += 1) {
                            var l = o.commands[s];
                            void 0 !== l.x && i.push({
                                x: l.x,
                                y: -l.y
                            }), void 0 !== l.x1 && c.push({
                                x: l.x1,
                                y: -l.y1
                            }), void 0 !== l.x2 && c.push({
                                x: l.x2,
                                y: -l.y2
                            })
                        }
                        e.fillStyle = "blue", h(i, t, r, a), e.fillStyle = "red", h(c, t, r, a)
                    }, ke.prototype.drawMetrics = function(e, t, r, n) {
                        var h;
                        t = void 0 !== t ? t : 0, r = void 0 !== r ? r : 0, n = void 0 !== n ? n : 24, h = 1 / this.path.unitsPerEm * n, e.lineWidth = 1, e.strokeStyle = "black", Fe.line(e, t, -1e4, t, 1e4), Fe.line(e, -1e4, r, 1e4, r);
                        var a = this.xMin || 0,
                            i = this.yMin || 0,
                            c = this.xMax || 0,
                            o = this.yMax || 0,
                            s = this.advanceWidth || 0;
                        e.strokeStyle = "blue", Fe.line(e, t + a * h, -1e4, t + a * h, 1e4), Fe.line(e, t + c * h, -1e4, t + c * h, 1e4), Fe.line(e, -1e4, r + -i * h, 1e4, r + -i * h), Fe.line(e, -1e4, r + -o * h, 1e4, r + -o * h), e.strokeStyle = "green", Fe.line(e, t + s * h, -1e4, t + s * h, 1e4)
                    }, Pe.prototype.get = function(e) {
                        if (void 0 === this.glyphs[e]) {
                            this.font._push(e), "function" === typeof this.glyphs[e] && (this.glyphs[e] = this.glyphs[e]());
                            var t = this.glyphs[e],
                                r = this.font._IndexToUnicodeMap[e];
                            if (r)
                                for (var n = 0; n < r.unicodes.length; n++) t.addUnicode(r.unicodes[n]);
                            this.font.cffEncoding ? this.font.isCIDFont ? t.name = "gid" + e : t.name = this.font.cffEncoding.charset[e] : this.font.glyphNames.names && (t.name = this.font.glyphNames.glyphIndexToName(e)), this.glyphs[e].advanceWidth = this.font._hmtxTableData[e].advanceWidth, this.glyphs[e].leftSideBearing = this.font._hmtxTableData[e].leftSideBearing
                        } else "function" === typeof this.glyphs[e] && (this.glyphs[e] = this.glyphs[e]());
                        return this.glyphs[e]
                    }, Pe.prototype.push = function(e, t) {
                        this.glyphs[e] = t, this.length++
                    };
                    var ue = {
                        GlyphSet: Pe,
                        glyphLoader: function(e, t) {
                            return new ke({
                                index: t,
                                font: e
                            })
                        },
                        ttfGlyphLoader: function(e, t, r, n, h, a) {
                            return function() {
                                var i = new ke({
                                    index: t,
                                    font: e
                                });
                                return i.path = function() {
                                    r(i, n, h);
                                    var t = a(e.glyphs, i);
                                    return t.unitsPerEm = e.unitsPerEm, t
                                }, me(i, "xMin", "_xMin"), me(i, "xMax", "_xMax"), me(i, "yMin", "_yMin"), me(i, "yMax", "_yMax"), i
                            }
                        },
                        cffGlyphLoader: function(e, t, r, n) {
                            return function() {
                                var h = new ke({
                                    index: t,
                                    font: e
                                });
                                return h.path = function() {
                                    var t = r(e, h, n);
                                    return t.unitsPerEm = e.unitsPerEm, t
                                }, h
                            }
                        }
                    };

                    function Xe(e, t) {
                        if (e === t) return !0;
                        if (Array.isArray(e) && Array.isArray(t)) {
                            if (e.length !== t.length) return !1;
                            for (var r = 0; r < e.length; r += 1)
                                if (!Xe(e[r], t[r])) return !1;
                            return !0
                        }
                        return !1
                    }

                    function Ne(e) {
                        var t;
                        return t = e.length < 1240 ? 107 : e.length < 33900 ? 1131 : 32768, t
                    }

                    function He(e, t, r) {
                        var n, h, a = [],
                            i = [],
                            c = ce.getCard16(e, t);
                        if (0 !== c) {
                            var o = ce.getByte(e, t + 2);
                            n = t + (c + 1) * o + 2;
                            for (var s = t + 3, l = 0; l < c + 1; l += 1) a.push(ce.getOffset(e, s, o)), s += o;
                            h = n + a[c]
                        } else h = t + 2;
                        for (var p = 0; p < a.length - 1; p += 1) {
                            var d = ce.getBytes(e, n + a[p], n + a[p + 1]);
                            r && (d = r(d)), i.push(d)
                        }
                        return {
                            objects: i,
                            startOffset: t,
                            endOffset: h
                        }
                    }

                    function ze(e, t) {
                        var r, n, h, a;
                        if (28 === t) return r = e.parseByte(), n = e.parseByte(), r << 8 | n;
                        if (29 === t) return r = e.parseByte(), n = e.parseByte(), h = e.parseByte(), a = e.parseByte(), r << 24 | n << 16 | h << 8 | a;
                        if (30 === t) return function(e) {
                            var t = "",
                                r = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ".", "E", "E-", null, "-"];
                            while (1) {
                                var n = e.parseByte(),
                                    h = n >> 4,
                                    a = 15 & n;
                                if (15 === h) break;
                                if (t += r[h], 15 === a) break;
                                t += r[a]
                            }
                            return parseFloat(t)
                        }(e);
                        if (t >= 32 && t <= 246) return t - 139;
                        if (t >= 247 && t <= 250) return r = e.parseByte(), 256 * (t - 247) + r + 108;
                        if (t >= 251 && t <= 254) return r = e.parseByte(), 256 * -(t - 251) - r - 108;
                        throw new Error("Invalid b0 " + t)
                    }

                    function Qe(e, t, r) {
                        t = void 0 !== t ? t : 0;
                        var n = new ce.Parser(e, t),
                            h = [],
                            a = [];
                        r = void 0 !== r ? r : e.length;
                        while (n.relativeOffset < r) {
                            var i = n.parseByte();
                            i <= 21 ? (12 === i && (i = 1200 + n.parseByte()), h.push([i, a]), a = []) : a.push(ze(n, i))
                        }
                        return function(e) {
                            for (var t = {}, r = 0; r < e.length; r += 1) {
                                var n = e[r][0],
                                    h = e[r][1],
                                    a = void 0;
                                if (a = 1 === h.length ? h[0] : h, t.hasOwnProperty(n) && !isNaN(t[n])) throw new Error("Object " + t + " already has key " + n);
                                t[n] = a
                            }
                            return t
                        }(h)
                    }

                    function Re(e, t) {
                        return t = t <= 390 ? le[t] : e[t - 391], t
                    }

                    function ve(e, t, r) {
                        for (var n, h = {}, a = 0; a < t.length; a += 1) {
                            var i = t[a];
                            if (Array.isArray(i.type)) {
                                var c = [];
                                c.length = i.type.length;
                                for (var o = 0; o < i.type.length; o++) n = void 0 !== e[i.op] ? e[i.op][o] : void 0, void 0 === n && (n = void 0 !== i.value && void 0 !== i.value[o] ? i.value[o] : null), "SID" === i.type[o] && (n = Re(r, n)), c[o] = n;
                                h[i.name] = c
                            } else n = e[i.op], void 0 === n && (n = void 0 !== i.value ? i.value : null), "SID" === i.type && (n = Re(r, n)), h[i.name] = n
                        }
                        return h
                    }
                    var Ie = [{
                            name: "version",
                            op: 0,
                            type: "SID"
                        }, {
                            name: "notice",
                            op: 1,
                            type: "SID"
                        }, {
                            name: "copyright",
                            op: 1200,
                            type: "SID"
                        }, {
                            name: "fullName",
                            op: 2,
                            type: "SID"
                        }, {
                            name: "familyName",
                            op: 3,
                            type: "SID"
                        }, {
                            name: "weight",
                            op: 4,
                            type: "SID"
                        }, {
                            name: "isFixedPitch",
                            op: 1201,
                            type: "number",
                            value: 0
                        }, {
                            name: "italicAngle",
                            op: 1202,
                            type: "number",
                            value: 0
                        }, {
                            name: "underlinePosition",
                            op: 1203,
                            type: "number",
                            value: -100
                        }, {
                            name: "underlineThickness",
                            op: 1204,
                            type: "number",
                            value: 50
                        }, {
                            name: "paintType",
                            op: 1205,
                            type: "number",
                            value: 0
                        }, {
                            name: "charstringType",
                            op: 1206,
                            type: "number",
                            value: 2
                        }, {
                            name: "fontMatrix",
                            op: 1207,
                            type: ["real", "real", "real", "real", "real", "real"],
                            value: [.001, 0, 0, .001, 0, 0]
                        }, {
                            name: "uniqueId",
                            op: 13,
                            type: "number"
                        }, {
                            name: "fontBBox",
                            op: 5,
                            type: ["number", "number", "number", "number"],
                            value: [0, 0, 0, 0]
                        }, {
                            name: "strokeWidth",
                            op: 1208,
                            type: "number",
                            value: 0
                        }, {
                            name: "xuid",
                            op: 14,
                            type: [],
                            value: null
                        }, {
                            name: "charset",
                            op: 15,
                            type: "offset",
                            value: 0
                        }, {
                            name: "encoding",
                            op: 16,
                            type: "offset",
                            value: 0
                        }, {
                            name: "charStrings",
                            op: 17,
                            type: "offset",
                            value: 0
                        }, {
                            name: "private",
                            op: 18,
                            type: ["number", "offset"],
                            value: [0, 0]
                        }, {
                            name: "ros",
                            op: 1230,
                            type: ["SID", "SID", "number"]
                        }, {
                            name: "cidFontVersion",
                            op: 1231,
                            type: "number",
                            value: 0
                        }, {
                            name: "cidFontRevision",
                            op: 1232,
                            type: "number",
                            value: 0
                        }, {
                            name: "cidFontType",
                            op: 1233,
                            type: "number",
                            value: 0
                        }, {
                            name: "cidCount",
                            op: 1234,
                            type: "number",
                            value: 8720
                        }, {
                            name: "uidBase",
                            op: 1235,
                            type: "number"
                        }, {
                            name: "fdArray",
                            op: 1236,
                            type: "offset"
                        }, {
                            name: "fdSelect",
                            op: 1237,
                            type: "offset"
                        }, {
                            name: "fontName",
                            op: 1238,
                            type: "SID"
                        }],
                        we = [{
                            name: "subrs",
                            op: 19,
                            type: "offset",
                            value: 0
                        }, {
                            name: "defaultWidthX",
                            op: 20,
                            type: "number",
                            value: 0
                        }, {
                            name: "nominalWidthX",
                            op: 21,
                            type: "number",
                            value: 0
                        }];

                    function ye(e, t) {
                        var r = Qe(e, 0, e.byteLength);
                        return ve(r, Ie, t)
                    }

                    function Ce(e, t, r, n) {
                        var h = Qe(e, t, r);
                        return ve(h, we, n)
                    }

                    function Ae(e, t, r, n) {
                        for (var h = [], a = 0; a < r.length; a += 1) {
                            var i = new DataView(new Uint8Array(r[a]).buffer),
                                c = ye(i, n);
                            c._subrs = [], c._subrsBias = 0, c._defaultWidthX = 0, c._nominalWidthX = 0;
                            var o = c.private[0],
                                s = c.private[1];
                            if (0 !== o && 0 !== s) {
                                var l = Ce(e, s + t, o, n);
                                if (c._defaultWidthX = l.defaultWidthX, c._nominalWidthX = l.nominalWidthX, 0 !== l.subrs) {
                                    var p = s + l.subrs,
                                        d = He(e, p + t);
                                    c._subrs = d.objects, c._subrsBias = Ne(c._subrs)
                                }
                                c._privateDict = l
                            }
                            h.push(c)
                        }
                        return h
                    }

                    function De(e, t, r) {
                        var n, a, i, c, o, s, l, p, d = new R,
                            b = [],
                            g = 0,
                            j = !1,
                            x = !1,
                            V = 0,
                            f = 0;
                        if (e.isCIDFont) {
                            var F = e.tables.cff.topDict._fdSelect[t.index],
                                k = e.tables.cff.topDict._fdArray[F];
                            o = k._subrs, s = k._subrsBias, l = k._defaultWidthX, p = k._nominalWidthX
                        } else o = e.tables.cff.topDict._subrs, s = e.tables.cff.topDict._subrsBias, l = e.tables.cff.topDict._defaultWidthX, p = e.tables.cff.topDict._nominalWidthX;
                        var m = l;

                        function P(e, t) {
                            x && d.closePath(), d.moveTo(e, t), x = !0
                        }

                        function u() {
                            var e;
                            e = b.length % 2 !== 0, e && !j && (m = b.shift() + p), g += b.length >> 1, b.length = 0, j = !0
                        }
                        return function r(l) {
                            var F, k, X, N, H, z, Q, R, v, I, w, y, C = 0;
                            while (C < l.length) {
                                var A = l[C];
                                switch (C += 1, A) {
                                    case 1:
                                        u();
                                        break;
                                    case 3:
                                        u();
                                        break;
                                    case 4:
                                        b.length > 1 && !j && (m = b.shift() + p, j = !0), f += b.pop(), P(V, f);
                                        break;
                                    case 5:
                                        while (b.length > 0) V += b.shift(), f += b.shift(), d.lineTo(V, f);
                                        break;
                                    case 6:
                                        while (b.length > 0) {
                                            if (V += b.shift(), d.lineTo(V, f), 0 === b.length) break;
                                            f += b.shift(), d.lineTo(V, f)
                                        }
                                        break;
                                    case 7:
                                        while (b.length > 0) {
                                            if (f += b.shift(), d.lineTo(V, f), 0 === b.length) break;
                                            V += b.shift(), d.lineTo(V, f)
                                        }
                                        break;
                                    case 8:
                                        while (b.length > 0) n = V + b.shift(), a = f + b.shift(), i = n + b.shift(), c = a + b.shift(), V = i + b.shift(), f = c + b.shift(), d.curveTo(n, a, i, c, V, f);
                                        break;
                                    case 10:
                                        H = b.pop() + s, z = o[H], z && r(z);
                                        break;
                                    case 11:
                                        return;
                                    case 12:
                                        switch (A = l[C], C += 1, A) {
                                            case 35:
                                                n = V + b.shift(), a = f + b.shift(), i = n + b.shift(), c = a + b.shift(), Q = i + b.shift(), R = c + b.shift(), v = Q + b.shift(), I = R + b.shift(), w = v + b.shift(), y = I + b.shift(), V = w + b.shift(), f = y + b.shift(), b.shift(), d.curveTo(n, a, i, c, Q, R), d.curveTo(v, I, w, y, V, f);
                                                break;
                                            case 34:
                                                n = V + b.shift(), a = f, i = n + b.shift(), c = a + b.shift(), Q = i + b.shift(), R = c, v = Q + b.shift(), I = c, w = v + b.shift(), y = f, V = w + b.shift(), d.curveTo(n, a, i, c, Q, R), d.curveTo(v, I, w, y, V, f);
                                                break;
                                            case 36:
                                                n = V + b.shift(), a = f + b.shift(), i = n + b.shift(), c = a + b.shift(), Q = i + b.shift(), R = c, v = Q + b.shift(), I = c, w = v + b.shift(), y = I + b.shift(), V = w + b.shift(), d.curveTo(n, a, i, c, Q, R), d.curveTo(v, I, w, y, V, f);
                                                break;
                                            case 37:
                                                n = V + b.shift(), a = f + b.shift(), i = n + b.shift(), c = a + b.shift(), Q = i + b.shift(), R = c + b.shift(), v = Q + b.shift(), I = R + b.shift(), w = v + b.shift(), y = I + b.shift(), Math.abs(w - V) > Math.abs(y - f) ? V = w + b.shift() : f = y + b.shift(), d.curveTo(n, a, i, c, Q, R), d.curveTo(v, I, w, y, V, f);
                                                break;
                                            default:
                                                h("log", "Glyph " + t.index + ": unknown operator 1200" + A, " at utils/opentype.js:4546"), b.length = 0
                                        }
                                        break;
                                    case 14:
                                        b.length > 0 && !j && (m = b.shift() + p, j = !0), x && (d.closePath(), x = !1);
                                        break;
                                    case 18:
                                        u();
                                        break;
                                    case 19:
                                    case 20:
                                        u(), C += g + 7 >> 3;
                                        break;
                                    case 21:
                                        b.length > 2 && !j && (m = b.shift() + p, j = !0), f += b.pop(), V += b.pop(), P(V, f);
                                        break;
                                    case 22:
                                        b.length > 1 && !j && (m = b.shift() + p, j = !0), V += b.pop(), P(V, f);
                                        break;
                                    case 23:
                                        u();
                                        break;
                                    case 24:
                                        while (b.length > 2) n = V + b.shift(), a = f + b.shift(), i = n + b.shift(), c = a + b.shift(), V = i + b.shift(), f = c + b.shift(), d.curveTo(n, a, i, c, V, f);
                                        V += b.shift(), f += b.shift(), d.lineTo(V, f);
                                        break;
                                    case 25:
                                        while (b.length > 6) V += b.shift(), f += b.shift(), d.lineTo(V, f);
                                        n = V + b.shift(), a = f + b.shift(), i = n + b.shift(), c = a + b.shift(), V = i + b.shift(), f = c + b.shift(), d.curveTo(n, a, i, c, V, f);
                                        break;
                                    case 26:
                                        b.length % 2 && (V += b.shift());
                                        while (b.length > 0) n = V, a = f + b.shift(), i = n + b.shift(), c = a + b.shift(), V = i, f = c + b.shift(), d.curveTo(n, a, i, c, V, f);
                                        break;
                                    case 27:
                                        b.length % 2 && (f += b.shift());
                                        while (b.length > 0) n = V + b.shift(), a = f, i = n + b.shift(), c = a + b.shift(), V = i + b.shift(), f = c, d.curveTo(n, a, i, c, V, f);
                                        break;
                                    case 28:
                                        F = l[C], k = l[C + 1], b.push((F << 24 | k << 16) >> 16), C += 2;
                                        break;
                                    case 29:
                                        H = b.pop() + e.gsubrsBias, z = e.gsubrs[H], z && r(z);
                                        break;
                                    case 30:
                                        while (b.length > 0) {
                                            if (n = V, a = f + b.shift(), i = n + b.shift(), c = a + b.shift(), V = i + b.shift(), f = c + (1 === b.length ? b.shift() : 0), d.curveTo(n, a, i, c, V, f), 0 === b.length) break;
                                            n = V + b.shift(), a = f, i = n + b.shift(), c = a + b.shift(), f = c + b.shift(), V = i + (1 === b.length ? b.shift() : 0), d.curveTo(n, a, i, c, V, f)
                                        }
                                        break;
                                    case 31:
                                        while (b.length > 0) {
                                            if (n = V + b.shift(), a = f, i = n + b.shift(), c = a + b.shift(), f = c + b.shift(), V = i + (1 === b.length ? b.shift() : 0), d.curveTo(n, a, i, c, V, f), 0 === b.length) break;
                                            n = V, a = f + b.shift(), i = n + b.shift(), c = a + b.shift(), V = i + b.shift(), f = c + (1 === b.length ? b.shift() : 0), d.curveTo(n, a, i, c, V, f)
                                        }
                                        break;
                                    default:
                                        A < 32 ? h("log", "Glyph " + t.index + ": unknown operator " + A, " at utils/opentype.js:4716") : A < 247 ? b.push(A - 139) : A < 251 ? (F = l[C], C += 1, b.push(256 * (A - 247) + F + 108)) : A < 255 ? (F = l[C], C += 1, b.push(256 * -(A - 251) - F - 108)) : (F = l[C], k = l[C + 1], X = l[C + 2], N = l[C + 3], C += 4, b.push((F << 24 | k << 16 | X << 8 | N) / 65536))
                                }
                            }
                        }(r), t.advanceWidth = m, d
                    }

                    function Se(e, t) {
                        var r, n = le.indexOf(e);
                        return n >= 0 && (r = n), n = t.indexOf(e), n >= 0 ? r = n + le.length : (r = le.length + t.length, t.push(e)), r
                    }

                    function Ye(e, t, r) {
                        for (var n = {}, h = 0; h < e.length; h += 1) {
                            var a = e[h],
                                i = t[a.name];
                            void 0 === i || Xe(i, a.value) || ("SID" === a.type && (i = Se(i, r)), n[a.op] = {
                                name: a.name,
                                type: a.type,
                                value: i
                            })
                        }
                        return n
                    }

                    function Be(e, t) {
                        var r = new $.Record("Top DICT", [{
                            name: "dict",
                            type: "DICT",
                            value: {}
                        }]);
                        return r.dict = Ye(Ie, e, t), r
                    }

                    function Te(e) {
                        var t = new $.Record("Top DICT INDEX", [{
                            name: "topDicts",
                            type: "INDEX",
                            value: []
                        }]);
                        return t.topDicts = [{
                            name: "topDict_0",
                            type: "TABLE",
                            value: e
                        }], t
                    }

                    function Le(e) {
                        var t = [],
                            r = e.path;
                        t.push({
                            name: "width",
                            type: "NUMBER",
                            value: e.advanceWidth
                        });
                        for (var n = 0, h = 0, a = 0; a < r.commands.length; a += 1) {
                            var i = void 0,
                                c = void 0,
                                o = r.commands[a];
                            if ("Q" === o.type) {
                                o = {
                                    type: "C",
                                    x: o.x,
                                    y: o.y,
                                    x1: Math.round(1 / 3 * n + 2 / 3 * o.x1),
                                    y1: Math.round(1 / 3 * h + 2 / 3 * o.y1),
                                    x2: Math.round(1 / 3 * o.x + 2 / 3 * o.x1),
                                    y2: Math.round(1 / 3 * o.y + 2 / 3 * o.y1)
                                }
                            }
                            if ("M" === o.type) i = Math.round(o.x - n), c = Math.round(o.y - h), t.push({
                                name: "dx",
                                type: "NUMBER",
                                value: i
                            }), t.push({
                                name: "dy",
                                type: "NUMBER",
                                value: c
                            }), t.push({
                                name: "rmoveto",
                                type: "OP",
                                value: 21
                            }), n = Math.round(o.x), h = Math.round(o.y);
                            else if ("L" === o.type) i = Math.round(o.x - n), c = Math.round(o.y - h), t.push({
                                name: "dx",
                                type: "NUMBER",
                                value: i
                            }), t.push({
                                name: "dy",
                                type: "NUMBER",
                                value: c
                            }), t.push({
                                name: "rlineto",
                                type: "OP",
                                value: 5
                            }), n = Math.round(o.x), h = Math.round(o.y);
                            else if ("C" === o.type) {
                                var s = Math.round(o.x1 - n),
                                    l = Math.round(o.y1 - h),
                                    p = Math.round(o.x2 - o.x1),
                                    d = Math.round(o.y2 - o.y1);
                                i = Math.round(o.x - o.x2), c = Math.round(o.y - o.y2), t.push({
                                    name: "dx1",
                                    type: "NUMBER",
                                    value: s
                                }), t.push({
                                    name: "dy1",
                                    type: "NUMBER",
                                    value: l
                                }), t.push({
                                    name: "dx2",
                                    type: "NUMBER",
                                    value: p
                                }), t.push({
                                    name: "dy2",
                                    type: "NUMBER",
                                    value: d
                                }), t.push({
                                    name: "dx",
                                    type: "NUMBER",
                                    value: i
                                }), t.push({
                                    name: "dy",
                                    type: "NUMBER",
                                    value: c
                                }), t.push({
                                    name: "rrcurveto",
                                    type: "OP",
                                    value: 8
                                }), n = Math.round(o.x), h = Math.round(o.y)
                            }
                        }
                        return t.push({
                            name: "endchar",
                            type: "OP",
                            value: 14
                        }), t
                    }
                    var Me = {
                        parse: function(e, t, r, n) {
                            r.tables.cff = {};
                            var h = function(e, t) {
                                    var r = {};
                                    return r.formatMajor = ce.getCard8(e, t), r.formatMinor = ce.getCard8(e, t + 1), r.size = ce.getCard8(e, t + 2), r.offsetSize = ce.getCard8(e, t + 3), r.startOffset = t, r.endOffset = t + 4, r
                                }(e, t),
                                a = He(e, h.endOffset, ce.bytesToString),
                                i = He(e, a.endOffset),
                                c = He(e, i.endOffset, ce.bytesToString),
                                o = He(e, c.endOffset);
                            r.gsubrs = o.objects, r.gsubrsBias = Ne(r.gsubrs);
                            var s = Ae(e, t, i.objects, c.objects);
                            if (1 !== s.length) throw new Error("CFF table has too many fonts in 'FontSet' - count of fonts NameIndex.length = " + s.length);
                            var l = s[0];
                            if (r.tables.cff.topDict = l, l._privateDict && (r.defaultWidthX = l._privateDict.defaultWidthX, r.nominalWidthX = l._privateDict.nominalWidthX), void 0 !== l.ros[0] && void 0 !== l.ros[1] && (r.isCIDFont = !0), r.isCIDFont) {
                                var p = l.fdArray,
                                    d = l.fdSelect;
                                if (0 === p || 0 === d) throw new Error("Font is marked as a CID font, but FDArray and/or FDSelect information is missing");
                                p += t;
                                var b = He(e, p),
                                    g = Ae(e, t, b.objects, c.objects);
                                l._fdArray = g, d += t, l._fdSelect = function(e, t, r, n) {
                                    var h, a = [],
                                        i = new ce.Parser(e, t),
                                        c = i.parseCard8();
                                    if (0 === c)
                                        for (var o = 0; o < r; o++) {
                                            if (h = i.parseCard8(), h >= n) throw new Error("CFF table CID Font FDSelect has bad FD index value " + h + " (FD count " + n + ")");
                                            a.push(h)
                                        } else {
                                            if (3 !== c) throw new Error("CFF Table CID Font FDSelect table has unsupported format " + c);
                                            var s, l = i.parseCard16(),
                                                p = i.parseCard16();
                                            if (0 !== p) throw new Error("CFF Table CID Font FDSelect format 3 range has bad initial GID " + p);
                                            for (var d = 0; d < l; d++) {
                                                if (h = i.parseCard8(), s = i.parseCard16(), h >= n) throw new Error("CFF table CID Font FDSelect has bad FD index value " + h + " (FD count " + n + ")");
                                                if (s > r) throw new Error("CFF Table CID Font FDSelect format 3 range has bad GID " + s);
                                                for (; p < s; p++) a.push(h);
                                                p = s
                                            }
                                            if (s !== r) throw new Error("CFF Table CID Font FDSelect format 3 range has bad final GID " + s)
                                        }
                                    return a
                                }(e, d, r.numGlyphs, g.length)
                            }
                            var j, x = t + l.private[1],
                                V = Ce(e, x, l.private[0], c.objects);
                            if (r.defaultWidthX = V.defaultWidthX, r.nominalWidthX = V.nominalWidthX, 0 !== V.subrs) {
                                var f = x + V.subrs,
                                    F = He(e, f);
                                r.subrs = F.objects, r.subrsBias = Ne(r.subrs)
                            } else r.subrs = [], r.subrsBias = 0;
                            n.lowMemory ? (j = function(e, t) {
                                var r, n, h = [],
                                    a = ce.getCard16(e, t);
                                if (0 !== a) {
                                    var i = ce.getByte(e, t + 2);
                                    r = t + (a + 1) * i + 2;
                                    for (var c = t + 3, o = 0; o < a + 1; o += 1) h.push(ce.getOffset(e, c, i)), c += i;
                                    n = r + h[a]
                                } else n = t + 2;
                                return {
                                    offsets: h,
                                    startOffset: t,
                                    endOffset: n
                                }
                            }(e, t + l.charStrings), r.nGlyphs = j.offsets.length) : (j = He(e, t + l.charStrings), r.nGlyphs = j.objects.length);
                            var k = function(e, t, r, n) {
                                var h, a, i = new ce.Parser(e, t);
                                r -= 1;
                                var c = [".notdef"],
                                    o = i.parseCard8();
                                if (0 === o)
                                    for (var s = 0; s < r; s += 1) h = i.parseSID(), c.push(Re(n, h));
                                else if (1 === o)
                                    while (c.length <= r) {
                                        h = i.parseSID(), a = i.parseCard8();
                                        for (var l = 0; l <= a; l += 1) c.push(Re(n, h)), h += 1
                                    } else {
                                        if (2 !== o) throw new Error("Unknown charset format " + o);
                                        while (c.length <= r) {
                                            h = i.parseSID(), a = i.parseCard16();
                                            for (var p = 0; p <= a; p += 1) c.push(Re(n, h)), h += 1
                                        }
                                    }
                                return c
                            }(e, t + l.charset, r.nGlyphs, c.objects);
                            if (0 === l.encoding ? r.cffEncoding = new xe(pe, k) : 1 === l.encoding ? r.cffEncoding = new xe(de, k) : r.cffEncoding = function(e, t, r) {
                                    var n, h = {},
                                        a = new ce.Parser(e, t),
                                        i = a.parseCard8();
                                    if (0 === i)
                                        for (var c = a.parseCard8(), o = 0; o < c; o += 1) n = a.parseCard8(), h[n] = o;
                                    else {
                                        if (1 !== i) throw new Error("Unknown encoding format " + i);
                                        var s = a.parseCard8();
                                        n = 1;
                                        for (var l = 0; l < s; l += 1)
                                            for (var p = a.parseCard8(), d = a.parseCard8(), b = p; b <= p + d; b += 1) h[b] = n, n += 1
                                    }
                                    return new xe(h, r)
                                }(e, t + l.encoding, k), r.encoding = r.encoding || r.cffEncoding, r.glyphs = new ue.GlyphSet(r), n.lowMemory) r._push = function(n) {
                                var h = function(e, t, r, n, h) {
                                    var a = ce.getCard16(r, n),
                                        i = 0;
                                    if (0 !== a) {
                                        var c = ce.getByte(r, n + 2);
                                        i = n + (a + 1) * c + 2
                                    }
                                    var o = ce.getBytes(r, i + t[e], i + t[e + 1]);
                                    return h && (o = h(o)), o
                                }(n, j.offsets, e, t + l.charStrings);
                                r.glyphs.push(n, ue.cffGlyphLoader(r, n, De, h))
                            };
                            else
                                for (var m = 0; m < r.nGlyphs; m += 1) {
                                    var P = j.objects[m];
                                    r.glyphs.push(m, ue.cffGlyphLoader(r, m, De, P))
                                }
                        },
                        make: function(e, t) {
                            for (var r, n = new $.Table("CFF ", [{
                                    name: "header",
                                    type: "RECORD"
                                }, {
                                    name: "nameIndex",
                                    type: "RECORD"
                                }, {
                                    name: "topDictIndex",
                                    type: "RECORD"
                                }, {
                                    name: "stringIndex",
                                    type: "RECORD"
                                }, {
                                    name: "globalSubrIndex",
                                    type: "RECORD"
                                }, {
                                    name: "charsets",
                                    type: "RECORD"
                                }, {
                                    name: "charStringsIndex",
                                    type: "RECORD"
                                }, {
                                    name: "privateDict",
                                    type: "RECORD"
                                }]), h = 1 / t.unitsPerEm, a = {
                                    version: t.version,
                                    fullName: t.fullName,
                                    familyName: t.familyName,
                                    weight: t.weightName,
                                    fontBBox: t.fontBBox || [0, 0, 0, 0],
                                    fontMatrix: [h, 0, 0, h, 0, 0],
                                    charset: 999,
                                    encoding: 0,
                                    charStrings: 999,
                                    private: [0, 999]
                                }, i = [], c = 1; c < e.length; c += 1) r = e.get(c), i.push(r.name);
                            var o = [];
                            n.header = function() {
                                return new $.Record("Header", [{
                                    name: "major",
                                    type: "Card8",
                                    value: 1
                                }, {
                                    name: "minor",
                                    type: "Card8",
                                    value: 0
                                }, {
                                    name: "hdrSize",
                                    type: "Card8",
                                    value: 4
                                }, {
                                    name: "major",
                                    type: "Card8",
                                    value: 1
                                }])
                            }(), n.nameIndex = function(e) {
                                var t = new $.Record("Name INDEX", [{
                                    name: "names",
                                    type: "INDEX",
                                    value: []
                                }]);
                                t.names = [];
                                for (var r = 0; r < e.length; r += 1) t.names.push({
                                    name: "name_" + r,
                                    type: "NAME",
                                    value: e[r]
                                });
                                return t
                            }([t.postScriptName]);
                            var s = Be(a, o);
                            n.topDictIndex = Te(s), n.globalSubrIndex = function() {
                                return new $.Record("Global Subr INDEX", [{
                                    name: "subrs",
                                    type: "INDEX",
                                    value: []
                                }])
                            }(), n.charsets = function(e, t) {
                                for (var r = new $.Record("Charsets", [{
                                        name: "format",
                                        type: "Card8",
                                        value: 0
                                    }]), n = 0; n < e.length; n += 1) {
                                    var h = e[n],
                                        a = Se(h, t);
                                    r.fields.push({
                                        name: "glyph_" + n,
                                        type: "SID",
                                        value: a
                                    })
                                }
                                return r
                            }(i, o), n.charStringsIndex = function(e) {
                                for (var t = new $.Record("CharStrings INDEX", [{
                                        name: "charStrings",
                                        type: "INDEX",
                                        value: []
                                    }]), r = 0; r < e.length; r += 1) {
                                    var n = e.get(r),
                                        h = Le(n);
                                    t.charStrings.push({
                                        name: n.name,
                                        type: "CHARSTRING",
                                        value: h
                                    })
                                }
                                return t
                            }(e), n.privateDict = function(e, t) {
                                var r = new $.Record("Private DICT", [{
                                    name: "dict",
                                    type: "DICT",
                                    value: {}
                                }]);
                                return r.dict = Ye(we, e, t), r
                            }({}, o), n.stringIndex = function(e) {
                                var t = new $.Record("String INDEX", [{
                                    name: "strings",
                                    type: "INDEX",
                                    value: []
                                }]);
                                t.strings = [];
                                for (var r = 0; r < e.length; r += 1) t.strings.push({
                                    name: "string_" + r,
                                    type: "STRING",
                                    value: e[r]
                                });
                                return t
                            }(o);
                            var l = n.header.sizeOf() + n.nameIndex.sizeOf() + n.topDictIndex.sizeOf() + n.stringIndex.sizeOf() + n.globalSubrIndex.sizeOf();
                            return a.charset = l, a.encoding = 0, a.charStrings = a.charset + n.charsets.sizeOf(), a.private[1] = a.charStrings + n.charStringsIndex.sizeOf(), s = Be(a, o), n.topDictIndex = Te(s), n
                        }
                    };
                    var qe = {
                        parse: function(e, t) {
                            var r = {},
                                n = new ce.Parser(e, t);
                            return r.version = n.parseVersion(), r.fontRevision = Math.round(1e3 * n.parseFixed()) / 1e3, r.checkSumAdjustment = n.parseULong(), r.magicNumber = n.parseULong(), w.argument(159483uniAnimatedViewRender === r.magicNumber, "Font header has wrong magic number."), r.flags = n.parseUShort(), r.unitsPerEm = n.parseUShort(), r.created = n.parseLongDateTime(), r.modified = n.parseLongDateTime(), r.xMin = n.parseShort(), r.yMin = n.parseShort(), r.xMax = n.parseShort(), r.yMax = n.parseShort(), r.macStyle = n.parseUShort(), r.lowestRecPPEM = n.parseUShort(), r.fontDirectionHint = n.parseShort(), r.indexToLocFormat = n.parseShort(), r.glyphDataFormat = n.parseShort(), r
                        },
                        make: function(e) {
                            var t = Math.round((new Date).getTime() / 1e3) + 2082844800,
                                r = t;
                            return e.createdTimestamp && (r = e.createdTimestamp + 2082844800), new $.Table("head", [{
                                name: "version",
                                type: "FIXED",
                                value: 65536
                            }, {
                                name: "fontRevision",
                                type: "FIXED",
                                value: 65536
                            }, {
                                name: "checkSumAdjustment",
                                type: "ULONG",
                                value: 0
                            }, {
                                name: "magicNumber",
                                type: "ULONG",
                                value: 159483uniAnimatedViewRender
                            }, {
                                name: "flags",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "unitsPerEm",
                                type: "USHORT",
                                value: 1e3
                            }, {
                                name: "created",
                                type: "LONGDATETIME",
                                value: r
                            }, {
                                name: "modified",
                                type: "LONGDATETIME",
                                value: t
                            }, {
                                name: "xMin",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "yMin",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "xMax",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "yMax",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "macStyle",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "lowestRecPPEM",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "fontDirectionHint",
                                type: "SHORT",
                                value: 2
                            }, {
                                name: "indexToLocFormat",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "glyphDataFormat",
                                type: "SHORT",
                                value: 0
                            }], e)
                        }
                    };
                    var Ge = {
                        parse: function(e, t) {
                            var r = {},
                                n = new ce.Parser(e, t);
                            return r.version = n.parseVersion(), r.ascender = n.parseShort(), r.descender = n.parseShort(), r.lineGap = n.parseShort(), r.advanceWidthMax = n.parseUShort(), r.minLeftSideBearing = n.parseShort(), r.minRightSideBearing = n.parseShort(), r.xMaxExtent = n.parseShort(), r.caretSlopeRise = n.parseShort(), r.caretSlopeRun = n.parseShort(), r.caretOffset = n.parseShort(), n.relativeOffset += 8, r.metricDataFormat = n.parseShort(), r.numberOfHMetrics = n.parseUShort(), r
                        },
                        make: function(e) {
                            return new $.Table("hhea", [{
                                name: "version",
                                type: "FIXED",
                                value: 65536
                            }, {
                                name: "ascender",
                                type: "FWORD",
                                value: 0
                            }, {
                                name: "descender",
                                type: "FWORD",
                                value: 0
                            }, {
                                name: "lineGap",
                                type: "FWORD",
                                value: 0
                            }, {
                                name: "advanceWidthMax",
                                type: "UFWORD",
                                value: 0
                            }, {
                                name: "minLeftSideBearing",
                                type: "FWORD",
                                value: 0
                            }, {
                                name: "minRightSideBearing",
                                type: "FWORD",
                                value: 0
                            }, {
                                name: "xMaxExtent",
                                type: "FWORD",
                                value: 0
                            }, {
                                name: "caretSlopeRise",
                                type: "SHORT",
                                value: 1
                            }, {
                                name: "caretSlopeRun",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "caretOffset",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "reserved1",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "reserved2",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "reserved3",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "reserved4",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "metricDataFormat",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "numberOfHMetrics",
                                type: "USHORT",
                                value: 0
                            }], e)
                        }
                    };
                    var Ee = {
                        parse: function(e, t, r, n, h, a, i) {
                            i.lowMemory ? function(e, t, r, n, h) {
                                var a, i;
                                e._hmtxTableData = {};
                                for (var c = new ce.Parser(t, r), o = 0; o < h; o += 1) o < n && (a = c.parseUShort(), i = c.parseShort()), e._hmtxTableData[o] = {
                                    advanceWidth: a,
                                    leftSideBearing: i
                                }
                            }(e, t, r, n, h) : function(e, t, r, n, h) {
                                for (var a, i, c = new ce.Parser(e, t), o = 0; o < n; o += 1) {
                                    o < r && (a = c.parseUShort(), i = c.parseShort());
                                    var s = h.get(o);
                                    s.advanceWidth = a, s.leftSideBearing = i
                                }
                            }(t, r, n, h, a)
                        },
                        make: function(e) {
                            for (var t = new $.Table("hmtx", []), r = 0; r < e.length; r += 1) {
                                var n = e.get(r),
                                    h = n.advanceWidth || 0,
                                    a = n.leftSideBearing || 0;
                                t.fields.push({
                                    name: "advanceWidth_" + r,
                                    type: "USHORT",
                                    value: h
                                }), t.fields.push({
                                    name: "leftSideBearing_" + r,
                                    type: "SHORT",
                                    value: a
                                })
                            }
                            return t
                        }
                    };
                    var We = {
                        make: function(e) {
                            for (var t = new $.Table("ltag", [{
                                    name: "version",
                                    type: "ULONG",
                                    value: 1
                                }, {
                                    name: "flags",
                                    type: "ULONG",
                                    value: 0
                                }, {
                                    name: "numTags",
                                    type: "ULONG",
                                    value: e.length
                                }]), r = "", n = 12 + 4 * e.length, h = 0; h < e.length; ++h) {
                                var a = r.indexOf(e[h]);
                                a < 0 && (a = r.length, r += e[h]), t.fields.push({
                                    name: "offset " + h,
                                    type: "USHORT",
                                    value: n + a
                                }), t.fields.push({
                                    name: "length " + h,
                                    type: "USHORT",
                                    value: e[h].length
                                })
                            }
                            return t.fields.push({
                                name: "stringPool",
                                type: "CHARARRAY",
                                value: r
                            }), t
                        },
                        parse: function(e, t) {
                            var r = new ce.Parser(e, t),
                                n = r.parseULong();
                            w.argument(1 === n, "Unsupported ltag table version."), r.skip("uLong", 1);
                            for (var h = r.parseULong(), a = [], i = 0; i < h; i++) {
                                for (var c = "", o = t + r.parseUShort(), s = r.parseUShort(), l = o; l < o + s; ++l) c += String.fromCharCode(e.getInt8(l));
                                a.push(c)
                            }
                            return a
                        }
                    };
                    var _e = {
                            parse: function(e, t) {
                                var r = {},
                                    n = new ce.Parser(e, t);
                                return r.version = n.parseVersion(), r.numGlyphs = n.parseUShort(), 1 === r.version && (r.maxPoints = n.parseUShort(), r.maxContours = n.parseUShort(), r.maxCompositePoints = n.parseUShort(), r.maxCompositeContours = n.parseUShort(), r.maxZones = n.parseUShort(), r.maxTwilightPoints = n.parseUShort(), r.maxStorage = n.parseUShort(), r.maxFunctionDefs = n.parseUShort(), r.maxInstructionDefs = n.parseUShort(), r.maxStackElements = n.parseUShort(), r.maxSizeOfInstructions = n.parseUShort(), r.maxComponentElements = n.parseUShort(), r.maxComponentDepth = n.parseUShort()), r
                            },
                            make: function(e) {
                                return new $.Table("maxp", [{
                                    name: "version",
                                    type: "FIXED",
                                    value: 20480
                                }, {
                                    name: "numGlyphs",
                                    type: "USHORT",
                                    value: e
                                }])
                            }
                        },
                        Je = ["copyright", "fontFamily", "fontSubfamily", "uniqueID", "fullName", "version", "postScriptName", "trademark", "manufacturer", "designer", "description", "manufacturerURL", "designerURL", "license", "licenseURL", "reserved", "preferredFamily", "preferredSubfamily", "compatibleFullName", "sampleText", "postScriptFindFontName", "wwsFamily", "wwsSubfamily"],
                        Ue = {
                            0: "en",
                            1: "fr",
                            2: "de",
                            3: "it",
                            4: "nl",
                            5: "sv",
                            6: "es",
                            7: "da",
                            8: "pt",
                            9: "no",
                            10: "he",
                            11: "ja",
                            12: "ar",
                            13: "fi",
                            14: "el",
                            15: "is",
                            16: "mt",
                            17: "tr",
                            18: "hr",
                            19: "zh-Hant",
                            20: "ur",
                            21: "hi",
                            22: "th",
                            23: "ko",
                            24: "lt",
                            25: "pl",
                            26: "hu",
                            27: "es",
                            28: "lv",
                            29: "se",
                            30: "fo",
                            31: "fa",
                            32: "ru",
                            33: "zh",
                            34: "nl-BE",
                            35: "ga",
                            36: "sq",
                            37: "ro",
                            38: "cz",
                            39: "sk",
                            40: "si",
                            41: "yi",
                            42: "sr",
                            43: "mk",
                            44: "bg",
                            45: "uk",
                            46: "be",
                            47: "uz",
                            48: "kk",
                            49: "az-Cyrl",
                            50: "az-Arab",
                            51: "hy",
                            52: "ka",
                            53: "mo",
                            54: "ky",
                            55: "tg",
                            56: "tk",
                            57: "mn-CN",
                            58: "mn",
                            59: "ps",
                            60: "ks",
                            61: "ku",
                            62: "sd",
                            63: "bo",
                            64: "ne",
                            65: "sa",
                            66: "mr",
                            67: "bn",
                            68: "as",
                            69: "gu",
                            70: "pa",
                            71: "or",
                            72: "ml",
                            73: "kn",
                            74: "ta",
                            75: "te",
                            76: "si",
                            77: "my",
                            78: "km",
                            79: "lo",
                            80: "vi",
                            81: "id",
                            82: "tl",
                            83: "ms",
                            84: "ms-Arab",
                            85: "am",
                            86: "ti",
                            87: "om",
                            88: "so",
                            89: "sw",
                            90: "rw",
                            91: "rn",
                            92: "ny",
                            93: "mg",
                            94: "eo",
                            128: "cy",
                            129: "eu",
                            130: "ca",
                            131: "la",
                            132: "qu",
                            133: "gn",
                            134: "ay",
                            135: "tt",
                            136: "ug",
                            137: "dz",
                            138: "jv",
                            139: "su",
                            140: "gl",
                            141: "af",
                            142: "br",
                            143: "iu",
                            144: "gd",
                            145: "gv",
                            146: "ga",
                            147: "to",
                            148: "el-polyton",
                            149: "kl",
                            150: "az",
                            151: "nn"
                        },
                        Oe = {
                            0: 0,
                            1: 0,
                            2: 0,
                            3: 0,
                            4: 0,
                            5: 0,
                            6: 0,
                            7: 0,
                            8: 0,
                            9: 0,
                            10: 5,
                            11: 1,
                            12: 4,
                            13: 0,
                            14: 6,
                            15: 0,
                            16: 0,
                            17: 0,
                            18: 0,
                            19: 2,
                            20: 4,
                            21: 9,
                            22: 21,
                            23: 3,
                            24: 29,
                            25: 29,
                            26: 29,
                            27: 29,
                            28: 29,
                            29: 0,
                            30: 0,
                            31: 4,
                            32: 7,
                            33: 25,
                            34: 0,
                            35: 0,
                            36: 0,
                            37: 0,
                            38: 29,
                            39: 29,
                            40: 0,
                            41: 5,
                            42: 7,
                            43: 7,
                            44: 7,
                            45: 7,
                            46: 7,
                            47: 7,
                            48: 7,
                            49: 7,
                            50: 4,
                            51: 24,
                            52: 23,
                            53: 7,
                            54: 7,
                            55: 7,
                            56: 7,
                            57: 27,
                            58: 7,
                            59: 4,
                            60: 4,
                            61: 4,
                            62: 4,
                            63: 26,
                            64: 9,
                            65: 9,
                            66: 9,
                            67: 13,
                            68: 13,
                            69: 11,
                            70: 10,
                            71: 12,
                            72: 17,
                            73: 16,
                            74: 14,
                            75: 15,
                            76: 18,
                            77: 19,
                            78: 20,
                            79: 22,
                            80: 30,
                            81: 0,
                            82: 0,
                            83: 0,
                            84: 4,
                            85: 28,
                            86: 28,
                            87: 28,
                            88: 0,
                            89: 0,
                            90: 0,
                            91: 0,
                            92: 0,
                            93: 0,
                            94: 0,
                            128: 0,
                            129: 0,
                            130: 0,
                            131: 0,
                            132: 0,
                            133: 0,
                            134: 0,
                            135: 7,
                            136: 4,
                            137: 26,
                            138: 0,
                            139: 0,
                            140: 0,
                            141: 0,
                            142: 0,
                            143: 28,
                            144: 0,
                            145: 0,
                            146: 0,
                            147: 0,
                            148: 6,
                            149: 0,
                            150: 0,
                            151: 0
                        },
                        Ze = {
                            1078: "af",
                            1052: "sq",
                            1156: "gsw",
                            1118: "am",
                            5121: "ar-DZ",
                            15361: "ar-BH",
                            3073: "ar",
                            2049: "ar-IQ",
                            11265: "ar-JO",
                            13313: "ar-KW",
                            12289: "ar-LB",
                            4097: "ar-LY",
                            6145: "ary",
                            8193: "ar-OM",
                            16385: "ar-QA",
                            1025: "ar-SA",
                            10241: "ar-SY",
                            7169: "aeb",
                            14337: "ar-AE",
                            9217: "ar-YE",
                            1067: "hy",
                            1101: "as",
                            2092: "az-Cyrl",
                            1068: "az",
                            1133: "ba",
                            1069: "eu",
                            1059: "be",
                            2117: "bn",
                            1093: "bn-IN",
                            8218: "bs-Cyrl",
                            5146: "bs",
                            1150: "br",
                            1026: "bg",
                            1027: "ca",
                            3076: "zh-HK",
                            5124: "zh-MO",
                            2052: "zh",
                            4100: "zh-SG",
                            1028: "zh-TW",
                            1155: "co",
                            1050: "hr",
                            4122: "hr-BA",
                            1029: "cs",
                            1030: "da",
                            1164: "prs",
                            1125: "dv",
                            2067: "nl-BE",
                            1043: "nl",
                            3081: "en-AU",
                            10249: "en-BZ",
                            4105: "en-CA",
                            9225: "en-029",
                            16393: "en-IN",
                            6153: "en-IE",
                            8201: "en-JM",
                            17417: "en-MY",
                            5129: "en-NZ",
                            13321: "en-PH",
                            18441: "en-SG",
                            7177: "en-ZA",
                            11273: "en-TT",
                            2057: "en-GB",
                            1033: "en",
                            12297: "en-ZW",
                            1061: "et",
                            1080: "fo",
                            1124: "fil",
                            1035: "fi",
                            2060: "fr-BE",
                            3084: "fr-CA",
                            1036: "fr",
                            5132: "fr-LU",
                            6156: "fr-MC",
                            4108: "fr-CH",
                            1122: "fy",
                            1110: "gl",
                            1079: "ka",
                            3079: "de-AT",
                            1031: "de",
                            5127: "de-LI",
                            4103: "de-LU",
                            2055: "de-CH",
                            1032: "el",
                            1135: "kl",
                            1095: "gu",
                            1128: "ha",
                            1037: "he",
                            1081: "hi",
                            1038: "hu",
                            1039: "is",
                            1136: "ig",
                            1057: "id",
                            1117: "iu",
                            2141: "iu-Latn",
                            2108: "ga",
                            1076: "xh",
                            1077: "zu",
                            1040: "it",
                            2064: "it-CH",
                            1041: "ja",
                            1099: "kn",
                            1087: "kk",
                            1107: "km",
                            1158: "quc",
                            1159: "rw",
                            1089: "sw",
                            1111: "kok",
                            1042: "ko",
                            1088: "ky",
                            1108: "lo",
                            1062: "lv",
                            1063: "lt",
                            2094: "dsb",
                            1134: "lb",
                            1071: "mk",
                            2110: "ms-BN",
                            1086: "ms",
                            1100: "ml",
                            1082: "mt",
                            1153: "mi",
                            1146: "arn",
                            1102: "mr",
                            1148: "moh",
                            1104: "mn",
                            2128: "mn-CN",
                            1121: "ne",
                            1044: "nb",
                            2068: "nn",
                            1154: "oc",
                            1096: "or",
                            1123: "ps",
                            1045: "pl",
                            1046: "pt",
                            2070: "pt-PT",
                            1094: "pa",
                            1131: "qu-BO",
                            2155: "qu-EC",
                            3179: "qu",
                            1048: "ro",
                            1047: "rm",
                            1049: "ru",
                            9275: "smn",
                            4155: "smj-NO",
                            5179: "smj",
                            3131: "se-FI",
                            1083: "se",
                            2107: "se-SE",
                            8251: "sms",
                            6203: "sma-NO",
                            7227: "sms",
                            1103: "sa",
                            7194: "sr-Cyrl-BA",
                            3098: "sr",
                            6170: "sr-Latn-BA",
                            2074: "sr-Latn",
                            1132: "nso",
                            1074: "tn",
                            1115: "si",
                            1051: "sk",
                            1060: "sl",
                            11274: "es-AR",
                            16394: "es-BO",
                            13322: "es-CL",
                            9226: "es-CO",
                            5130: "es-CR",
                            7178: "es-DO",
                            12298: "es-EC",
                            17418: "es-SV",
                            4106: "es-GT",
                            18442: "es-HN",
                            2058: "es-MX",
                            19466: "es-NI",
                            6154: "es-PA",
                            15370: "es-PY",
                            10250: "es-PE",
                            20490: "es-PR",
                            3082: "es",
                            1034: "es",
                            21514: "es-US",
                            14346: "es-UY",
                            8202: "es-VE",
                            2077: "sv-FI",
                            1053: "sv",
                            1114: "syr",
                            1064: "tg",
                            2143: "tzm",
                            1097: "ta",
                            1092: "tt",
                            1098: "te",
                            1054: "th",
                            1105: "bo",
                            1055: "tr",
                            1090: "tk",
                            1152: "ug",
                            1058: "uk",
                            1070: "hsb",
                            1056: "ur",
                            2115: "uz-Cyrl",
                            1091: "uz",
                            1066: "vi",
                            1106: "cy",
                            1160: "wo",
                            1157: "sah",
                            1144: "ii",
                            1130: "yo"
                        };

                    function Ke(e, t, r) {
                        switch (e) {
                            case 0:
                                if (65535 === t) return "und";
                                if (r) return r[t];
                                break;
                            case 1:
                                return Ue[t];
                            case 3:
                                return Ze[t]
                        }
                    }
                    var $e = {
                            0: "macintosh",
                            1: "x-mac-japanese",
                            2: "x-mac-chinesetrad",
                            3: "x-mac-korean",
                            6: "x-mac-greek",
                            7: "x-mac-cyrillic",
                            9: "x-mac-devanagai",
                            10: "x-mac-gurmukhi",
                            11: "x-mac-gujarati",
                            12: "x-mac-oriya",
                            13: "x-mac-bengali",
                            14: "x-mac-tamil",
                            15: "x-mac-telugu",
                            16: "x-mac-kannada",
                            17: "x-mac-malayalam",
                            18: "x-mac-sinhalese",
                            19: "x-mac-burmese",
                            20: "x-mac-khmer",
                            21: "x-mac-thai",
                            22: "x-mac-lao",
                            23: "x-mac-georgian",
                            24: "x-mac-armenian",
                            25: "x-mac-chinesesimp",
                            26: "x-mac-tibetan",
                            27: "x-mac-mongolian",
                            28: "x-mac-ethiopic",
                            29: "x-mac-ce",
                            30: "x-mac-vietnamese",
                            31: "x-mac-extarabic"
                        },
                        et = {
                            15: "x-mac-icelandic",
                            17: "x-mac-turkish",
                            18: "x-mac-croatian",
                            24: "x-mac-ce",
                            25: "x-mac-ce",
                            26: "x-mac-ce",
                            27: "x-mac-ce",
                            28: "x-mac-ce",
                            30: "x-mac-icelandic",
                            37: "x-mac-romanian",
                            38: "x-mac-ce",
                            39: "x-mac-ce",
                            40: "x-mac-ce",
                            143: "x-mac-inuit",
                            146: "x-mac-gaelic"
                        };

                    function tt(e, t, r) {
                        switch (e) {
                            case 0:
                                return "utf-16";
                            case 1:
                                return et[r] || $e[t];
                            case 3:
                                if (1 === t || 10 === t) return "utf-16";
                                break
                        }
                    }

                    function rt(e) {
                        var t = {};
                        for (var r in e) t[e[r]] = parseInt(r);
                        return t
                    }

                    function nt(e, t, r, n, h, a) {
                        return new $.Record("NameRecord", [{
                            name: "platformID",
                            type: "USHORT",
                            value: e
                        }, {
                            name: "encodingID",
                            type: "USHORT",
                            value: t
                        }, {
                            name: "languageID",
                            type: "USHORT",
                            value: r
                        }, {
                            name: "nameID",
                            type: "USHORT",
                            value: n
                        }, {
                            name: "length",
                            type: "USHORT",
                            value: h
                        }, {
                            name: "offset",
                            type: "USHORT",
                            value: a
                        }])
                    }

                    function ht(e, t) {
                        var r = function(e, t) {
                            var r = e.length,
                                n = t.length - r + 1;
                            e: for (var h = 0; h < n; h++)
                                for (; h < n; h++) {
                                    for (var a = 0; a < r; a++)
                                        if (t[h + a] !== e[a]) continue e;
                                    return h
                                }
                            return -1
                        }(e, t);
                        if (r < 0) {
                            r = t.length;
                            for (var n = 0, h = e.length; n < h; ++n) t.push(e[n])
                        }
                        return r
                    }
                    var at = {
                            parse: function(e, t, r) {
                                for (var n = {}, h = new ce.Parser(e, t), a = h.parseUShort(), i = h.parseUShort(), c = h.offset + h.parseUShort(), o = 0; o < i; o++) {
                                    var s = h.parseUShort(),
                                        l = h.parseUShort(),
                                        p = h.parseUShort(),
                                        d = h.parseUShort(),
                                        b = Je[d] || d,
                                        g = h.parseUShort(),
                                        j = h.parseUShort(),
                                        x = Ke(s, p, r),
                                        V = tt(s, l, p);
                                    if (void 0 !== V && void 0 !== x) {
                                        var f = void 0;
                                        if (f = "utf-16" === V ? y.UTF16(e, c + j, g) : y.MACSTRING(e, c + j, g, V), f) {
                                            var F = n[b];
                                            void 0 === F && (F = n[b] = {}), F[x] = f
                                        }
                                    }
                                }
                                return 1 === a && h.parseUShort(), n
                            },
                            make: function(e, t) {
                                var r, n = [],
                                    h = {},
                                    a = rt(Je);
                                for (var i in e) {
                                    var c = a[i];
                                    if (void 0 === c && (c = i), r = parseInt(c), isNaN(r)) throw new Error('Name table entry "' + i + '" does not exist, see nameTableNames for complete list.');
                                    h[r] = e[i], n.push(r)
                                }
                                for (var o = rt(Ue), s = rt(Ze), l = [], p = [], d = 0; d < n.length; d++) {
                                    r = n[d];
                                    var b = h[r];
                                    for (var g in b) {
                                        var j = b[g],
                                            x = 1,
                                            V = o[g],
                                            f = Oe[V],
                                            F = tt(x, f, V),
                                            k = C.MACSTRING(j, F);
                                        void 0 === k && (x = 0, V = t.indexOf(g), V < 0 && (V = t.length, t.push(g)), f = 4, k = C.UTF16(j));
                                        var m = ht(k, p);
                                        l.push(nt(x, f, V, r, k.length, m));
                                        var P = s[g];
                                        if (void 0 !== P) {
                                            var u = C.UTF16(j),
                                                X = ht(u, p);
                                            l.push(nt(3, 1, P, r, u.length, X))
                                        }
                                    }
                                }
                                l.sort((function(e, t) {
                                    return e.platformID - t.platformID || e.encodingID - t.encodingID || e.languageID - t.languageID || e.nameID - t.nameID
                                }));
                                for (var N = new $.Table("name", [{
                                        name: "format",
                                        type: "USHORT",
                                        value: 0
                                    }, {
                                        name: "count",
                                        type: "USHORT",
                                        value: l.length
                                    }, {
                                        name: "stringOffset",
                                        type: "USHORT",
                                        value: 6 + 12 * l.length
                                    }]), H = 0; H < l.length; H++) N.fields.push({
                                    name: "record_" + H,
                                    type: "RECORD",
                                    value: l[H]
                                });
                                return N.fields.push({
                                    name: "strings",
                                    type: "LITERAL",
                                    value: p
                                }), N
                            }
                        },
                        it = [{
                            begin: 0,
                            end: 127
                        }, {
                            begin: 128,
                            end: 255
                        }, {
                            begin: 256,
                            end: 383
                        }, {
                            begin: 384,
                            end: 591
                        }, {
                            begin: 592,
                            end: 687
                        }, {
                            begin: 688,
                            end: 767
                        }, {
                            begin: 768,
                            end: 879
                        }, {
                            begin: 880,
                            end: 1023
                        }, {
                            begin: 11392,
                            end: 11519
                        }, {
                            begin: 1024,
                            end: 1279
                        }, {
                            begin: 1328,
                            end: 1423
                        }, {
                            begin: 1424,
                            end: 1535
                        }, {
                            begin: 42240,
                            end: 42559
                        }, {
                            begin: 1536,
                            end: 1791
                        }, {
                            begin: 1984,
                            end: 2047
                        }, {
                            begin: 2304,
                            end: 2431
                        }, {
                            begin: 2432,
                            end: 2559
                        }, {
                            begin: 2560,
                            end: 2687
                        }, {
                            begin: 2688,
                            end: 2815
                        }, {
                            begin: 2816,
                            end: 2943
                        }, {
                            begin: 2944,
                            end: 3071
                        }, {
                            begin: 3072,
                            end: 3199
                        }, {
                            begin: 3200,
                            end: 3327
                        }, {
                            begin: 3328,
                            end: 3455
                        }, {
                            begin: 3584,
                            end: 3711
                        }, {
                            begin: 3712,
                            end: 3839
                        }, {
                            begin: 4256,
                            end: 4351
                        }, {
                            begin: 6912,
                            end: 7039
                        }, {
                            begin: 4352,
                            end: 4607
                        }, {
                            begin: 7680,
                            end: 7935
                        }, {
                            begin: 7936,
                            end: 8191
                        }, {
                            begin: 8192,
                            end: 8303
                        }, {
                            begin: 8304,
                            end: 8351
                        }, {
                            begin: 8352,
                            end: 8399
                        }, {
                            begin: 8400,
                            end: 8447
                        }, {
                            begin: 8448,
                            end: 8527
                        }, {
                            begin: 8528,
                            end: 8591
                        }, {
                            begin: 8592,
                            end: 8703
                        }, {
                            begin: 8704,
                            end: 8959
                        }, {
                            begin: 8960,
                            end: 9215
                        }, {
                            begin: 9216,
                            end: 9279
                        }, {
                            begin: 9280,
                            end: 9311
                        }, {
                            begin: 9312,
                            end: 9471
                        }, {
                            begin: 9472,
                            end: 9599
                        }, {
                            begin: 9600,
                            end: 9631
                        }, {
                            begin: 9632,
                            end: 9727
                        }, {
                            begin: 9728,
                            end: 9983
                        }, {
                            begin: 9984,
                            end: 10175
                        }, {
                            begin: 12288,
                            end: 12351
                        }, {
                            begin: 12352,
                            end: 12447
                        }, {
                            begin: 12448,
                            end: 12543
                        }, {
                            begin: 12544,
                            end: 12591
                        }, {
                            begin: 12592,
                            end: 12687
                        }, {
                            begin: 43072,
                            end: 43135
                        }, {
                            begin: 12800,
                            end: 13055
                        }, {
                            begin: 13056,
                            end: 13311
                        }, {
                            begin: 44032,
                            end: 55215
                        }, {
                            begin: 55296,
                            end: 57343
                        }, {
                            begin: 67840,
                            end: 67871
                        }, {
                            begin: 19968,
                            end: 40959
                        }, {
                            begin: 57344,
                            end: 63743
                        }, {
                            begin: 12736,
                            end: 12783
                        }, {
                            begin: 64256,
                            end: 64335
                        }, {
                            begin: 64336,
                            end: 65023
                        }, {
                            begin: 65056,
                            end: 65071
                        }, {
                            begin: 65040,
                            end: 65055
                        }, {
                            begin: 65104,
                            end: 65135
                        }, {
                            begin: 65136,
                            end: 65279
                        }, {
                            begin: 65280,
                            end: 65519
                        }, {
                            begin: 65520,
                            end: 65535
                        }, {
                            begin: 3840,
                            end: 4095
                        }, {
                            begin: 1792,
                            end: 1871
                        }, {
                            begin: 1920,
                            end: 1983
                        }, {
                            begin: 3456,
                            end: 3583
                        }, {
                            begin: 4096,
                            end: 4255
                        }, {
                            begin: 4608,
                            end: 4991
                        }, {
                            begin: 5024,
                            end: 5119
                        }, {
                            begin: 5120,
                            end: 5759
                        }, {
                            begin: 5760,
                            end: 5791
                        }, {
                            begin: 5792,
                            end: 5887
                        }, {
                            begin: 6016,
                            end: 6143
                        }, {
                            begin: 6144,
                            end: 6319
                        }, {
                            begin: 10240,
                            end: 10495
                        }, {
                            begin: 40960,
                            end: 42127
                        }, {
                            begin: 5888,
                            end: 5919
                        }, {
                            begin: 66304,
                            end: 66351
                        }, {
                            begin: 66352,
                            end: 66383
                        }, {
                            begin: 66560,
                            end: 66639
                        }, {
                            begin: 118784,
                            end: 119039
                        }, {
                            begin: 119808,
                            end: 120831
                        }, {
                            begin: 1044480,
                            end: 1048573
                        }, {
                            begin: 65024,
                            end: 65039
                        }, {
                            begin: 917504,
                            end: 917631
                        }, {
                            begin: 6400,
                            end: 6479
                        }, {
                            begin: 6480,
                            end: 6527
                        }, {
                            begin: 6528,
                            end: 6623
                        }, {
                            begin: 6656,
                            end: 6687
                        }, {
                            begin: 11264,
                            end: 11359
                        }, {
                            begin: 11568,
                            end: 11647
                        }, {
                            begin: 19904,
                            end: 19967
                        }, {
                            begin: 43008,
                            end: 43055
                        }, {
                            begin: 65536,
                            end: 65663
                        }, {
                            begin: 65856,
                            end: 65935
                        }, {
                            begin: 66432,
                            end: 66463
                        }, {
                            begin: 66464,
                            end: 66527
                        }, {
                            begin: 66640,
                            end: 66687
                        }, {
                            begin: 66688,
                            end: 66735
                        }, {
                            begin: 67584,
                            end: 67647
                        }, {
                            begin: 68096,
                            end: 68191
                        }, {
                            begin: 119552,
                            end: 119647
                        }, {
                            begin: 73728,
                            end: 74751
                        }, {
                            begin: 119648,
                            end: 119679
                        }, {
                            begin: 7040,
                            end: 7103
                        }, {
                            begin: 7168,
                            end: 7247
                        }, {
                            begin: 7248,
                            end: 7295
                        }, {
                            begin: 43136,
                            end: 43231
                        }, {
                            begin: 43264,
                            end: 43311
                        }, {
                            begin: 43312,
                            end: 43359
                        }, {
                            begin: 43520,
                            end: 43615
                        }, {
                            begin: 65936,
                            end: 65999
                        }, {
                            begin: 66e3,
                            end: 66047
                        }, {
                            begin: 66208,
                            end: 66271
                        }, {
                            begin: 127024,
                            end: 127135
                        }];
                    var ct = {
                        parse: function(e, t) {
                            var r = {},
                                n = new ce.Parser(e, t);
                            r.version = n.parseUShort(), r.xAvgCharWidth = n.parseShort(), r.usWeightClass = n.parseUShort(), r.usWidthClass = n.parseUShort(), r.fsType = n.parseUShort(), r.ySubscriptXSize = n.parseShort(), r.ySubscriptYSize = n.parseShort(), r.ySubscriptXOffset = n.parseShort(), r.ySubscriptYOffset = n.parseShort(), r.ySuperscriptXSize = n.parseShort(), r.ySuperscriptYSize = n.parseShort(), r.ySuperscriptXOffset = n.parseShort(), r.ySuperscriptYOffset = n.parseShort(), r.yStrikeoutSize = n.parseShort(), r.yStrikeoutPosition = n.parseShort(), r.sFamilyClass = n.parseShort(), r.panose = [];
                            for (var h = 0; h < 10; h++) r.panose[h] = n.parseByte();
                            return r.ulUnicodeRange1 = n.parseULong(), r.ulUnicodeRange2 = n.parseULong(), r.ulUnicodeRange3 = n.parseULong(), r.ulUnicodeRange4 = n.parseULong(), r.achVendID = String.fromCharCode(n.parseByte(), n.parseByte(), n.parseByte(), n.parseByte()), r.fsSelection = n.parseUShort(), r.usFirstCharIndex = n.parseUShort(), r.usLastCharIndex = n.parseUShort(), r.sTypoAscender = n.parseShort(), r.sTypoDescender = n.parseShort(), r.sTypoLineGap = n.parseShort(), r.usWinAscent = n.parseUShort(), r.usWinDescent = n.parseUShort(), r.version >= 1 && (r.ulCodePageRange1 = n.parseULong(), r.ulCodePageRange2 = n.parseULong()), r.version >= 2 && (r.sxHeight = n.parseShort(), r.sCapHeight = n.parseShort(), r.usDefaultChar = n.parseUShort(), r.usBreakChar = n.parseUShort(), r.usMaxContent = n.parseUShort()), r
                        },
                        make: function(e) {
                            return new $.Table("OS/2", [{
                                name: "version",
                                type: "USHORT",
                                value: 3
                            }, {
                                name: "xAvgCharWidth",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "usWeightClass",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "usWidthClass",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "fsType",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "ySubscriptXSize",
                                type: "SHORT",
                                value: 650
                            }, {
                                name: "ySubscriptYSize",
                                type: "SHORT",
                                value: 699
                            }, {
                                name: "ySubscriptXOffset",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "ySubscriptYOffset",
                                type: "SHORT",
                                value: 140
                            }, {
                                name: "ySuperscriptXSize",
                                type: "SHORT",
                                value: 650
                            }, {
                                name: "ySuperscriptYSize",
                                type: "SHORT",
                                value: 699
                            }, {
                                name: "ySuperscriptXOffset",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "ySuperscriptYOffset",
                                type: "SHORT",
                                value: 479
                            }, {
                                name: "yStrikeoutSize",
                                type: "SHORT",
                                value: 49
                            }, {
                                name: "yStrikeoutPosition",
                                type: "SHORT",
                                value: 258
                            }, {
                                name: "sFamilyClass",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "bFamilyType",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "bSerifStyle",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "bWeight",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "bProportion",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "bContrast",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "bStrokeVariation",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "bArmStyle",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "bLetterform",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "bMidline",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "bXHeight",
                                type: "BYTE",
                                value: 0
                            }, {
                                name: "ulUnicodeRange1",
                                type: "ULONG",
                                value: 0
                            }, {
                                name: "ulUnicodeRange2",
                                type: "ULONG",
                                value: 0
                            }, {
                                name: "ulUnicodeRange3",
                                type: "ULONG",
                                value: 0
                            }, {
                                name: "ulUnicodeRange4",
                                type: "ULONG",
                                value: 0
                            }, {
                                name: "achVendID",
                                type: "CHARARRAY",
                                value: "XXXX"
                            }, {
                                name: "fsSelection",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "usFirstCharIndex",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "usLastCharIndex",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "sTypoAscender",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "sTypoDescender",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "sTypoLineGap",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "usWinAscent",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "usWinDescent",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "ulCodePageRange1",
                                type: "ULONG",
                                value: 0
                            }, {
                                name: "ulCodePageRange2",
                                type: "ULONG",
                                value: 0
                            }, {
                                name: "sxHeight",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "sCapHeight",
                                type: "SHORT",
                                value: 0
                            }, {
                                name: "usDefaultChar",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "usBreakChar",
                                type: "USHORT",
                                value: 0
                            }, {
                                name: "usMaxContext",
                                type: "USHORT",
                                value: 0
                            }], e)
                        },
                        unicodeRanges: it,
                        getUnicodeRange: function(e) {
                            for (var t = 0; t < it.length; t += 1) {
                                var r = it[t];
                                if (e >= r.begin && e < r.end) return t
                            }
                            return -1
                        }
                    };
                    var ot = {
                            parse: function(e, t) {
                                var r = {},
                                    n = new ce.Parser(e, t);
                                switch (r.version = n.parseVersion(), r.italicAngle = n.parseFixed(), r.underlinePosition = n.parseShort(), r.underlineThickness = n.parseShort(), r.isFixedPitch = n.parseULong(), r.minMemType42 = n.parseULong(), r.maxMemType42 = n.parseULong(), r.minMemType1 = n.parseULong(), r.maxMemType1 = n.parseULong(), r.version) {
                                    case 1:
                                        r.names = be.slice();
                                        break;
                                    case 2:
                                        r.numberOfGlyphs = n.parseUShort(), r.glyphNameIndex = new Array(r.numberOfGlyphs);
                                        for (var h = 0; h < r.numberOfGlyphs; h++) r.glyphNameIndex[h] = n.parseUShort();
                                        r.names = [];
                                        for (var a = 0; a < r.numberOfGlyphs; a++)
                                            if (r.glyphNameIndex[a] >= be.length) {
                                                var i = n.parseChar();
                                                r.names.push(n.parseString(i))
                                            } break;
                                    case 2.5:
                                        r.numberOfGlyphs = n.parseUShort(), r.offset = new Array(r.numberOfGlyphs);
                                        for (var c = 0; c < r.numberOfGlyphs; c++) r.offset[c] = n.parseChar();
                                        break
                                }
                                return r
                            },
                            make: function() {
                                return new $.Table("post", [{
                                    name: "version",
                                    type: "FIXED",
                                    value: 196608
                                }, {
                                    name: "italicAngle",
                                    type: "FIXED",
                                    value: 0
                                }, {
                                    name: "underlinePosition",
                                    type: "FWORD",
                                    value: 0
                                }, {
                                    name: "underlineThickness",
                                    type: "FWORD",
                                    value: 0
                                }, {
                                    name: "isFixedPitch",
                                    type: "ULONG",
                                    value: 0
                                }, {
                                    name: "minMemType42",
                                    type: "ULONG",
                                    value: 0
                                }, {
                                    name: "maxMemType42",
                                    type: "ULONG",
                                    value: 0
                                }, {
                                    name: "minMemType1",
                                    type: "ULONG",
                                    value: 0
                                }, {
                                    name: "maxMemType1",
                                    type: "ULONG",
                                    value: 0
                                }])
                            }
                        },
                        st = new Array(9);
                    st[1] = function() {
                        var e = this.offset + this.relativeOffset,
                            t = this.parseUShort();
                        return 1 === t ? {
                            substFormat: 1,
                            coverage: this.parsePointer(ae.coverage),
                            deltaGlyphId: this.parseUShort()
                        } : 2 === t ? {
                            substFormat: 2,
                            coverage: this.parsePointer(ae.coverage),
                            substitute: this.parseOffset16List()
                        } : void w.assert(!1, "0x" + e.toString(16) + ": lookup type 1 format must be 1 or 2.")
                    }, st[2] = function() {
                        var e = this.parseUShort();
                        return w.argument(1 === e, "GSUB Multiple Substitution Subtable identifier-format must be 1"), {
                            substFormat: e,
                            coverage: this.parsePointer(ae.coverage),
                            sequences: this.parseListOfLists()
                        }
                    }, st[3] = function() {
                        var e = this.parseUShort();
                        return w.argument(1 === e, "GSUB Alternate Substitution Subtable identifier-format must be 1"), {
                            substFormat: e,
                            coverage: this.parsePointer(ae.coverage),
                            alternateSets: this.parseListOfLists()
                        }
                    }, st[4] = function() {
                        var e = this.parseUShort();
                        return w.argument(1 === e, "GSUB ligature table identifier-format must be 1"), {
                            substFormat: e,
                            coverage: this.parsePointer(ae.coverage),
                            ligatureSets: this.parseListOfLists((function() {
                                return {
                                    ligGlyph: this.parseUShort(),
                                    components: this.parseUShortList(this.parseUShort() - 1)
                                }
                            }))
                        }
                    };
                    var lt = {
                        sequenceIndex: ae.uShort,
                        lookupListIndex: ae.uShort
                    };
                    st[5] = function() {
                        var e = this.offset + this.relativeOffset,
                            t = this.parseUShort();
                        if (1 === t) return {
                            substFormat: t,
                            coverage: this.parsePointer(ae.coverage),
                            ruleSets: this.parseListOfLists((function() {
                                var e = this.parseUShort(),
                                    t = this.parseUShort();
                                return {
                                    input: this.parseUShortList(e - 1),
                                    lookupRecords: this.parseRecordList(t, lt)
                                }
                            }))
                        };
                        if (2 === t) return {
                            substFormat: t,
                            coverage: this.parsePointer(ae.coverage),
                            classDef: this.parsePointer(ae.classDef),
                            classSets: this.parseListOfLists((function() {
                                var e = this.parseUShort(),
                                    t = this.parseUShort();
                                return {
                                    classes: this.parseUShortList(e - 1),
                                    lookupRecords: this.parseRecordList(t, lt)
                                }
                            }))
                        };
                        if (3 === t) {
                            var r = this.parseUShort(),
                                n = this.parseUShort();
                            return {
                                substFormat: t,
                                coverages: this.parseList(r, ae.pointer(ae.coverage)),
                                lookupRecords: this.parseRecordList(n, lt)
                            }
                        }
                        w.assert(!1, "0x" + e.toString(16) + ": lookup type 5 format must be 1, 2 or 3.")
                    }, st[6] = function() {
                        var e = this.offset + this.relativeOffset,
                            t = this.parseUShort();
                        return 1 === t ? {
                            substFormat: 1,
                            coverage: this.parsePointer(ae.coverage),
                            chainRuleSets: this.parseListOfLists((function() {
                                return {
                                    backtrack: this.parseUShortList(),
                                    input: this.parseUShortList(this.parseShort() - 1),
                                    lookahead: this.parseUShortList(),
                                    lookupRecords: this.parseRecordList(lt)
                                }
                            }))
                        } : 2 === t ? {
                            substFormat: 2,
                            coverage: this.parsePointer(ae.coverage),
                            backtrackClassDef: this.parsePointer(ae.classDef),
                            inputClassDef: this.parsePointer(ae.classDef),
                            lookaheadClassDef: this.parsePointer(ae.classDef),
                            chainClassSet: this.parseListOfLists((function() {
                                return {
                                    backtrack: this.parseUShortList(),
                                    input: this.parseUShortList(this.parseShort() - 1),
                                    lookahead: this.parseUShortList(),
                                    lookupRecords: this.parseRecordList(lt)
                                }
                            }))
                        } : 3 === t ? {
                            substFormat: 3,
                            backtrackCoverage: this.parseList(ae.pointer(ae.coverage)),
                            inputCoverage: this.parseList(ae.pointer(ae.coverage)),
                            lookaheadCoverage: this.parseList(ae.pointer(ae.coverage)),
                            lookupRecords: this.parseRecordList(lt)
                        } : void w.assert(!1, "0x" + e.toString(16) + ": lookup type 6 format must be 1, 2 or 3.")
                    }, st[7] = function() {
                        var e = this.parseUShort();
                        w.argument(1 === e, "GSUB Extension Substitution subtable identifier-format must be 1");
                        var t = this.parseUShort(),
                            r = new ae(this.data, this.offset + this.parseULong());
                        return {
                            substFormat: 1,
                            lookupType: t,
                            extension: st[t].call(r)
                        }
                    }, st[8] = function() {
                        var e = this.parseUShort();
                        return w.argument(1 === e, "GSUB Reverse Chaining Contextual Single Substitution Subtable identifier-format must be 1"), {
                            substFormat: e,
                            coverage: this.parsePointer(ae.coverage),
                            backtrackCoverage: this.parseList(ae.pointer(ae.coverage)),
                            lookaheadCoverage: this.parseList(ae.pointer(ae.coverage)),
                            substitutes: this.parseUShortList()
                        }
                    };
                    var pt = new Array(9);
                    pt[1] = function(e) {
                        return 1 === e.substFormat ? new $.Table("substitutionTable", [{
                            name: "substFormat",
                            type: "USHORT",
                            value: 1
                        }, {
                            name: "coverage",
                            type: "TABLE",
                            value: new $.Coverage(e.coverage)
                        }, {
                            name: "deltaGlyphID",
                            type: "USHORT",
                            value: e.deltaGlyphId
                        }]) : new $.Table("substitutionTable", [{
                            name: "substFormat",
                            type: "USHORT",
                            value: 2
                        }, {
                            name: "coverage",
                            type: "TABLE",
                            value: new $.Coverage(e.coverage)
                        }].concat($.ushortList("substitute", e.substitute)))
                    }, pt[2] = function(e) {
                        return w.assert(1 === e.substFormat, "Lookup type 2 substFormat must be 1."), new $.Table("substitutionTable", [{
                            name: "substFormat",
                            type: "USHORT",
                            value: 1
                        }, {
                            name: "coverage",
                            type: "TABLE",
                            value: new $.Coverage(e.coverage)
                        }].concat($.tableList("seqSet", e.sequences, (function(e) {
                            return new $.Table("sequenceSetTable", $.ushortList("sequence", e))
                        }))))
                    }, pt[3] = function(e) {
                        return w.assert(1 === e.substFormat, "Lookup type 3 substFormat must be 1."), new $.Table("substitutionTable", [{
                            name: "substFormat",
                            type: "USHORT",
                            value: 1
                        }, {
                            name: "coverage",
                            type: "TABLE",
                            value: new $.Coverage(e.coverage)
                        }].concat($.tableList("altSet", e.alternateSets, (function(e) {
                            return new $.Table("alternateSetTable", $.ushortList("alternate", e))
                        }))))
                    }, pt[4] = function(e) {
                        return w.assert(1 === e.substFormat, "Lookup type 4 substFormat must be 1."), new $.Table("substitutionTable", [{
                            name: "substFormat",
                            type: "USHORT",
                            value: 1
                        }, {
                            name: "coverage",
                            type: "TABLE",
                            value: new $.Coverage(e.coverage)
                        }].concat($.tableList("ligSet", e.ligatureSets, (function(e) {
                            return new $.Table("ligatureSetTable", $.tableList("ligature", e, (function(e) {
                                return new $.Table("ligatureTable", [{
                                    name: "ligGlyph",
                                    type: "USHORT",
                                    value: e.ligGlyph
                                }].concat($.ushortList("component", e.components, e.components.length + 1)))
                            })))
                        }))))
                    }, pt[6] = function(e) {
                        if (1 === e.substFormat) {
                            var t = new $.Table("chainContextTable", [{
                                name: "substFormat",
                                type: "USHORT",
                                value: e.substFormat
                            }, {
                                name: "coverage",
                                type: "TABLE",
                                value: new $.Coverage(e.coverage)
                            }].concat($.tableList("chainRuleSet", e.chainRuleSets, (function(e) {
                                return new $.Table("chainRuleSetTable", $.tableList("chainRule", e, (function(e) {
                                    var t = $.ushortList("backtrackGlyph", e.backtrack, e.backtrack.length).concat($.ushortList("inputGlyph", e.input, e.input.length + 1)).concat($.ushortList("lookaheadGlyph", e.lookahead, e.lookahead.length)).concat($.ushortList("substitution", [], e.lookupRecords.length));
                                    return e.lookupRecords.forEach((function(e, r) {
                                        t = t.concat({
                                            name: "sequenceIndex" + r,
                                            type: "USHORT",
                                            value: e.sequenceIndex
                                        }).concat({
                                            name: "lookupListIndex" + r,
                                            type: "USHORT",
                                            value: e.lookupListIndex
                                        })
                                    })), new $.Table("chainRuleTable", t)
                                })))
                            }))));
                            return t
                        }
                        if (2 === e.substFormat) w.assert(!1, "lookup type 6 format 2 is not yet supported.");
                        else if (3 === e.substFormat) {
                            var r = [{
                                name: "substFormat",
                                type: "USHORT",
                                value: e.substFormat
                            }];
                            r.push({
                                name: "backtrackGlyphCount",
                                type: "USHORT",
                                value: e.backtrackCoverage.length
                            }), e.backtrackCoverage.forEach((function(e, t) {
                                r.push({
                                    name: "backtrackCoverage" + t,
                                    type: "TABLE",
                                    value: new $.Coverage(e)
                                })
                            })), r.push({
                                name: "inputGlyphCount",
                                type: "USHORT",
                                value: e.inputCoverage.length
                            }), e.inputCoverage.forEach((function(e, t) {
                                r.push({
                                    name: "inputCoverage" + t,
                                    type: "TABLE",
                                    value: new $.Coverage(e)
                                })
                            })), r.push({
                                name: "lookaheadGlyphCount",
                                type: "USHORT",
                                value: e.lookaheadCoverage.length
                            }), e.lookaheadCoverage.forEach((function(e, t) {
                                r.push({
                                    name: "lookaheadCoverage" + t,
                                    type: "TABLE",
                                    value: new $.Coverage(e)
                                })
                            })), r.push({
                                name: "substitutionCount",
                                type: "USHORT",
                                value: e.lookupRecords.length
                            }), e.lookupRecords.forEach((function(e, t) {
                                r = r.concat({
                                    name: "sequenceIndex" + t,
                                    type: "USHORT",
                                    value: e.sequenceIndex
                                }).concat({
                                    name: "lookupListIndex" + t,
                                    type: "USHORT",
                                    value: e.lookupListIndex
                                })
                            }));
                            var n = new $.Table("chainContextTable", r);
                            return n
                        }
                        w.assert(!1, "lookup type 6 format must be 1, 2 or 3.")
                    };
                    var dt = {
                        parse: function(e, t) {
                            t = t || 0;
                            var r = new ae(e, t),
                                n = r.parseVersion(1);
                            return w.argument(1 === n || 1.1 === n, "Unsupported GSUB table version."), 1 === n ? {
                                version: n,
                                scripts: r.parseScriptList(),
                                features: r.parseFeatureList(),
                                lookups: r.parseLookupList(st)
                            } : {
                                version: n,
                                scripts: r.parseScriptList(),
                                features: r.parseFeatureList(),
                                lookups: r.parseLookupList(st),
                                variations: r.parseFeatureVariationsList()
                            }
                        },
                        make: function(e) {
                            return new $.Table("GSUB", [{
                                name: "version",
                                type: "ULONG",
                                value: 65536
                            }, {
                                name: "scripts",
                                type: "TABLE",
                                value: new $.ScriptList(e.scripts)
                            }, {
                                name: "features",
                                type: "TABLE",
                                value: new $.FeatureList(e.features)
                            }, {
                                name: "lookups",
                                type: "TABLE",
                                value: new $.LookupList(e.lookups, pt)
                            }])
                        }
                    };
                    var bt = {
                        parse: function(e, t) {
                            var r = new ce.Parser(e, t),
                                n = r.parseULong();
                            w.argument(1 === n, "Unsupported META table version."), r.parseULong(), r.parseULong();
                            for (var h = r.parseULong(), a = {}, i = 0; i < h; i++) {
                                var c = r.parseTag(),
                                    o = r.parseULong(),
                                    s = r.parseULong(),
                                    l = y.UTF8(e, t + o, s);
                                a[c] = l
                            }
                            return a
                        },
                        make: function(e) {
                            var t = Object.keys(e).length,
                                r = "",
                                n = 16 + 12 * t,
                                h = new $.Table("meta", [{
                                    name: "version",
                                    type: "ULONG",
                                    value: 1
                                }, {
                                    name: "flags",
                                    type: "ULONG",
                                    value: 0
                                }, {
                                    name: "offset",
                                    type: "ULONG",
                                    value: n
                                }, {
                                    name: "numTags",
                                    type: "ULONG",
                                    value: t
                                }]);
                            for (var a in e) {
                                var i = r.length;
                                r += e[a], h.fields.push({
                                    name: "tag " + a,
                                    type: "TAG",
                                    value: a
                                }), h.fields.push({
                                    name: "offset " + a,
                                    type: "ULONG",
                                    value: n + i
                                }), h.fields.push({
                                    name: "length " + a,
                                    type: "ULONG",
                                    value: e[a].length
                                })
                            }
                            return h.fields.push({
                                name: "stringPool",
                                type: "CHARARRAY",
                                value: r
                            }), h
                        }
                    };

                    function gt(e) {
                        return Math.log(e) / Math.log(2) | 0
                    }

                    function jt(e) {
                        while (e.length % 4 !== 0) e.push(0);
                        for (var t = 0, r = 0; r < e.length; r += 4) t += (e[r] << 24) + (e[r + 1] << 16) + (e[r + 2] << 8) + e[r + 3];
                        return t %= Math.pow(2, 32), t
                    }

                    function xt(e, t, r, n) {
                        return new $.Record("Table Record", [{
                            name: "tag",
                            type: "TAG",
                            value: void 0 !== e ? e : ""
                        }, {
                            name: "checkSum",
                            type: "ULONG",
                            value: void 0 !== t ? t : 0
                        }, {
                            name: "offset",
                            type: "ULONG",
                            value: void 0 !== r ? r : 0
                        }, {
                            name: "length",
                            type: "ULONG",
                            value: void 0 !== n ? n : 0
                        }])
                    }

                    function Vt(e) {
                        var t = new $.Table("sfnt", [{
                            name: "version",
                            type: "TAG",
                            value: "OTTO"
                        }, {
                            name: "numTables",
                            type: "USHORT",
                            value: 0
                        }, {
                            name: "searchRange",
                            type: "USHORT",
                            value: 0
                        }, {
                            name: "entrySelector",
                            type: "USHORT",
                            value: 0
                        }, {
                            name: "rangeShift",
                            type: "USHORT",
                            value: 0
                        }]);
                        t.tables = e, t.numTables = e.length;
                        var r = Math.pow(2, gt(t.numTables));
                        t.searchRange = 16 * r, t.entrySelector = gt(r), t.rangeShift = 16 * t.numTables - t.searchRange;
                        var n = [],
                            h = [],
                            a = t.sizeOf() + xt().sizeOf() * t.numTables;
                        while (a % 4 !== 0) a += 1, h.push({
                            name: "padding",
                            type: "BYTE",
                            value: 0
                        });
                        for (var i = 0; i < e.length; i += 1) {
                            var c = e[i];
                            w.argument(4 === c.tableName.length, "Table name" + c.tableName + " is invalid.");
                            var o = c.sizeOf(),
                                s = xt(c.tableName, jt(c.encode()), a, o);
                            n.push({
                                name: s.tag + " Table Record",
                                type: "RECORD",
                                value: s
                            }), h.push({
                                name: c.tableName + " table",
                                type: "RECORD",
                                value: c
                            }), a += o, w.argument(!isNaN(a), "Something went wrong calculating the offset.");
                            while (a % 4 !== 0) a += 1, h.push({
                                name: "padding",
                                type: "BYTE",
                                value: 0
                            })
                        }
                        return n.sort((function(e, t) {
                            return e.value.tag > t.value.tag ? 1 : -1
                        })), t.fields = t.fields.concat(n), t.fields = t.fields.concat(h), t
                    }

                    function ft(e, t, r) {
                        for (var n = 0; n < t.length; n += 1) {
                            var h = e.charToGlyphIndex(t[n]);
                            if (h > 0) {
                                var a = e.glyphs.get(h);
                                return a.getMetrics()
                            }
                        }
                        return r
                    }

                    function Ft(e) {
                        for (var t = 0, r = 0; r < e.length; r += 1) t += e[r];
                        return t / e.length
                    }
                    var kt = {
                        make: Vt,
                        fontToTable: function(e) {
                            for (var t, r = [], n = [], h = [], a = [], i = [], c = [], o = [], s = 0, l = 0, p = 0, d = 0, b = 0, g = 0; g < e.glyphs.length; g += 1) {
                                var j = e.glyphs.get(g),
                                    x = 0 | j.unicode;
                                if (isNaN(j.advanceWidth)) throw new Error("Glyph " + j.name + " (" + g + "): advanceWidth is not a number.");
                                (t > x || void 0 === t) && x > 0 && (t = x), s < x && (s = x);
                                var V = ct.getUnicodeRange(x);
                                if (V < 32) l |= 1 << V;
                                else if (V < 64) p |= 1 << V - 32;
                                else if (V < 96) d |= 1 << V - 64;
                                else {
                                    if (!(V < 123)) throw new Error("Unicode ranges bits > 123 are reserved for internal usage");
                                    b |= 1 << V - 96
                                }
                                if (".notdef" !== j.name) {
                                    var f = j.getMetrics();
                                    r.push(f.xMin), n.push(f.yMin), h.push(f.xMax), a.push(f.yMax), c.push(f.leftSideBearing), o.push(f.rightSideBearing), i.push(j.advanceWidth)
                                }
                            }
                            var F = {
                                xMin: Math.min.apply(null, r),
                                yMin: Math.min.apply(null, n),
                                xMax: Math.max.apply(null, h),
                                yMax: Math.max.apply(null, a),
                                advanceWidthMax: Math.max.apply(null, i),
                                advanceWidthAvg: Ft(i),
                                minLeftSideBearing: Math.min.apply(null, c),
                                maxLeftSideBearing: Math.max.apply(null, c),
                                minRightSideBearing: Math.min.apply(null, o)
                            };
                            F.ascender = e.ascender, F.descender = e.descender;
                            var k = qe.make({
                                    flags: 3,
                                    unitsPerEm: e.unitsPerEm,
                                    xMin: F.xMin,
                                    yMin: F.yMin,
                                    xMax: F.xMax,
                                    yMax: F.yMax,
                                    lowestRecPPEM: 3,
                                    createdTimestamp: e.createdTimestamp
                                }),
                                m = Ge.make({
                                    ascender: F.ascender,
                                    descender: F.descender,
                                    advanceWidthMax: F.advanceWidthMax,
                                    minLeftSideBearing: F.minLeftSideBearing,
                                    minRightSideBearing: F.minRightSideBearing,
                                    xMaxExtent: F.maxLeftSideBearing + (F.xMax - F.xMin),
                                    numberOfHMetrics: e.glyphs.length
                                }),
                                P = _e.make(e.glyphs.length),
                                u = ct.make(Object.assign({
                                    xAvgCharWidth: Math.round(F.advanceWidthAvg),
                                    usFirstCharIndex: t,
                                    usLastCharIndex: s,
                                    ulUnicodeRange1: l,
                                    ulUnicodeRange2: p,
                                    ulUnicodeRange3: d,
                                    ulUnicodeRange4: b,
                                    sTypoAscender: F.ascender,
                                    sTypoDescender: F.descender,
                                    sTypoLineGap: 0,
                                    usWinAscent: F.yMax,
                                    usWinDescent: Math.abs(F.yMin),
                                    ulCodePageRange1: 1,
                                    sxHeight: ft(e, "xyvw", {
                                        yMax: Math.round(F.ascender / 2)
                                    }).yMax,
                                    sCapHeight: ft(e, "HIKLEFJMNTZBDPRAGOQSUVWXY", F).yMax,
                                    usDefaultChar: e.hasChar(" ") ? 32 : 0,
                                    usBreakChar: e.hasChar(" ") ? 32 : 0
                                }, e.tables.os2)),
                                X = Ee.make(e.glyphs),
                                N = se.make(e.glyphs),
                                H = e.getEnglishName("fontFamily"),
                                z = e.getEnglishName("fontSubfamily"),
                                Q = H + " " + z,
                                R = e.getEnglishName("postScriptName");
                            R || (R = H.replace(/\s/g, "") + "-" + z);
                            var v = {};
                            for (var I in e.names) v[I] = e.names[I];
                            v.uniqueID || (v.uniqueID = {
                                en: e.getEnglishName("manufacturer") + ":" + Q
                            }), v.postScriptName || (v.postScriptName = {
                                en: R
                            }), v.preferredFamily || (v.preferredFamily = e.names.fontFamily), v.preferredSubfamily || (v.preferredSubfamily = e.names.fontSubfamily);
                            var w = [],
                                y = at.make(v, w),
                                C = w.length > 0 ? We.make(w) : void 0,
                                A = ot.make(),
                                D = Me.make(e.glyphs, {
                                    version: e.getEnglishName("version"),
                                    fullName: Q,
                                    familyName: H,
                                    weightName: z,
                                    postScriptName: R,
                                    unitsPerEm: e.unitsPerEm,
                                    fontBBox: [0, F.yMin, F.ascender, F.advanceWidthMax]
                                }),
                                S = e.metas && Object.keys(e.metas).length > 0 ? bt.make(e.metas) : void 0,
                                Y = [k, m, P, u, y, N, A, D, X];
                            C && Y.push(C), e.tables.gsub && Y.push(dt.make(e.tables.gsub)), S && Y.push(S);
                            for (var B = Vt(Y), T = B.encode(), L = jt(T), M = B.fields, q = !1, G = 0; G < M.length; G += 1)
                                if ("head table" === M[G].name) {
                                    M[G].value.checkSumAdjustment = 2981146554 - L, q = !0;
                                    break
                                } if (!q) throw new Error("Could not find head table with checkSum to adjust.");
                            return B
                        },
                        computeCheckSum: jt
                    };

                    function mt(e, t) {
                        var r = 0,
                            n = e.length - 1;
                        while (r <= n) {
                            var h = r + n >>> 1,
                                a = e[h].tag;
                            if (a === t) return h;
                            a < t ? r = h + 1 : n = h - 1
                        }
                        return -r - 1
                    }

                    function Pt(e, t) {
                        var r = 0,
                            n = e.length - 1;
                        while (r <= n) {
                            var h = r + n >>> 1,
                                a = e[h];
                            if (a === t) return h;
                            a < t ? r = h + 1 : n = h - 1
                        }
                        return -r - 1
                    }

                    function ut(e, t) {
                        var r, n = 0,
                            h = e.length - 1;
                        while (n <= h) {
                            var a = n + h >>> 1;
                            r = e[a];
                            var i = r.start;
                            if (i === t) return r;
                            i < t ? n = a + 1 : h = a - 1
                        }
                        if (n > 0) return r = e[n - 1], t > r.end ? 0 : r
                    }

                    function Xt(e, t) {
                        this.font = e, this.tableName = t
                    }

                    function Nt(e) {
                        Xt.call(this, e, "gpos")
                    }

                    function Ht(e) {
                        Xt.call(this, e, "gsub")
                    }

                    function zt(e, t) {
                        var r = e.length;
                        if (r !== t.length) return !1;
                        for (var n = 0; n < r; n++)
                            if (e[n] !== t[n]) return !1;
                        return !0
                    }

                    function Qt(e, t, r) {
                        for (var n = e.subtables, h = 0; h < n.length; h++) {
                            var a = n[h];
                            if (a.substFormat === t) return a
                        }
                        if (r) return n.push(r), r
                    }

                    function Rt(e) {
                        for (var t = new ArrayBuffer(e.length), r = new Uint8Array(t), n = 0; n < e.length; ++n) r[n] = e[n];
                        return t
                    }

                    function vt(e, t) {
                        if (!e) throw t
                    }

                    function It(e, t, r, n, h) {
                        var a;
                        return (t & n) > 0 ? (a = e.parseByte(), 0 === (t & h) && (a = -a), a = r + a) : a = (t & h) > 0 ? r : r + e.parseShort(), a
                    }

                    function wt(e, t, r) {
                        var n, h, a = new ce.Parser(t, r);
                        if (e.numberOfContours = a.parseShort(), e._xMin = a.parseShort(), e._yMin = a.parseShort(), e._xMax = a.parseShort(), e._yMax = a.parseShort(), e.numberOfContours > 0) {
                            for (var i = e.endPointIndices = [], c = 0; c < e.numberOfContours; c += 1) i.push(a.parseUShort());
                            e.instructionLength = a.parseUShort(), e.instructions = [];
                            for (var o = 0; o < e.instructionLength; o += 1) e.instructions.push(a.parseByte());
                            var s = i[i.length - 1] + 1;
                            n = [];
                            for (var l = 0; l < s; l += 1)
                                if (h = a.parseByte(), n.push(h), (8 & h) > 0)
                                    for (var p = a.parseByte(), d = 0; d < p; d += 1) n.push(h), l += 1;
                            if (w.argument(n.length === s, "Bad flags."), i.length > 0) {
                                var b, g = [];
                                if (s > 0) {
                                    for (var j = 0; j < s; j += 1) h = n[j], b = {}, b.onCurve = !!(1 & h), b.lastPointOfContour = i.indexOf(j) >= 0, g.push(b);
                                    for (var x = 0, V = 0; V < s; V += 1) h = n[V], b = g[V], b.x = It(a, h, x, 2, 16), x = b.x;
                                    for (var f = 0, F = 0; F < s; F += 1) h = n[F], b = g[F], b.y = It(a, h, f, 4, 32), f = b.y
                                }
                                e.points = g
                            } else e.points = []
                        } else if (0 === e.numberOfContours) e.points = [];
                        else {
                            e.isComposite = !0, e.points = [], e.components = [];
                            var k = !0;
                            while (k) {
                                n = a.parseUShort();
                                var m = {
                                    glyphIndex: a.parseUShort(),
                                    xScale: 1,
                                    scale01: 0,
                                    scale10: 0,
                                    yScale: 1,
                                    dx: 0,
                                    dy: 0
                                };
                                (1 & n) > 0 ? (2 & n) > 0 ? (m.dx = a.parseShort(), m.dy = a.parseShort()) : m.matchedPoints = [a.parseUShort(), a.parseUShort()] : (2 & n) > 0 ? (m.dx = a.parseChar(), m.dy = a.parseChar()) : m.matchedPoints = [a.parseByte(), a.parseByte()], (8 & n) > 0 ? m.xScale = m.yScale = a.parseF2Dot14() : (64 & n) > 0 ? (m.xScale = a.parseF2Dot14(), m.yScale = a.parseF2Dot14()) : (128 & n) > 0 && (m.xScale = a.parseF2Dot14(), m.scale01 = a.parseF2Dot14(), m.scale10 = a.parseF2Dot14(), m.yScale = a.parseF2Dot14()), e.components.push(m), k = !!(32 & n)
                            }
                            if (256 & n) {
                                e.instructionLength = a.parseUShort(), e.instructions = [];
                                for (var P = 0; P < e.instructionLength; P += 1) e.instructions.push(a.parseByte())
                            }
                        }
                    }

                    function yt(e, t) {
                        for (var r = [], n = 0; n < e.length; n += 1) {
                            var h = e[n],
                                a = {
                                    x: t.xScale * h.x + t.scale01 * h.y + t.dx,
                                    y: t.scale10 * h.x + t.yScale * h.y + t.dy,
                                    onCurve: h.onCurve,
                                    lastPointOfContour: h.lastPointOfContour
                                };
                            r.push(a)
                        }
                        return r
                    }

                    function Ct(e) {
                        var t = new R;
                        if (!e) return t;
                        for (var r = function(e) {
                                for (var t = [], r = [], n = 0; n < e.length; n += 1) {
                                    var h = e[n];
                                    r.push(h), h.lastPointOfContour && (t.push(r), r = [])
                                }
                                return w.argument(0 === r.length, "There are still points left in the current contour."), t
                            }(e), n = 0; n < r.length; ++n) {
                            var h = r[n],
                                a = null,
                                i = h[h.length - 1],
                                c = h[0];
                            if (i.onCurve) t.moveTo(i.x, i.y);
                            else if (c.onCurve) t.moveTo(c.x, c.y);
                            else {
                                var o = {
                                    x: .5 * (i.x + c.x),
                                    y: .5 * (i.y + c.y)
                                };
                                t.moveTo(o.x, o.y)
                            }
                            for (var s = 0; s < h.length; ++s)
                                if (a = i, i = c, c = h[(s + 1) % h.length], i.onCurve) t.lineTo(i.x, i.y);
                                else {
                                    var l = c;
                                    a.onCurve || {
                                        x: .5 * (i.x + a.x),
                                        y: .5 * (i.y + a.y)
                                    }, c.onCurve || (l = {
                                        x: .5 * (i.x + c.x),
                                        y: .5 * (i.y + c.y)
                                    }), t.quadraticCurveTo(i.x, i.y, l.x, l.y)
                                } t.closePath()
                        }
                        return t
                    }

                    function At(e, t) {
                        if (t.isComposite)
                            for (var r = 0; r < t.components.length; r += 1) {
                                var n = t.components[r],
                                    h = e.get(n.glyphIndex);
                                if (h.getPath(), h.points) {
                                    var a = void 0;
                                    if (void 0 === n.matchedPoints) a = yt(h.points, n);
                                    else {
                                        if (n.matchedPoints[0] > t.points.length - 1 || n.matchedPoints[1] > h.points.length - 1) throw Error("Matched points out of range in " + t.name);
                                        var i = t.points[n.matchedPoints[0]],
                                            c = h.points[n.matchedPoints[1]],
                                            o = {
                                                xScale: n.xScale,
                                                scale01: n.scale01,
                                                scale10: n.scale10,
                                                yScale: n.yScale,
                                                dx: 0,
                                                dy: 0
                                            };
                                        c = yt([c], o)[0], o.dx = i.x - c.x, o.dy = i.y - c.y, a = yt(h.points, o)
                                    }
                                    t.points = t.points.concat(a)
                                }
                            }
                        return Ct(t.points)
                    }
                    Xt.prototype = {
                        searchTag: mt,
                        binSearch: Pt,
                        getTable: function(e) {
                            var t = this.font.tables[this.tableName];
                            return !t && e && (t = this.font.tables[this.tableName] = this.createDefaultTable()), t
                        },
                        getScriptNames: function() {
                            var e = this.getTable();
                            return e ? e.scripts.map((function(e) {
                                return e.tag
                            })) : []
                        },
                        getDefaultScriptName: function() {
                            var e = this.getTable();
                            if (e) {
                                for (var t = !1, r = 0; r < e.scripts.length; r++) {
                                    var n = e.scripts[r].tag;
                                    if ("DFLT" === n) return n;
                                    "latn" === n && (t = !0)
                                }
                                return t ? "latn" : void 0
                            }
                        },
                        getScriptTable: function(e, t) {
                            var r = this.getTable(t);
                            if (r) {
                                e = e || "DFLT";
                                var n = r.scripts,
                                    h = mt(r.scripts, e);
                                if (h >= 0) return n[h].script;
                                if (t) {
                                    var a = {
                                        tag: e,
                                        script: {
                                            defaultLangSys: {
                                                reserved: 0,
                                                reqFeatureIndex: 65535,
                                                featureIndexes: []
                                            },
                                            langSysRecords: []
                                        }
                                    };
                                    return n.splice(-1 - h, 0, a), a.script
                                }
                            }
                        },
                        getLangSysTable: function(e, t, r) {
                            var n = this.getScriptTable(e, r);
                            if (n) {
                                if (!t || "dflt" === t || "DFLT" === t) return n.defaultLangSys;
                                var h = mt(n.langSysRecords, t);
                                if (h >= 0) return n.langSysRecords[h].langSys;
                                if (r) {
                                    var a = {
                                        tag: t,
                                        langSys: {
                                            reserved: 0,
                                            reqFeatureIndex: 65535,
                                            featureIndexes: []
                                        }
                                    };
                                    return n.langSysRecords.splice(-1 - h, 0, a), a.langSys
                                }
                            }
                        },
                        getFeatureTable: function(e, t, r, n) {
                            var h = this.getLangSysTable(e, t, n);
                            if (h) {
                                for (var a, i = h.featureIndexes, c = this.font.tables[this.tableName].features, o = 0; o < i.length; o++)
                                    if (a = c[i[o]], a.tag === r) return a.feature;
                                if (n) {
                                    var s = c.length;
                                    return w.assert(0 === s || r >= c[s - 1].tag, "Features must be added in alphabetical order."), a = {
                                        tag: r,
                                        feature: {
                                            params: 0,
                                            lookupListIndexes: []
                                        }
                                    }, c.push(a), i.push(s), a.feature
                                }
                            }
                        },
                        getLookupTables: function(e, t, r, n, h) {
                            var a = this.getFeatureTable(e, t, r, h),
                                i = [];
                            if (a) {
                                for (var c, o = a.lookupListIndexes, s = this.font.tables[this.tableName].lookups, l = 0; l < o.length; l++) c = s[o[l]], c.lookupType === n && i.push(c);
                                if (0 === i.length && h) {
                                    c = {
                                        lookupType: n,
                                        lookupFlag: 0,
                                        subtables: [],
                                        markFilteringSet: void 0
                                    };
                                    var p = s.length;
                                    return s.push(c), o.push(p), [c]
                                }
                            }
                            return i
                        },
                        getGlyphClass: function(e, t) {
                            switch (e.format) {
                                case 1:
                                    return e.startGlyph <= t && t < e.startGlyph + e.classes.length ? e.classes[t - e.startGlyph] : 0;
                                case 2:
                                    var r = ut(e.ranges, t);
                                    return r ? r.classId : 0
                            }
                        },
                        getCoverageIndex: function(e, t) {
                            switch (e.format) {
                                case 1:
                                    var r = Pt(e.glyphs, t);
                                    return r >= 0 ? r : -1;
                                case 2:
                                    var n = ut(e.ranges, t);
                                    return n ? n.index + t - n.start : -1
                            }
                        },
                        expandCoverage: function(e) {
                            if (1 === e.format) return e.glyphs;
                            for (var t = [], r = e.ranges, n = 0; n < r.length; n++)
                                for (var h = r[n], a = h.start, i = h.end, c = a; c <= i; c++) t.push(c);
                            return t
                        }
                    }, Nt.prototype = Xt.prototype, Nt.prototype.init = function() {
                        var e = this.getDefaultScriptName();
                        this.defaultKerningTables = this.getKerningTables(e)
                    }, Nt.prototype.getKerningValue = function(e, t, r) {
                        for (var n = 0; n < e.length; n++)
                            for (var h = e[n].subtables, a = 0; a < h.length; a++) {
                                var i = h[a],
                                    c = this.getCoverageIndex(i.coverage, t);
                                if (!(c < 0)) switch (i.posFormat) {
                                    case 1:
                                        for (var o = i.pairSets[c], s = 0; s < o.length; s++) {
                                            var l = o[s];
                                            if (l.secondGlyph === r) return l.value1 && l.value1.xAdvance || 0
                                        }
                                        break;
                                    case 2:
                                        var p = this.getGlyphClass(i.classDef1, t),
                                            d = this.getGlyphClass(i.classDef2, r),
                                            b = i.classRecords[p][d];
                                        return b.value1 && b.value1.xAdvance || 0
                                }
                            }
                        return 0
                    }, Nt.prototype.getKerningTables = function(e, t) {
                        if (this.font.tables.gpos) return this.getLookupTables(e, t, "kern", 2)
                    }, Ht.prototype = Xt.prototype, Ht.prototype.createDefaultTable = function() {
                        return {
                            version: 1,
                            scripts: [{
                                tag: "DFLT",
                                script: {
                                    defaultLangSys: {
                                        reserved: 0,
                                        reqFeatureIndex: 65535,
                                        featureIndexes: []
                                    },
                                    langSysRecords: []
                                }
                            }],
                            features: [],
                            lookups: []
                        }
                    }, Ht.prototype.getSingle = function(e, t, r) {
                        for (var n = [], h = this.getLookupTables(t, r, e, 1), a = 0; a < h.length; a++)
                            for (var i = h[a].subtables, c = 0; c < i.length; c++) {
                                var o = i[c],
                                    s = this.expandCoverage(o.coverage),
                                    l = void 0;
                                if (1 === o.substFormat) {
                                    var p = o.deltaGlyphId;
                                    for (l = 0; l < s.length; l++) {
                                        var d = s[l];
                                        n.push({
                                            sub: d,
                                            by: d + p
                                        })
                                    }
                                } else {
                                    var b = o.substitute;
                                    for (l = 0; l < s.length; l++) n.push({
                                        sub: s[l],
                                        by: b[l]
                                    })
                                }
                            }
                        return n
                    }, Ht.prototype.getMultiple = function(e, t, r) {
                        for (var n = [], h = this.getLookupTables(t, r, e, 2), a = 0; a < h.length; a++)
                            for (var i = h[a].subtables, c = 0; c < i.length; c++) {
                                var o = i[c],
                                    s = this.expandCoverage(o.coverage),
                                    l = void 0;
                                for (l = 0; l < s.length; l++) {
                                    var p = s[l],
                                        d = o.sequences[l];
                                    n.push({
                                        sub: p,
                                        by: d
                                    })
                                }
                            }
                        return n
                    }, Ht.prototype.getAlternates = function(e, t, r) {
                        for (var n = [], h = this.getLookupTables(t, r, e, 3), a = 0; a < h.length; a++)
                            for (var i = h[a].subtables, c = 0; c < i.length; c++)
                                for (var o = i[c], s = this.expandCoverage(o.coverage), l = o.alternateSets, p = 0; p < s.length; p++) n.push({
                                    sub: s[p],
                                    by: l[p]
                                });
                        return n
                    }, Ht.prototype.getLigatures = function(e, t, r) {
                        for (var n = [], h = this.getLookupTables(t, r, e, 4), a = 0; a < h.length; a++)
                            for (var i = h[a].subtables, c = 0; c < i.length; c++)
                                for (var o = i[c], s = this.expandCoverage(o.coverage), l = o.ligatureSets, p = 0; p < s.length; p++)
                                    for (var d = s[p], b = l[p], g = 0; g < b.length; g++) {
                                        var j = b[g];
                                        n.push({
                                            sub: [d].concat(j.components),
                                            by: j.ligGlyph
                                        })
                                    }
                        return n
                    }, Ht.prototype.addSingle = function(e, t, r, n) {
                        var h = this.getLookupTables(r, n, e, 1, !0)[0],
                            a = Qt(h, 2, {
                                substFormat: 2,
                                coverage: {
                                    format: 1,
                                    glyphs: []
                                },
                                substitute: []
                            });
                        w.assert(1 === a.coverage.format, "Single: unable to modify coverage table format " + a.coverage.format);
                        var i = t.sub,
                            c = this.binSearch(a.coverage.glyphs, i);
                        c < 0 && (c = -1 - c, a.coverage.glyphs.splice(c, 0, i), a.substitute.splice(c, 0, 0)), a.substitute[c] = t.by
                    }, Ht.prototype.addMultiple = function(e, t, r, n) {
                        w.assert(t.by instanceof Array && t.by.length > 1, 'Multiple: "by" must be an array of two or more ids');
                        var h = this.getLookupTables(r, n, e, 2, !0)[0],
                            a = Qt(h, 1, {
                                substFormat: 1,
                                coverage: {
                                    format: 1,
                                    glyphs: []
                                },
                                sequences: []
                            });
                        w.assert(1 === a.coverage.format, "Multiple: unable to modify coverage table format " + a.coverage.format);
                        var i = t.sub,
                            c = this.binSearch(a.coverage.glyphs, i);
                        c < 0 && (c = -1 - c, a.coverage.glyphs.splice(c, 0, i), a.sequences.splice(c, 0, 0)), a.sequences[c] = t.by
                    }, Ht.prototype.addAlternate = function(e, t, r, n) {
                        var h = this.getLookupTables(r, n, e, 3, !0)[0],
                            a = Qt(h, 1, {
                                substFormat: 1,
                                coverage: {
                                    format: 1,
                                    glyphs: []
                                },
                                alternateSets: []
                            });
                        w.assert(1 === a.coverage.format, "Alternate: unable to modify coverage table format " + a.coverage.format);
                        var i = t.sub,
                            c = this.binSearch(a.coverage.glyphs, i);
                        c < 0 && (c = -1 - c, a.coverage.glyphs.splice(c, 0, i), a.alternateSets.splice(c, 0, 0)), a.alternateSets[c] = t.by
                    }, Ht.prototype.addLigature = function(e, t, r, n) {
                        var h = this.getLookupTables(r, n, e, 4, !0)[0],
                            a = h.subtables[0];
                        a || (a = {
                            substFormat: 1,
                            coverage: {
                                format: 1,
                                glyphs: []
                            },
                            ligatureSets: []
                        }, h.subtables[0] = a), w.assert(1 === a.coverage.format, "Ligature: unable to modify coverage table format " + a.coverage.format);
                        var i = t.sub[0],
                            c = t.sub.slice(1),
                            o = {
                                ligGlyph: t.by,
                                components: c
                            },
                            s = this.binSearch(a.coverage.glyphs, i);
                        if (s >= 0) {
                            for (var l = a.ligatureSets[s], p = 0; p < l.length; p++)
                                if (zt(l[p].components, c)) return;
                            l.push(o)
                        } else s = -1 - s, a.coverage.glyphs.splice(s, 0, i), a.ligatureSets.splice(s, 0, [o])
                    }, Ht.prototype.getFeature = function(e, t, r) {
                        if (/ss\d\d/.test(e)) return this.getSingle(e, t, r);
                        switch (e) {
                            case "aalt":
                            case "salt":
                                return this.getSingle(e, t, r).concat(this.getAlternates(e, t, r));
                            case "dlig":
                            case "liga":
                            case "rlig":
                                return this.getLigatures(e, t, r);
                            case "ccmp":
                                return this.getMultiple(e, t, r).concat(this.getLigatures(e, t, r));
                            case "stch":
                                return this.getMultiple(e, t, r)
                        }
                    }, Ht.prototype.add = function(e, t, r, n) {
                        if (/ss\d\d/.test(e)) return this.addSingle(e, t, r, n);
                        switch (e) {
                            case "aalt":
                            case "salt":
                                return "number" === typeof t.by ? this.addSingle(e, t, r, n) : this.addAlternate(e, t, r, n);
                            case "dlig":
                            case "liga":
                            case "rlig":
                                return this.addLigature(e, t, r, n);
                            case "ccmp":
                                return t.by instanceof Array ? this.addMultiple(e, t, r, n) : this.addLigature(e, t, r, n)
                        }
                    };
                    var Dt, St, Yt, Bt, Tt = {
                        getPath: Ct,
                        parse: function(e, t, r, n, h) {
                            return h.lowMemory ? function(e, t, r, n) {
                                var h = new ue.GlyphSet(n);
                                return n._push = function(a) {
                                    var i = r[a],
                                        c = r[a + 1];
                                    i !== c ? h.push(a, ue.ttfGlyphLoader(n, a, wt, e, t + i, At)) : h.push(a, ue.glyphLoader(n, a))
                                }, h
                            }(e, t, r, n) : function(e, t, r, n) {
                                for (var h = new ue.GlyphSet(n), a = 0; a < r.length - 1; a += 1) {
                                    var i = r[a],
                                        c = r[a + 1];
                                    i !== c ? h.push(a, ue.ttfGlyphLoader(n, a, wt, e, t + i, At)) : h.push(a, ue.glyphLoader(n, a))
                                }
                                return h
                            }(e, t, r, n)
                        }
                    };

                    function Lt(e) {
                        this.font = e, this.getCommands = function(e) {
                            return Tt.getPath(e).commands
                        }, this._fpgmState = this._prepState = void 0, this._errorState = 0
                    }

                    function Mt(e) {
                        return e
                    }

                    function qt(e) {
                        return Math.sign(e) * Math.round(Math.abs(e))
                    }

                    function Gt(e) {
                        return Math.sign(e) * Math.round(Math.abs(2 * e)) / 2
                    }

                    function Et(e) {
                        return Math.sign(e) * (Math.round(Math.abs(e) + .5) - .5)
                    }

                    function Wt(e) {
                        return Math.sign(e) * Math.ceil(Math.abs(e))
                    }

                    function _t(e) {
                        return Math.sign(e) * Math.floor(Math.abs(e))
                    }
                    var Jt = function(e) {
                            var t = this.srPeriod,
                                r = this.srPhase,
                                n = this.srThreshold,
                                h = 1;
                            return e < 0 && (e = -e, h = -1), e += n - r, e = Math.trunc(e / t) * t, e += r, e < 0 ? r * h : e * h
                        },
                        Ut = {
                            x: 1,
                            y: 0,
                            axis: "x",
                            distance: function(e, t, r, n) {
                                return (r ? e.xo : e.x) - (n ? t.xo : t.x)
                            },
                            interpolate: function(e, t, r, n) {
                                var h, a, i, c, o, s, l;
                                if (!n || n === this) return h = e.xo - t.xo, a = e.xo - r.xo, o = t.x - t.xo, s = r.x - r.xo, i = Math.abs(h), c = Math.abs(a), l = i + c, 0 === l ? void(e.x = e.xo + (o + s) / 2) : void(e.x = e.xo + (o * c + s * i) / l);
                                h = n.distance(e, t, !0, !0), a = n.distance(e, r, !0, !0), o = n.distance(t, t, !1, !0), s = n.distance(r, r, !1, !0), i = Math.abs(h), c = Math.abs(a), l = i + c, 0 !== l ? Ut.setRelative(e, e, (o * c + s * i) / l, n, !0) : Ut.setRelative(e, e, (o + s) / 2, n, !0)
                            },
                            normalSlope: Number.NEGATIVE_INFINITY,
                            setRelative: function(e, t, r, n, h) {
                                if (n && n !== this) {
                                    var a = h ? t.xo : t.x,
                                        i = h ? t.yo : t.y,
                                        c = a + r * n.x,
                                        o = i + r * n.y;
                                    e.x = c + (e.y - o) / n.normalSlope
                                } else e.x = (h ? t.xo : t.x) + r
                            },
                            slope: 0,
                            touch: function(e) {
                                e.xTouched = !0
                            },
                            touched: function(e) {
                                return e.xTouched
                            },
                            untouch: function(e) {
                                e.xTouched = !1
                            }
                        },
                        Ot = {
                            x: 0,
                            y: 1,
                            axis: "y",
                            distance: function(e, t, r, n) {
                                return (r ? e.yo : e.y) - (n ? t.yo : t.y)
                            },
                            interpolate: function(e, t, r, n) {
                                var h, a, i, c, o, s, l;
                                if (!n || n === this) return h = e.yo - t.yo, a = e.yo - r.yo, o = t.y - t.yo, s = r.y - r.yo, i = Math.abs(h), c = Math.abs(a), l = i + c, 0 === l ? void(e.y = e.yo + (o + s) / 2) : void(e.y = e.yo + (o * c + s * i) / l);
                                h = n.distance(e, t, !0, !0), a = n.distance(e, r, !0, !0), o = n.distance(t, t, !1, !0), s = n.distance(r, r, !1, !0), i = Math.abs(h), c = Math.abs(a), l = i + c, 0 !== l ? Ot.setRelative(e, e, (o * c + s * i) / l, n, !0) : Ot.setRelative(e, e, (o + s) / 2, n, !0)
                            },
                            normalSlope: 0,
                            setRelative: function(e, t, r, n, h) {
                                if (n && n !== this) {
                                    var a = h ? t.xo : t.x,
                                        i = h ? t.yo : t.y,
                                        c = a + r * n.x,
                                        o = i + r * n.y;
                                    e.y = o + n.normalSlope * (e.x - c)
                                } else e.y = (h ? t.yo : t.y) + r
                            },
                            slope: Number.POSITIVE_INFINITY,
                            touch: function(e) {
                                e.yTouched = !0
                            },
                            touched: function(e) {
                                return e.yTouched
                            },
                            untouch: function(e) {
                                e.yTouched = !1
                            }
                        };

                    function Zt(e, t) {
                        this.x = e, this.y = t, this.axis = void 0, this.slope = t / e, this.normalSlope = -e / t, Object.freeze(this)
                    }

                    function Kt(e, t) {
                        var r = Math.sqrt(e * e + t * t);
                        return e /= r, t /= r, 1 === e && 0 === t ? Ut : 0 === e && 1 === t ? Ot : new Zt(e, t)
                    }

                    function $t(e, t, r, n) {
                        this.x = this.xo = Math.round(64 * e) / 64, this.y = this.yo = Math.round(64 * t) / 64, this.lastPointOfContour = r, this.onCurve = n, this.prevPointOnContour = void 0, this.nextPointOnContour = void 0, this.xTouched = !1, this.yTouched = !1, Object.preventExtensions(this)
                    }
                    Object.freeze(Ut), Object.freeze(Ot), Zt.prototype.distance = function(e, t, r, n) {
                        return this.x * Ut.distance(e, t, r, n) + this.y * Ot.distance(e, t, r, n)
                    }, Zt.prototype.interpolate = function(e, t, r, n) {
                        var h, a, i, c, o, s, l;
                        i = n.distance(e, t, !0, !0), c = n.distance(e, r, !0, !0), h = n.distance(t, t, !1, !0), a = n.distance(r, r, !1, !0), o = Math.abs(i), s = Math.abs(c), l = o + s, 0 !== l ? this.setRelative(e, e, (h * s + a * o) / l, n, !0) : this.setRelative(e, e, (h + a) / 2, n, !0)
                    }, Zt.prototype.setRelative = function(e, t, r, n, h) {
                        n = n || this;
                        var a = h ? t.xo : t.x,
                            i = h ? t.yo : t.y,
                            c = a + r * n.x,
                            o = i + r * n.y,
                            s = n.normalSlope,
                            l = this.slope,
                            p = e.x,
                            d = e.y;
                        e.x = (l * p - s * c + o - d) / (l - s), e.y = l * (e.x - p) + d
                    }, Zt.prototype.touch = function(e) {
                        e.xTouched = !0, e.yTouched = !0
                    }, $t.prototype.nextTouched = function(e) {
                        var t = this.nextPointOnContour;
                        while (!e.touched(t) && t !== this) t = t.nextPointOnContour;
                        return t
                    }, $t.prototype.prevTouched = function(e) {
                        var t = this.prevPointOnContour;
                        while (!e.touched(t) && t !== this) t = t.prevPointOnContour;
                        return t
                    };
                    var er = Object.freeze(new $t(0, 0)),
                        tr = {
                            cvCutIn: 17 / 16,
                            deltaBase: 9,
                            deltaShift: .125,
                            loop: 1,
                            minDis: 1,
                            autoFlip: !0
                        };

                    function rr(e, t) {
                        switch (this.env = e, this.stack = [], this.prog = t, e) {
                            case "glyf":
                                this.zp0 = this.zp1 = this.zp2 = 1, this.rp0 = this.rp1 = this.rp2 = 0;
                            case "prep":
                                this.fv = this.pv = this.dpv = Ut, this.round = qt
                        }
                    }

                    function nr(e) {
                        for (var t = e.tZone = new Array(e.gZone.length), r = 0; r < t.length; r++) t[r] = new $t(0, 0)
                    }

                    function hr(e, t) {
                        var r, n = e.prog,
                            h = e.ip,
                            a = 1;
                        do {
                            if (r = n[++h], 88 === r) a++;
                            else if (89 === r) a--;
                            else if (64 === r) h += n[h + 1] + 1;
                            else if (65 === r) h += 2 * n[h + 1] + 1;
                            else if (r >= 176 && r <= 183) h += r - 176 + 1;
                            else if (r >= 184 && r <= 191) h += 2 * (r - 184 + 1);
                            else if (t && 1 === a && 27 === r) break
                        } while (a > 0);
                        e.ip = h
                    }

                    function ar(e, r) {
                        t.DEBUG && h("log", r.step, "SVTCA[" + e.axis + "]", " at utils/opentype.js:9338"), r.fv = r.pv = r.dpv = e
                    }

                    function ir(e, r) {
                        t.DEBUG && h("log", r.step, "SPVTCA[" + e.axis + "]", " at utils/opentype.js:9346"), r.pv = r.dpv = e
                    }

                    function cr(e, r) {
                        t.DEBUG && h("log", r.step, "SFVTCA[" + e.axis + "]", " at utils/opentype.js:9354"), r.fv = e
                    }

                    function or(e, r) {
                        var n, a, i = r.stack,
                            c = i.pop(),
                            o = i.pop(),
                            s = r.z2[c],
                            l = r.z1[o];
                        t.DEBUG && h("log", "SPVTL[" + e + "]", c, o, " at utils/opentype.js:9368"), e ? (n = s.y - l.y, a = l.x - s.x) : (n = l.x - s.x, a = l.y - s.y), r.pv = r.dpv = Kt(n, a)
                    }

                    function sr(e, r) {
                        var n, a, i = r.stack,
                            c = i.pop(),
                            o = i.pop(),
                            s = r.z2[c],
                            l = r.z1[o];
                        t.DEBUG && h("log", "SFVTL[" + e + "]", c, o, " at utils/opentype.js:9393"), e ? (n = s.y - l.y, a = l.x - s.x) : (n = l.x - s.x, a = l.y - s.y), r.fv = Kt(n, a)
                    }

                    function lr(e) {
                        t.DEBUG && h("log", e.step, "POP[]", " at utils/opentype.js:9699"), e.stack.pop()
                    }

                    function pr(e, r) {
                        var n = r.stack.pop(),
                            a = r.z0[n],
                            i = r.fv,
                            c = r.pv;
                        t.DEBUG && h("log", r.step, "MDAP[" + e + "]", n, " at utils/opentype.js:9841");
                        var o = c.distance(a, er);
                        e && (o = r.round(o)), i.setRelative(a, er, o, c), i.touch(a), r.rp0 = r.rp1 = n
                    }

                    function dr(e, r) {
                        var n, a, i, c = r.z2,
                            o = c.length - 2;
                        t.DEBUG && h("log", r.step, "IUP[" + e.axis + "]", " at utils/opentype.js:9862");
                        for (var s = 0; s < o; s++) n = c[s], e.touched(n) || (a = n.prevTouched(e), a !== n && (i = n.nextTouched(e), a === i && e.setRelative(n, n, e.distance(a, a, !1, !0), e, !0), e.interpolate(n, a, i, e)))
                    }

                    function br(e, r) {
                        var n = r.stack,
                            a = e ? r.rp1 : r.rp2,
                            i = (e ? r.z0 : r.z1)[a],
                            c = r.fv,
                            o = r.pv,
                            s = r.loop,
                            l = r.z2;
                        while (s--) {
                            var p = n.pop(),
                                d = l[p],
                                b = o.distance(i, i, !1, !0);
                            c.setRelative(d, d, b, o), c.touch(d), t.DEBUG && h("log", r.step, (r.loop > 1 ? "loop " + (r.loop - s) + ": " : "") + "SHP[" + (e ? "rp1" : "rp2") + "]", p, " at utils/opentype.js:9909")
                        }
                        r.loop = 1
                    }

                    function gr(e, r) {
                        var n = r.stack,
                            a = e ? r.rp1 : r.rp2,
                            i = (e ? r.z0 : r.z1)[a],
                            c = r.fv,
                            o = r.pv,
                            s = n.pop(),
                            l = r.z2[r.contours[s]],
                            p = l;
                        t.DEBUG && h("log", r.step, "SHC[" + e + "]", s, " at utils/opentype.js:9935");
                        var d = o.distance(i, i, !1, !0);
                        do {
                            p !== i && c.setRelative(p, p, d, o), p = p.nextPointOnContour
                        } while (p !== l)
                    }

                    function jr(e, r) {
                        var n, a, i = r.stack,
                            c = e ? r.rp1 : r.rp2,
                            o = (e ? r.z0 : r.z1)[c],
                            s = r.fv,
                            l = r.pv,
                            p = i.pop();
                        switch (t.DEBUG && h("log", r.step, "SHZ[" + e + "]", p, " at utils/opentype.js:9956"), p) {
                            case 0:
                                n = r.tZone;
                                break;
                            case 1:
                                n = r.gZone;
                                break;
                            default:
                                throw new Error("Invalid zone")
                        }
                        for (var d = l.distance(o, o, !1, !0), b = n.length - 2, g = 0; g < b; g++) a = n[g], s.setRelative(a, a, d, l)
                    }

                    function xr(e, r) {
                        var n = r.stack,
                            a = n.pop() / 64,
                            i = n.pop(),
                            c = r.z1[i],
                            o = r.z0[r.rp0],
                            s = r.fv,
                            l = r.pv;
                        s.setRelative(c, o, a, l), s.touch(c), t.DEBUG && h("log", r.step, "MSIRP[" + e + "]", a, i, " at utils/opentype.js:10051"), r.rp1 = r.rp0, r.rp2 = i, e && (r.rp0 = i)
                    }

                    function Vr(e, r) {
                        var n = r.stack,
                            a = n.pop(),
                            i = n.pop(),
                            c = r.z0[i],
                            o = r.fv,
                            s = r.pv,
                            l = r.cvt[a];
                        t.DEBUG && h("log", r.step, "MIAP[" + e + "]", a, "(", l, ")", i, " at utils/opentype.js:10108");
                        var p = s.distance(c, er);
                        e && (Math.abs(p - l) < r.cvCutIn && (p = l), p = r.round(p)), o.setRelative(c, er, p, s), 0 === r.zp0 && (c.xo = c.x, c.yo = c.y), o.touch(c), r.rp0 = r.rp1 = i
                    }

                    function fr(e, r) {
                        var n = r.stack,
                            a = n.pop(),
                            i = r.z2[a];
                        t.DEBUG && h("log", r.step, "GC[" + e + "]", a, " at utils/opentype.js:10232"), n.push(64 * r.dpv.distance(i, er, e, !1))
                    }

                    function Fr(e, r) {
                        var n = r.stack,
                            a = n.pop(),
                            i = n.pop(),
                            c = r.z1[a],
                            o = r.z0[i],
                            s = r.dpv.distance(o, c, e, e);
                        t.DEBUG && h("log", r.step, "MD[" + e + "]", a, i, "->", s, " at utils/opentype.js:10247"), r.stack.push(Math.round(64 * s))
                    }

                    function kr(e, r) {
                        var n = r.stack,
                            a = n.pop(),
                            i = r.fv,
                            c = r.pv,
                            o = r.ppem,
                            s = r.deltaBase + 16 * (e - 1),
                            l = r.deltaShift,
                            p = r.z0;
                        t.DEBUG && h("log", r.step, "DELTAP[" + e + "]", a, n, " at utils/opentype.js:10435");
                        for (var d = 0; d < a; d++) {
                            var b = n.pop(),
                                g = n.pop(),
                                j = s + ((240 & g) >> 4);
                            if (j === o) {
                                var x = (15 & g) - 8;
                                x >= 0 && x++, t.DEBUG && h("log", r.step, "DELTAPFIX", b, "by", x * l, " at utils/opentype.js:10445");
                                var V = p[b];
                                i.setRelative(V, V, x * l, c)
                            }
                        }
                    }

                    function mr(e, r) {
                        var n = r.stack,
                            a = n.pop();
                        t.DEBUG && h("log", r.step, "ROUND[]", " at utils/opentype.js:10572"), n.push(64 * r.round(a / 64))
                    }

                    function Pr(e, r) {
                        var n = r.stack,
                            a = n.pop(),
                            i = r.ppem,
                            c = r.deltaBase + 16 * (e - 1),
                            o = r.deltaShift;
                        t.DEBUG && h("log", r.step, "DELTAC[" + e + "]", a, n, " at utils/opentype.js:10600");
                        for (var s = 0; s < a; s++) {
                            var l = n.pop(),
                                p = n.pop(),
                                d = c + ((240 & p) >> 4);
                            if (d === i) {
                                var b = (15 & p) - 8;
                                b >= 0 && b++;
                                var g = b * o;
                                t.DEBUG && h("log", r.step, "DELTACFIX", l, "by", g, " at utils/opentype.js:10613"), r.cvt[l] += g
                            }
                        }
                    }

                    function ur(e, r) {
                        var n, a, i = r.stack,
                            c = i.pop(),
                            o = i.pop(),
                            s = r.z2[c],
                            l = r.z1[o];
                        t.DEBUG && h("log", r.step, "SDPVTL[" + e + "]", c, o, " at utils/opentype.js:10761"), e ? (n = s.y - l.y, a = l.x - s.x) : (n = l.x - s.x, a = l.y - s.y), r.dpv = Kt(n, a)
                    }

                    function Xr(e, r) {
                        var n = r.stack,
                            a = r.prog,
                            i = r.ip;
                        t.DEBUG && h("log", r.step, "PUSHB[" + e + "]", " at utils/opentype.js:10867");
                        for (var c = 0; c < e; c++) n.push(a[++i]);
                        r.ip = i
                    }

                    function Nr(e, r) {
                        var n = r.ip,
                            a = r.prog,
                            i = r.stack;
                        t.DEBUG && h("log", r.ip, "PUSHW[" + e + "]", " at utils/opentype.js:10881");
                        for (var c = 0; c < e; c++) {
                            var o = a[++n] << 8 | a[++n];
                            32768 & o && (o = -(1 + (65535 ^ o))), i.push(o)
                        }
                        r.ip = n
                    }

                    function Hr(e, r, n, a, i, c) {
                        var o, s, l, p, d = c.stack,
                            b = e && d.pop(),
                            g = d.pop(),
                            j = c.rp0,
                            x = c.z0[j],
                            V = c.z1[g],
                            f = c.minDis,
                            F = c.fv,
                            k = c.dpv;
                        s = o = k.distance(V, x, !0, !0), l = s >= 0 ? 1 : -1, s = Math.abs(s), e && (p = c.cvt[b], a && Math.abs(s - p) < c.cvCutIn && (s = p)), n && s < f && (s = f), a && (s = c.round(s)), F.setRelative(V, x, l * s, k), F.touch(V), t.DEBUG && h("log", c.step, (e ? "MIRP[" : "MDRP[") + (r ? "M" : "m") + (n ? ">" : "_") + (a ? "R" : "_") + (0 === i ? "Gr" : 1 === i ? "Bl" : 2 === i ? "Wh" : "") + "]", e ? b + "(" + c.cvt[b] + "," + p + ")" : "", g, "(d =", o, "->", l * s, ")", " at utils/opentype.js:10938"), c.rp1 = c.rp0, c.rp2 = g, r && (c.rp0 = g)
                    }

                    function zr(e) {
                        this.char = e, this.state = {}, this.activeState = null
                    }

                    function Qr(e, t, r) {
                        this.contextName = r, this.startIndex = e, this.endOffset = t
                    }

                    function Rr(e, t, r) {
                        this.contextName = e, this.openRange = null, this.ranges = [], this.checkStart = t, this.checkEnd = r
                    }

                    function vr(e, t) {
                        this.context = e, this.index = t, this.length = e.length, this.current = e[t], this.backtrack = e.slice(0, t), this.lookahead = e.slice(t + 1)
                    }

                    function Ir(e) {
                        this.eventId = e, this.subscribers = []
                    }

                    function wr(e) {
                        var t = this,
                            r = ["start", "end", "next", "newToken", "contextStart", "contextEnd", "insertToken", "removeToken", "removeRange", "replaceToken", "replaceRange", "composeRUD", "updateContextsRanges"];
                        r.forEach((function(e) {
                            Object.defineProperty(t.events, e, {
                                value: new Ir(e)
                            })
                        })), e && r.forEach((function(r) {
                            var n = e[r];
                            "function" === typeof n && t.events[r].subscribe(n)
                        }));
                        ["insertToken", "removeToken", "removeRange", "replaceToken", "replaceRange", "composeRUD"].forEach((function(e) {
                            t.events[e].subscribe(t.updateContextsRanges)
                        }))
                    }

                    function yr(e) {
                        this.tokens = [], this.registeredContexts = {}, this.contextCheckers = [], this.events = {}, this.registeredModifiers = [], wr.call(this, e)
                    }

                    function Cr(e) {
                        return /[\u0600-\u065F\u066A-\u06D2\u06FA-\u06FF]/.test(e)
                    }

                    function Ar(e) {
                        return /[\u0630\u0690\u0621\u0631\u0661\u0671\u0622\u0632\u0672\u0692\u06C2\u0623\u0673\u0693\u06C3\u0624\u0694\u06C4\u0625\u0675\u0695\u06C5\u06E5\u0676\u0696\u06C6\u0627\u0677\u0697\u06C7\u0648\u0688\u0698\u06C8\u0689\u0699\u06C9\u068A\u06CA\u066B\u068B\u06CB\u068C\u068D\u06CD\u06FD\u068E\u06EE\u06FE\u062F\u068F\u06CF\u06EF]/.test(e)
                    }

                    function Dr(e) {
                        return /[\u0600-\u0605\u060C-\u060E\u0610-\u061B\u061E\u064B-\u065F\u0670\u06D6-\u06DC\u06DF-\u06E4\u06E7\u06E8\u06EA-\u06ED]/.test(e)
                    }

                    function Sr(e) {
                        return /[A-z]/.test(e)
                    }

                    function Yr(e) {
                        this.font = e, this.features = {}
                    }

                    function Br(e) {
                        this.id = e.id, this.tag = e.tag, this.substitution = e.substitution
                    }

                    function Tr(e, t) {
                        if (!e) return -1;
                        switch (t.format) {
                            case 1:
                                return t.glyphs.indexOf(e);
                            case 2:
                                for (var r = t.ranges, n = 0; n < r.length; n++) {
                                    var h = r[n];
                                    if (e >= h.start && e <= h.end) {
                                        var a = e - h.start;
                                        return h.index + a
                                    }
                                }
                                break;
                            default:
                                return -1
                        }
                        return -1
                    }

                    function Lr(e, t) {
                        var r = Tr(e, t.coverage);
                        return -1 === r ? null : e + t.deltaGlyphId
                    }

                    function Mr(e, t) {
                        var r = Tr(e, t.coverage);
                        return -1 === r ? null : t.substitute[r]
                    }

                    function qr(e, t) {
                        for (var r = [], n = 0; n < e.length; n++) {
                            var h = e[n],
                                a = t.current;
                            a = Array.isArray(a) ? a[0] : a;
                            var i = Tr(a, h); - 1 !== i && r.push(i)
                        }
                        return r.length !== e.length ? -1 : r
                    }

                    function Gr(e, t) {
                        var r = t.inputCoverage.length + t.lookaheadCoverage.length + t.backtrackCoverage.length;
                        if (e.context.length < r) return [];
                        var n = qr(t.inputCoverage, e);
                        if (-1 === n) return [];
                        var h = t.inputCoverage.length - 1;
                        if (e.lookahead.length < t.lookaheadCoverage.length) return [];
                        var a = e.lookahead.slice(h);
                        while (a.length && Dr(a[0].char)) a.shift();
                        var i = new vr(a, 0),
                            c = qr(t.lookaheadCoverage, i),
                            o = [].concat(e.backtrack);
                        o.reverse();
                        while (o.length && Dr(o[0].char)) o.shift();
                        if (o.length < t.backtrackCoverage.length) return [];
                        var s = new vr(o, 0),
                            l = qr(t.backtrackCoverage, s),
                            p = n.length === t.inputCoverage.length && c.length === t.lookaheadCoverage.length && l.length === t.backtrackCoverage.length,
                            d = [];
                        if (p)
                            for (var b = 0; b < t.lookupRecords.length; b++)
                                for (var g = t.lookupRecords[b], j = g.lookupListIndex, x = this.getLookupByIndex(j), V = 0; V < x.subtables.length; V++) {
                                    var f = x.subtables[V],
                                        F = this.getLookupMethod(x, f),
                                        k = this.getSubstitutionType(x, f);
                                    if ("12" === k)
                                        for (var m = 0; m < n.length; m++) {
                                            var P = e.get(m),
                                                u = F(P);
                                            u && d.push(u)
                                        }
                                }
                        return d
                    }

                    function Er(e, t) {
                        var r, n = e.current,
                            h = Tr(n, t.coverage);
                        if (-1 === h) return null;
                        for (var a = t.ligatureSets[h], i = 0; i < a.length; i++) {
                            r = a[i];
                            for (var c = 0; c < r.components.length; c++) {
                                var o = e.lookahead[c],
                                    s = r.components[c];
                                if (o !== s) break;
                                if (c === r.components.length - 1) return r
                            }
                        }
                        return null
                    }

                    function Wr(e, t) {
                        var r = Tr(e, t.coverage);
                        return -1 === r ? null : t.sequences[r]
                    }
                    Lt.prototype.exec = function(r, n) {
                        if ("number" !== typeof n) throw new Error("Point size is not a number!");
                        if (!(this._errorState > 2)) {
                            var a = this.font,
                                i = this._prepState;
                            if (!i || i.ppem !== n) {
                                var c = this._fpgmState;
                                if (!c) {
                                    rr.prototype = tr, c = this._fpgmState = new rr("fpgm", a.tables.fpgm), c.funcs = [], c.font = a, t.DEBUG && (h("log", "---EXEC FPGM---", " at utils/opentype.js:8987"), c.step = -1);
                                    try {
                                        St(c)
                                    } catch (e) {
                                        return h("log", "Hinting error in FPGM:" + e, " at utils/opentype.js:8994"), void(this._errorState = 3)
                                    }
                                }
                                rr.prototype = c, i = this._prepState = new rr("prep", a.tables.prep), i.ppem = n;
                                var o = a.tables.cvt;
                                if (o)
                                    for (var s = i.cvt = new Array(o.length), l = n / a.unitsPerEm, p = 0; p < o.length; p++) s[p] = o[p] * l;
                                else i.cvt = [];
                                t.DEBUG && (h("log", "---EXEC PREP---", " at utils/opentype.js:9025"), i.step = -1);
                                try {
                                    St(i)
                                } catch (e) {
                                    this._errorState < 2 && h("log", "Hinting error in PREP:" + e, " at utils/opentype.js:9033"), this._errorState = 2
                                }
                            }
                            if (!(this._errorState > 1)) try {
                                return Yt(r, i)
                            } catch (e) {
                                return this._errorState < 1 && (h("log", "Hinting error:" + e, " at utils/opentype.js:9045"), h("log", "Note: further hinting errors are silenced", " at utils/opentype.js:9046")), void(this._errorState = 1)
                            }
                        }
                    }, Yt = function(e, r) {
                        var n, a, i, c = r.ppem / r.font.unitsPerEm,
                            o = c,
                            s = e.components;
                        if (rr.prototype = r, s) {
                            var l = r.font;
                            a = [], n = [];
                            for (var p = 0; p < s.length; p++) {
                                var d = s[p],
                                    b = l.glyphs.get(d.glyphIndex);
                                i = new rr("glyf", b.instructions), t.DEBUG && (h("log", "---EXEC COMP " + p + "---", " at utils/opentype.js:9085"), i.step = -1), Bt(b, i, c, o);
                                for (var g = Math.round(d.dx * c), j = Math.round(d.dy * o), x = i.gZone, V = i.contours, f = 0; f < x.length; f++) {
                                    var F = x[f];
                                    F.xTouched = F.yTouched = !1, F.xo = F.x = F.x + g, F.yo = F.y = F.y + j
                                }
                                var k = a.length;
                                a.push.apply(a, x);
                                for (var m = 0; m < V.length; m++) n.push(V[m] + k)
                            }
                            e.instructions && !i.inhibitGridFit && (i = new rr("glyf", e.instructions), i.gZone = i.z0 = i.z1 = i.z2 = a, i.contours = n, a.push(new $t(0, 0), new $t(Math.round(e.advanceWidth * c), 0)), t.DEBUG && (h("log", "---EXEC COMPOSITE---", " at utils/opentype.js:9126"), i.step = -1), St(i), a.length -= 2)
                        } else i = new rr("glyf", e.instructions), t.DEBUG && (h("log", "---EXEC GLYPH---", " at utils/opentype.js:9069"), i.step = -1), Bt(e, i, c, o), a = i.gZone;
                        return a
                    }, Bt = function(e, r, n, a) {
                        for (var i, c, o, s = e.points || [], l = s.length, p = r.gZone = r.z0 = r.z1 = r.z2 = [], d = r.contours = [], b = 0; b < l; b++) i = s[b], p[b] = new $t(i.x * n, i.y * a, i.lastPointOfContour, i.onCurve);
                        for (var g = 0; g < l; g++) i = p[g], c || (c = i, d.push(g)), i.lastPointOfContour ? (i.nextPointOnContour = c, c.prevPointOnContour = i, c = void 0) : (o = p[g + 1], i.nextPointOnContour = o, o.prevPointOnContour = i);
                        if (!r.inhibitGridFit) {
                            if (t.DEBUG) {
                                h("log", "PROCESSING GLYPH", r.stack, " at utils/opentype.js:9190");
                                for (var j = 0; j < l; j++) h("log", j, p[j].x, p[j].y, " at utils/opentype.js:9192")
                            }
                            if (p.push(new $t(0, 0), new $t(Math.round(e.advanceWidth * n), 0)), St(r), p.length -= 2, t.DEBUG) {
                                h("log", "FINISHED GLYPH", r.stack, " at utils/opentype.js:9207");
                                for (var x = 0; x < l; x++) h("log", x, p[x].x, p[x].y, " at utils/opentype.js:9209")
                            }
                        }
                    }, St = function(e) {
                        var r = e.prog;
                        if (r) {
                            var n, h = r.length;
                            for (e.ip = 0; e.ip < h; e.ip++) {
                                if (t.DEBUG && e.step++, n = Dt[r[e.ip]], !n) throw new Error("unknown instruction: 0x" + Number(r[e.ip]).toString(16));
                                n(e)
                            }
                        }
                    }, Dt = [ar.bind(void 0, Ot), ar.bind(void 0, Ut), ir.bind(void 0, Ot), ir.bind(void 0, Ut), cr.bind(void 0, Ot), cr.bind(void 0, Ut), or.bind(void 0, 0), or.bind(void 0, 1), sr.bind(void 0, 0), sr.bind(void 0, 1), function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "SPVFS[]", n, a, " at utils/opentype.js:9416"), e.pv = e.dpv = Kt(a, n)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "SPVFS[]", n, a, " at utils/opentype.js:9428"), e.fv = Kt(a, n)
                    }, function(e) {
                        var r = e.stack,
                            n = e.pv;
                        t.DEBUG && h("log", e.step, "GPV[]", " at utils/opentype.js:9439"), r.push(16384 * n.x), r.push(16384 * n.y)
                    }, function(e) {
                        var r = e.stack,
                            n = e.fv;
                        t.DEBUG && h("log", e.step, "GFV[]", " at utils/opentype.js:9451"), r.push(16384 * n.x), r.push(16384 * n.y)
                    }, function(e) {
                        e.fv = e.pv, t.DEBUG && h("log", e.step, "SFVTPV[]", " at utils/opentype.js:9462")
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop(),
                            i = r.pop(),
                            c = r.pop(),
                            o = r.pop(),
                            s = e.z0,
                            l = e.z1,
                            p = s[n],
                            d = s[a],
                            b = l[i],
                            g = l[c],
                            j = e.z2[o];
                        t.DEBUG && h("log", "ISECT[], ", n, a, i, c, o, " at utils/opentype.js:9483");
                        var x = p.x,
                            V = p.y,
                            f = d.x,
                            F = d.y,
                            k = b.x,
                            m = b.y,
                            P = g.x,
                            u = g.y,
                            X = (x - f) * (m - u) - (V - F) * (k - P),
                            N = x * F - V * f,
                            H = k * u - m * P;
                        j.x = (N * (k - P) - H * (x - f)) / X, j.y = (N * (m - u) - H * (V - F)) / X
                    }, function(e) {
                        e.rp0 = e.stack.pop(), t.DEBUG && h("log", e.step, "SRP0[]", e.rp0, " at utils/opentype.js:9510")
                    }, function(e) {
                        e.rp1 = e.stack.pop(), t.DEBUG && h("log", e.step, "SRP1[]", e.rp1, " at utils/opentype.js:9518")
                    }, function(e) {
                        e.rp2 = e.stack.pop(), t.DEBUG && h("log", e.step, "SRP2[]", e.rp2, " at utils/opentype.js:9526")
                    }, function(e) {
                        var r = e.stack.pop();
                        switch (t.DEBUG && h("log", e.step, "SZP0[]", r, " at utils/opentype.js:9534"), e.zp0 = r, r) {
                            case 0:
                                e.tZone || nr(e), e.z0 = e.tZone;
                                break;
                            case 1:
                                e.z0 = e.gZone;
                                break;
                            default:
                                throw new Error("Invalid zone pointer")
                        }
                    }, function(e) {
                        var r = e.stack.pop();
                        switch (t.DEBUG && h("log", e.step, "SZP1[]", r, " at utils/opentype.js:9556"), e.zp1 = r, r) {
                            case 0:
                                e.tZone || nr(e), e.z1 = e.tZone;
                                break;
                            case 1:
                                e.z1 = e.gZone;
                                break;
                            default:
                                throw new Error("Invalid zone pointer")
                        }
                    }, function(e) {
                        var r = e.stack.pop();
                        switch (t.DEBUG && h("log", e.step, "SZP2[]", r, " at utils/opentype.js:9578"), e.zp2 = r, r) {
                            case 0:
                                e.tZone || nr(e), e.z2 = e.tZone;
                                break;
                            case 1:
                                e.z2 = e.gZone;
                                break;
                            default:
                                throw new Error("Invalid zone pointer")
                        }
                    }, function(e) {
                        var r = e.stack.pop();
                        switch (t.DEBUG && h("log", e.step, "SZPS[]", r, " at utils/opentype.js:9600"), e.zp0 = e.zp1 = e.zp2 = r, r) {
                            case 0:
                                e.tZone || nr(e), e.z0 = e.z1 = e.z2 = e.tZone;
                                break;
                            case 1:
                                e.z0 = e.z1 = e.z2 = e.gZone;
                                break;
                            default:
                                throw new Error("Invalid zone pointer")
                        }
                    }, function(e) {
                        e.loop = e.stack.pop(), t.DEBUG && h("log", e.step, "SLOOP[]", e.loop, " at utils/opentype.js:9622")
                    }, function(e) {
                        t.DEBUG && h("log", e.step, "RTG[]", " at utils/opentype.js:9628"), e.round = qt
                    }, function(e) {
                        t.DEBUG && h("log", e.step, "RTHG[]", " at utils/opentype.js:9636"), e.round = Et
                    }, function(e) {
                        var r = e.stack.pop();
                        t.DEBUG && h("log", e.step, "SMD[]", r, " at utils/opentype.js:9646"), e.minDis = r / 64
                    }, function(e) {
                        t.DEBUG && h("log", e.step, "ELSE[]", " at utils/opentype.js:9660"), hr(e, !1)
                    }, function(e) {
                        var r = e.stack.pop();
                        t.DEBUG && h("log", e.step, "JMPR[]", r, " at utils/opentype.js:9670"), e.ip += r - 1
                    }, function(e) {
                        var r = e.stack.pop();
                        t.DEBUG && h("log", e.step, "SCVTCI[]", r, " at utils/opentype.js:9681"), e.cvCutIn = r / 64
                    }, void 0, void 0, function(e) {
                        var r = e.stack;
                        t.DEBUG && h("log", e.step, "DUP[]", " at utils/opentype.js:9691"), r.push(r[r.length - 1])
                    }, lr, function(e) {
                        t.DEBUG && h("log", e.step, "CLEAR[]", " at utils/opentype.js:9707"), e.stack.length = 0
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "SWAP[]", " at utils/opentype.js:9720"), r.push(n), r.push(a)
                    }, function(e) {
                        var r = e.stack;
                        t.DEBUG && h("log", e.step, "DEPTH[]", " at utils/opentype.js:9731"), r.push(r.length)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "CINDEX[]", n, " at utils/opentype.js:9796"), r.push(r[r.length - n])
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "MINDEX[]", n, " at utils/opentype.js:9809"), r.push(r.splice(r.length - n, 1)[0])
                    }, void 0, void 0, void 0, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "LOOPCALL[]", n, a, " at utils/opentype.js:9743");
                        var i = e.ip,
                            c = e.prog;
                        e.prog = e.funcs[n];
                        for (var o = 0; o < a; o++) St(e), t.DEBUG && h("log", ++e.step, o + 1 < a ? "next loopcall" : "done loopcall", o, " at utils/opentype.js:9755");
                        e.ip = i, e.prog = c
                    }, function(e) {
                        var r = e.stack.pop();
                        t.DEBUG && h("log", e.step, "CALL[]", r, " at utils/opentype.js:9772");
                        var n = e.ip,
                            a = e.prog;
                        e.prog = e.funcs[r], St(e), e.ip = n, e.prog = a, t.DEBUG && h("log", ++e.step, "returning from", r, " at utils/opentype.js:9787")
                    }, function(e) {
                        if ("fpgm" !== e.env) throw new Error("FDEF not allowed here");
                        var r = e.stack,
                            n = e.prog,
                            a = e.ip,
                            i = r.pop(),
                            c = a;
                        t.DEBUG && h("log", e.step, "FDEF[]", i, " at utils/opentype.js:9825");
                        while (45 !== n[++a]);
                        e.ip = a, e.funcs[i] = n.slice(c + 1, a)
                    }, void 0, pr.bind(void 0, 0), pr.bind(void 0, 1), dr.bind(void 0, Ot), dr.bind(void 0, Ut), br.bind(void 0, 0), br.bind(void 0, 1), gr.bind(void 0, 0), gr.bind(void 0, 1), jr.bind(void 0, 0), jr.bind(void 0, 1), function(e) {
                        var r = e.stack,
                            n = e.loop,
                            a = e.fv,
                            i = r.pop() / 64,
                            c = e.z2;
                        while (n--) {
                            var o = r.pop(),
                                s = c[o];
                            t.DEBUG && h("log", e.step, (e.loop > 1 ? "loop " + (e.loop - n) + ": " : "") + "SHPIX[]", o, i, " at utils/opentype.js:9990"), a.setRelative(s, s, i), a.touch(s)
                        }
                        e.loop = 1
                    }, function(e) {
                        var r = e.stack,
                            n = e.rp1,
                            a = e.rp2,
                            i = e.loop,
                            c = e.z0[n],
                            o = e.z1[a],
                            s = e.fv,
                            l = e.dpv,
                            p = e.z2;
                        while (i--) {
                            var d = r.pop(),
                                b = p[d];
                            t.DEBUG && h("log", e.step, (e.loop > 1 ? "loop " + (e.loop - i) + ": " : "") + "IP[]", d, n, "<->", a, " at utils/opentype.js:10022"), s.interpolate(b, c, o, l), s.touch(b)
                        }
                        e.loop = 1
                    }, xr.bind(void 0, 0), xr.bind(void 0, 1), function(e) {
                        var r = e.stack,
                            n = e.rp0,
                            a = e.z0[n],
                            i = e.loop,
                            c = e.fv,
                            o = e.pv,
                            s = e.z1;
                        while (i--) {
                            var l = r.pop(),
                                p = s[l];
                            t.DEBUG && h("log", e.step, (e.loop > 1 ? "loop " + (e.loop - i) + ": " : "") + "ALIGNRP[]", l, " at utils/opentype.js:10074"), c.setRelative(p, a, 0, o), c.touch(p)
                        }
                        e.loop = 1
                    }, function(e) {
                        t.DEBUG && h("log", e.step, "RTDG[]", " at utils/opentype.js:10091"), e.round = Gt
                    }, Vr.bind(void 0, 0), Vr.bind(void 0, 1), function(e) {
                        var r = e.prog,
                            n = e.ip,
                            a = e.stack,
                            i = r[++n];
                        t.DEBUG && h("log", e.step, "NPUSHB[]", i, " at utils/opentype.js:10144");
                        for (var c = 0; c < i; c++) a.push(r[++n]);
                        e.ip = n
                    }, function(e) {
                        var r = e.ip,
                            n = e.prog,
                            a = e.stack,
                            i = n[++r];
                        t.DEBUG && h("log", e.step, "NPUSHW[]", i, " at utils/opentype.js:10159");
                        for (var c = 0; c < i; c++) {
                            var o = n[++r] << 8 | n[++r];
                            32768 & o && (o = -(1 + (65535 ^ o))), a.push(o)
                        }
                        e.ip = r
                    }, function(e) {
                        var r = e.stack,
                            n = e.store;
                        n || (n = e.store = []);
                        var a = r.pop(),
                            i = r.pop();
                        t.DEBUG && h("log", e.step, "WS", a, i, " at utils/opentype.js:10181"), n[i] = a
                    }, function(e) {
                        var r = e.stack,
                            n = e.store,
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "RS", a, " at utils/opentype.js:10194");
                        var i = n && n[a] || 0;
                        r.push(i)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "WCVTP", n, a, " at utils/opentype.js:10209"), e.cvt[a] = n / 64
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "RCVT", n, " at utils/opentype.js:10220"), r.push(64 * e.cvt[n])
                    }, fr.bind(void 0, 0), fr.bind(void 0, 1), void 0, Fr.bind(void 0, 0), Fr.bind(void 0, 1), function(e) {
                        t.DEBUG && h("log", e.step, "MPPEM[]", " at utils/opentype.js:10255"), e.stack.push(e.ppem)
                    }, void 0, function(e) {
                        t.DEBUG && h("log", e.step, "FLIPON[]", " at utils/opentype.js:10262"), e.autoFlip = !0
                    }, void 0, void 0, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "LT[]", n, a, " at utils/opentype.js:10273"), r.push(a < n ? 1 : 0)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "LTEQ[]", n, a, " at utils/opentype.js:10285"), r.push(a <= n ? 1 : 0)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "GT[]", n, a, " at utils/opentype.js:10297"), r.push(a > n ? 1 : 0)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "GTEQ[]", n, a, " at utils/opentype.js:10309"), r.push(a >= n ? 1 : 0)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "EQ[]", n, a, " at utils/opentype.js:10321"), r.push(n === a ? 1 : 0)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "NEQ[]", n, a, " at utils/opentype.js:10333"), r.push(n !== a ? 1 : 0)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "ODD[]", n, " at utils/opentype.js:10344"), r.push(Math.trunc(n) % 2 ? 1 : 0)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "EVEN[]", n, " at utils/opentype.js:10355"), r.push(Math.trunc(n) % 2 ? 0 : 1)
                    }, function(e) {
                        var r = e.stack.pop();
                        t.DEBUG && h("log", e.step, "IF[]", r, " at utils/opentype.js:10365"), r || (hr(e, !0), t.DEBUG && h("log", e.step, "EIF[]", " at utils/opentype.js:10372"))
                    }, function(e) {
                        t.DEBUG && h("log", e.step, "EIF[]", " at utils/opentype.js:10383")
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "AND[]", n, a, " at utils/opentype.js:10393"), r.push(n && a ? 1 : 0)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "OR[]", n, a, " at utils/opentype.js:10405"), r.push(n || a ? 1 : 0)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "NOT[]", n, " at utils/opentype.js:10416"), r.push(n ? 0 : 1)
                    }, kr.bind(void 0, 1), function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "SDB[]", n, " at utils/opentype.js:10458"), e.deltaBase = n
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "SDS[]", n, " at utils/opentype.js:10469"), e.deltaShift = Math.pow(.5, n)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "ADD[]", n, a, " at utils/opentype.js:10481"), r.push(a + n)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "SUB[]", n, a, " at utils/opentype.js:10493"), r.push(a - n)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "DIV[]", n, a, " at utils/opentype.js:10505"), r.push(64 * a / n)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "MUL[]", n, a, " at utils/opentype.js:10517"), r.push(a * n / 64)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "ABS[]", n, " at utils/opentype.js:10528"), r.push(Math.abs(n))
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "NEG[]", n, " at utils/opentype.js:10539"), r.push(-n)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "FLOOR[]", n, " at utils/opentype.js:10550"), r.push(64 * Math.floor(n / 64))
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop();
                        t.DEBUG && h("log", e.step, "CEILING[]", n, " at utils/opentype.js:10561"), r.push(64 * Math.ceil(n / 64))
                    }, mr.bind(void 0, 0), mr.bind(void 0, 1), mr.bind(void 0, 2), mr.bind(void 0, 3), void 0, void 0, void 0, void 0, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "WCVTF[]", n, a, " at utils/opentype.js:10584"), e.cvt[a] = n * e.ppem / e.font.unitsPerEm
                    }, kr.bind(void 0, 2), kr.bind(void 0, 3), Pr.bind(void 0, 1), Pr.bind(void 0, 2), Pr.bind(void 0, 3), function(e) {
                        var r, n = e.stack.pop();
                        switch (t.DEBUG && h("log", e.step, "SROUND[]", n, " at utils/opentype.js:10624"), e.round = Jt, 192 & n) {
                            case 0:
                                r = .5;
                                break;
                            case 64:
                                r = 1;
                                break;
                            case 128:
                                r = 2;
                                break;
                            default:
                                throw new Error("invalid SROUND value")
                        }
                        switch (e.srPeriod = r, 48 & n) {
                            case 0:
                                e.srPhase = 0;
                                break;
                            case 16:
                                e.srPhase = .25 * r;
                                break;
                            case 32:
                                e.srPhase = .5 * r;
                                break;
                            case 48:
                                e.srPhase = .75 * r;
                                break;
                            default:
                                throw new Error("invalid SROUND value")
                        }
                        n &= 15, e.srThreshold = 0 === n ? 0 : (n / 8 - .5) * r
                    }, function(e) {
                        var r, n = e.stack.pop();
                        switch (t.DEBUG && h("log", e.step, "S45ROUND[]", n, " at utils/opentype.js:10673"), e.round = Jt, 192 & n) {
                            case 0:
                                r = Math.sqrt(2) / 2;
                                break;
                            case 64:
                                r = Math.sqrt(2);
                                break;
                            case 128:
                                r = 2 * Math.sqrt(2);
                                break;
                            default:
                                throw new Error("invalid S45ROUND value")
                        }
                        switch (e.srPeriod = r, 48 & n) {
                            case 0:
                                e.srPhase = 0;
                                break;
                            case 16:
                                e.srPhase = .25 * r;
                                break;
                            case 32:
                                e.srPhase = .5 * r;
                                break;
                            case 48:
                                e.srPhase = .75 * r;
                                break;
                            default:
                                throw new Error("invalid S45ROUND value")
                        }
                        n &= 15, e.srThreshold = 0 === n ? 0 : (n / 8 - .5) * r
                    }, void 0, void 0, function(e) {
                        t.DEBUG && h("log", e.step, "ROFF[]", " at utils/opentype.js:10721"), e.round = Mt
                    }, void 0, function(e) {
                        t.DEBUG && h("log", e.step, "RUTG[]", " at utils/opentype.js:10729"), e.round = Wt
                    }, function(e) {
                        t.DEBUG && h("log", e.step, "RDTG[]", " at utils/opentype.js:10737"), e.round = _t
                    }, lr, lr, void 0, void 0, void 0, void 0, void 0, function(e) {
                        var r = e.stack.pop();
                        t.DEBUG && h("log", e.step, "SCANCTRL[]", r, " at utils/opentype.js:10749")
                    }, ur.bind(void 0, 0), ur.bind(void 0, 1), function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = 0;
                        t.DEBUG && h("log", e.step, "GETINFO[]", n, " at utils/opentype.js:10784"), 1 & n && (a = 35), 32 & n && (a |= 4096), r.push(a)
                    }, void 0, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop(),
                            i = r.pop();
                        t.DEBUG && h("log", e.step, "ROLL[]", " at utils/opentype.js:10806"), r.push(a), r.push(n), r.push(i)
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "MAX[]", n, a, " at utils/opentype.js:10820"), r.push(Math.max(a, n))
                    }, function(e) {
                        var r = e.stack,
                            n = r.pop(),
                            a = r.pop();
                        t.DEBUG && h("log", e.step, "MIN[]", n, a, " at utils/opentype.js:10832"), r.push(Math.min(a, n))
                    }, function(e) {
                        var r = e.stack.pop();
                        t.DEBUG && h("log", e.step, "SCANTYPE[]", r, " at utils/opentype.js:10842")
                    }, function(e) {
                        var r = e.stack.pop(),
                            n = e.stack.pop();
                        switch (t.DEBUG && h("log", e.step, "INSTCTRL[]", r, n, " at utils/opentype.js:10851"), r) {
                            case 1:
                                return void(e.inhibitGridFit = !!n);
                            case 2:
                                return void(e.ignoreCvt = !!n);
                            default:
                                throw new Error("invalid INSTCTRL[] selector")
                        }
                    }, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, void 0, Xr.bind(void 0, 1), Xr.bind(void 0, 2), Xr.bind(void 0, 3), Xr.bind(void 0, 4), Xr.bind(void 0, 5), Xr.bind(void 0, 6), Xr.bind(void 0, 7), Xr.bind(void 0, 8), Nr.bind(void 0, 1), Nr.bind(void 0, 2), Nr.bind(void 0, 3), Nr.bind(void 0, 4), Nr.bind(void 0, 5), Nr.bind(void 0, 6), Nr.bind(void 0, 7), Nr.bind(void 0, 8), Hr.bind(void 0, 0, 0, 0, 0, 0), Hr.bind(void 0, 0, 0, 0, 0, 1), Hr.bind(void 0, 0, 0, 0, 0, 2), Hr.bind(void 0, 0, 0, 0, 0, 3), Hr.bind(void 0, 0, 0, 0, 1, 0), Hr.bind(void 0, 0, 0, 0, 1, 1), Hr.bind(void 0, 0, 0, 0, 1, 2), Hr.bind(void 0, 0, 0, 0, 1, 3), Hr.bind(void 0, 0, 0, 1, 0, 0), Hr.bind(void 0, 0, 0, 1, 0, 1), Hr.bind(void 0, 0, 0, 1, 0, 2), Hr.bind(void 0, 0, 0, 1, 0, 3), Hr.bind(void 0, 0, 0, 1, 1, 0), Hr.bind(void 0, 0, 0, 1, 1, 1), Hr.bind(void 0, 0, 0, 1, 1, 2), Hr.bind(void 0, 0, 0, 1, 1, 3), Hr.bind(void 0, 0, 1, 0, 0, 0), Hr.bind(void 0, 0, 1, 0, 0, 1), Hr.bind(void 0, 0, 1, 0, 0, 2), Hr.bind(void 0, 0, 1, 0, 0, 3), Hr.bind(void 0, 0, 1, 0, 1, 0), Hr.bind(void 0, 0, 1, 0, 1, 1), Hr.bind(void 0, 0, 1, 0, 1, 2), Hr.bind(void 0, 0, 1, 0, 1, 3), Hr.bind(void 0, 0, 1, 1, 0, 0), Hr.bind(void 0, 0, 1, 1, 0, 1), Hr.bind(void 0, 0, 1, 1, 0, 2), Hr.bind(void 0, 0, 1, 1, 0, 3), Hr.bind(void 0, 0, 1, 1, 1, 0), Hr.bind(void 0, 0, 1, 1, 1, 1), Hr.bind(void 0, 0, 1, 1, 1, 2), Hr.bind(void 0, 0, 1, 1, 1, 3), Hr.bind(void 0, 1, 0, 0, 0, 0), Hr.bind(void 0, 1, 0, 0, 0, 1), Hr.bind(void 0, 1, 0, 0, 0, 2), Hr.bind(void 0, 1, 0, 0, 0, 3), Hr.bind(void 0, 1, 0, 0, 1, 0), Hr.bind(void 0, 1, 0, 0, 1, 1), Hr.bind(void 0, 1, 0, 0, 1, 2), Hr.bind(void 0, 1, 0, 0, 1, 3), Hr.bind(void 0, 1, 0, 1, 0, 0), Hr.bind(void 0, 1, 0, 1, 0, 1), Hr.bind(void 0, 1, 0, 1, 0, 2), Hr.bind(void 0, 1, 0, 1, 0, 3), Hr.bind(void 0, 1, 0, 1, 1, 0), Hr.bind(void 0, 1, 0, 1, 1, 1), Hr.bind(void 0, 1, 0, 1, 1, 2), Hr.bind(void 0, 1, 0, 1, 1, 3), Hr.bind(void 0, 1, 1, 0, 0, 0), Hr.bind(void 0, 1, 1, 0, 0, 1), Hr.bind(void 0, 1, 1, 0, 0, 2), Hr.bind(void 0, 1, 1, 0, 0, 3), Hr.bind(void 0, 1, 1, 0, 1, 0), Hr.bind(void 0, 1, 1, 0, 1, 1), Hr.bind(void 0, 1, 1, 0, 1, 2), Hr.bind(void 0, 1, 1, 0, 1, 3), Hr.bind(void 0, 1, 1, 1, 0, 0), Hr.bind(void 0, 1, 1, 1, 0, 1), Hr.bind(void 0, 1, 1, 1, 0, 2), Hr.bind(void 0, 1, 1, 1, 0, 3), Hr.bind(void 0, 1, 1, 1, 1, 0), Hr.bind(void 0, 1, 1, 1, 1, 1), Hr.bind(void 0, 1, 1, 1, 1, 2), Hr.bind(void 0, 1, 1, 1, 1, 3)], zr.prototype.setState = function(e, t) {
                        return this.state[e] = t, this.activeState = {
                            key: e,
                            value: this.state[e]
                        }, this.activeState
                    }, zr.prototype.getState = function(e) {
                        return this.state[e] || null
                    }, yr.prototype.inboundIndex = function(e) {
                        return e >= 0 && e < this.tokens.length
                    }, yr.prototype.composeRUD = function(e) {
                        var t = this,
                            r = e.map((function(e) {
                                return t[e[0]].apply(t, e.slice(1).concat(!0))
                            })),
                            n = function(e) {
                                return "object" === s(e) && e.hasOwnProperty("FAIL")
                            };
                        if (r.every(n)) return {
                            FAIL: "composeRUD: one or more operations hasn't completed successfully",
                            report: r.filter(n)
                        };
                        this.dispatch("composeRUD", [r.filter((function(e) {
                            return !n(e)
                        }))])
                    }, yr.prototype.replaceRange = function(e, t, r, n) {
                        t = null !== t ? t : this.tokens.length;
                        var h = r.every((function(e) {
                            return e instanceof zr
                        }));
                        if (!isNaN(e) && this.inboundIndex(e) && h) {
                            var a = this.tokens.splice.apply(this.tokens, [e, t].concat(r));
                            return n || this.dispatch("replaceToken", [e, t, r]), [a, r]
                        }
                        return {
                            FAIL: "replaceRange: invalid tokens or startIndex."
                        }
                    }, yr.prototype.replaceToken = function(e, t, r) {
                        if (!isNaN(e) && this.inboundIndex(e) && t instanceof zr) {
                            var n = this.tokens.splice(e, 1, t);
                            return r || this.dispatch("replaceToken", [e, t]), [n[0], t]
                        }
                        return {
                            FAIL: "replaceToken: invalid token or index."
                        }
                    }, yr.prototype.removeRange = function(e, t, r) {
                        t = isNaN(t) ? this.tokens.length : t;
                        var n = this.tokens.splice(e, t);
                        return r || this.dispatch("removeRange", [n, e, t]), n
                    }, yr.prototype.removeToken = function(e, t) {
                        if (!isNaN(e) && this.inboundIndex(e)) {
                            var r = this.tokens.splice(e, 1);
                            return t || this.dispatch("removeToken", [r, e]), r
                        }
                        return {
                            FAIL: "removeToken: invalid token index."
                        }
                    }, yr.prototype.insertToken = function(e, t, r) {
                        var n = e.every((function(e) {
                            return e instanceof zr
                        }));
                        return n ? (this.tokens.splice.apply(this.tokens, [t, 0].concat(e)), r || this.dispatch("insertToken", [e, t]), e) : {
                            FAIL: "insertToken: invalid token(s)."
                        }
                    }, yr.prototype.registerModifier = function(e, t, r) {
                        this.events.newToken.subscribe((function(n, h) {
                            var a = [n, h],
                                i = null === t || !0 === t.apply(this, a),
                                c = [n, h];
                            if (i) {
                                var o = r.apply(this, c);
                                n.setState(e, o)
                            }
                        })), this.registeredModifiers.push(e)
                    }, Ir.prototype.subscribe = function(e) {
                        return "function" === typeof e ? this.subscribers.push(e) - 1 : {
                            FAIL: "invalid '" + this.eventId + "' event handler"
                        }
                    }, Ir.prototype.unsubscribe = function(e) {
                        this.subscribers.splice(e, 1)
                    }, vr.prototype.setCurrentIndex = function(e) {
                        this.index = e, this.current = this.context[e], this.backtrack = this.context.slice(0, e), this.lookahead = this.context.slice(e + 1)
                    }, vr.prototype.get = function(e) {
                        switch (!0) {
                            case 0 === e:
                                return this.current;
                            case e < 0 && Math.abs(e) <= this.backtrack.length:
                                return this.backtrack.slice(e)[0];
                            case e > 0 && e <= this.lookahead.length:
                                return this.lookahead[e - 1];
                            default:
                                return null
                        }
                    }, yr.prototype.rangeToText = function(e) {
                        if (e instanceof Qr) return this.getRangeTokens(e).map((function(e) {
                            return e.char
                        })).join("")
                    }, yr.prototype.getText = function() {
                        return this.tokens.map((function(e) {
                            return e.char
                        })).join("")
                    }, yr.prototype.getContext = function(e) {
                        var t = this.registeredContexts[e];
                        return t || null
                    }, yr.prototype.on = function(e, t) {
                        var r = this.events[e];
                        return r ? r.subscribe(t) : null
                    }, yr.prototype.dispatch = function(e, t) {
                        var r = this,
                            n = this.events[e];
                        n instanceof Ir && n.subscribers.forEach((function(e) {
                            e.apply(r, t || [])
                        }))
                    }, yr.prototype.registerContextChecker = function(e, t, r) {
                        if (this.getContext(e)) return {
                            FAIL: "context name '" + e + "' is already registered."
                        };
                        if ("function" !== typeof t) return {
                            FAIL: "missing context start check."
                        };
                        if ("function" !== typeof r) return {
                            FAIL: "missing context end check."
                        };
                        var n = new Rr(e, t, r);
                        return this.registeredContexts[e] = n, this.contextCheckers.push(n), n
                    }, yr.prototype.getRangeTokens = function(e) {
                        var t = e.startIndex + e.endOffset;
                        return [].concat(this.tokens.slice(e.startIndex, t))
                    }, yr.prototype.getContextRanges = function(e) {
                        var t = this.getContext(e);
                        return t ? t.ranges : {
                            FAIL: "context checker '" + e + "' is not registered."
                        }
                    }, yr.prototype.resetContextsRanges = function() {
                        var e = this.registeredContexts;
                        for (var t in e)
                            if (e.hasOwnProperty(t)) {
                                var r = e[t];
                                r.ranges = []
                            }
                    }, yr.prototype.updateContextsRanges = function() {
                        this.resetContextsRanges();
                        for (var e = this.tokens.map((function(e) {
                                return e.char
                            })), t = 0; t < e.length; t++) {
                            var r = new vr(e, t);
                            this.runContextCheck(r)
                        }
                        this.dispatch("updateContextsRanges", [this.registeredContexts])
                    }, yr.prototype.setEndOffset = function(e, t) {
                        var r = this.getContext(t).openRange.startIndex,
                            n = new Qr(r, e, t),
                            h = this.getContext(t).ranges;
                        return n.rangeId = t + "." + h.length, h.push(n), this.getContext(t).openRange = null, n
                    }, yr.prototype.runContextCheck = function(e) {
                        var t = this,
                            r = e.index;
                        this.contextCheckers.forEach((function(n) {
                            var h = n.contextName,
                                a = t.getContext(h).openRange;
                            if (!a && n.checkStart(e) && (a = new Qr(r, null, h), t.getContext(h).openRange = a, t.dispatch("contextStart", [h, r])), a && n.checkEnd(e)) {
                                var i = r - a.startIndex + 1,
                                    c = t.setEndOffset(i, h);
                                t.dispatch("contextEnd", [h, c])
                            }
                        }))
                    }, yr.prototype.tokenize = function(e) {
                        this.tokens = [], this.resetContextsRanges();
                        var t = Array.from(e);
                        this.dispatch("start");
                        for (var r = 0; r < t.length; r++) {
                            var n = t[r],
                                h = new vr(t, r);
                            this.dispatch("next", [h]), this.runContextCheck(h);
                            var a = new zr(n);
                            this.tokens.push(a), this.dispatch("newToken", [a, h])
                        }
                        return this.dispatch("end", [this.tokens]), this.tokens
                    }, Yr.prototype.getDefaultScriptFeaturesIndexes = function() {
                        for (var e = this.font.tables.gsub.scripts, t = 0; t < e.length; t++) {
                            var r = e[t];
                            if ("DFLT" === r.tag) return r.script.defaultLangSys.featureIndexes
                        }
                        return []
                    }, Yr.prototype.getScriptFeaturesIndexes = function(e) {
                        var t = this.font.tables;
                        if (!t.gsub) return [];
                        if (!e) return this.getDefaultScriptFeaturesIndexes();
                        for (var r = this.font.tables.gsub.scripts, n = 0; n < r.length; n++) {
                            var h = r[n];
                            if (h.tag === e && h.script.defaultLangSys) return h.script.defaultLangSys.featureIndexes;
                            var a = h.langSysRecords;
                            if (a)
                                for (var i = 0; i < a.length; i++) {
                                    var c = a[i];
                                    if (c.tag === e) {
                                        var o = c.langSys;
                                        return o.featureIndexes
                                    }
                                }
                        }
                        return this.getDefaultScriptFeaturesIndexes()
                    }, Yr.prototype.mapTagsToFeatures = function(e, t) {
                        for (var r = {}, n = 0; n < e.length; n++) {
                            var h = e[n].tag,
                                a = e[n].feature;
                            r[h] = a
                        }
                        this.features[t].tags = r
                    }, Yr.prototype.getScriptFeatures = function(e) {
                        var t = this.features[e];
                        if (this.features.hasOwnProperty(e)) return t;
                        var r = this.getScriptFeaturesIndexes(e);
                        if (!r) return null;
                        var n = this.font.tables.gsub;
                        return t = r.map((function(e) {
                            return n.features[e]
                        })), this.features[e] = t, this.mapTagsToFeatures(t, e), t
                    }, Yr.prototype.getSubstitutionType = function(e, t) {
                        var r = e.lookupType.toString(),
                            n = t.substFormat.toString();
                        return r + n
                    }, Yr.prototype.getLookupMethod = function(e, t) {
                        var r = this,
                            n = this.getSubstitutionType(e, t);
                        switch (n) {
                            case "11":
                                return function(e) {
                                    return Lr.apply(r, [e, t])
                                };
                            case "12":
                                return function(e) {
                                    return Mr.apply(r, [e, t])
                                };
                            case "63":
                                return function(e) {
                                    return Gr.apply(r, [e, t])
                                };
                            case "41":
                                return function(e) {
                                    return Er.apply(r, [e, t])
                                };
                            case "21":
                                return function(e) {
                                    return Wr.apply(r, [e, t])
                                };
                            default:
                                throw new Error("lookupType: " + e.lookupType + " - substFormat: " + t.substFormat + " is not yet supported")
                        }
                    }, Yr.prototype.lookupFeature = function(e) {
                        var t = e.contextParams,
                            r = t.index,
                            n = this.getFeature({
                                tag: e.tag,
                                script: e.script
                            });
                        if (!n) return new Error("font '" + this.font.names.fullName.en + "' doesn't support feature '" + e.tag + "' for script '" + e.script + "'.");
                        for (var h = this.getFeatureLookups(n), a = [].concat(t.context), i = 0; i < h.length; i++)
                            for (var c = h[i], o = this.getLookupSubtables(c), s = 0; s < o.length; s++) {
                                var l = o[s],
                                    p = this.getSubstitutionType(c, l),
                                    d = this.getLookupMethod(c, l),
                                    b = void 0;
                                switch (p) {
                                    case "11":
                                        b = d(t.current), b && a.splice(r, 1, new Br({
                                            id: 11,
                                            tag: e.tag,
                                            substitution: b
                                        }));
                                        break;
                                    case "12":
                                        b = d(t.current), b && a.splice(r, 1, new Br({
                                            id: 12,
                                            tag: e.tag,
                                            substitution: b
                                        }));
                                        break;
                                    case "63":
                                        b = d(t), Array.isArray(b) && b.length && a.splice(r, 1, new Br({
                                            id: 63,
                                            tag: e.tag,
                                            substitution: b
                                        }));
                                        break;
                                    case "41":
                                        b = d(t), b && a.splice(r, 1, new Br({
                                            id: 41,
                                            tag: e.tag,
                                            substitution: b
                                        }));
                                        break;
                                    case "21":
                                        b = d(t.current), b && a.splice(r, 1, new Br({
                                            id: 21,
                                            tag: e.tag,
                                            substitution: b
                                        }));
                                        break
                                }
                                t = new vr(a, r), Array.isArray(b) && !b.length || (b = null)
                            }
                        return a.length ? a : null
                    }, Yr.prototype.supports = function(e) {
                        if (!e.script) return !1;
                        this.getScriptFeatures(e.script);
                        var t = this.features.hasOwnProperty(e.script);
                        if (!e.tag) return t;
                        var r = this.features[e.script].some((function(t) {
                            return t.tag === e.tag
                        }));
                        return t && r
                    }, Yr.prototype.getLookupSubtables = function(e) {
                        return e.subtables || null
                    }, Yr.prototype.getLookupByIndex = function(e) {
                        var t = this.font.tables.gsub.lookups;
                        return t[e] || null
                    }, Yr.prototype.getFeatureLookups = function(e) {
                        return e.lookupListIndexes.map(this.getLookupByIndex.bind(this))
                    }, Yr.prototype.getFeature = function(e) {
                        if (!this.font) return {
                            FAIL: "No font was found"
                        };
                        this.features.hasOwnProperty(e.script) || this.getScriptFeatures(e.script);
                        var t = this.features[e.script];
                        return t ? t.tags[e.tag] ? this.features[e.script].tags[e.tag] : null : {
                            FAIL: "No feature for script " + e.script
                        }
                    };
                    var _r = {
                        startCheck: function(e) {
                            var t = e.current,
                                r = e.get(-1);
                            return null === r && Cr(t) || !Cr(r) && Cr(t)
                        },
                        endCheck: function(e) {
                            var t = e.get(1);
                            return null === t || !Cr(t)
                        }
                    };
                    var Jr = {
                        startCheck: function(e) {
                            var t = e.current,
                                r = e.get(-1);
                            return (Cr(t) || Dr(t)) && !Cr(r)
                        },
                        endCheck: function(e) {
                            var t = e.get(1);
                            switch (!0) {
                                case null === t:
                                    return !0;
                                case !Cr(t) && !Dr(t):
                                    var r = function(e) {
                                        return /\s/.test(e)
                                    }(t);
                                    if (!r) return !0;
                                    if (r) {
                                        var n;
                                        if (n = e.lookahead.some((function(e) {
                                                return Cr(e) || Dr(e)
                                            })), !n) return !0
                                    }
                                    break;
                                default:
                                    return !1
                            }
                        }
                    };
                    var Ur = {
                        11: function(e, t, r) {
                            t[r].setState(e.tag, e.substitution)
                        },
                        12: function(e, t, r) {
                            t[r].setState(e.tag, e.substitution)
                        },
                        63: function(e, t, r) {
                            e.substitution.forEach((function(n, h) {
                                var a = t[r + h];
                                a.setState(e.tag, n)
                            }))
                        },
                        41: function(e, t, r) {
                            var n = t[r];
                            n.setState(e.tag, e.substitution.ligGlyph);
                            for (var h = e.substitution.components.length, a = 0; a < h; a++) n = t[r + a + 1], n.setState("deleted", !0)
                        }
                    };

                    function Or(e, t, r) {
                        e instanceof Br && Ur[e.id] && Ur[e.id](e, t, r)
                    }

                    function Zr(e) {
                        var t = this,
                            r = this.featuresTags["arab"],
                            n = this.tokenizer.getRangeTokens(e);
                        if (1 !== n.length) {
                            var a = new vr(n.map((function(e) {
                                    return e.getState("glyphIndex")
                                })), 0),
                                i = new vr(n.map((function(e) {
                                    return e.char
                                })), 0);
                            n.forEach((function(e, c) {
                                if (!Dr(e.char)) {
                                    a.setCurrentIndex(c), i.setCurrentIndex(c);
                                    var o, s = 0;
                                    switch (function(e) {
                                            for (var t = [].concat(e.backtrack), r = t.length - 1; r >= 0; r--) {
                                                var n = t[r],
                                                    h = Ar(n),
                                                    a = Dr(n);
                                                if (!h && !a) return !0;
                                                if (h) return !1
                                            }
                                            return !1
                                        }(i) && (s |= 1), function(e) {
                                            if (Ar(e.current)) return !1;
                                            for (var t = 0; t < e.lookahead.length; t++) {
                                                var r = e.lookahead[t],
                                                    n = Dr(r);
                                                if (!n) return !0
                                            }
                                            return !1
                                        }(i) && (s |= 2), s) {
                                        case 1:
                                            o = "fina";
                                            break;
                                        case 2:
                                            o = "init";
                                            break;
                                        case 3:
                                            o = "medi";
                                            break
                                    }
                                    if (-1 !== r.indexOf(o)) {
                                        var l = t.query.lookupFeature({
                                            tag: o,
                                            script: "arab",
                                            contextParams: a
                                        });
                                        if (l instanceof Error) return h("info", l.message, " at utils/opentype.js:12717");
                                        l.forEach((function(e, t) {
                                            e instanceof Br && (Or(e, n, t), a.context[t] = e.substitution)
                                        }))
                                    }
                                }
                            }))
                        }
                    }

                    function Kr(e, t) {
                        var r = e.map((function(e) {
                            return e.activeState.value
                        }));
                        return new vr(r, t || 0)
                    }

                    function $r(e) {
                        var t = this,
                            r = this.tokenizer.getRangeTokens(e),
                            n = Kr(r);
                        n.context.forEach((function(e, h) {
                            n.setCurrentIndex(h);
                            var a = t.query.lookupFeature({
                                tag: "rlig",
                                script: "arab",
                                contextParams: n
                            });
                            a.length && (a.forEach((function(e) {
                                return Or(e, r, h)
                            })), n = Kr(r))
                        }))
                    }
                    var en = {
                        startCheck: function(e) {
                            var t = e.current,
                                r = e.get(-1);
                            return null === r && Sr(t) || !Sr(r) && Sr(t)
                        },
                        endCheck: function(e) {
                            var t = e.get(1);
                            return null === t || !Sr(t)
                        }
                    };

                    function tn(e, t) {
                        var r = e.map((function(e) {
                            return e.activeState.value
                        }));
                        return new vr(r, t || 0)
                    }

                    function rn(e) {
                        var t = this,
                            r = this.tokenizer.getRangeTokens(e),
                            n = tn(r);
                        n.context.forEach((function(e, h) {
                            n.setCurrentIndex(h);
                            var a = t.query.lookupFeature({
                                tag: "liga",
                                script: "latn",
                                contextParams: n
                            });
                            a.length && (a.forEach((function(e) {
                                return Or(e, r, h)
                            })), n = tn(r))
                        }))
                    }

                    function nn(e) {
                        this.baseDir = e || "ltr", this.tokenizer = new yr, this.featuresTags = {}
                    }

                    function hn(e) {
                        var t = this.contextChecks[e + "Check"];
                        return this.tokenizer.registerContextChecker(e, t.startCheck, t.endCheck)
                    }

                    function an() {
                        return hn.call(this, "latinWord"), hn.call(this, "arabicWord"), hn.call(this, "arabicSentence"), this.tokenizer.tokenize(this.text)
                    }

                    function cn() {
                        var e = this,
                            t = this.tokenizer.getContextRanges("arabicSentence");
                        t.forEach((function(t) {
                            var r = e.tokenizer.getRangeTokens(t);
                            e.tokenizer.replaceRange(t.startIndex, t.endOffset, r.reverse())
                        }))
                    }

                    function on() {
                        if (-1 === this.tokenizer.registeredModifiers.indexOf("glyphIndex")) throw new Error("glyphIndex modifier is required to apply arabic presentation features.")
                    }

                    function sn() {
                        var e = this;
                        if (this.featuresTags.hasOwnProperty("arab")) {
                            on.call(this);
                            var t = this.tokenizer.getContextRanges("arabicWord");
                            t.forEach((function(t) {
                                Zr.call(e, t)
                            }))
                        }
                    }

                    function ln() {
                        var e = this;
                        if (this.featuresTags.hasOwnProperty("arab")) {
                            var t = this.featuresTags["arab"];
                            if (-1 !== t.indexOf("rlig")) {
                                on.call(this);
                                var r = this.tokenizer.getContextRanges("arabicWord");
                                r.forEach((function(t) {
                                    $r.call(e, t)
                                }))
                            }
                        }
                    }

                    function pn() {
                        var e = this;
                        if (this.featuresTags.hasOwnProperty("latn")) {
                            var t = this.featuresTags["latn"];
                            if (-1 !== t.indexOf("liga")) {
                                on.call(this);
                                var r = this.tokenizer.getContextRanges("latinWord");
                                r.forEach((function(t) {
                                    rn.call(e, t)
                                }))
                            }
                        }
                    }

                    function dn(e) {
                        e = e || {}, e.tables = e.tables || {}, e.empty || (vt(e.familyName, "When creating a new Font object, familyName is required."), vt(e.styleName, "When creating a new Font object, styleName is required."), vt(e.unitsPerEm, "When creating a new Font object, unitsPerEm is required."), vt(e.ascender, "When creating a new Font object, ascender is required."), vt(e.descender <= 0, "When creating a new Font object, negative descender value is required."), this.names = {
                            fontFamily: {
                                en: e.familyName || " "
                            },
                            fontSubfamily: {
                                en: e.styleName || " "
                            },
                            fullName: {
                                en: e.fullName || e.familyName + " " + e.styleName
                            },
                            postScriptName: {
                                en: e.postScriptName || (e.familyName + e.styleName).replace(/\s/g, "")
                            },
                            designer: {
                                en: e.designer || " "
                            },
                            designerURL: {
                                en: e.designerURL || " "
                            },
                            manufacturer: {
                                en: e.manufacturer || " "
                            },
                            manufacturerURL: {
                                en: e.manufacturerURL || " "
                            },
                            license: {
                                en: e.license || " "
                            },
                            licenseURL: {
                                en: e.licenseURL || " "
                            },
                            version: {
                                en: e.version || "Version 0.1"
                            },
                            description: {
                                en: e.description || " "
                            },
                            copyright: {
                                en: e.copyright || " "
                            },
                            trademark: {
                                en: e.trademark || " "
                            }
                        }, this.unitsPerEm = e.unitsPerEm || 1e3, this.ascender = e.ascender, this.descender = e.descender, this.createdTimestamp = e.createdTimestamp, this.tables = Object.assign(e.tables, {
                            os2: Object.assign({
                                usWeightClass: e.weightClass || this.usWeightClasses.MEDIUM,
                                usWidthClass: e.widthClass || this.usWidthClasses.MEDIUM,
                                fsSelection: e.fsSelection || this.fsSelectionValues.REGULAR
                            }, e.tables.os2)
                        })), this.supported = !0, this.glyphs = new ue.GlyphSet(this, e.glyphs || []), this.encoding = new ge(this), this.position = new Nt(this), this.substitution = new Ht(this), this.tables = this.tables || {}, this._push = null, this._hmtxTableData = {}, Object.defineProperty(this, "hinting", {
                            get: function() {
                                return this._hinting ? this._hinting : "truetype" === this.outlinesFormat ? this._hinting = new Lt(this) : void 0
                            }
                        })
                    }

                    function bn(e, t) {
                        var r = JSON.stringify(e),
                            n = 256;
                        for (var h in t) {
                            var a = parseInt(h);
                            if (a && !(a < 256)) {
                                if (JSON.stringify(t[h]) === r) return a;
                                n <= a && (n = a + 1)
                            }
                        }
                        return t[n] = e, n
                    }

                    function gn(e, t, r) {
                        var n = bn(t.name, r);
                        return [{
                            name: "tag_" + e,
                            type: "TAG",
                            value: t.tag
                        }, {
                            name: "minValue_" + e,
                            type: "FIXED",
                            value: t.minValue << 16
                        }, {
                            name: "defaultValue_" + e,
                            type: "FIXED",
                            value: t.defaultValue << 16
                        }, {
                            name: "maxValue_" + e,
                            type: "FIXED",
                            value: t.maxValue << 16
                        }, {
                            name: "flags_" + e,
                            type: "USHORT",
                            value: 0
                        }, {
                            name: "nameID_" + e,
                            type: "USHORT",
                            value: n
                        }]
                    }

                    function jn(e, t, r) {
                        var n = {},
                            h = new ce.Parser(e, t);
                        return n.tag = h.parseTag(), n.minValue = h.parseFixed(), n.defaultValue = h.parseFixed(), n.maxValue = h.parseFixed(), h.skip("uShort", 1), n.name = r[h.parseUShort()] || {}, n
                    }

                    function xn(e, t, r, n) {
                        for (var h = bn(t.name, n), a = [{
                                name: "nameID_" + e,
                                type: "USHORT",
                                value: h
                            }, {
                                name: "flags_" + e,
                                type: "USHORT",
                                value: 0
                            }], i = 0; i < r.length; ++i) {
                            var c = r[i].tag;
                            a.push({
                                name: "axis_" + e + " " + c,
                                type: "FIXED",
                                value: t.coordinates[c] << 16
                            })
                        }
                        return a
                    }

                    function Vn(e, t, r, n) {
                        var h = {},
                            a = new ce.Parser(e, t);
                        h.name = n[a.parseUShort()] || {}, a.skip("uShort", 1), h.coordinates = {};
                        for (var i = 0; i < r.length; ++i) h.coordinates[r[i].tag] = a.parseFixed();
                        return h
                    }
                    nn.prototype.setText = function(e) {
                        this.text = e
                    }, nn.prototype.contextChecks = {
                        latinWordCheck: en,
                        arabicWordCheck: _r,
                        arabicSentenceCheck: Jr
                    }, nn.prototype.registerFeatures = function(e, t) {
                        var r = this,
                            n = t.filter((function(t) {
                                return r.query.supports({
                                    script: e,
                                    tag: t
                                })
                            }));
                        this.featuresTags.hasOwnProperty(e) ? this.featuresTags[e] = this.featuresTags[e].concat(n) : this.featuresTags[e] = n
                    }, nn.prototype.applyFeatures = function(e, t) {
                        if (!e) throw new Error("No valid font was provided to apply features");
                        this.query || (this.query = new Yr(e));
                        for (var r = 0; r < t.length; r++) {
                            var n = t[r];
                            this.query.supports({
                                script: n.script
                            }) && this.registerFeatures(n.script, n.tags)
                        }
                    }, nn.prototype.registerModifier = function(e, t, r) {
                        this.tokenizer.registerModifier(e, t, r)
                    }, nn.prototype.checkContextReady = function(e) {
                        return !!this.tokenizer.getContext(e)
                    }, nn.prototype.applyFeaturesToContexts = function() {
                        this.checkContextReady("arabicWord") && (sn.call(this), ln.call(this)), this.checkContextReady("latinWord") && pn.call(this), this.checkContextReady("arabicSentence") && cn.call(this)
                    }, nn.prototype.processText = function(e) {
                        this.text && this.text === e || (this.setText(e), an.call(this), this.applyFeaturesToContexts())
                    }, nn.prototype.getBidiText = function(e) {
                        return this.processText(e), this.tokenizer.getText()
                    }, nn.prototype.getTextGlyphs = function(e) {
                        this.processText(e);
                        for (var t = [], r = 0; r < this.tokenizer.tokens.length; r++) {
                            var n = this.tokenizer.tokens[r];
                            if (!n.state.deleted) {
                                var h = n.activeState.value;
                                t.push(Array.isArray(h) ? h[0] : h)
                            }
                        }
                        return t
                    }, dn.prototype.hasChar = function(e) {
                        return null !== this.encoding.charToGlyphIndex(e)
                    }, dn.prototype.charToGlyphIndex = function(e) {
                        return this.encoding.charToGlyphIndex(e)
                    }, dn.prototype.charToGlyph = function(e) {
                        var t = this.charToGlyphIndex(e),
                            r = this.glyphs.get(t);
                        return r || (r = this.glyphs.get(0)), r
                    }, dn.prototype.updateFeatures = function(e) {
                        return this.defaultRenderOptions.features.map((function(t) {
                            return "latn" === t.script ? {
                                script: "latn",
                                tags: t.tags.filter((function(t) {
                                    return e[t]
                                }))
                            } : t
                        }))
                    }, dn.prototype.stringToGlyphs = function(e, t) {
                        var r = this,
                            n = new nn;
                        n.registerModifier("glyphIndex", null, (function(e) {
                            return r.charToGlyphIndex(e.char)
                        }));
                        var h = t ? this.updateFeatures(t.features) : this.defaultRenderOptions.features;
                        n.applyFeatures(this, h);
                        for (var a = n.getTextGlyphs(e), i = a.length, c = new Array(i), o = this.glyphs.get(0), s = 0; s < i; s += 1) c[s] = this.glyphs.get(a[s]) || o;
                        return c
                    }, dn.prototype.nameToGlyphIndex = function(e) {
                        return this.glyphNames.nameToGlyphIndex(e)
                    }, dn.prototype.nameToGlyph = function(e) {
                        var t = this.nameToGlyphIndex(e),
                            r = this.glyphs.get(t);
                        return r || (r = this.glyphs.get(0)), r
                    }, dn.prototype.glyphIndexToName = function(e) {
                        return this.glyphNames.glyphIndexToName ? this.glyphNames.glyphIndexToName(e) : ""
                    }, dn.prototype.getKerningValue = function(e, t) {
                        e = e.index || e, t = t.index || t;
                        var r = this.position.defaultKerningTables;
                        return r ? this.position.getKerningValue(r, e, t) : this.kerningPairs[e + "," + t] || 0
                    }, dn.prototype.defaultRenderOptions = {
                        kerning: !0,
                        features: [{
                            script: "arab",
                            tags: ["init", "medi", "fina", "rlig"]
                        }, {
                            script: "latn",
                            tags: ["liga", "rlig"]
                        }]
                    }, dn.prototype.forEachGlyph = function(e, t, r, n, h, a) {
                        t = void 0 !== t ? t : 0, r = void 0 !== r ? r : 0, n = void 0 !== n ? n : 72, h = Object.assign({}, this.defaultRenderOptions, h);
                        var i, c = 1 / this.unitsPerEm * n,
                            o = this.stringToGlyphs(e, h);
                        if (h.kerning) {
                            var s = h.script || this.position.getDefaultScriptName();
                            i = this.position.getKerningTables(s, h.language)
                        }
                        for (var l = 0; l < o.length; l += 1) {
                            var p = o[l];
                            if (a.call(this, p, t, r, n, h), p.advanceWidth && (t += p.advanceWidth * c), h.kerning && l < o.length - 1) {
                                var d = i ? this.position.getKerningValue(i, p.index, o[l + 1].index) : this.getKerningValue(p, o[l + 1]);
                                t += d * c
                            }
                            h.letterSpacing ? t += h.letterSpacing * n : h.tracking && (t += h.tracking / 1e3 * n)
                        }
                        return t
                    }, dn.prototype.getPath = function(e, t, r, n, h) {
                        var a = new R;
                        return this.forEachGlyph(e, t, r, n, h, (function(e, t, r, n) {
                            var i = e.getPath(t, r, n, h, this);
                            a.extend(i)
                        })), a
                    }, dn.prototype.getPaths = function(e, t, r, n, h) {
                        var a = [];
                        return this.forEachGlyph(e, t, r, n, h, (function(e, t, r, n) {
                            var i = e.getPath(t, r, n, h, this);
                            a.push(i)
                        })), a
                    }, dn.prototype.getAdvanceWidth = function(e, t, r) {
                        return this.forEachGlyph(e, 0, 0, t, r, (function() {}))
                    }, dn.prototype.draw = function(e, t, r, n, h, a) {
                        this.getPath(t, r, n, h, a).draw(e)
                    }, dn.prototype.drawPoints = function(e, t, r, n, h, a) {
                        this.forEachGlyph(t, r, n, h, a, (function(t, r, n, h) {
                            t.drawPoints(e, r, n, h)
                        }))
                    }, dn.prototype.drawMetrics = function(e, t, r, n, h, a) {
                        this.forEachGlyph(t, r, n, h, a, (function(t, r, n, h) {
                            t.drawMetrics(e, r, n, h)
                        }))
                    }, dn.prototype.getEnglishName = function(e) {
                        var t = this.names[e];
                        if (t) return t.en
                    }, dn.prototype.validate = function() {
                        var e = this;

                        function t(t) {
                            var r = e.getEnglishName(t);
                            r && r.trim().length
                        }
                        t("fontFamily"), t("weightName"), t("manufacturer"), t("copyright"), t("version"), this.unitsPerEm
                    }, dn.prototype.toTables = function() {
                        return kt.fontToTable(this)
                    }, dn.prototype.toBuffer = function() {
                        return h("warn", "Font.toBuffer is deprecated. Use Font.toArrayBuffer instead.", " at utils/opentype.js:13551"), this.toArrayBuffer()
                    }, dn.prototype.toArrayBuffer = function() {
                        for (var e = this.toTables(), t = e.encode(), r = new ArrayBuffer(t.length), n = new Uint8Array(r), h = 0; h < t.length; h++) n[h] = t[h];
                        return r
                    }, dn.prototype.download = function(e) {
                        var t = this.getEnglishName("fontFamily"),
                            r = this.getEnglishName("fontSubfamily");
                        e = e || t.replace(/\s/g, "") + "-" + r + ".otf";
                        var i = this.toArrayBuffer();
                        if (function() {
                                return "undefined" !== typeof window
                            }())
                            if (window.URL = window.URL || window.webkitURL, window.URL) {
                                var c = new DataView(i),
                                    o = new Blob([c], {
                                        type: "font/opentype"
                                    }),
                                    s = document.createElement("a");
                                s.href = window.URL.createObjectURL(o), s.download = e;
                                var l = document.createEvent("MouseEvents");
                                l.initEvent("click", !0, !1), s.dispatchEvent(l)
                            } else h("warn", "Font file could not be downloaded. Try using a different browser.", " at utils/opentype.js:13594");
                        else {
                            var p = n("emptyModuleStub"),
                                d = function(e) {
                                    for (var t = new a(e.byteLength), r = new Uint8Array(e), n = 0; n < t.length; ++n) t[n] = r[n];
                                    return t
                                }(i);
                            p.writeFileSync(e, d)
                        }
                    }, dn.prototype.fsSelectionValues = {
                        ITALIC: 1,
                        UNDERSCORE: 2,
                        NEGATIVE: 4,
                        OUTLINED: 8,
                        STRIKEOUT: 16,
                        BOLD: 32,
                        REGULAR: 64,
                        USER_TYPO_METRICS: 128,
                        WWS: 256,
                        OBLIQUE: 512
                    }, dn.prototype.usWidthClasses = {
                        ULTRA_CONDENSED: 1,
                        EXTRA_CONDENSED: 2,
                        CONDENSED: 3,
                        SEMI_CONDENSED: 4,
                        MEDIUM: 5,
                        SEMI_EXPANDED: 6,
                        EXPANDED: 7,
                        EXTRA_EXPANDED: 8,
                        ULTRA_EXPANDED: 9
                    }, dn.prototype.usWeightClasses = {
                        THIN: 100,
                        EXTRA_LIGHT: 200,
                        LIGHT: 300,
                        NORMAL: 400,
                        MEDIUM: 500,
                        SEMI_BOLD: 600,
                        BOLD: 700,
                        EXTRA_BOLD: 800,
                        BLACK: 900
                    };
                    var fn = {
                            make: function(e, t) {
                                var r = new $.Table("fvar", [{
                                    name: "version",
                                    type: "ULONG",
                                    value: 65536
                                }, {
                                    name: "offsetToData",
                                    type: "USHORT",
                                    value: 0
                                }, {
                                    name: "countSizePairs",
                                    type: "USHORT",
                                    value: 2
                                }, {
                                    name: "axisCount",
                                    type: "USHORT",
                                    value: e.axes.length
                                }, {
                                    name: "axisSize",
                                    type: "USHORT",
                                    value: 20
                                }, {
                                    name: "instanceCount",
                                    type: "USHORT",
                                    value: e.instances.length
                                }, {
                                    name: "instanceSize",
                                    type: "USHORT",
                                    value: 4 + 4 * e.axes.length
                                }]);
                                r.offsetToData = r.sizeOf();
                                for (var n = 0; n < e.axes.length; n++) r.fields = r.fields.concat(gn(n, e.axes[n], t));
                                for (var h = 0; h < e.instances.length; h++) r.fields = r.fields.concat(xn(h, e.instances[h], e.axes, t));
                                return r
                            },
                            parse: function(e, t, r) {
                                var n = new ce.Parser(e, t),
                                    h = n.parseULong();
                                w.argument(65536 === h, "Unsupported fvar table version.");
                                var a = n.parseOffset16();
                                n.skip("uShort", 1);
                                for (var i = n.parseUShort(), c = n.parseUShort(), o = n.parseUShort(), s = n.parseUShort(), l = [], p = 0; p < i; p++) l.push(jn(e, t + a + p * c, r));
                                for (var d = [], b = t + a + i * c, g = 0; g < o; g++) d.push(Vn(e, b + g * s, l, r));
                                return {
                                    axes: l,
                                    instances: d
                                }
                            }
                        },
                        Fn = function() {
                            return {
                                coverage: this.parsePointer(ae.coverage),
                                attachPoints: this.parseList(ae.pointer(ae.uShortList))
                            }
                        },
                        kn = function() {
                            var e = this.parseUShort();
                            return w.argument(1 === e || 2 === e || 3 === e, "Unsupported CaretValue table version."), 1 === e ? {
                                coordinate: this.parseShort()
                            } : 2 === e ? {
                                pointindex: this.parseShort()
                            } : 3 === e ? {
                                coordinate: this.parseShort()
                            } : void 0
                        },
                        mn = function() {
                            return this.parseList(ae.pointer(kn))
                        },
                        Pn = function() {
                            return {
                                coverage: this.parsePointer(ae.coverage),
                                ligGlyphs: this.parseList(ae.pointer(mn))
                            }
                        },
                        un = function() {
                            return this.parseUShort(), this.parseList(ae.pointer(ae.coverage))
                        };
                    var Xn = {
                            parse: function(e, t) {
                                t = t || 0;
                                var r = new ae(e, t),
                                    n = r.parseVersion(1);
                                w.argument(1 === n || 1.2 === n || 1.3 === n, "Unsupported GDEF table version.");
                                var h = {
                                    version: n,
                                    classDef: r.parsePointer(ae.classDef),
                                    attachList: r.parsePointer(Fn),
                                    ligCaretList: r.parsePointer(Pn),
                                    markAttachClassDef: r.parsePointer(ae.classDef)
                                };
                                return n >= 1.2 && (h.markGlyphSets = r.parsePointer(un)), h
                            }
                        },
                        Nn = new Array(10);
                    Nn[1] = function() {
                        var e = this.offset + this.relativeOffset,
                            t = this.parseUShort();
                        return 1 === t ? {
                            posFormat: 1,
                            coverage: this.parsePointer(ae.coverage),
                            value: this.parseValueRecord()
                        } : 2 === t ? {
                            posFormat: 2,
                            coverage: this.parsePointer(ae.coverage),
                            values: this.parseValueRecordList()
                        } : void w.assert(!1, "0x" + e.toString(16) + ": GPOS lookup type 1 format must be 1 or 2.")
                    }, Nn[2] = function() {
                        var e = this.offset + this.relativeOffset,
                            t = this.parseUShort();
                        w.assert(1 === t || 2 === t, "0x" + e.toString(16) + ": GPOS lookup type 2 format must be 1 or 2.");
                        var r = this.parsePointer(ae.coverage),
                            n = this.parseUShort(),
                            h = this.parseUShort();
                        if (1 === t) return {
                            posFormat: t,
                            coverage: r,
                            valueFormat1: n,
                            valueFormat2: h,
                            pairSets: this.parseList(ae.pointer(ae.list((function() {
                                return {
                                    secondGlyph: this.parseUShort(),
                                    value1: this.parseValueRecord(n),
                                    value2: this.parseValueRecord(h)
                                }
                            }))))
                        };
                        if (2 === t) {
                            var a = this.parsePointer(ae.classDef),
                                i = this.parsePointer(ae.classDef),
                                c = this.parseUShort(),
                                o = this.parseUShort();
                            return {
                                posFormat: t,
                                coverage: r,
                                valueFormat1: n,
                                valueFormat2: h,
                                classDef1: a,
                                classDef2: i,
                                class1Count: c,
                                class2Count: o,
                                classRecords: this.parseList(c, ae.list(o, (function() {
                                    return {
                                        value1: this.parseValueRecord(n),
                                        value2: this.parseValueRecord(h)
                                    }
                                })))
                            }
                        }
                    }, Nn[3] = function() {
                        return {
                            error: "GPOS Lookup 3 not supported"
                        }
                    }, Nn[4] = function() {
                        return {
                            error: "GPOS Lookup 4 not supported"
                        }
                    }, Nn[5] = function() {
                        return {
                            error: "GPOS Lookup 5 not supported"
                        }
                    }, Nn[6] = function() {
                        return {
                            error: "GPOS Lookup 6 not supported"
                        }
                    }, Nn[7] = function() {
                        return {
                            error: "GPOS Lookup 7 not supported"
                        }
                    }, Nn[8] = function() {
                        return {
                            error: "GPOS Lookup 8 not supported"
                        }
                    }, Nn[9] = function() {
                        return {
                            error: "GPOS Lookup 9 not supported"
                        }
                    };
                    var Hn = new Array(10);
                    var zn = {
                        parse: function(e, t) {
                            t = t || 0;
                            var r = new ae(e, t),
                                n = r.parseVersion(1);
                            return w.argument(1 === n || 1.1 === n, "Unsupported GPOS table version " + n), 1 === n ? {
                                version: n,
                                scripts: r.parseScriptList(),
                                features: r.parseFeatureList(),
                                lookups: r.parseLookupList(Nn)
                            } : {
                                version: n,
                                scripts: r.parseScriptList(),
                                features: r.parseFeatureList(),
                                lookups: r.parseLookupList(Nn),
                                variations: r.parseFeatureVariationsList()
                            }
                        },
                        make: function(e) {
                            return new $.Table("GPOS", [{
                                name: "version",
                                type: "ULONG",
                                value: 65536
                            }, {
                                name: "scripts",
                                type: "TABLE",
                                value: new $.ScriptList(e.scripts)
                            }, {
                                name: "features",
                                type: "TABLE",
                                value: new $.FeatureList(e.features)
                            }, {
                                name: "lookups",
                                type: "TABLE",
                                value: new $.LookupList(e.lookups, Hn)
                            }])
                        }
                    };
                    var Qn = {
                        parse: function(e, t) {
                            var r = new ce.Parser(e, t),
                                n = r.parseUShort();
                            if (0 === n) return function(e) {
                                var t = {};
                                e.skip("uShort");
                                var r = e.parseUShort();
                                w.argument(0 === r, "Unsupported kern sub-table version."), e.skip("uShort", 2);
                                var n = e.parseUShort();
                                e.skip("uShort", 3);
                                for (var h = 0; h < n; h += 1) {
                                    var a = e.parseUShort(),
                                        i = e.parseUShort(),
                                        c = e.parseShort();
                                    t[a + "," + i] = c
                                }
                                return t
                            }(r);
                            if (1 === n) return function(e) {
                                var t = {};
                                e.skip("uShort");
                                var r = e.parseULong();
                                r > 1 && h("warn", "Only the first kern subtable is supported.", " at utils/opentype.js:13991"), e.skip("uLong");
                                var n = e.parseUShort(),
                                    a = 255 & n;
                                if (e.skip("uShort"), 0 === a) {
                                    var i = e.parseUShort();
                                    e.skip("uShort", 3);
                                    for (var c = 0; c < i; c += 1) {
                                        var o = e.parseUShort(),
                                            s = e.parseUShort(),
                                            l = e.parseShort();
                                        t[o + "," + s] = l
                                    }
                                }
                                return t
                            }(r);
                            throw new Error("Unsupported kern table version (" + n + ").")
                        }
                    };
                    var Rn = {
                        parse: function(e, t, r, n) {
                            for (var h = new ce.Parser(e, t), a = n ? h.parseUShort : h.parseULong, i = [], c = 0; c < r + 1; c += 1) {
                                var o = a.call(h);
                                n && (o *= 2), i.push(o)
                            }
                            return i
                        }
                    };

                    function vn(e, t) {
                        return t(null, Rt(e))
                    }

                    function In(e, t) {
                        for (var r = [], n = 12, h = 0; h < t; h += 1) {
                            var a = ce.getTag(e, n),
                                i = ce.getULong(e, n + 4),
                                c = ce.getULong(e, n + 8),
                                o = ce.getULong(e, n + 12);
                            r.push({
                                tag: a,
                                checksum: i,
                                offset: c,
                                length: o,
                                compression: !1
                            }), n += 16
                        }
                        return r
                    }

                    function wn(e, t) {
                        if ("WOFF" === t.compression) {
                            var r = new Uint8Array(e.buffer, t.offset + 2, t.compressedLength - 2),
                                n = new Uint8Array(t.length);
                            if (H(r, n), n.byteLength !== t.length) throw new Error("Decompression error: " + t.tag + " decompressed length doesn't match recorded length");
                            var h = new DataView(n.buffer, 0);
                            return {
                                data: h,
                                offset: 0
                            }
                        }
                        return {
                            data: e,
                            offset: t.offset
                        }
                    }

                    function yn(e, t) {
                        var r, n;
                        t = void 0 === t || null === t ? {} : t;
                        var h, a, i, c, o, s, l, p, d, b, g, j, x, V = new dn({
                                empty: !0
                            }),
                            f = new DataView(e, 0),
                            F = [],
                            k = ce.getTag(f, 0);
                        if (k === String.fromCharCode(0, 1, 0, 0) || "true" === k || "typ1" === k) V.outlinesFormat = "truetype", h = ce.getUShort(f, 4), F = In(f, h);
                        else if ("OTTO" === k) V.outlinesFormat = "cff", h = ce.getUShort(f, 4), F = In(f, h);
                        else {
                            if ("wOFF" !== k) throw new Error("Unsupported OpenType signature " + k);
                            var m = ce.getTag(f, 4);
                            if (m === String.fromCharCode(0, 1, 0, 0)) V.outlinesFormat = "truetype";
                            else {
                                if ("OTTO" !== m) throw new Error("Unsupported OpenType flavor " + k);
                                V.outlinesFormat = "cff"
                            }
                            h = ce.getUShort(f, 12), F = function(e, t) {
                                for (var r = [], n = 44, h = 0; h < t; h += 1) {
                                    var a = ce.getTag(e, n),
                                        i = ce.getULong(e, n + 4),
                                        c = ce.getULong(e, n + 8),
                                        o = ce.getULong(e, n + 12),
                                        s = void 0;
                                    s = c < o && "WOFF", r.push({
                                        tag: a,
                                        offset: i,
                                        compression: s,
                                        compressedLength: c,
                                        length: o
                                    }), n += 20
                                }
                                return r
                            }(f, h)
                        }
                        for (var P = 0; P < h; P += 1) {
                            var u = F[P],
                                X = void 0;
                            switch (u.tag) {
                                case "cmap":
                                    X = wn(f, u), V.tables.cmap = se.parse(X.data, X.offset), V.encoding = new je(V.tables.cmap);
                                    break;
                                case "cvt ":
                                    X = wn(f, u), x = new ce.Parser(X.data, X.offset), V.tables.cvt = x.parseShortList(u.length / 2);
                                    break;
                                case "fvar":
                                    i = u;
                                    break;
                                case "fpgm":
                                    X = wn(f, u), x = new ce.Parser(X.data, X.offset), V.tables.fpgm = x.parseByteList(u.length);
                                    break;
                                case "head":
                                    X = wn(f, u), V.tables.head = qe.parse(X.data, X.offset), V.unitsPerEm = V.tables.head.unitsPerEm, r = V.tables.head.indexToLocFormat;
                                    break;
                                case "hhea":
                                    X = wn(f, u), V.tables.hhea = Ge.parse(X.data, X.offset), V.ascender = V.tables.hhea.ascender, V.descender = V.tables.hhea.descender, V.numberOfHMetrics = V.tables.hhea.numberOfHMetrics;
                                    break;
                                case "hmtx":
                                    p = u;
                                    break;
                                case "ltag":
                                    X = wn(f, u), n = We.parse(X.data, X.offset);
                                    break;
                                case "maxp":
                                    X = wn(f, u), V.tables.maxp = _e.parse(X.data, X.offset), V.numGlyphs = V.tables.maxp.numGlyphs;
                                    break;
                                case "name":
                                    g = u;
                                    break;
                                case "OS/2":
                                    X = wn(f, u), V.tables.os2 = ct.parse(X.data, X.offset);
                                    break;
                                case "post":
                                    X = wn(f, u), V.tables.post = ot.parse(X.data, X.offset), V.glyphNames = new Ve(V.tables.post);
                                    break;
                                case "prep":
                                    X = wn(f, u), x = new ce.Parser(X.data, X.offset), V.tables.prep = x.parseByteList(u.length);
                                    break;
                                case "glyf":
                                    c = u;
                                    break;
                                case "loca":
                                    b = u;
                                    break;
                                case "CFF ":
                                    a = u;
                                    break;
                                case "kern":
                                    d = u;
                                    break;
                                case "GDEF":
                                    o = u;
                                    break;
                                case "GPOS":
                                    s = u;
                                    break;
                                case "GSUB":
                                    l = u;
                                    break;
                                case "meta":
                                    j = u;
                                    break
                            }
                        }
                        var N = wn(f, g);
                        if (V.tables.name = at.parse(N.data, N.offset, n), V.names = V.tables.name, c && b) {
                            var H = 0 === r,
                                z = wn(f, b),
                                Q = Rn.parse(z.data, z.offset, V.numGlyphs, H),
                                R = wn(f, c);
                            V.glyphs = Tt.parse(R.data, R.offset, Q, V, t)
                        } else {
                            if (!a) throw new Error("Font doesn't contain TrueType or CFF outlines.");
                            var v = wn(f, a);
                            Me.parse(v.data, v.offset, V, t)
                        }
                        var I = wn(f, p);
                        if (Ee.parse(V, I.data, I.offset, V.numberOfHMetrics, V.numGlyphs, V.glyphs, t), fe(V, t), d) {
                            var w = wn(f, d);
                            V.kerningPairs = Qn.parse(w.data, w.offset)
                        } else V.kerningPairs = {};
                        if (o) {
                            var y = wn(f, o);
                            V.tables.gdef = Xn.parse(y.data, y.offset)
                        }
                        if (s) {
                            var C = wn(f, s);
                            V.tables.gpos = zn.parse(C.data, C.offset), V.position.init()
                        }
                        if (l) {
                            var A = wn(f, l);
                            V.tables.gsub = dt.parse(A.data, A.offset)
                        }
                        if (i) {
                            var D = wn(f, i);
                            V.tables.fvar = fn.parse(D.data, D.offset, V.names)
                        }
                        if (j) {
                            var S = wn(f, j);
                            V.tables.meta = bt.parse(S.data, S.offset), V.metas = V.tables.meta
                        }
                        return V
                    }

                    function Cn(t, r, n) {
                        n = void 0 === n || null === n ? {} : n;
                        var h = vn;
                        return new Promise((function(a, i) {
                            h(t, (function(t, h) {
                                if (t) {
                                    if (r) return r(t);
                                    i(t)
                                }
                                var c;
                                try {
                                    c = yn(h, n)
                                } catch (e) {
                                    if (r) return r(e, null);
                                    i(e)
                                }
                                if (r) return r(null, c);
                                a(c)
                            }))
                        }))
                    }

                    function An(e, t) {
                        var r = n("emptyModuleStub"),
                            h = r.readFileSync(e);
                        return yn(Rt(h), t)
                    }
                    var Dn = Object.freeze({
                        __proto__: null,
                        Font: dn,
                        Glyph: ke,
                        Path: R,
                        BoundingBox: Q,
                        _parse: ce,
                        parse: yn,
                        load: Cn,
                        loadSync: An
                    });
                    t.BoundingBox = Q, t.Font = dn, t.Glyph = ke, t.Path = R, t._parse = ce, t.default = Dn, t.load = Cn, t.loadSync = An, t.parse = yn, Object.defineProperty(t, "__esModule", {
                        value: !0
                    })
                }))
            }).call(this, n("enhancedConsoleLogger")["default"], n("buffer").Buffer)
        },