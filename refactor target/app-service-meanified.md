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



# Device Config Page UI Mock

---

## Top-Level Structure

- [Meta]
  - `page-style: overflow: hidden`
  - `root-font-size: [screen_width]`
- [Navigation Bar]
  - Title: `[ntitle]`
  - Background: `#10161C`
  - Animation: `easeIn`
---
- [Main Container View]
  - Class: `"rtl"` if `rtl` is true

---

## Address Code/DMX Section

- If **`dmx` is present**:
  - [Button Group]
    - [Icon] (image)
    - [Label] (DMX Address Code)
    - [Decrement Button] (supports long press, touch end, click)
    - [Input] (bound to `valArr[0]`)
    - [Increment Button] (supports long press, touch end, click)

- If **`dmx` is absent**:
  - [Button Group 1]
    - [Label]  (Address Code)
    - [Decrement Button]
    - [Input] (bound to `valArr[0]`)
    - [Increment Button]

  - [Button Group 2]
    - [Label]  (Display Range)
    - [Decrement Button]
    - [Input] (bound to `valArr1`)
    - [Increment Button]

---

## XY Display Mode Section

- [Radio Group] (on change: `radioChange`)
  - [Border Group 1]
    - [Label] "正常显示" (Normal Display)
    - Four radio choices (0,1,2,3), each with a label and colored text if selected
  - [Border Group 2]
    - [Label] "XY互换" (XY Swap)
    - Four radio choices (4,5,6,7), each with a label and colored text if selected

---

## RGB Light Adjustment Section

- [RGB Controls]
  - [Row 1]
    - [Label] "红光调光" (Red Light Dimming)
    - [Decrement Button]
    - [Input] (bound to `valArr2`)
    - [Increment Button]
    - Inputs/buttons disabled if `cfg==0`
  - [Row 2]
    - [Label] "绿光调光" (Green Light Dimming)
    - [Decrement Button]
    - [Input] (bound to `valArr3`)
    - [Increment Button]
    - Inputs/buttons disabled if `cfg==0`
  - [Row 3]
    - [Label] "蓝光调光" (Blue Light Dimming)
    - [Decrement Button]
    - [Input] (bound to `valArr4`)
    - [Increment Button]
    - Inputs/buttons disabled if `cfg==0`

---

## Lighting Source Section

- [Radio Group] (on change: `radioChange`)
  - [Label] "激光光源" (Laser Light Source)
  - [Option 1] 单色 ("Single Color") - disabled if `showCtr.light1`
  - [Option 2] 双色 ("Dual Color") - visible/disabled based on `showCtr.light2`
  - [Option 3] 全彩 ("Full Color") - disabled if `showCtr.light3`
  - [Option 4] (extra) - visible if `showCtr.lightExt`

---

## Adjustment Mode Section

- [Radio Group] (on change: `radioChange`)
  - [Label] "调制模式" (Modulation Mode)
  - [Option 1] (disabled if `!features.ttlAn`, checked if `cfg==0`)
  - [Option 2] (disabled if `!features.ttlAn`, checked if `cfg==255`)
  - [Option 3] (extra)

---

## System Language Section

- [Radio Group]
  - [Label] "系统语言" (System Language)
  - [Clickable Label] `[langName]` (underlined, colored), triggers `selectLang` on click

---

## Software Version Section

- [Radio Group]
  - [Label] "软件版本" (Software Version)
  - [Label] `[version]` or "1.1.1" (colored, aligned)

---

## Model Section

- [Radio Group]
  - [Label] "当前机型" (Current Model)
  - [Label] `[machine]` (colored, aligned)

---

## Interaction Summary

- **All increment/decrement buttons**: support click, long press, and touch end for value adjustment.
- **All inputs**: two-way bound (v-model style), support input and blur events.
- **Radio choices**: highlight selected, disabled/enabled as per props.
- **Language selection**: clickable, underlined, triggers a handler.
- **All sections**: support RTL layout (margin/padding/justification swap if `rtl` is true).

---

## Notes

- Most text labels are i18n (`$t(...)`).
- Layout is mobile-focused (uses "rem" units).
- Some controls (like dual-color light) appear conditionally.
- The component is highly dynamic based on props/data such as `dmx`, `showCtr`, `features`, `rtl`, `cfg`, etc.

---