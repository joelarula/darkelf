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
    - [Label] (Normal Display)
    - Four radio choices (0,1,2,3), each with a label and colored text if selected
  - [Border Group 2]
    - [Label]  (XY Swap)
    - Four radio choices (4,5,6,7), each with a label and colored text if selected

---

## RGB Light Adjustment Section

- [RGB Controls]
  - [Row 1]
    - [Label]  (Red Light Dimming)
    - [Decrement Button]
    - [Input] (bound to `valArr2`)
    - [Increment Button]
    - Inputs/buttons disabled if `cfg==0`
  - [Row 2]
    - [Label]  (Green Light Dimming)
    - [Decrement Button]
    - [Input] (bound to `valArr3`)
    - [Increment Button]
    - Inputs/buttons disabled if `cfg==0`
  - [Row 3]
    - [Label]  (Blue Light Dimming)
    - [Decrement Button]
    - [Input] (bound to `valArr4`)
    - [Increment Button]
    - Inputs/buttons disabled if `cfg==0`

---

## Lighting Source Section

- [Radio Group] (on change: `radioChange`)
  - [Label]  (Laser Light Source)
  - [Option 1]  ("Single Color") - disabled if `showCtr.light1`
  - [Option 2]  ("Dual Color") - visible/disabled based on `showCtr.light2`
  - [Option 3]  ("Full Color") - disabled if `showCtr.light3`
  - [Option 4] (extra) - visible if `showCtr.lightExt`

---

## Adjustment Mode Section

- [Radio Group] (on change: `radioChange`)
  - [Label] (Modulation Mode)
  - [Option 1] (disabled if `!features.ttlAn`, checked if `cfg==0`)
  - [Option 2] (disabled if `!features.ttlAn`, checked if `cfg==255`)
  - [Option 3] (extra)

---

## System Language Section

- [Radio Group]
  - [Label]  (System Language)
  - [Clickable Label] `[langName]` (underlined, colored), triggers `selectLang` on click

---

## Software Version Section

- [Radio Group]
  - [Label]  (Software Version)
  - [Label] `[version]` or "1.1.1" (colored, aligned)

---

## Model Section

- [Radio Group]
  - [Label] (Current Model)
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





Below is a UI mock in Markdown format for the colorDisplayConfigPageComponent, styled similarly to the provided example. The mock captures the component's structure, styling, dynamic behavior, and event handlers, based on the JavaScript render logic provided. It represents the UI hierarchy, including navigation, color selection, radio groups, sliders, and a floating button, while maintaining a clear and concise format.

Color Display Configuration Page UI Mock

[ Meta ]

Page Meta: overflow: hidden, root font size set by screen_width

[ Root View ]

Class: "rtl" if rtl is true, else none
Style: width: 100%; height: 100%;


[ Navigation Bar ]

Component: NavigationBar
Props:

Title: {{ntitle}} (i18n)
Background Color: #10161C
Color Animation Timing Function: easeIn



[ Main Content Section ]

Component: View
Class: "rtl" if rtl is true, else none

[ Color Selection Group ]

Component: View
Class: display-color-group
Structure: Grid of up to 9 color buttons (3 rows, 3 columns)
For each color button (index i from 0 to 8 in colorDisplayOrder):

Condition: Render if 3 * row + col <= 8
Component: View
Class: btn-color-group
Children:

[Button View]

Class: btn-view-style
Data Attribute: data-tag={{colorDisplayOrder[i].idx}}
On Click: btnColorChange
Children:

[Sub View]

Class: btn-view-sub
Children:

[Color Preview]

Condition: Render if 3 * row + col < 7
Class: btn-view-color
Style: background-color: {{colorDisplayOrder[i].color}}


[Label]

Class: btn-color-text
Text: {{colorDisplayOrder[i].name}} (i18n)



[Title View]

Class: btn-color-title
Show: Only if public.txColor == colorDisplayOrder[i].idx


[ Playback Mode Section ]

Condition: Render if prjIndex != 1
Component: RadioGroup
On Change: radioChange
Children:

[Playback Mode Radio]

Class: radio-play-mode
Children:

[Label]

Class: radio-play-label
Children:

[Radio]

Checked: If item.pyMode == 0


[Text]

Style: color: {{item.pyMode == 0 ? '#51D1EA' : '#687C8E'}}; font-size: 30rem
Text:  ("Loop Playback", i18n)



[Selection Mode Radio]

Class: radio-play-mode
Children:

[Label]

Class: radio-play-label
Children:

[Radio]

Checked: If item.pyMode != 0


[Text]

Style: color: {{item.pyMode != 0 ? '#51D1EA' : '#687C8E'}}; font-size: 30rem
Text: ("Selected Playback", i18n)



[Select All Button]

Class: btn-view-style2
Style: pointer-events: none; opacity: 0.3 if item.pyMode == 0, else normal
On Click: selectAutoBtnClick(1)
Children:

[Sub View]

Class: btn-view-sub
Children:

[Label]

Class: btn-color-text
Text:  ("Select All", i18n)










[Invert Selection Button]

Class: btn-view-style2
Style: pointer-events: none; opacity: 0.3 if item.pyMode == 0, else normal
On Click: selectAutoBtnClick(2)
Children:

[Sub View]

Class: btn-view-sub
Children:

[Label]

Class: btn-color-text
Text: 反选 ("Invert Selection", i18n)










[Clear Selection Button]

Class: btn-view-style2
Style: pointer-events: none; opacity: 0.3 if item.pyMode == 0, else normal
On Click: selectAutoBtnClick(3)
Children:

[Sub View]

Class: btn-view-sub
Children:

[Label]

Class: btn-color-text
Text: 清除 ("Clear", i18n)













