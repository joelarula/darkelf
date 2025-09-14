# Device Control Page UI Mock

---

## [ Meta ]
- Page Meta: `overflow: hidden`, font size set by `screen_width`

---

## [ Root View ]
- **Class:** `"rtl"` if `rtl` is true, else none
- **ID:** `"containerId"`
- **Style:** `background-color: #10161C; width: 100%; height: 100%; display: flex; justify-content: center; padding-top: [statusBarHeight]px`

---

### [ Top Bar ]
- [Image] (No source specified)
- [Settings Button]
    - Style: `"margin-left: 20rem;"` if `rtl`
    - **On Click:** `settingClick`
    - Contains:
        - [Image] (No source specified)
        - [Text] (Translates to "设置" / "Settings")

- [Warning Icon]
    - Position: `absolute; left: calc(100vw - 100rem); top: [warnTop]px`
    - [Image] (No source specified)

---

### [ Title Section ]
- [View] class="title-view-class"
    - [Text] class="title-text-class" (likely main title)
    - [Text] class="title-text-class-sub" (likely subtitle)

---

### [ Connection & Device On/Off Section ]
- [Connect Device Button]
    - **On Click:** `cnnLaser`
    - Contains:
        - [Image]
        - [Text] "Click me to connect" (i18n)

- [Device On/Off Switch]
    - **On Click:** `onOffChange`
    - Class: `"view-device-onoff1"`
    - Style: If `deviceOn`: `justify-content: flex-end;background-color: #75E4FD;`
    - Contains:
        - [View] class="view-device-onoff2"
            - Style: If `deviceOn`: `background: #39CD78;` else `background: red;`

---

### [ Device Status Section ]
- [Device Connection State]
    - Style: If `cnnState`: `background-color: #39CD78;` else `background-color: #597181;`
    - Shape: Circle
    - Next to it:
        - [Text] font-size: 24rem; If `cnnState`: color `#DADADA`, else `#597181`
            - Text: If `cnnState`: "Connected" + device name, else "Bluetooth Not connected"

- [Device Name & Status]
    - [Text] "设备" ("Device")
    - [Text] "ON" ("OFF") colored green if `deviceOn`, red otherwise

---

## [ Function Buttons or LED/Color Controls ]

### If **NOT** `ledDevTag` (Normal device function buttons):
- Render function buttons in rows of 3:
    - For each group:
        - [View] class="fun-btn-group"
            - Up to 3 [View]s class="fun-btn-view"
                - [Main Func Button]
                    - Style: If selected, "background: linear-gradient(0deg, #0566D1, #4FC8E3);"
                    - Data attribute: `data-tag` = tag of function
                    - **On Click:** `prjClick`
                    - Contains:
                        - [Sub Button View]
                            - Style: If selected, "background-color: #2B4863;"
                            - [Image] src="/static/imgs/main/[func.img]"
                        - [Text] (function name, i18n)

### If **ledDevTag** (LED/Color device controls):
- [Color Settings Group]
    - [Label] "Color settings"
    - [Slider] (on change: `slTxDistChange`)

- [Shake Mode Group]
    - [Label] "Shake mode"
    - [Radio] "Automatic" (checked if `xyCnf.auto`)
    - [Radio] "Manual" (checked if not `xyCnf.auto`)

- [Shake Phase Group]
    - [Label] "Shake phase"
    - [Radio] "X+Y+" (checked if phase==0)
    - [Radio] "X-Y+" (checked if phase==1)
    - [Radio] "X+Y-" (checked if phase==2)
    - [Radio] "X-Y-" (checked if phase==3)

- [XY Canvas Section]
    - View opacity: 0.3 if auto, 1 otherwise
    - For each `xy` in `xyCnf.xy`:
        - [Canvas] (touch events: start/move/end/cancel)
    - For each `xy` in `xyCnf.xy`:
        - [Label] (shows `xy.title`)

---

# Event Handlers
- `settingClick`: Settings button
- `cnnLaser`: Connect device
- `onOffChange`: Toggle device on/off
- `prjClick`: Select function button
- `slTxDistChange`: Color slider change
- `radioChange`: Shake mode change
- `radioPhaseChange`: Shake phase change
- Canvas touch events: `chTouchstart`, `chTouchmove`, `chTouchend`

---

# Notes
- All text labels are i18n (translated).
- Visual appearance and logic is highly dynamic based on: `rtl`, `deviceOn`, `cnnState`, `ledDevTag`, `functionsShow`, `xyCnf`, etc.
- Button and element styles change based on state variables.
- Some elements (like images) have no src in the render code but likely set elsewhere.
- Layout is mobile-oriented (using "rem" units).

---
