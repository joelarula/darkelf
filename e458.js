        e458: function(e, t, r) {
            (function(t) {
                var r = function(e) {
                    return e = e.toString(), e[1] ? e : "0" + e
                };
                e.exports = {
                    update: function(e) {},
                    lineTheta: function(e, t, r) {
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
                            c = Math.sqrt(Math.pow(h.x, 2) + Math.pow(h.y, 2)),
                            o = Math.acos(a / (i * c));
                        return o
                    },
                    formatTime: function(e) {
                        var t = e.getFullYear(),
                            n = e.getMonth() + 1,
                            h = e.getDate(),
                            a = e.getHours(),
                            i = e.getMinutes(),
                            c = e.getSeconds();
                        return [t, n, h].map(r).join("/") + " " + [a, i, c].map(r).join(":")
                    },
                    getstr: function() {
                        return "lisn3188 is ok"
                    },
                    strToUUID: function(e) {
                        var r, n = e.length;
                        if (n <= 0) return "";
                        var h = "";
                        for (r = 0; r < n; r++) "0123456789-abcdefABCDEF".indexOf(e.charAt(r)) < 0 ? t("log", "error = " + e.charAt(r), " at utils/util.js:29") : h += e.charAt(r);
                        return h
                    },
                    isUUID: function(e) {
                        t("log", "input  = " + e, " at utils/util.js:38");
                        var r = e.split("-", -1);
                        return 5 == r.length && (8 == r[0].length && 4 == r[1].length && 4 == r[2].length && 4 == r[3].length && 12 == r[4].length)
                    }
                }
            }).call(this, r("f3b9")["default"])
        },