[ Selection Buttons Section ]

Condition: Render if prjIndex != 1
Component: View
Class: buttons-views
Style: opacity: {{item.pyMode != 0 ? '1' : '0.3'}}
Structure: Grid of up to 50 selection buttons (10 rows, 5 columns)
For each row (index row from 0 to 9):

Children:

[Outdoor Tips Label 1]

Condition: Render if row == 0 && showOutDoorTips
Text: out_door_tips1 (i18n)


[Outdoor Tips Label 2]

Condition: Render if row == 6 && showOutDoorTips
Text: out_door_tips2 (i18n)


[Outdoor Tips Label 3]

Condition: Render if row == 8 && showOutDoorTips
Text: out_door_tips3 (i18n)


[Button Row]

Class: num-container2
For each column (index col from 0 to 4):

[Button]

Class: btn-select-item
Style: background: #51D1EA; color: #222222 if item.ckValues[5 * row + col] == 1, else default
Disabled: If item.pyMode == 0
Data Attribute: data-tag={{5 * row + col + 1}}
On Click: btnSelectClick
Text: {{5 * row + col + 1}}











[ Bottom Parameters Section ]

Component: View
Class: parm-view-bottom
Children:

[Playback Mode Group]

Component: RadioGroup
Class: display-btn-group
On Change: radioChange
Children:

[Label]

Class: display-btn-lable
Text: 播放模式 ("Playback Mode", i18n)


[Auto Mode Radio]

Class: fun-radio
Children:

[Radio]

Checked: If public.rdMode == 0


[Text]

Style: font-size: 30rem; color: {{public.rdMode == 0 ? '#76CEE7' : '#687C8E'}}
Text: 自走 ("Auto Mode", i18n)






[Voice Control Radio]

Class: fun-radio
Children:

[Radio]

Checked: If public.rdMode == 255


[Text]

Style: font-size: 30rem; color: {{public.rdMode == 255 ? '#76CEE7' : '#687C8E'}}
Text: 声控 ("Voice Control", i18n)










[Auto Speed Slider]

Class: display-btn-group
Style: opacity: {{public.rdMode == 0 ? '1' : '0.5'}}
Children:

[Label]

Class: display-btn-lable
Text: 自走速度 ("Auto Speed", i18n)


[Slider]

Class: fun-slider
Disabled: If public.rdMode != 0
Value: {{public.runSpeed}}
On Change: slRunChange






[Voice Sensitivity Slider]

Class: display-btn-group
Style: opacity: {{public.rdMode == 255 ? '1' : '0.5'}}
Children:

[Label]

Class: display-btn-lable
Text: 声控灵敏度 ("Voice Sensitivity", i18n)


[Slider]

Class: fun-slider
Disabled: If public.rdMode != 255
Value: {{public.soundVal}}
On Change: slSoundChange









[ Floating Button ]

Condition: Render if features.xyCnf
Component: Button
Class: floating-button
Style: left: {{position.x}}px; top: {{position.y}}px
Events:

On Touch Start: onBtnSetTouchStart
On Touch Move: onBtnSetTouchMove
On Touch End: onBtnSetTouchEnd
On Click: onBtnSetClick


Children:

[Image]

Source: {{barsPng}}






Event Handlers

btnColorChange: Triggered when a color button is clicked
radioChange: Triggered when a radio button in playback mode or bottom parameters changes
selectAutoBtnClick: Triggered when "Select All", "Invert Selection", or "Clear" buttons are clicked
btnSelectClick: Triggered when a selection button is clicked
slRunChange: Triggered when the auto speed slider changes
slSoundChange: Triggered when the voice sensitivity slider changes
onBtnSetTouchStart: Triggered on touch start for the floating button
onBtnSetTouchMove: Triggered on touch move for the floating button
onBtnSetTouchEnd: Triggered on touch end for the floating button
onBtnSetClick: Triggered when the floating button is clicked


Notes

All text labels are i18n (translated using $t).
The UI is highly dynamic, with visibility, styles, and interactivity controlled by variables like rtl, prjIndex, item.pyMode, public.rdMode, public.txColor, showOutDoorTips, and features.xyCnf.
Color buttons are limited to 9, with some conditionally hidden (3 * row + col < 7 for color previews).
Selection buttons are arranged in a 10x5 grid, with up to 50 buttons, and outdoor tips appear at specific rows.
Sliders and buttons in the bottom section are enabled/disabled based on public.rdMode (0 for auto, 255 for voice control).
The floating button supports touch-based dragging and clicking, with its position set dynamically.
Layout is mobile-oriented, using rem units for font sizes and responsive positioning.


Explanation

Structure: The Markdown mock mirrors the provided example's format, breaking down the UI into sections (e.g., Navigation Bar, Color Selection Group, Playback Mode Section) with clear hierarchies.
Dynamic Behavior: Conditions (e.g., prjIndex != 1), dynamic styles (e.g., opacity: {{item.pyMode != 0 ? '1' : '0.3'}}), and data-driven attributes (e.g., data-tag={{colorDisplayOrder[i].idx}}) are preserved as expressions.
Event Handlers: All interactive elements' events are listed in the "Event Handlers" section for clarity.
Styling and Classes: Classes like btn-view-style, fun-slider, and floating-button are included, with dynamic styles where applicable.
i18n: Text labels use placeholders like {{ $t('循环播放') }} to indicate internationalization, as in the original code.
Simplification: Vue-specific render details (e.g., e._$s, _i) are abstracted to focus on the UI structure and behavior, making the mock suitable for prototyping or documentation.

If you need adjustments, such as a different level of detail, additional styling specifics, or a different interpretation of "iMock," please let me know!2.8sFastHow can Grok help?