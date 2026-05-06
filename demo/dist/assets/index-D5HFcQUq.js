(async ()=>{
    (function() {
        const n = document.createElement("link").relList;
        if (n && n.supports && n.supports("modulepreload")) return;
        for (const i of document.querySelectorAll('link[rel="modulepreload"]'))t(i);
        new MutationObserver((i)=>{
            for (const s of i)if (s.type === "childList") for (const a of s.addedNodes)a.tagName === "LINK" && a.rel === "modulepreload" && t(a);
        }).observe(document, {
            childList: !0,
            subtree: !0
        });
        function o(i) {
            const s = {};
            return i.integrity && (s.integrity = i.integrity), i.referrerPolicy && (s.referrerPolicy = i.referrerPolicy), i.crossOrigin === "use-credentials" ? s.credentials = "include" : i.crossOrigin === "anonymous" ? s.credentials = "omit" : s.credentials = "same-origin", s;
        }
        function t(i) {
            if (i.ep) return;
            i.ep = !0;
            const s = o(i);
            fetch(i.href, s);
        }
    })();
    const Ue = [
        "#4a9eff",
        "#ff6b6b",
        "#51cf66",
        "#fcc419",
        "#cc5de8",
        "#20c997",
        "#ff922b",
        "#a9e34b",
        "#e599f7",
        "#66d9e8"
    ], ct = "#1a1a1a", dt = "#2a2a2a", Ze = 4, ut = 2;
    function V(e) {
        const n = e.getContext("2d"), o = window.devicePixelRatio || 1, t = e.getBoundingClientRect();
        return e.width = t.width * o, e.height = t.height * o, n.scale(o, o), {
            ctx: n,
            width: t.width,
            height: t.height,
            scale: 1,
            offsetX: t.width / 2,
            offsetY: t.height / 2
        };
    }
    function fe(e, n, o = 40) {
        const t = n.flat();
        if (t.length === 0) return;
        let i = 1 / 0, s = -1 / 0, a = 1 / 0, d = -1 / 0;
        for (const [c, r] of t)c < i && (i = c), c > s && (s = c), r < a && (a = r), r > d && (d = r);
        const m = s - i || 1, p = d - a || 1, l = e.width - o * 2, g = e.height - o * 2;
        e.scale = Math.min(l / m, g / p), e.offsetX = o + (l - m * e.scale) / 2 - i * e.scale, e.offsetY = o + (g - p * e.scale) / 2 + d * e.scale;
    }
    function W(e, n, o) {
        return [
            n * e.scale + e.offsetX,
            -o * e.scale + e.offsetY
        ];
    }
    function ze(e, n, o) {
        return [
            Math.round((n - e.offsetX) / e.scale),
            Math.round(-(o - e.offsetY) / e.scale)
        ];
    }
    function ce(e) {
        e.ctx.clearRect(0, 0, e.width, e.height);
    }
    function de(e, n = 50) {
        const { ctx: o, width: t, height: i } = e;
        o.strokeStyle = ct, o.lineWidth = .5;
        const a = n * e.scale;
        if (a < 10) return;
        const d = e.offsetX % a, m = e.offsetY % a;
        o.beginPath();
        for(let p = d; p < t; p += a)o.moveTo(p, 0), o.lineTo(p, i);
        for(let p = m; p < i; p += a)o.moveTo(0, p), o.lineTo(t, p);
        o.stroke(), o.strokeStyle = dt, o.lineWidth = 1, o.beginPath(), o.moveTo(e.offsetX, 0), o.lineTo(e.offsetX, i), o.moveTo(0, e.offsetY), o.lineTo(t, e.offsetY), o.stroke();
    }
    function N(e, n, o = {}) {
        if (n.length < 2) return;
        const { ctx: t } = e, { fill: i, stroke: s = "#4a9eff", lineWidth: a = ut, showVertices: d = !0, vertexColor: m, closed: p = !0, dashed: l = !1 } = o;
        t.beginPath(), l && t.setLineDash([
            6,
            4
        ]);
        const [g, c] = W(e, n[0][0], n[0][1]);
        t.moveTo(g, c);
        for(let r = 1; r < n.length; r++){
            const [u, w] = W(e, n[r][0], n[r][1]);
            t.lineTo(u, w);
        }
        if (p && t.closePath(), i && (t.fillStyle = i, t.globalAlpha = .15, t.fill(), t.globalAlpha = 1), t.strokeStyle = s, t.lineWidth = a, t.stroke(), t.setLineDash([]), d) for (const [r, u] of n){
            const [w, h] = W(e, r, u);
            t.beginPath(), t.arc(w, h, Ze, 0, Math.PI * 2), t.fillStyle = m || s, t.fill();
        }
    }
    function ne(e, n, o, t = "#fff", i = Ze) {
        const [s, a] = W(e, n, o);
        e.ctx.beginPath(), e.ctx.arc(s, a, i, 0, Math.PI * 2), e.ctx.fillStyle = t, e.ctx.fill();
    }
    function K(e, n, o, t, i = "#aaa", s = [
        8,
        -8
    ]) {
        const [a, d] = W(e, n, o);
        e.ctx.font = "11px SF Mono, Fira Code, monospace", e.ctx.fillStyle = i, e.ctx.fillText(t, a + s[0], d + s[1]);
    }
    function ft(e, n, o, t = "#4a9eff", i = 8) {
        const [s, a] = W(e, n[0], n[1]), [d, m] = W(e, o[0], o[1]), p = Math.atan2(m - a, d - s), { ctx: l } = e, g = (s + d) / 2, c = (a + m) / 2;
        l.beginPath(), l.moveTo(g + i * Math.cos(p), c + i * Math.sin(p)), l.lineTo(g - i * Math.cos(p - Math.PI / 6), c - i * Math.sin(p - Math.PI / 6)), l.lineTo(g - i * Math.cos(p + Math.PI / 6), c - i * Math.sin(p + Math.PI / 6)), l.closePath(), l.fillStyle = t, l.fill();
    }
    function ae(e, n, o, t = "#4a9eff", i = 2, s = !1) {
        const [a, d] = W(e, n[0], n[1]), [m, p] = W(e, o[0], o[1]), { ctx: l } = e;
        s && l.setLineDash([
            6,
            4
        ]), l.beginPath(), l.moveTo(a, d), l.lineTo(m, p), l.strokeStyle = t, l.lineWidth = i, l.stroke(), l.setLineDash([]);
    }
    function Se(e) {
        return Ue[e % Ue.length];
    }
    function et(e, n, o = {}) {
        for(let t = 0; t < n.length; t++){
            const i = Se(t);
            N(e, n[t], {
                fill: i,
                stroke: i,
                showVertices: o.showVertices ?? !0
            });
        }
    }
    const gt = 12;
    function me(e, n) {
        let o = [], t = !1, i = null, s = null, a = !1;
        function d(l) {
            if (!a) return;
            const g = e.getBoundingClientRect(), c = l.clientX - g.left, r = l.clientY - g.top, u = n(), [w, h] = ze(u, c, r);
            if (o.length >= 3) {
                const [_, B] = [
                    o[0][0] * u.scale + u.offsetX,
                    -o[0][1] * u.scale + u.offsetY
                ], v = c - _, b = r - B;
                if (Math.sqrt(v * v + b * b) < gt) {
                    t = !1, s?.(o.slice());
                    return;
                }
            }
            t = !0, o.push([
                w,
                h
            ]), i?.(o.slice());
        }
        function m(l) {
            l.preventDefault(), !(!a || o.length === 0) && o.length >= 3 && (t = !1, s?.(o.slice()));
        }
        return {
            get points () {
                return o;
            },
            get isDrawing () {
                return t;
            },
            enable () {
                a || (a = !0, e.addEventListener("click", d), e.addEventListener("contextmenu", m));
            },
            disable () {
                a = !1, e.removeEventListener("click", d), e.removeEventListener("contextmenu", m);
            },
            clear () {
                o = [], t = !1, i?.(o.slice());
            },
            setOnChange (l) {
                i = l;
            },
            setOnComplete (l) {
                s = l;
            }
        };
    }
    const G = [
        {
            name: "Triangle",
            points: [
                [
                    0,
                    0
                ],
                [
                    100,
                    0
                ],
                [
                    50,
                    80
                ]
            ]
        },
        {
            name: "Square",
            points: [
                [
                    0,
                    0
                ],
                [
                    100,
                    0
                ],
                [
                    100,
                    100
                ],
                [
                    0,
                    100
                ]
            ]
        },
        {
            name: "Convex Pentagon",
            points: [
                [
                    50,
                    0
                ],
                [
                    100,
                    35
                ],
                [
                    80,
                    90
                ],
                [
                    20,
                    90
                ],
                [
                    0,
                    35
                ]
            ]
        },
        {
            name: "L-Shape",
            points: [
                [
                    0,
                    0
                ],
                [
                    60,
                    0
                ],
                [
                    60,
                    40
                ],
                [
                    30,
                    40
                ],
                [
                    30,
                    80
                ],
                [
                    0,
                    80
                ]
            ]
        },
        {
            name: "Star",
            points: [
                [
                    50,
                    0
                ],
                [
                    62,
                    35
                ],
                [
                    100,
                    35
                ],
                [
                    70,
                    57
                ],
                [
                    80,
                    95
                ],
                [
                    50,
                    72
                ],
                [
                    20,
                    95
                ],
                [
                    30,
                    57
                ],
                [
                    0,
                    35
                ],
                [
                    38,
                    35
                ]
            ]
        },
        {
            name: "Arrow",
            points: [
                [
                    0,
                    30
                ],
                [
                    60,
                    30
                ],
                [
                    60,
                    0
                ],
                [
                    100,
                    50
                ],
                [
                    60,
                    100
                ],
                [
                    60,
                    70
                ],
                [
                    0,
                    70
                ]
            ]
        },
        {
            name: "U-Shape",
            points: [
                [
                    0,
                    0
                ],
                [
                    100,
                    0
                ],
                [
                    100,
                    80
                ],
                [
                    70,
                    80
                ],
                [
                    70,
                    30
                ],
                [
                    30,
                    30
                ],
                [
                    30,
                    80
                ],
                [
                    0,
                    80
                ]
            ]
        },
        {
            name: "Zigzag",
            points: [
                [
                    0,
                    0
                ],
                [
                    30,
                    50
                ],
                [
                    60,
                    0
                ],
                [
                    90,
                    50
                ],
                [
                    120,
                    0
                ],
                [
                    120,
                    40
                ],
                [
                    0,
                    40
                ]
            ]
        },
        {
            name: "Diamond",
            points: [
                [
                    50,
                    0
                ],
                [
                    100,
                    50
                ],
                [
                    50,
                    100
                ],
                [
                    0,
                    50
                ]
            ]
        },
        {
            name: "Concave Hexagon",
            points: [
                [
                    20,
                    0
                ],
                [
                    80,
                    0
                ],
                [
                    100,
                    50
                ],
                [
                    80,
                    100
                ],
                [
                    20,
                    100
                ],
                [
                    40,
                    50
                ]
            ]
        }
    ], Xe = [
        {
            name: "Needle (compactness)",
            points: [
                [
                    0,
                    0
                ],
                [
                    100,
                    0
                ],
                [
                    100,
                    1
                ],
                [
                    0,
                    1
                ]
            ]
        },
        {
            name: "Tiny (edge length)",
            points: [
                [
                    0,
                    0
                ],
                [
                    .5,
                    0
                ],
                [
                    .5,
                    .5
                ],
                [
                    0,
                    .5
                ]
            ]
        },
        {
            name: "Thin Spike (compactness)",
            points: [
                [
                    0,
                    0
                ],
                [
                    80,
                    0
                ],
                [
                    80,
                    5
                ],
                [
                    40,
                    2
                ],
                [
                    0,
                    5
                ]
            ]
        },
        {
            name: "Sliver (compactness)",
            points: [
                [
                    0,
                    0
                ],
                [
                    200,
                    0
                ],
                [
                    200,
                    .5
                ],
                [
                    0,
                    .5
                ]
            ]
        },
        {
            name: "Many Corridors (parts)",
            points: [
                [
                    0,
                    0
                ],
                [
                    100,
                    0
                ],
                [
                    100,
                    10
                ],
                [
                    20,
                    10
                ],
                [
                    20,
                    20
                ],
                [
                    100,
                    20
                ],
                [
                    100,
                    30
                ],
                [
                    20,
                    30
                ],
                [
                    20,
                    40
                ],
                [
                    100,
                    40
                ],
                [
                    100,
                    50
                ],
                [
                    20,
                    50
                ],
                [
                    20,
                    60
                ],
                [
                    100,
                    60
                ],
                [
                    100,
                    70
                ],
                [
                    0,
                    70
                ],
                [
                    0,
                    60
                ],
                [
                    10,
                    60
                ],
                [
                    10,
                    50
                ],
                [
                    0,
                    50
                ],
                [
                    0,
                    40
                ],
                [
                    10,
                    40
                ],
                [
                    10,
                    30
                ],
                [
                    0,
                    30
                ],
                [
                    0,
                    20
                ],
                [
                    10,
                    20
                ],
                [
                    10,
                    10
                ],
                [
                    0,
                    10
                ]
            ]
        }
    ], qe = [
        {
            name: "T-junction (partial edge)",
            parts: [
                [
                    [
                        0,
                        0
                    ],
                    [
                        40,
                        0
                    ],
                    [
                        40,
                        10
                    ],
                    [
                        0,
                        10
                    ]
                ],
                [
                    [
                        10,
                        10
                    ],
                    [
                        30,
                        10
                    ],
                    [
                        20,
                        20
                    ]
                ]
            ]
        },
        {
            name: "Vertex-only contact",
            parts: [
                [
                    [
                        0,
                        0
                    ],
                    [
                        10,
                        0
                    ],
                    [
                        10,
                        10
                    ],
                    [
                        0,
                        10
                    ]
                ],
                [
                    [
                        10,
                        10
                    ],
                    [
                        20,
                        10
                    ],
                    [
                        20,
                        20
                    ],
                    [
                        10,
                        20
                    ]
                ]
            ]
        },
        {
            name: "Disconnected parts",
            parts: [
                [
                    [
                        0,
                        0
                    ],
                    [
                        10,
                        0
                    ],
                    [
                        10,
                        10
                    ],
                    [
                        0,
                        10
                    ]
                ],
                [
                    [
                        30,
                        0
                    ],
                    [
                        40,
                        0
                    ],
                    [
                        40,
                        10
                    ],
                    [
                        30,
                        10
                    ]
                ]
            ]
        },
        {
            name: "Valid L-shape (shared edge)",
            parts: [
                [
                    [
                        0,
                        0
                    ],
                    [
                        20,
                        0
                    ],
                    [
                        20,
                        10
                    ],
                    [
                        10,
                        10
                    ],
                    [
                        0,
                        10
                    ]
                ],
                [
                    [
                        0,
                        10
                    ],
                    [
                        10,
                        10
                    ],
                    [
                        10,
                        20
                    ],
                    [
                        0,
                        20
                    ]
                ]
            ]
        }
    ], Te = [
        {
            name: "Two Squares (overlapping)",
            a: [
                [
                    0,
                    0
                ],
                [
                    60,
                    0
                ],
                [
                    60,
                    60
                ],
                [
                    0,
                    60
                ]
            ],
            b: [
                [
                    30,
                    30
                ],
                [
                    90,
                    30
                ],
                [
                    90,
                    90
                ],
                [
                    30,
                    90
                ]
            ]
        },
        {
            name: "Two Squares (separate)",
            a: [
                [
                    0,
                    0
                ],
                [
                    40,
                    0
                ],
                [
                    40,
                    40
                ],
                [
                    0,
                    40
                ]
            ],
            b: [
                [
                    60,
                    0
                ],
                [
                    100,
                    0
                ],
                [
                    100,
                    40
                ],
                [
                    60,
                    40
                ]
            ]
        },
        {
            name: "Triangle + Square",
            a: [
                [
                    0,
                    0
                ],
                [
                    80,
                    0
                ],
                [
                    80,
                    80
                ],
                [
                    0,
                    80
                ]
            ],
            b: [
                [
                    40,
                    20
                ],
                [
                    120,
                    50
                ],
                [
                    40,
                    80
                ]
            ]
        },
        {
            name: "Touching edges",
            a: [
                [
                    0,
                    0
                ],
                [
                    50,
                    0
                ],
                [
                    50,
                    50
                ],
                [
                    0,
                    50
                ]
            ],
            b: [
                [
                    50,
                    0
                ],
                [
                    100,
                    0
                ],
                [
                    100,
                    50
                ],
                [
                    50,
                    50
                ]
            ]
        }
    ], _t = "/assets/exact_poly_bg-C7v-_mof.wasm", pt = async (e = {}, n)=>{
        let o;
        if (n.startsWith("data:")) {
            const t = n.replace(/^data:.*?base64,/, "");
            let i;
            if (typeof Buffer == "function" && typeof Buffer.from == "function") i = Buffer.from(t, "base64");
            else if (typeof atob == "function") {
                const s = atob(t);
                i = new Uint8Array(s.length);
                for(let a = 0; a < s.length; a++)i[a] = s.charCodeAt(a);
            } else throw new Error("Cannot decode base64-encoded data URL");
            o = await WebAssembly.instantiate(i, e);
        } else {
            const t = await fetch(n), i = t.headers.get("Content-Type") || "";
            if ("instantiateStreaming" in WebAssembly && i.startsWith("application/wasm")) o = await WebAssembly.instantiateStreaming(t, e);
            else {
                const s = await t.arrayBuffer();
                o = await WebAssembly.instantiate(s, e);
            }
        }
        return o.instance.exports;
    };
    function bt(e, n) {
        const o = le(e, f.__wbindgen_malloc, f.__wbindgen_realloc), t = T, i = f.area_display_from_twice_area(o, t, F(n) ? 0 : oe(n));
        if (i[2]) throw S(i[1]);
        return BigInt.asUintN(64, i[0]);
    }
    function mt(e, n) {
        const o = le(e, f.__wbindgen_malloc, f.__wbindgen_realloc), t = T, i = f.areas_conserved_values(o, t, n);
        if (i[2]) throw S(i[1]);
        return i[0] !== 0;
    }
    function ht(e, n) {
        const o = O(e, f.__wbindgen_malloc), t = T, i = f.bayazit_decompose_polygon(o, t, n);
        if (i[2]) throw S(i[1]);
        return S(i[0]);
    }
    function vt(e, n, o, t, i, s) {
        let a, d;
        try {
            const m = f.cross2d(e, n, o, t, i, s);
            return a = m[0], d = m[1], q(m[0], m[1]);
        } finally{
            f.__wbindgen_free(a, d, 1);
        }
    }
    function He(e, n, o, t, i) {
        const s = O(e, f.__wbindgen_malloc), a = T, d = f.decompose_polygon(s, a, n, F(o) ? 16777215 : o ? 1 : 0, F(t) ? 16777215 : t ? 1 : 0, F(i) ? 0 : oe(i));
        if (d[2]) throw S(d[1]);
        return S(d[0]);
    }
    function Je(e) {
        const n = O(e, f.__wbindgen_malloc), o = T, t = f.ear_clip_triangulate_polygon(n, o);
        if (t[2]) throw S(t[1]);
        return S(t[0]);
    }
    function tt(e) {
        const n = O(e, f.__wbindgen_malloc), o = T, t = f.ensure_ccw(n, o);
        if (t[2]) throw S(t[1]);
        return S(t[0]);
    }
    function yt(e) {
        const n = O(e, f.__wbindgen_malloc), o = T, t = f.exact_vertex_partition_polygon(n, o);
        if (t[2]) throw S(t[1]);
        return S(t[0]);
    }
    function wt(e, n) {
        const o = O(e, f.__wbindgen_malloc), t = T, i = O(n, f.__wbindgen_malloc), s = T, a = f.has_exact_shared_edge(o, t, i, s);
        if (a[2]) throw S(a[1]);
        return a[0] !== 0;
    }
    function De(e) {
        const n = O(e, f.__wbindgen_malloc), o = T, t = f.is_ccw(n, o);
        if (t[2]) throw S(t[1]);
        return t[0] !== 0;
    }
    function $e(e) {
        const n = O(e, f.__wbindgen_malloc), o = T, t = f.is_convex(n, o);
        if (t[2]) throw S(t[1]);
        return t[0] !== 0;
    }
    function xt(e, n, o, t, i, s) {
        return f.is_left(e, n, o, t, i, s) !== 0;
    }
    function It(e, n, o, t, i, s) {
        return f.is_reflex(e, n, o, t, i, s) !== 0;
    }
    function Bt(e, n, o, t, i, s) {
        return f.is_right(e, n, o, t, i, s) !== 0;
    }
    function nt(e) {
        const n = O(e, f.__wbindgen_malloc), o = T, t = f.is_simple(n, o);
        if (t[2]) throw S(t[1]);
        return t[0] !== 0;
    }
    function $t(e) {
        const n = O(e, f.__wbindgen_malloc), o = T, t = f.normalize_polygon(n, o);
        if (t[2]) throw S(t[1]);
        return S(t[0]);
    }
    function Et(e) {
        const n = f.optimize_partition(e);
        if (n[2]) throw S(n[1]);
        return S(n[0]);
    }
    function Ke(e, n, o, t, i, s) {
        let a, d;
        try {
            const m = f.orientation(e, n, o, t, i, s);
            return a = m[0], d = m[1], q(m[0], m[1]);
        } finally{
            f.__wbindgen_free(a, d, 1);
        }
    }
    function Re(e) {
        let n, o;
        try {
            const s = O(e, f.__wbindgen_malloc), a = T, d = f.perimeter_l1(s, a);
            var t = d[0], i = d[1];
            if (d[3]) throw t = 0, i = 0, S(d[2]);
            return n = t, o = i, q(t, i);
        } finally{
            f.__wbindgen_free(n, o, 1);
        }
    }
    function Ct(e, n, o) {
        const t = O(o, f.__wbindgen_malloc), i = T, s = f.point_inside_or_on_boundary(e, n, t, i);
        if (s[2]) throw S(s[1]);
        return s[0] !== 0;
    }
    function St(e, n, o) {
        const t = O(o, f.__wbindgen_malloc), i = T, s = f.point_on_polygon_boundary(e, n, t, i);
        if (s[2]) throw S(s[1]);
        return s[0] !== 0;
    }
    function kt(e, n, o) {
        const t = O(o, f.__wbindgen_malloc), i = T, s = f.point_strictly_inside_convex(e, n, t, i);
        if (s[2]) throw S(s[1]);
        return s[0] !== 0;
    }
    function ot(e) {
        const n = O(e, f.__wbindgen_malloc), o = T, t = f.remove_collinear(n, o);
        if (t[2]) throw S(t[1]);
        return S(t[0]);
    }
    function Tt(e, n) {
        const o = O(e, f.__wbindgen_malloc), t = T, i = O(n, f.__wbindgen_malloc), s = T, a = f.sat_overlap(o, t, i, s);
        if (a[2]) throw S(a[1]);
        return a[0] !== 0;
    }
    function At(e, n) {
        const o = O(e, f.__wbindgen_malloc), t = T, i = O(n, f.__wbindgen_malloc), s = T, a = f.sat_overlap_with_aabb(o, t, i, s);
        if (a[2]) throw S(a[1]);
        return a[0] !== 0;
    }
    function Ge(e, n, o, t, i, s, a, d) {
        return f.segments_intersect(e, n, o, t, i, s, a, d) !== 0;
    }
    function Lt(e, n, o, t, i, s, a, d) {
        return f.segments_properly_intersect(e, n, o, t, i, s, a, d) !== 0;
    }
    function Pt(e) {
        let n, o;
        try {
            const s = O(e, f.__wbindgen_malloc), a = T, d = f.signed_area_2x(s, a);
            var t = d[0], i = d[1];
            if (d[3]) throw t = 0, i = 0, S(d[2]);
            return n = t, o = i, q(t, i);
        } finally{
            f.__wbindgen_free(n, o, 1);
        }
    }
    function Ee(e) {
        let n, o;
        try {
            const s = O(e, f.__wbindgen_malloc), a = T, d = f.twice_area(s, a);
            var t = d[0], i = d[1];
            if (d[3]) throw t = 0, i = 0, S(d[2]);
            return n = t, o = i, q(t, i);
        } finally{
            f.__wbindgen_free(n, o, 1);
        }
    }
    function Mt(e, n, o) {
        const t = le(e, f.__wbindgen_malloc, f.__wbindgen_realloc), i = T, s = le(n, f.__wbindgen_malloc, f.__wbindgen_realloc), a = T, d = f.validate_compactness(t, i, s, a, F(o) ? 0 : oe(o));
        if (d[3]) throw S(d[2]);
        let m;
        return d[0] !== 0 && (m = q(d[0], d[1]).slice(), f.__wbindgen_free(d[0], d[1] * 1, 1)), m;
    }
    function Ot(e, n, o) {
        const t = O(e, f.__wbindgen_malloc), i = T, s = f.validate_decomposition(t, i, n, F(o) ? 0 : oe(o));
        if (s[2]) throw S(s[1]);
        return S(s[0]);
    }
    function Nt(e, n) {
        const o = O(e, f.__wbindgen_malloc), t = T, i = f.validate_edge_lengths(o, t, F(n) ? 0 : oe(n));
        if (i[3]) throw S(i[2]);
        let s;
        return i[0] !== 0 && (s = q(i[0], i[1]).slice(), f.__wbindgen_free(i[0], i[1] * 1, 1)), s;
    }
    function Dt(e, n, o) {
        const t = f.validate_multipart_topology(e, F(n) ? 16777215 : n ? 1 : 0, F(o) ? 0 : oe(o));
        if (t[2]) throw S(t[1]);
        return S(t[0]);
    }
    function Rt(e, n) {
        const o = O(e, f.__wbindgen_malloc), t = T, i = f.validate_part(o, t, F(n) ? 0 : oe(n));
        if (i[3]) throw S(i[2]);
        let s;
        return i[0] !== 0 && (s = q(i[0], i[1]).slice(), f.__wbindgen_free(i[0], i[1] * 1, 1)), s;
    }
    function Ft(e, n) {
        return Error(q(e, n));
    }
    function jt(e) {
        return Number(e);
    }
    function Vt(e, n) {
        const o = String(n), t = le(o, f.__wbindgen_malloc, f.__wbindgen_realloc), i = T;
        Q().setInt32(e + 4, i, !0), Q().setInt32(e + 0, t, !0);
    }
    function zt(e, n) {
        const o = n, t = typeof o == "bigint" ? o : void 0;
        Q().setBigInt64(e + 8, F(t) ? BigInt(0) : t, !0), Q().setInt32(e + 0, !F(t), !0);
    }
    function Ht(e) {
        const n = e, o = typeof n == "boolean" ? n : void 0;
        return F(o) ? 16777215 : o ? 1 : 0;
    }
    function Yt(e, n) {
        const o = Fe(n), t = le(o, f.__wbindgen_malloc, f.__wbindgen_realloc), i = T;
        Q().setInt32(e + 4, i, !0), Q().setInt32(e + 0, t, !0);
    }
    function Wt(e, n) {
        return e in n;
    }
    function Ut(e) {
        return typeof e == "bigint";
    }
    function Xt(e) {
        return typeof e == "function";
    }
    function qt(e) {
        const n = e;
        return typeof n == "object" && n !== null;
    }
    function Jt(e) {
        return e === void 0;
    }
    function Kt(e, n) {
        return e === n;
    }
    function Gt(e, n) {
        return e == n;
    }
    function Qt(e, n) {
        const o = n, t = typeof o == "number" ? o : void 0;
        Q().setFloat64(e + 8, F(t) ? 0 : t, !0), Q().setInt32(e + 0, !F(t), !0);
    }
    function Zt(e, n) {
        return e >> n;
    }
    function en(e, n) {
        const o = n, t = typeof o == "string" ? o : void 0;
        var i = F(t) ? 0 : le(t, f.__wbindgen_malloc, f.__wbindgen_realloc), s = T;
        Q().setInt32(e + 4, s, !0), Q().setInt32(e + 0, i, !0);
    }
    function tn(e, n) {
        throw new Error(q(e, n));
    }
    function nn() {
        return Ye(function(e, n) {
            return e.call(n);
        }, arguments);
    }
    function on(e) {
        return e.done;
    }
    function sn() {
        return Ye(function(e, n) {
            return Reflect.get(e, n);
        }, arguments);
    }
    function an(e, n) {
        return e[n >>> 0];
    }
    function rn(e, n) {
        return e[n];
    }
    function ln(e) {
        let n;
        try {
            n = e instanceof ArrayBuffer;
        } catch  {
            n = !1;
        }
        return n;
    }
    function cn(e) {
        let n;
        try {
            n = e instanceof Uint8Array;
        } catch  {
            n = !1;
        }
        return n;
    }
    function dn(e) {
        return Array.isArray(e);
    }
    function un(e) {
        return Number.isSafeInteger(e);
    }
    function fn() {
        return Symbol.iterator;
    }
    function gn(e) {
        return e.length;
    }
    function _n(e) {
        return e.length;
    }
    function pn(e) {
        return new Uint8Array(e);
    }
    function bn() {
        return new Array;
    }
    function mn() {
        return new Object;
    }
    function hn() {
        return Ye(function(e) {
            return e.next();
        }, arguments);
    }
    function vn(e) {
        return e.next;
    }
    function yn(e, n, o) {
        Uint8Array.prototype.set.call(Tn(e, n), o);
    }
    function wn(e, n, o) {
        e[n >>> 0] = o;
    }
    function xn(e, n, o) {
        e[n] = o;
    }
    function In(e) {
        return e.value;
    }
    function Bn(e) {
        return e;
    }
    function $n(e) {
        return e;
    }
    function En(e, n) {
        return q(e, n);
    }
    function Cn(e, n) {
        return BigInt.asUintN(64, e) | BigInt.asUintN(64, n) << BigInt(64);
    }
    function Sn(e) {
        return BigInt.asUintN(64, e);
    }
    function kn() {
        const e = f.__wbindgen_externrefs, n = e.grow(4);
        e.set(0, void 0), e.set(n + 0, void 0), e.set(n + 1, null), e.set(n + 2, !0), e.set(n + 3, !1);
    }
    function oe(e) {
        const n = f.__externref_table_alloc();
        return f.__wbindgen_externrefs.set(n, e), n;
    }
    function Fe(e) {
        const n = typeof e;
        if (n == "number" || n == "boolean" || e == null) return `${e}`;
        if (n == "string") return `"${e}"`;
        if (n == "symbol") {
            const i = e.description;
            return i == null ? "Symbol" : `Symbol(${i})`;
        }
        if (n == "function") {
            const i = e.name;
            return typeof i == "string" && i.length > 0 ? `Function(${i})` : "Function";
        }
        if (Array.isArray(e)) {
            const i = e.length;
            let s = "[";
            i > 0 && (s += Fe(e[0]));
            for(let a = 1; a < i; a++)s += ", " + Fe(e[a]);
            return s += "]", s;
        }
        const o = /\[object ([^\]]+)\]/.exec(toString.call(e));
        let t;
        if (o && o.length > 1) t = o[1];
        else return toString.call(e);
        if (t == "Object") try {
            return "Object(" + JSON.stringify(e) + ")";
        } catch  {
            return "Object";
        }
        return e instanceof Error ? `${e.name}: ${e.message}
${e.stack}` : t;
    }
    function Tn(e, n) {
        return e = e >>> 0, pe().subarray(e / 1, e / 1 + n);
    }
    let ye = null;
    function An() {
        return (ye === null || ye.byteLength === 0) && (ye = new BigUint64Array(f.memory.buffer)), ye;
    }
    let ue = null;
    function Q() {
        return (ue === null || ue.buffer.detached === !0 || ue.buffer.detached === void 0 && ue.buffer !== f.memory.buffer) && (ue = new DataView(f.memory.buffer)), ue;
    }
    function q(e, n) {
        return e = e >>> 0, Pn(e, n);
    }
    let we = null;
    function pe() {
        return (we === null || we.byteLength === 0) && (we = new Uint8Array(f.memory.buffer)), we;
    }
    function Ye(e, n) {
        try {
            return e.apply(this, n);
        } catch (o) {
            const t = oe(o);
            f.__wbindgen_exn_store(t);
        }
    }
    function F(e) {
        return e == null;
    }
    function O(e, n) {
        const o = n(e.length * 8, 8) >>> 0;
        return An().set(e, o / 8), T = e.length, o;
    }
    function le(e, n, o) {
        if (o === void 0) {
            const d = be.encode(e), m = n(d.length, 1) >>> 0;
            return pe().subarray(m, m + d.length).set(d), T = d.length, m;
        }
        let t = e.length, i = n(t, 1) >>> 0;
        const s = pe();
        let a = 0;
        for(; a < t; a++){
            const d = e.charCodeAt(a);
            if (d > 127) break;
            s[i + a] = d;
        }
        if (a !== t) {
            a !== 0 && (e = e.slice(a)), i = o(i, t, t = a + e.length * 3, 1) >>> 0;
            const d = pe().subarray(i + a, i + t), m = be.encodeInto(e, d);
            a += m.written, i = o(i, t, a, 1) >>> 0;
        }
        return T = a, i;
    }
    function S(e) {
        const n = f.__wbindgen_externrefs.get(e);
        return f.__externref_table_dealloc(e), n;
    }
    let Ie = new TextDecoder("utf-8", {
        ignoreBOM: !0,
        fatal: !0
    });
    Ie.decode();
    const Ln = 2146435072;
    let Ae = 0;
    function Pn(e, n) {
        return Ae += n, Ae >= Ln && (Ie = new TextDecoder("utf-8", {
            ignoreBOM: !0,
            fatal: !0
        }), Ie.decode(), Ae = n), Ie.decode(pe().subarray(e, e + n));
    }
    const be = new TextEncoder;
    "encodeInto" in be || (be.encodeInto = function(e, n) {
        const o = be.encode(e);
        return n.set(o), {
            read: e.length,
            written: o.length
        };
    });
    let T = 0, f;
    function Mn(e) {
        f = e;
    }
    URL = globalThis.URL;
    const On = await pt({
        "./exact_poly_bg.js": {
            __wbg_set_3bf1de9fab0cd644: wn,
            __wbg_next_0340c4ae324393c3: hn,
            __wbg_done_9158f7cc8751ba32: on,
            __wbg_value_ee3a06f4579184fa: In,
            __wbg_length_3d4ecd04bd8d22f1: gn,
            __wbg_get_unchecked_17f53dad852b9588: an,
            __wbg_get_with_ref_key_6412cf3094599694: rn,
            __wbg_set_6be42768c690e380: xn,
            __wbg_String_8564e559799eccda: Vt,
            __wbg_new_682678e2f47e32bc: bn,
            __wbg_new_aa8d0fa9762c29bd: mn,
            __wbg_new_0c7403db6e782f19: pn,
            __wbg_length_9f1775224cf1d815: _n,
            __wbg_prototypesetcall_a6b02eb00b0f4ce2: yn,
            __wbg_instanceof_Uint8Array_152ba1f289edcf3f: cn,
            __wbg_instanceof_ArrayBuffer_7c8433c6ed14ffe3: ln,
            __wbg_isArray_c3109d14ffc06469: dn,
            __wbg_isSafeInteger_4fc213d1989d6d2a: un,
            __wbg_iterator_013bc09ec998c2a7: fn,
            __wbg_next_7646edaa39458ef7: vn,
            __wbg_get_1affdbdd5573b16a: sn,
            __wbg_call_14b169f759b26747: nn,
            __wbg___wbindgen_string_get_7ed5322991caaec5: en,
            __wbg___wbindgen_number_get_c7f42aed0525c451: Qt,
            __wbg___wbindgen_in_a5d8b22e52b24dd1: Wt,
            __wbg___wbindgen_shr_436553cbaef41a66: Zt,
            __wbg___wbindgen_throw_6b64449b9b9ed33c: tn,
            __wbg___wbindgen_jsval_eq_d3465d8a07697228: Kt,
            __wbg_Number_32bf70a599af1d4b: jt,
            __wbg_Error_960c155d3d49e4c2: Ft,
            __wbg___wbindgen_is_bigint_ec25c7f91b4d9e93: Ut,
            __wbg___wbindgen_is_object_63322ec0cd6ea4ef: qt,
            __wbg___wbindgen_boolean_get_6ea149f0a8dcc5ff: Ht,
            __wbg___wbindgen_is_function_3baa9db1a987f47d: Xt,
            __wbg___wbindgen_is_undefined_29a43b4d42920abd: Jt,
            __wbg___wbindgen_jsval_loose_eq_cac3565e89b4134c: Gt,
            __wbg___wbindgen_bigint_get_as_i64_3d3aba5d616c6a51: zt,
            __wbg___wbindgen_debug_string_ab4b34d23d6778bd: Yt,
            __wbindgen_init_externref_table: kn,
            __wbindgen_cast_0000000000000001: Bn,
            __wbindgen_cast_0000000000000002: $n,
            __wbindgen_cast_0000000000000003: En,
            __wbindgen_cast_0000000000000004: Cn,
            __wbindgen_cast_0000000000000005: Sn
        }
    }, _t), { memory: Nn, bayazit_decompose_polygon: Dn, classify_contact: Rn, collect_steiner_points: Fn, collinear_segments_overlap_area: jn, contains_polygon: Vn, decompose_polygon: zn, ear_clip_triangulate_polygon: Hn, exact_partition_only_original_vertices: Yn, exact_vertex_partition_polygon: Wn, has_exact_shared_edge: Un, point_inside_any_part: Xn, point_inside_or_on_boundary: qn, point_on_polygon_boundary: Jn, point_strictly_inside_convex: Kn, segments_contact: Gn, validate_decomposition: Qn, validate_multipart_topology: Zn, is_convex: eo, perimeter_l1: to, validate_compactness: no, validate_edge_lengths: oo, validate_part: io, cross2d: so, edge_squared_length: ao, is_collinear_pts: ro, is_left: lo, is_left_or_on: co, is_reflex: uo, is_right_or_on: fo, orientation: go, point_on_segment: _o, segments_intersect: po, segments_properly_intersect: bo, is_right: mo, ensure_ccw: ho, is_ccw: vo, is_simple: yo, normalize_polygon: wo, remove_collinear: xo, rotate_polygon: Io, merge_convex_pair: Bo, optimize_partition: $o, area_display_from_twice_area: Eo, areas_conserved_values: Co, convex_parts_overlap: So, find_overlapping_parts: ko, parts_overlap: To, sat_overlap: Ao, sat_overlap_with_aabb: Lo, signed_area_2x: Po, twice_area: Mo, __wbindgen_malloc: Oo, __wbindgen_realloc: No, __wbindgen_exn_store: Do, __externref_table_alloc: Ro, __wbindgen_externrefs: Fo, __externref_table_dealloc: jo, __wbindgen_free: Vo, __wbindgen_start: it } = On, zo = Object.freeze(Object.defineProperty({
        __proto__: null,
        __externref_table_alloc: Ro,
        __externref_table_dealloc: jo,
        __wbindgen_exn_store: Do,
        __wbindgen_externrefs: Fo,
        __wbindgen_free: Vo,
        __wbindgen_malloc: Oo,
        __wbindgen_realloc: No,
        __wbindgen_start: it,
        area_display_from_twice_area: Eo,
        areas_conserved_values: Co,
        bayazit_decompose_polygon: Dn,
        classify_contact: Rn,
        collect_steiner_points: Fn,
        collinear_segments_overlap_area: jn,
        contains_polygon: Vn,
        convex_parts_overlap: So,
        cross2d: so,
        decompose_polygon: zn,
        ear_clip_triangulate_polygon: Hn,
        edge_squared_length: ao,
        ensure_ccw: ho,
        exact_partition_only_original_vertices: Yn,
        exact_vertex_partition_polygon: Wn,
        find_overlapping_parts: ko,
        has_exact_shared_edge: Un,
        is_ccw: vo,
        is_collinear_pts: ro,
        is_convex: eo,
        is_left: lo,
        is_left_or_on: co,
        is_reflex: uo,
        is_right: mo,
        is_right_or_on: fo,
        is_simple: yo,
        memory: Nn,
        merge_convex_pair: Bo,
        normalize_polygon: wo,
        optimize_partition: $o,
        orientation: go,
        parts_overlap: To,
        perimeter_l1: to,
        point_inside_any_part: Xn,
        point_inside_or_on_boundary: qn,
        point_on_polygon_boundary: Jn,
        point_on_segment: _o,
        point_strictly_inside_convex: Kn,
        remove_collinear: xo,
        rotate_polygon: Io,
        sat_overlap: Ao,
        sat_overlap_with_aabb: Lo,
        segments_contact: Gn,
        segments_intersect: po,
        segments_properly_intersect: bo,
        signed_area_2x: Po,
        twice_area: Mo,
        validate_compactness: no,
        validate_decomposition: Qn,
        validate_edge_lengths: oo,
        validate_multipart_topology: Zn,
        validate_part: io
    }, Symbol.toStringTag, {
        value: "Module"
    }));
    Mn(zo);
    it();
    const Ce = 1e6;
    function D(e) {
        const n = new BigInt64Array(e.length * 2);
        for(let o = 0; o < e.length; o++)n[o * 2] = BigInt(Math.round(e[o][0] * Ce)), n[o * 2 + 1] = BigInt(Math.round(e[o][1] * Ce));
        return n;
    }
    function X(e) {
        const n = [];
        for(let o = 0; o < e.length; o += 2)n.push([
            Number(e[o]) / Ce,
            Number(e[o + 1]) / Ce
        ]);
        return n;
    }
    let st = [], Le = !1;
    const je = new Set;
    function Z() {
        return st;
    }
    function R(e) {
        if (st = e, !Le) {
            Le = !0;
            for (const n of je)n();
            Le = !1;
        }
    }
    function he(e) {
        return je.add(e), ()=>je.delete(e);
    }
    const Be = {
        max_parts: 10,
        max_vertices_per_part: 64,
        min_edge_length_squared: 1e12,
        min_compactness_ppm: 15e4,
        area_divisor: 2e12
    }, Pe = {
        max_parts: 999999,
        max_vertices_per_part: 999999,
        min_edge_length_squared: 0,
        min_compactness_ppm: 0,
        area_divisor: 1
    };
    let re = {
        ...Be
    }, Me = !1;
    const Ve = new Set;
    function at() {
        return re;
    }
    function ee() {
        return {
            max_parts: re.max_parts,
            max_vertices_per_part: re.max_vertices_per_part,
            min_edge_length_squared: BigInt(re.min_edge_length_squared),
            min_compactness_ppm: BigInt(re.min_compactness_ppm),
            area_divisor: BigInt(re.area_divisor)
        };
    }
    function Oe(e) {
        if (re = {
            ...e
        }, !Me) {
            Me = !0;
            for (const n of Ve)n();
            Me = !1;
        }
    }
    function We(e) {
        return Ve.add(e), ()=>Ve.delete(e);
    }
    function Qe(e) {
        if (!e) return "—";
        if (typeof e == "string") return e;
        if ("Rotation" in e) {
            const n = typeof e.Rotation.inner == "string" ? e.Rotation.inner : JSON.stringify(e.Rotation.inner);
            return `Rotation(${e.Rotation.offset}, ${n})`;
        }
        return JSON.stringify(e);
    }
    function Ho(e) {
        return "Success" in e ? "ok" : "TooManyParts" in e ? "warn" : "error";
    }
    function Yo(e) {
        return "Success" in e ? `Success: ${e.Success.part_count} parts` : "TooManyParts" in e ? `TooManyParts: ${e.TooManyParts.count}` : "ValidationFailed" in e ? `ValidationFailed: ${e.ValidationFailed.errors.join(", ")}` : "AlgorithmFailed" in e ? `AlgorithmFailed: "${e.AlgorithmFailed.error}"` : JSON.stringify(e);
    }
    function Wo() {
        let e, n, o, t = [], i = [], s = [], a = [], d, m, p = !1, l = "decompose", g = !0, c = !1, r = !1, u, w, h, _ = null, B = !0, v = !0, b = null, I = null;
        const k = 1e6;
        function A($) {
            if ($.length < 3) return {};
            const y = D($), x = {};
            try {
                x["2x area"] = Ee(y);
            } catch  {}
            try {
                x.CCW = String(De(y));
            } catch  {}
            try {
                x.Simple = String(nt(y));
            } catch  {}
            try {
                x.Convex = String($e(y));
            } catch  {}
            try {
                x["Perimeter L1"] = Re(y);
            } catch  {}
            try {
                const E = Nt(y, ee());
                x["Edge lengths"] = E ?? "OK";
            } catch  {}
            try {
                const E = Ee(y), C = Re(y), M = Mt(E, C, ee());
                x["Compactness (boundary)"] = M ?? "OK";
            } catch  {}
            try {
                const E = Rt(y, ee());
                x["Validate part (structural)"] = E ?? "OK";
            } catch  {}
            try {
                const E = X(ot(y));
                E.length < $.length && (x["Collinear removed"] = `${$.length} → ${E.length}`);
            } catch  {}
            return x;
        }
        function P($) {
            return JSON.stringify($.map(([y, x])=>[
                    y,
                    x
                ]));
        }
        function z($) {
            return "[" + $.map(([y, x])=>`${Math.round(y * k)}, ${Math.round(x * k)}`).join(", ") + "]";
        }
        function J($) {
            const y = [];
            for(let x = 0; x < $.length; x++){
                const E = (x + 1) % $.length, C = $[E][0] - $[x][0], M = $[E][1] - $[x][1], L = C * C + M * M, ie = Math.sqrt(L), _e = BigInt(Math.round(C * k)) ** 2n + BigInt(Math.round(M * k)) ** 2n;
                y.push(`  ${x}→${E}: len=${ie.toFixed(2)} lenSq_wasm=${_e}`);
            }
            return y;
        }
        function j() {
            if (!B) {
                _.style.display = "none";
                return;
            }
            if (_.style.display = "block", t.length < 3) {
                _.innerHTML = `<details class="debug-details" open>
        <summary>Debug</summary>
        <div class="debug-content"><span class="debug-muted">No polygon</span></div>
      </details>`;
                return;
            }
            const $ = at(), y = [];
            y.push(`<div class="debug-section">
      <div class="debug-section-title">Polygon (${t.length} vertices)</div>
      <div class="debug-copy-row">
        <span class="debug-label">Coords:</span>
        <code class="debug-code" title="Click to copy">${Y(P(t))}</code>
      </div>
      <div class="debug-copy-row">
        <span class="debug-label">WASM flat:</span>
        <code class="debug-code" title="Click to copy">${Y(z(t))}</code>
      </div>
      <div class="debug-copy-row">
        <span class="debug-label">Vertices:</span>
        <code class="debug-code">${t.map(([C, M], L)=>`${L}: (${C}, ${M})`).join(`
`)}</code>
      </div>
    </div>`);
            const x = A(t);
            if (p && (x["Normalized to CCW"] = "yes (input was CW — flipped before decompose)"), Object.keys(x).length > 0) {
                const C = Object.entries(x).map(([M, L])=>{
                    const ie = L === "OK" || L.startsWith("OK") ? "debug-ok" : L === "true" ? "" : L === "false" ? "debug-warn" : "";
                    return `<div class="debug-row"><span class="debug-label">${M}</span><span class="debug-value ${ie}">${Y(L)}</span></div>`;
                }).join("");
                y.push(`<div class="debug-section">
        <div class="debug-section-title">Polygon Properties</div>
        ${C}
      </div>`);
            }
            const E = J(t);
            if (y.push(`<div class="debug-section">
      <div class="debug-section-title">Edges</div>
      <code class="debug-code">${E.join(`
`)}</code>
    </div>`), y.push(`<div class="debug-section">
      <div class="debug-section-title">Config</div>
      <code class="debug-code">${Y(JSON.stringify($, null, 2))}</code>
    </div>`), i.length > 0) try {
                const C = D(t), M = i.map((H)=>Array.from(D(H))), L = Ot(C, M, ee()), ie = L.valid ? L.warn_count > 0 ? "PASS (demo only) — has warnings, will fail on-chain with real coords" : "PASS — valid for on-chain" : "FAIL — will reject on-chain", _e = L.valid ? L.warn_count > 0 ? "debug-warn" : "debug-ok" : "debug-fail", ke = L.checks.map((H)=>{
                    let se, ve;
                    return H.severity === "error" && !H.passed ? (se = "debug-fail", ve = "FAIL") : H.severity === "warn" ? (se = "debug-warn", ve = "WARN") : (se = "debug-ok", ve = "OK"), `<div class="debug-row">
            <span class="debug-label"><span class="debug-value ${se}">[${ve}]</span> ${H.name}</span>
            <span class="debug-value ${se}">${Y(H.detail)}</span>
          </div>`;
                }).join("");
                y.push(`<div class="debug-section">
          <div class="debug-section-title">On-chain Validation</div>
          <div class="debug-row">
            <span class="debug-label">Status</span>
            <span class="debug-value ${_e}">${ie}</span>
          </div>
          <div class="debug-row">
            <span class="debug-label">Errors / Warnings</span>
            <span class="debug-value">${L.error_count} / ${L.warn_count}</span>
          </div>
          <div class="debug-row">
            <span class="debug-label">Original 2A</span>
            <span class="debug-value">${L.original_twice_area}</span>
          </div>
          <div class="debug-row">
            <span class="debug-label">Parts sum 2A</span>
            <span class="debug-value">${L.parts_twice_area_sum}</span>
          </div>
          ${ke}
        </div>`);
            } catch (C) {
                y.push(`<div class="debug-section">
          <div class="debug-section-title">On-chain Validation</div>
          <div class="debug-value debug-warn">${Y(String(C))}</div>
        </div>`);
            }
            if (i.length > 0) {
                const C = i.map((M, L)=>{
                    const ie = A(M), _e = Object.entries(ie).map(([ke, H])=>{
                        const se = H === "OK" ? "debug-ok" : H.includes("fail") || H.includes("error") || H === "false" ? "debug-warn" : "";
                        return `<div class="debug-row"><span class="debug-label">${ke}</span><span class="debug-value ${se}">${Y(H)}</span></div>`;
                    }).join("");
                    return `<div class="debug-part">
          <div class="debug-section-title">Part ${L} (${M.length} verts)</div>
          <div class="debug-copy-row">
            <code class="debug-code" title="Click to copy">${Y(P(M))}</code>
          </div>
          <div class="debug-copy-row">
            <span class="debug-label">WASM flat:</span>
            <code class="debug-code" title="Click to copy">${Y(z(M))}</code>
          </div>
          ${_e}
        </div>`;
                }).join("");
                y.push(`<div class="debug-section">
        <div class="debug-section-title">Parts Detail</div>
        ${C}
      </div>`);
            }
            _.innerHTML = `<details class="debug-details" open>
      <summary>Debug</summary>
      <div class="debug-content">${y.join("")}</div>
    </details>`, _.querySelectorAll(".debug-code").forEach((C)=>{
                C.style.cursor = "pointer", C.addEventListener("click", ()=>{
                    navigator.clipboard.writeText(C.textContent || "");
                    const M = C.style.outline;
                    C.style.outline = "1px solid #51cf66", setTimeout(()=>C.style.outline = M, 300);
                });
            });
        }
        function Y($) {
            return $.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
        }
        function U() {
            if (i = [], s = [], a = [], d = void 0, m = void 0, p = !1, w.textContent = "", !(t.length < 3)) {
                try {
                    const $ = D(t);
                    p = !De($);
                    const x = p ? BigInt64Array.from(tt($), (E)=>typeof E == "bigint" ? E : BigInt(E)) : $;
                    if (l === "decompose") {
                        const E = He(x, g, c || void 0, r || void 0, ee());
                        i = E.parts.map((C)=>X(C)), a = X(E.steiner_points), d = E.strategy, m = E.trace;
                    } else if (l === "bayazit") i = ht(x, g).map((C)=>X(C));
                    else if (l === "exact_partition") i = yt(x).map((C)=>X(C));
                    else if (l === "ear_clip") i = Je(x).map((C)=>X(C));
                    else if (l === "hertel_mehlhorn") {
                        const E = Je(x);
                        s = E.map((L)=>X(L));
                        const C = E.map((L)=>Array.from(L));
                        i = Et(C).map((L)=>X(L));
                    }
                } catch ($) {
                    w.textContent = String($);
                }
                rt(), lt(), j(), ge();
            }
        }
        function rt() {
            const $ = [];
            $.push(te("Vertices", String(t.length))), $.push(te("Algorithm", l)), d && $.push(te("Strategy", Qe(d))), $.push(te("Parts", String(i.length))), l === "hertel_mehlhorn" && s.length > 0 && ($.push(te("Triangles", String(s.length))), $.push(te("Optimized", `${s.length} → ${i.length}`))), a.length > 0 && $.push(te("Steiner pts", String(a.length)));
            for(let y = 0; y < i.length; y++)$.push(te(`Part ${y}`, `${i[y].length} verts`));
            u.innerHTML = `<h3>Result</h3>${$.join("")}`;
        }
        function lt() {
            if (!m || m.length === 0) {
                h.innerHTML = "", h.style.display = "none";
                return;
            }
            h.style.display = "block";
            const $ = m.map((y, x)=>{
                const E = Ho(y.outcome);
                return `<div class="trace-entry">
        <span class="trace-index">#${x}</span>
        <span class="trace-strategy">${Qe(y.strategy)}</span>
        <span class="trace-rotation">rot=${y.rotation}</span>
        <span class="info-value ${E}">${Yo(y.outcome)}</span>
      </div>`;
            }).join("");
            h.innerHTML = `
      <details class="trace-details">
        <summary>Trace (${m.length} attempts)</summary>
        <div class="trace-list">${$}</div>
      </details>
    `;
        }
        function te($, y, x = "") {
            return `<div class="info-row"><span class="info-label">${$}</span><span class="info-value ${x}">${y}</span></div>`;
        }
        function ge() {
            n = V(e);
            const $ = i.length > 0 ? [
                ...i,
                ...s.length > 0 ? s : []
            ] : t.length > 0 ? [
                t
            ] : [];
            if ($.length > 0 && fe(n, $), ce(n), de(n), i.length > 0) {
                if (N(n, t, {
                    stroke: "#333",
                    showVertices: !1,
                    lineWidth: 1,
                    dashed: !0
                }), l === "hertel_mehlhorn" && s.length > 0) for (const y of s)N(n, y, {
                    stroke: "#444",
                    lineWidth: 1,
                    dashed: !0,
                    showVertices: !1
                });
                et(n, i);
            } else t.length > 0 && N(n, t);
            if (v && t.length >= 3) for(let y = 0; y < t.length; y++){
                const [x, E] = t[y];
                K(n, x, E, `v${y}`, "#888", [
                    10,
                    -10
                ]);
            }
            if (v && i.length > 0) for(let y = 0; y < i.length; y++){
                const x = i[y];
                for(let E = 0; E < x.length; E++){
                    const [C, M] = x[E];
                    K(n, C, M, `p${y}.${E}`, Se(y), [
                        10,
                        4 + y * 12
                    ]);
                }
            }
            for (const [y, x] of a)ne(n, y, x, "#ff6b6b", 6), K(n, y, x, "S", "#ff6b6b");
            o.isDrawing && o.points.length > 0 && N(n, o.points, {
                stroke: "#666",
                closed: !1,
                dashed: !0
            });
        }
        return {
            id: "decomposition",
            label: "Decomposition",
            create () {
                const $ = document.createElement("div");
                return $.id = "tab-decomposition", $.innerHTML = `
        <div class="toolbar">
          <select id="decomp-preset">
            <option value="">— Preset —</option>
            ${G.map((y)=>`<option value="${y.name}">${y.name}</option>`).join("")}
            <optgroup label="Invalid on-chain">
              ${Xe.map((y)=>`<option value="${y.name}">${y.name}</option>`).join("")}
            </optgroup>
          </select>
          <div class="sep"></div>
          <select id="decomp-algo">
            <option value="decompose">Cascade</option>
            <option value="bayazit">Bayazit</option>
            <option value="exact_partition">Exact Vertex Partition</option>
            <option value="ear_clip">Ear Clip</option>
            <option value="hertel_mehlhorn">Hertel-Mehlhorn</option>
          </select>
          <div class="checkbox-row"><input type="checkbox" id="decomp-steiner" checked /><label for="decomp-steiner">Steiner</label></div>
          <div class="checkbox-row"><input type="checkbox" id="decomp-trace" /><label for="decomp-trace">Trace</label></div>
          <div class="checkbox-row"><input type="checkbox" id="decomp-minimize" /><label for="decomp-minimize" title="Best-of all strategies">Minimize</label></div>
          <div class="sep"></div>
          <button class="btn btn-primary" id="decomp-run">Decompose</button>
          <button class="btn btn-danger" id="decomp-clear">Clear</button>
          <div class="sep"></div>
          <div class="checkbox-row"><input type="checkbox" id="decomp-debug" checked /><label for="decomp-debug">Debug</label></div>
          <div class="checkbox-row"><input type="checkbox" id="decomp-vertex-labels" checked /><label for="decomp-vertex-labels">Labels</label></div>
          <span class="status-text">Click to draw. Right-click to close.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="decomp-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="decomp-info">
              <h3>Result</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="decomp-trace-panel"></div>
            <div id="decomp-error" style="color:#ff4a4a;font-size:12px;"></div>
            <div id="decomp-debug-panel"></div>
          </div>
        </div>
      `, $;
            },
            activate () {
                e = document.getElementById("decomp-canvas"), u = document.getElementById("decomp-info"), w = document.getElementById("decomp-error"), h = document.getElementById("decomp-trace-panel"), _ = document.getElementById("decomp-debug-panel"), n = V(e), o = me(e, ()=>n), t = Z(), b = he(()=>{
                    t = Z(), i = [], s = [], a = [], t.length >= 3 ? U() : (j(), ge());
                }), o.setOnChange(()=>ge()), o.setOnComplete((x)=>{
                    t = x, o.clear(), R(t);
                }), o.enable(), document.getElementById("decomp-preset").addEventListener("change", (x)=>{
                    const E = x.target.value, C = G.find((M)=>M.name === E) ?? Xe.find((M)=>M.name === E);
                    C && (o.clear(), R(C.points.slice()));
                }), document.getElementById("decomp-clear").addEventListener("click", ()=>{
                    i = [], s = [], a = [], o.clear(), w.textContent = "", h.innerHTML = "", h.style.display = "none", document.getElementById("decomp-preset").value = "", R([]);
                });
                const $ = document.getElementById("decomp-minimize"), y = ()=>{
                    $.disabled = l !== "decompose", $.parentElement?.classList.toggle("disabled", $.disabled);
                };
                y(), document.getElementById("decomp-algo").addEventListener("change", (x)=>{
                    l = x.target.value, y(), t.length >= 3 && U();
                }), document.getElementById("decomp-steiner").addEventListener("change", (x)=>{
                    g = x.target.checked, t.length >= 3 && U();
                }), document.getElementById("decomp-trace").addEventListener("change", (x)=>{
                    c = x.target.checked, t.length >= 3 && U();
                }), document.getElementById("decomp-minimize").addEventListener("change", (x)=>{
                    r = x.target.checked, t.length >= 3 && U();
                }), document.getElementById("decomp-run").addEventListener("click", U), document.getElementById("decomp-debug").addEventListener("change", (x)=>{
                    B = x.target.checked, j();
                }), document.getElementById("decomp-vertex-labels").addEventListener("change", (x)=>{
                    v = x.target.checked, ge();
                }), I = We(()=>{
                    t.length >= 3 && U();
                }), t.length >= 3 ? U() : (j(), ge());
            },
            deactivate () {
                o?.disable(), b?.(), b = null, I?.(), I = null;
            }
        };
    }
    function Uo() {
        let e, n, o, t = [], i = [], s, a, d = null, m = null;
        function p() {
            if (i = [], a.textContent = "", t.length < 3) {
                l(null), c();
                return;
            }
            try {
                const r = D(t), u = Ee(r), w = Pt(r), h = bt(u, ee()), _ = Re(r);
                let B = null;
                const v = [];
                try {
                    i = He(r, !0, void 0, void 0, ee()).parts.map((I)=>X(I));
                    for (const I of i){
                        const k = D(I);
                        v.push(Ee(k));
                    }
                    B = mt(u, v);
                } catch  {}
                l({
                    twiceArea: u,
                    signedArea: w,
                    displayArea: h.toString(),
                    perimeter: _,
                    partsCount: i.length,
                    partAreas: v,
                    conserved: B
                });
            } catch (r) {
                a.textContent = String(r);
            }
            c();
        }
        function l(r) {
            if (!r) {
                s.innerHTML = '<h3>Metrics</h3><div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>';
                return;
            }
            const u = [];
            if (u.push(g("2x Area (unsigned)", r.twiceArea)), u.push(g("2x Area (signed)", r.signedArea)), u.push(g("Display Area", r.displayArea)), u.push(g("L1 Perimeter", r.perimeter)), r.partsCount > 0) {
                u.push('<div style="border-top:1px solid #222;margin:8px 0;"></div>'), u.push(g("Parts", String(r.partsCount)));
                for(let w = 0; w < r.partAreas.length; w++)u.push(g(`Part ${w} 2xArea`, r.partAreas[w]));
                r.conserved !== null && u.push(g("Area Conserved", r.conserved ? "Yes" : "No", r.conserved ? "ok" : "error"));
            }
            s.innerHTML = `<h3>Metrics</h3>${u.join("")}`;
        }
        function g(r, u, w = "") {
            return `<div class="info-row"><span class="info-label">${r}</span><span class="info-value ${w}">${u}</span></div>`;
        }
        function c() {
            n = V(e);
            const r = i.length > 0 ? [
                t,
                ...i
            ] : t.length > 0 ? [
                t
            ] : [];
            if (r.length > 0 && fe(n, r), ce(n), de(n), i.length > 0) {
                N(n, t, {
                    stroke: "#555",
                    showVertices: !1,
                    dashed: !0,
                    lineWidth: 1
                }), et(n, i, {
                    showVertices: !1
                });
                for(let u = 0; u < i.length; u++){
                    const w = i[u].reduce((_, B)=>_ + B[0], 0) / i[u].length, h = i[u].reduce((_, B)=>_ + B[1], 0) / i[u].length;
                    K(n, w, h, `P${u}`, Se(u), [
                        -6,
                        4
                    ]);
                }
            } else t.length > 0 && N(n, t);
            o.isDrawing && o.points.length > 0 && N(n, o.points, {
                stroke: "#666",
                closed: !1,
                dashed: !0
            });
        }
        return {
            id: "area",
            label: "Area & Metrics",
            create () {
                const r = document.createElement("div");
                return r.id = "tab-area", r.innerHTML = `
        <div class="toolbar">
          <select id="area-preset">
            <option value="">— Preset —</option>
            ${G.map((u)=>`<option value="${u.name}">${u.name}</option>`).join("")}
          </select>
          <button class="btn btn-danger" id="area-clear">Clear</button>
          <span class="status-text">Auto-decomposes to show area conservation.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="area-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="area-info">
              <h3>Metrics</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="area-error" style="color:#ff4a4a;font-size:12px;"></div>
          </div>
        </div>
      `, r;
            },
            activate () {
                e = document.getElementById("area-canvas"), s = document.getElementById("area-info"), a = document.getElementById("area-error"), n = V(e), o = me(e, ()=>n), t = Z(), d = he(()=>{
                    t = Z(), i = [], p();
                }), o.setOnChange(()=>c()), o.setOnComplete((r)=>{
                    o.clear(), R(r);
                }), o.enable(), document.getElementById("area-preset").addEventListener("change", (r)=>{
                    const u = r.target.value, w = G.find((h)=>h.name === u);
                    w && (o.clear(), R(w.points.slice()));
                }), document.getElementById("area-clear").addEventListener("click", ()=>{
                    i = [], o.clear(), a.textContent = "", document.getElementById("area-preset").value = "", R([]);
                }), m = We(()=>{
                    t.length >= 3 && p();
                }), t.length >= 3 ? p() : c();
            },
            deactivate () {
                o?.disable(), d?.(), d = null, m?.(), m = null;
            }
        };
    }
    function Xo() {
        let e, n, o, t = [], i, s, a = null;
        function d() {
            if (s.textContent = "", t.length < 3) {
                m(null), l();
                return;
            }
            try {
                const c = D(t), r = De(c), u = nt(c), w = $e(c), h = [];
                for(let _ = 0; _ < t.length; _++){
                    const B = t[(_ - 1 + t.length) % t.length], v = t[_], b = t[(_ + 1) % t.length];
                    It(BigInt(B[0]), BigInt(B[1]), BigInt(v[0]), BigInt(v[1]), BigInt(b[0]), BigInt(b[1])) && h.push(_);
                }
                m({
                    isCcw: r,
                    isSimple: u,
                    isConvex: w,
                    reflexCount: h.length,
                    reflexVertices: h
                }), l(h);
            } catch (c) {
                s.textContent = String(c), l();
            }
        }
        function m(c) {
            if (!c) {
                i.innerHTML = '<h3>Properties</h3><div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>';
                return;
            }
            const r = [];
            r.push(p("Vertices", String(t.length))), r.push(p("CCW", c.isCcw ? "Yes" : "No", c.isCcw ? "ok" : "warn")), r.push(p("Simple", c.isSimple ? "Yes" : "No", c.isSimple ? "ok" : "error")), r.push(p("Convex", c.isConvex ? "Yes" : "No", c.isConvex ? "ok" : "warn")), r.push(p("Reflex vertices", String(c.reflexCount), c.reflexCount > 0 ? "warn" : "ok")), i.innerHTML = `<h3>Properties</h3>${r.join("")}`;
        }
        function p(c, r, u = "") {
            return `<div class="info-row"><span class="info-label">${c}</span><span class="info-value ${u}">${r}</span></div>`;
        }
        function l(c = []) {
            if (n = V(e), t.length > 0 && fe(n, [
                t
            ]), ce(n), de(n), t.length < 2) {
                o.isDrawing && o.points.length > 0 && N(n, o.points, {
                    stroke: "#666",
                    closed: !1,
                    dashed: !0
                });
                return;
            }
            N(n, t, {
                showVertices: !1
            });
            for(let u = 0; u < t.length; u++){
                const w = (u + 1) % t.length;
                ft(n, t[u], t[w]);
            }
            const r = new Set(c);
            for(let u = 0; u < t.length; u++){
                const w = r.has(u), h = w ? "#ff6b6b" : "#4a9eff";
                ne(n, t[u][0], t[u][1], h, w ? 6 : 4), K(n, t[u][0], t[u][1], String(u), w ? "#ff6b6b" : "#888");
            }
            o.isDrawing && o.points.length > 0 && N(n, o.points, {
                stroke: "#666",
                closed: !1,
                dashed: !0
            });
        }
        function g(c) {
            if (!(t.length < 3)) {
                s.textContent = "";
                try {
                    const r = D(t);
                    let u = null;
                    if (c === "ccw") u = tt(r);
                    else if (c === "collinear") u = ot(r);
                    else if (c === "normalize") {
                        const w = $t(r);
                        if (w) u = w;
                        else {
                            s.textContent = "Normalization returned null (degenerate ring)";
                            return;
                        }
                    }
                    u && R(X(u));
                } catch (r) {
                    s.textContent = String(r);
                }
            }
        }
        return {
            id: "ring",
            label: "Ring Ops",
            create () {
                const c = document.createElement("div");
                return c.id = "tab-ring", c.innerHTML = `
        <div class="toolbar">
          <select id="ring-preset">
            <option value="">— Preset —</option>
            ${G.map((r)=>`<option value="${r.name}">${r.name}</option>`).join("")}
          </select>
          <div class="sep"></div>
          <button class="btn" id="ring-ccw">Ensure CCW</button>
          <button class="btn" id="ring-collinear">Remove Collinear</button>
          <button class="btn" id="ring-normalize">Normalize</button>
          <div class="sep"></div>
          <button class="btn btn-danger" id="ring-clear">Clear</button>
          <span class="status-text">Blue = convex, Red = reflex. Arrows = winding.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="ring-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="ring-info">
              <h3>Properties</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="ring-error" style="color:#ff4a4a;font-size:12px;"></div>
          </div>
        </div>
      `, c;
            },
            activate () {
                e = document.getElementById("ring-canvas"), i = document.getElementById("ring-info"), s = document.getElementById("ring-error"), n = V(e), o = me(e, ()=>n), t = Z(), a = he(()=>{
                    t = Z(), d();
                }), o.setOnChange(()=>l()), o.setOnComplete((c)=>{
                    o.clear(), R(c);
                }), o.enable(), document.getElementById("ring-preset").addEventListener("change", (c)=>{
                    const r = c.target.value, u = G.find((w)=>w.name === r);
                    u && (o.clear(), R(u.points.slice()));
                }), document.getElementById("ring-clear").addEventListener("click", ()=>{
                    o.clear(), s.textContent = "", document.getElementById("ring-preset").value = "", R([]);
                }), document.getElementById("ring-ccw").addEventListener("click", ()=>g("ccw")), document.getElementById("ring-collinear").addEventListener("click", ()=>g("collinear")), document.getElementById("ring-normalize").addEventListener("click", ()=>g("normalize")), t.length >= 3 ? d() : l();
            },
            deactivate () {
                o?.disable(), a?.(), a = null;
            }
        };
    }
    function qo() {
        let e, n, o, t = [], i = [], s = !1, a = "draw", d, m, p = null, l = null;
        function g(h, _) {
            if (!(t.length < 3)) try {
                const v = D(t), b = BigInt(Math.round(h * 1e6)), I = BigInt(Math.round(_ * 1e6)), k = Ct(b, I, v), A = St(b, I, v);
                let P = !1;
                s && (P = kt(b, I, v)), i.push({
                    x: h,
                    y: _,
                    inside: k,
                    boundary: A,
                    strictlyInside: P
                }), c(), u();
            } catch (B) {
                m.textContent = String(B);
            }
        }
        function c() {
            const h = [];
            if (h.push(r("Vertices", String(t.length))), h.push(r("Convex", s ? "Yes" : "No", s ? "ok" : "warn")), h.push(r("Test points", String(i.length))), i.length > 0) {
                const _ = i[i.length - 1];
                h.push('<div style="border-top:1px solid #222;margin:8px 0;"></div>'), h.push(r("Last point", `(${_.x}, ${_.y})`)), h.push(r("Inside/On", _.inside ? "Yes" : "No", _.inside ? "ok" : "error")), h.push(r("On boundary", _.boundary ? "Yes" : "No", _.boundary ? "warn" : "")), s && h.push(r("Strictly inside", _.strictlyInside ? "Yes" : "No", _.strictlyInside ? "ok" : ""));
            }
            d.innerHTML = `<h3>Spatial Query</h3>${h.join("")}`;
        }
        function r(h, _, B = "") {
            return `<div class="info-row"><span class="info-label">${h}</span><span class="info-value ${B}">${_}</span></div>`;
        }
        function u() {
            n = V(e), t.length > 0 && fe(n, [
                t
            ], 60), ce(n), de(n), t.length >= 2 && N(n, t, {
                fill: "#4a9eff"
            });
            for (const h of i){
                let _;
                h.boundary ? _ = "#fcc419" : h.inside ? _ = "#51cf66" : _ = "#ff6b6b", ne(n, h.x, h.y, _, 5);
            }
            o.isDrawing && o.points.length > 0 && N(n, o.points, {
                stroke: "#666",
                closed: !1,
                dashed: !0
            });
        }
        function w(h) {
            if (a !== "test" || t.length < 3) return;
            const _ = e.getBoundingClientRect(), [B, v] = ze(n, h.clientX - _.left, h.clientY - _.top);
            g(B, v);
        }
        return {
            id: "spatial",
            label: "Spatial Queries",
            create () {
                const h = document.createElement("div");
                return h.id = "tab-spatial", h.innerHTML = `
        <div class="toolbar">
          <select id="spatial-preset">
            <option value="">— Preset —</option>
            ${G.map((_)=>`<option value="${_.name}">${_.name}</option>`).join("")}
          </select>
          <div class="sep"></div>
          <button class="btn btn-primary" id="spatial-mode-draw">Draw</button>
          <button class="btn" id="spatial-mode-test">Test</button>
          <div class="sep"></div>
          <button class="btn" id="spatial-clear-pts">Clear Points</button>
          <button class="btn btn-danger" id="spatial-clear">Clear All</button>
          <span class="status-text" id="spatial-status">Draw polygon, then switch to Test.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="spatial-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="spatial-info">
              <h3>Spatial Query</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="spatial-error" style="color:#ff4a4a;font-size:12px;"></div>
            <div class="help-text">Green = inside, Yellow = boundary, Red = outside</div>
          </div>
        </div>
      `, h;
            },
            activate () {
                if (e = document.getElementById("spatial-canvas"), d = document.getElementById("spatial-info"), m = document.getElementById("spatial-error"), n = V(e), o = me(e, ()=>n), t = Z(), p = he(()=>{
                    t = Z(), i = [];
                    try {
                        s = t.length >= 3 ? $e(D(t)) : !1;
                    } catch  {
                        s = !1;
                    }
                    t.length >= 3 && l && l("test"), c(), u();
                }), o.setOnChange(()=>u()), o.setOnComplete((h)=>{
                    o.clear(), R(h);
                }), l = function(_) {
                    a = _;
                    const B = document.getElementById("spatial-mode-draw"), v = document.getElementById("spatial-mode-test"), b = document.getElementById("spatial-status");
                    _ === "draw" ? (B.classList.add("btn-primary"), v.classList.remove("btn-primary"), o.enable(), e.removeEventListener("click", w), b.textContent = "Click to draw polygon vertices. Right-click to close.") : (v.classList.add("btn-primary"), B.classList.remove("btn-primary"), o.disable(), e.addEventListener("click", w), b.textContent = "Click anywhere to test point inclusion.");
                }, t.length >= 3) {
                    try {
                        s = $e(D(t));
                    } catch  {
                        s = !1;
                    }
                    l("test"), c();
                } else l("draw");
                document.getElementById("spatial-preset").addEventListener("change", (h)=>{
                    const _ = h.target.value, B = G.find((v)=>v.name === _);
                    B && (o.clear(), i = [], R(B.points.slice()));
                }), document.getElementById("spatial-clear").addEventListener("click", ()=>{
                    i = [], o.clear(), m.textContent = "", l && l("draw"), document.getElementById("spatial-preset").value = "", R([]);
                }), document.getElementById("spatial-clear-pts").addEventListener("click", ()=>{
                    i = [], c(), u();
                }), document.getElementById("spatial-mode-draw").addEventListener("click", ()=>l?.("draw")), document.getElementById("spatial-mode-test").addEventListener("click", ()=>l?.("test")), u();
            },
            deactivate () {
                o?.disable(), e?.removeEventListener("click", w), p?.(), p = null;
            }
        };
    }
    function Jo() {
        let e, n, o = [], t = [], i = null, s = [
            0,
            0
        ], a = [
            0,
            0
        ], d, m, p = !1;
        function l() {
            if (o.length < 3 || t.length < 3) return null;
            try {
                const b = D(o), I = D(t);
                return {
                    overlaps: p ? At(b, I) : Tt(b, I)
                };
            } catch (b) {
                return m.textContent = String(b), null;
            }
        }
        function g() {
            m.textContent = "";
            const b = l(), I = [];
            I.push(c("Polygon A", `${o.length} verts`)), I.push(c("Polygon B", `${t.length} verts`)), I.push(c("Method", p ? "SAT + AABB" : "SAT")), b && I.push(c("Overlaps", b.overlaps ? "Yes" : "No", b.overlaps ? "error" : "ok")), d.innerHTML = `<h3>Overlap Detection</h3>${I.join("")}`;
        }
        function c(b, I, k = "") {
            return `<div class="info-row"><span class="info-label">${b}</span><span class="info-value ${k}">${I}</span></div>`;
        }
        function r() {
            n = V(e);
            const b = [
                o,
                t
            ].filter((A)=>A.length > 0);
            b.length > 0 && fe(n, b, 50), ce(n), de(n);
            const k = l()?.overlaps ? "#ff6b6b" : void 0;
            o.length >= 2 && N(n, o, {
                fill: k || "#4a9eff",
                stroke: k || "#4a9eff"
            }), t.length >= 2 && N(n, t, {
                fill: k || "#51cf66",
                stroke: k || "#51cf66"
            });
        }
        function u(b) {
            const I = b.reduce((A, P)=>A + P[0], 0) / b.length, k = b.reduce((A, P)=>A + P[1], 0) / b.length;
            return [
                I,
                k
            ];
        }
        function w(b, I, k) {
            return b.map(([A, P])=>[
                    Math.round(A + I),
                    Math.round(P + k)
                ]);
        }
        function h(b, I) {
            return (b[0] - I[0]) ** 2 + (b[1] - I[1]) ** 2;
        }
        function _(b) {
            const I = e.getBoundingClientRect(), k = b.clientX - I.left, A = b.clientY - I.top, P = (k - n.offsetX) / n.scale, z = -(A - n.offsetY) / n.scale, J = o.length > 0 ? h([
                P,
                z
            ], u(o)) : 1 / 0, j = t.length > 0 ? h([
                P,
                z
            ], u(t)) : 1 / 0;
            J < j && J < 1 / 0 ? (i = "a", s = u(o)) : j < 1 / 0 && (i = "b", s = u(t)), i && (a = [
                P - s[0],
                z - s[1]
            ]);
        }
        function B(b) {
            if (!i) return;
            const I = e.getBoundingClientRect(), k = b.clientX - I.left, A = b.clientY - I.top, P = (k - n.offsetX) / n.scale, z = -(A - n.offsetY) / n.scale, j = u(i === "a" ? o : t), Y = P - a[0] - j[0], U = z - a[1] - j[1];
            i === "a" ? o = w(o, Y, U) : t = w(t, Y, U), g(), r();
        }
        function v() {
            i = null;
        }
        return {
            id: "overlap",
            label: "Overlap & SAT",
            create () {
                const b = document.createElement("div");
                return b.id = "tab-overlap", b.innerHTML = `
        <div class="toolbar">
          <select id="overlap-preset">
            ${Te.map((I)=>`<option value="${I.name}">${I.name}</option>`).join("")}
          </select>
          <div class="checkbox-row"><input type="checkbox" id="overlap-aabb" /><label for="overlap-aabb">AABB pre-filter</label></div>
          <span class="status-text">Drag polygons. Blue=A, Green=B, Red=overlapping.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="overlap-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="overlap-info">
              <h3>Overlap Detection</h3>
            </div>
            <div id="overlap-error" style="color:#ff4a4a;font-size:12px;"></div>
          </div>
        </div>
      `, b;
            },
            activate () {
                e = document.getElementById("overlap-canvas"), d = document.getElementById("overlap-info"), m = document.getElementById("overlap-error"), n = V(e);
                const b = Te[0];
                o = b.a.slice(), t = b.b.slice(), e.addEventListener("mousedown", _), e.addEventListener("mousemove", B), e.addEventListener("mouseup", v), e.addEventListener("mouseleave", v), document.getElementById("overlap-preset").addEventListener("change", (I)=>{
                    const k = I.target.value, A = Te.find((P)=>P.name === k);
                    A && (o = A.a.slice(), t = A.b.slice(), m.textContent = "", g(), r());
                }), document.getElementById("overlap-aabb").addEventListener("change", (I)=>{
                    p = I.target.checked, g(), r();
                }), g(), r();
            },
            deactivate () {
                e?.removeEventListener("mousedown", _), e?.removeEventListener("mousemove", B), e?.removeEventListener("mouseup", v), e?.removeEventListener("mouseleave", v);
            }
        };
    }
    function Ko(e) {
        return e == null ? null : typeof e == "string" ? {
            HasHoles: {
                boundary_components: 0
            }
        } : e;
    }
    function Go(e) {
        return "NotConnected" in e ? `Parts not connected. Disconnected: [${e.NotConnected.disconnected_parts.join(", ")}]` : "HasHoles" in e ? `Polygon has holes. ${e.HasHoles.boundary_components} boundary components found.` : "TooManyParts" in e ? `Too many parts: ${e.TooManyParts.count} (max ${e.TooManyParts.max})` : "NotCompact" in e ? `Not compact enough: ${e.NotCompact.compactness_ppm} ppm (min ${e.NotCompact.min_ppm})` : "VertexOnlyContact" in e ? `Parts ${e.VertexOnlyContact.part_a} and ${e.VertexOnlyContact.part_b} have only vertex contact` : "UnsupportedContact" in e ? `Parts ${e.UnsupportedContact.part_a} and ${e.UnsupportedContact.part_b}: ${e.UnsupportedContact.reason}` : JSON.stringify(e);
    }
    function Qo(e) {
        return "NotConnected" in e ? "NotConnected" : "HasHoles" in e ? "HasHoles" : "TooManyParts" in e ? "TooManyParts" : "NotCompact" in e ? "NotCompact" : "VertexOnlyContact" in e ? "VertexOnlyContact" : "UnsupportedContact" in e ? "UnsupportedContact" : "Unknown";
    }
    function Zo(e) {
        return e && "NotConnected" in e ? new Set(e.NotConnected.disconnected_parts) : new Set;
    }
    function ei() {
        let e, n, o, t = [], i = [], s = [], a = null, d = !1, m = null, p, l, g = null, c = null;
        function r() {
            if (i = [], s = [], a = null, l.textContent = "", m) i = m;
            else if (t.length < 3) {
                u(), h();
                return;
            } else try {
                const _ = D(t);
                i = He(_, !0, void 0, void 0, ee()).parts.map((v)=>X(v));
            } catch (_) {
                l.textContent = String(_), u(), h();
                return;
            }
            try {
                if (i.length > 1) {
                    for(let v = 0; v < i.length; v++)for(let b = v + 1; b < i.length; b++)try {
                        const I = D(i[v]), k = D(i[b]), A = wt(I, k);
                        s.push({
                            partA: v,
                            partB: b,
                            shared: A
                        });
                    } catch  {
                        s.push({
                            partA: v,
                            partB: b,
                            shared: !1
                        });
                    }
                    const _ = i.map((v)=>{
                        const b = [];
                        for (const [I, k] of v)b.push(BigInt(I)), b.push(BigInt(k));
                        return b;
                    }), B = Dt(_, d || void 0, ee());
                    a = Ko(B);
                }
            } catch (_) {
                l.textContent = String(_);
            }
            u(), h();
        }
        function u() {
            const _ = [];
            if (m && _.push(w("Mode", "Manual parts", "warn")), _.push(w("Vertices", String(t.length))), _.push(w("Parts", String(i.length))), i.length > 1) {
                const B = s.filter((v)=>v.shared).length;
                if (_.push(w("Shared edges", `${B}/${s.length} pairs`)), a) {
                    const v = Qo(a);
                    _.push(w("Topology", v, "error")), _.push(`<div class="topo-error-detail">${Go(a)}</div>`);
                } else i.length > 1 && _.push(w("Topology", "Valid", "ok"));
                _.push('<div style="border-top:1px solid #222;margin:8px 0;"></div>');
                for (const v of s)_.push(w(`P${v.partA} — P${v.partB}`, v.shared ? "Shared edge" : "No shared edge", v.shared ? "ok" : "warn"));
            }
            p.innerHTML = `<h3>Topology</h3>${_.join("")}`;
        }
        function w(_, B, v = "") {
            return `<div class="info-row"><span class="info-label">${_}</span><span class="info-value ${v}">${B}</span></div>`;
        }
        function h() {
            n = V(e);
            const _ = i.length > 0 ? t.length > 0 ? [
                t,
                ...i
            ] : [
                ...i
            ] : t.length > 0 ? [
                t
            ] : [];
            if (_.length > 0 && fe(n, _), ce(n), de(n), i.length > 0) {
                t.length > 0 && N(n, t, {
                    stroke: "#333",
                    showVertices: !1,
                    dashed: !0,
                    lineWidth: 1
                });
                const B = Zo(a);
                for(let v = 0; v < i.length; v++)if (B.has(v)) N(n, i[v], {
                    fill: "#ff4a4a",
                    stroke: "#ff4a4a",
                    lineWidth: 2.5,
                    showVertices: !0
                });
                else {
                    const b = Se(v);
                    N(n, i[v], {
                        fill: b,
                        stroke: b,
                        showVertices: !0
                    });
                }
                for (const v of s){
                    if (!v.shared) continue;
                    const b = i[v.partA], I = i[v.partB];
                    for(let k = 0; k < b.length; k++){
                        const A = b[k], P = b[(k + 1) % b.length];
                        for(let z = 0; z < I.length; z++){
                            const J = I[z], j = I[(z + 1) % I.length];
                            (A[0] === j[0] && A[1] === j[1] && P[0] === J[0] && P[1] === J[1] || A[0] === J[0] && A[1] === J[1] && P[0] === j[0] && P[1] === j[1]) && ae(n, A, P, "#fcc419", 3);
                        }
                    }
                }
            } else t.length > 0 && N(n, t);
            o.isDrawing && o.points.length > 0 && N(n, o.points, {
                stroke: "#666",
                closed: !1,
                dashed: !0
            });
        }
        return {
            id: "topology",
            label: "Topology",
            create () {
                const _ = document.createElement("div");
                return _.id = "tab-topology", _.innerHTML = `
        <div class="toolbar">
          <select id="topo-preset">
            <option value="">— Preset —</option>
            ${G.map((B)=>`<option value="${B.name}">${B.name}</option>`).join("")}
            <optgroup label="Manual parts (validation)">
              ${qe.map((B)=>`<option value="topo:${B.name}">${B.name}</option>`).join("")}
            </optgroup>
          </select>
          <div class="checkbox-row"><input type="checkbox" id="topo-vertex-contact" /><label for="topo-vertex-contact">Allow vertex contact</label></div>
          <div class="sep"></div>
          <button class="btn btn-danger" id="topo-clear">Clear</button>
          <span class="status-text">Yellow = shared edges, Red = disconnected.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="topo-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="topo-info">
              <h3>Topology</h3>
              <div class="info-row"><span class="info-label">Draw a polygon to begin</span></div>
            </div>
            <div id="topo-error" style="color:#ff4a4a;font-size:12px;"></div>
          </div>
        </div>
      `, _;
            },
            activate () {
                e = document.getElementById("topo-canvas"), p = document.getElementById("topo-info"), l = document.getElementById("topo-error"), n = V(e), o = me(e, ()=>n), t = Z(), g = he(()=>{
                    t = Z(), i = [], s = [], a = null, m = null, r();
                }), o.setOnChange(()=>h()), o.setOnComplete((_)=>{
                    o.clear(), R(_);
                }), o.enable(), document.getElementById("topo-preset").addEventListener("change", (_)=>{
                    const B = _.target.value;
                    if (B.startsWith("topo:")) {
                        const v = B.slice(5), b = qe.find((I)=>I.name === v);
                        b && (o.clear(), m = b.parts.map((I)=>I.slice()), t = [], r());
                    } else {
                        m = null;
                        const v = G.find((b)=>b.name === B);
                        v && (o.clear(), R(v.points.slice()));
                    }
                }), document.getElementById("topo-clear").addEventListener("click", ()=>{
                    i = [], s = [], a = null, m = null, o.clear(), l.textContent = "", document.getElementById("topo-preset").value = "", R([]);
                }), document.getElementById("topo-vertex-contact").addEventListener("change", (_)=>{
                    d = _.target.checked, t.length >= 3 && r();
                }), c = We(()=>{
                    t.length >= 3 && r();
                }), t.length >= 3 ? r() : h();
            },
            deactivate () {
                o?.disable(), g?.(), g = null, c?.(), c = null;
            }
        };
    }
    function ti() {
        let e, n, o = "orientation", t = [], i;
        function s() {
            const p = [];
            if (o === "orientation") if (t.length >= 3) {
                const [l, g, c] = t, r = Ke(BigInt(l[0]), BigInt(l[1]), BigInt(g[0]), BigInt(g[1]), BigInt(c[0]), BigInt(c[1])), u = vt(BigInt(l[0]), BigInt(l[1]), BigInt(g[0]), BigInt(g[1]), BigInt(c[0]), BigInt(c[1])), w = xt(BigInt(l[0]), BigInt(l[1]), BigInt(g[0]), BigInt(g[1]), BigInt(c[0]), BigInt(c[1])), h = Bt(BigInt(l[0]), BigInt(l[1]), BigInt(g[0]), BigInt(g[1]), BigInt(c[0]), BigInt(c[1]));
                p.push(a("A", `(${l[0]}, ${l[1]})`)), p.push(a("B", `(${g[0]}, ${g[1]})`)), p.push(a("C", `(${c[0]}, ${c[1]})`)), p.push('<div style="border-top:1px solid #222;margin:8px 0;"></div>'), p.push(a("Orientation", r)), p.push(a("Cross2D", u)), p.push(a("C is left of AB", w ? "Yes" : "No", w ? "ok" : "")), p.push(a("C is right of AB", h ? "Yes" : "No", h ? "ok" : ""));
            } else p.push(a("Click", `${3 - t.length} more point(s)`));
            else if (t.length >= 4) {
                const [l, g, c, r] = t, u = Ge(BigInt(l[0]), BigInt(l[1]), BigInt(g[0]), BigInt(g[1]), BigInt(c[0]), BigInt(c[1]), BigInt(r[0]), BigInt(r[1])), w = Lt(BigInt(l[0]), BigInt(l[1]), BigInt(g[0]), BigInt(g[1]), BigInt(c[0]), BigInt(c[1]), BigInt(r[0]), BigInt(r[1]));
                p.push(a("Seg A", `(${l[0]},${l[1]}) → (${g[0]},${g[1]})`)), p.push(a("Seg B", `(${c[0]},${c[1]}) → (${r[0]},${r[1]})`)), p.push('<div style="border-top:1px solid #222;margin:8px 0;"></div>'), p.push(a("Intersects", u ? "Yes" : "No", u ? "error" : "ok")), p.push(a("Properly", w ? "Yes" : "No", w ? "error" : "ok"));
            } else p.push(a("Click", `${4 - t.length} more point(s)`)), t.length < 2 ? p.push(a("", "Click 2 pts for segment A")) : p.push(a("", "Click 2 pts for segment B"));
            i.innerHTML = `<h3>Result</h3>${p.join("")}`;
        }
        function a(p, l, g = "") {
            return `<div class="info-row"><span class="info-label">${p}</span><span class="info-value ${g}">${l}</span></div>`;
        }
        function d() {
            if (n = V(e), n.scale = 3, n.offsetX = n.width / 2, n.offsetY = n.height / 2, ce(n), de(n, 20), o === "orientation") {
                const p = [
                    "#4a9eff",
                    "#51cf66",
                    "#fcc419"
                ], l = [
                    "A",
                    "B",
                    "C"
                ];
                for(let g = 0; g < t.length; g++)ne(n, t[g][0], t[g][1], p[g], 6), K(n, t[g][0], t[g][1], l[g], p[g]);
                if (t.length >= 2 && ae(n, t[0], t[1], "#4a9eff", 2), t.length >= 3) {
                    ae(n, t[1], t[2], "#51cf66", 2, !0);
                    const g = Ke(BigInt(t[0][0]), BigInt(t[0][1]), BigInt(t[1][0]), BigInt(t[1][1]), BigInt(t[2][0]), BigInt(t[2][1]));
                    let c = "#555";
                    g === "CounterClockwise" ? c = "#51cf6630" : g === "Clockwise" && (c = "#ff6b6b30");
                    const { ctx: r } = n, [u, w] = W(n, t[0][0], t[0][1]), [h, _] = W(n, t[1][0], t[1][1]), [B, v] = W(n, t[2][0], t[2][1]);
                    r.beginPath(), r.moveTo(u, w), r.lineTo(h, _), r.lineTo(B, v), r.closePath(), r.fillStyle = c, r.fill();
                }
            } else t.length >= 1 && ne(n, t[0][0], t[0][1], "#4a9eff", 6), t.length >= 2 && (ne(n, t[1][0], t[1][1], "#4a9eff", 6), ae(n, t[0], t[1], "#4a9eff", 2), K(n, t[0][0], t[0][1], "A1", "#4a9eff"), K(n, t[1][0], t[1][1], "A2", "#4a9eff")), t.length >= 3 && ne(n, t[2][0], t[2][1], "#51cf66", 6), t.length >= 4 && (ne(n, t[3][0], t[3][1], "#51cf66", 6), ae(n, t[2], t[3], "#51cf66", 2), K(n, t[2][0], t[2][1], "B1", "#51cf66"), K(n, t[3][0], t[3][1], "B2", "#51cf66"), Ge(BigInt(t[0][0]), BigInt(t[0][1]), BigInt(t[1][0]), BigInt(t[1][1]), BigInt(t[2][0]), BigInt(t[2][1]), BigInt(t[3][0]), BigInt(t[3][1])) && (ae(n, t[0], t[1], "#ff6b6b", 3), ae(n, t[2], t[3], "#ff6b6b", 3)));
        }
        function m(p) {
            const l = e.getBoundingClientRect(), [g, c] = ze(n, p.clientX - l.left, p.clientY - l.top), r = o === "orientation" ? 3 : 4;
            t.length >= r && (t = []), t.push([
                g,
                c
            ]), s(), d();
        }
        return {
            id: "primitives",
            label: "Primitives",
            create () {
                const p = document.createElement("div");
                return p.id = "tab-primitives", p.innerHTML = `
        <div class="toolbar">
          <button class="btn btn-primary" id="prim-mode-orient">Orientation (3 pts)</button>
          <button class="btn" id="prim-mode-seg">Segments (4 pts)</button>
          <div class="sep"></div>
          <button class="btn btn-danger" id="prim-clear">Clear</button>
          <span class="status-text" id="prim-status">Click to place points.</span>
        </div>
        <div class="workspace">
          <div class="panel-canvas">
            <div class="canvas-container">
              <canvas id="prim-canvas" height="500"></canvas>
            </div>
          </div>
          <div class="panel-info">
            <div class="info-panel" id="prim-info">
              <h3>Result</h3>
              <div class="info-row"><span class="info-label">Click to place points</span></div>
            </div>
            <div class="help-text">
              <b>Orientation:</b> 3 pts (A,B,C) — C left/right of AB.<br>
              <b>Segments:</b> 4 pts — intersection test. Red = intersecting.
            </div>
          </div>
        </div>
      `, p;
            },
            activate () {
                e = document.getElementById("prim-canvas"), i = document.getElementById("prim-info"), n = V(e), e.addEventListener("click", m);
                function p(l) {
                    o = l, t = [];
                    const g = document.getElementById("prim-mode-orient"), c = document.getElementById("prim-mode-seg"), r = document.getElementById("prim-status");
                    l === "orientation" ? (g.classList.add("btn-primary"), c.classList.remove("btn-primary"), r.textContent = "Click 3 points: A, B, C. Tests orientation of C relative to AB.") : (c.classList.add("btn-primary"), g.classList.remove("btn-primary"), r.textContent = "Click 4 points: A1, A2 (segment A), B1, B2 (segment B). Tests intersection."), s(), d();
                }
                document.getElementById("prim-mode-orient").addEventListener("click", ()=>p("orientation")), document.getElementById("prim-mode-seg").addEventListener("click", ()=>p("segments")), document.getElementById("prim-clear").addEventListener("click", ()=>{
                    t = [], s(), d();
                }), d();
            },
            deactivate () {
                e?.removeEventListener("click", m);
            }
        };
    }
    const xe = [
        Wo(),
        Uo(),
        Xo(),
        qo(),
        Jo(),
        ei(),
        ti()
    ];
    let Ne = null;
    function ni() {
        const e = document.getElementById("config-toggle"), n = document.getElementById("config-panel"), o = document.getElementById("config-preset"), t = document.getElementById("config-max-parts"), i = document.getElementById("config-max-verts"), s = document.getElementById("config-min-edge"), a = document.getElementById("config-min-compact"), d = document.getElementById("config-area-div");
        function m(g) {
            t.value = String(g.max_parts), i.value = String(g.max_vertices_per_part), s.value = String(g.min_edge_length_squared), a.value = String(g.min_compactness_ppm), d.value = String(g.area_divisor);
        }
        function p() {
            return {
                max_parts: Number(t.value) || 1,
                max_vertices_per_part: Number(i.value) || 3,
                min_edge_length_squared: Number(s.value) || 0,
                min_compactness_ppm: Number(a.value) || 0,
                area_divisor: Number(d.value) || 1
            };
        }
        function l(g) {
            return JSON.stringify(g) === JSON.stringify(Be) ? "merca" : JSON.stringify(g) === JSON.stringify(Pe) ? "permissive" : "custom";
        }
        m(at()), e.addEventListener("click", ()=>{
            const g = n.style.display !== "none";
            n.style.display = g ? "none" : "block", e.classList.toggle("active", !g);
        }), o.addEventListener("change", ()=>{
            o.value === "merca" ? (m(Be), Oe(Be)) : o.value === "permissive" && (m(Pe), Oe(Pe));
        });
        for (const g of [
            t,
            i,
            s,
            a,
            d
        ])g.addEventListener("change", ()=>{
            const c = p();
            o.value = l(c), Oe(c);
        });
    }
    function oi() {
        const e = document.getElementById("tabs"), n = document.getElementById("content");
        for (const t of xe){
            const i = document.createElement("button");
            i.className = "tab-btn", i.textContent = t.label, i.dataset.tabId = t.id, i.addEventListener("click", ()=>o(t.id)), e.appendChild(i);
        }
        ni(), xe.length > 0 && o(xe[0].id);
        function o(t) {
            const i = xe.find((a)=>a.id === t);
            if (!i) return;
            Ne && Ne.deactivate(), e.querySelectorAll(".tab-btn").forEach((a)=>{
                a.classList.toggle("active", a.dataset.tabId === t);
            }), n.innerHTML = "";
            const s = i.create();
            n.appendChild(s), i.activate(), Ne = i;
        }
    }
    oi();
})();
