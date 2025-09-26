# Text Group Editor Page – UI Mockup

---

## 1. **Top Navigation & Meta**
- Page meta: `overflow: hidden`, responsive root font size.
- Navigation bar with:
  - Title (`ntitle`)
  - Background color: `#10161C`
  - Color animation

---

## 2. **Main Content (RTL support)**
- Main container, RTL class if enabled.

---

## 3. **Color Selection Grid**
- **Color Group Section**
  - 3x3 grid of color buttons (max 9).
  - Each button shows:
    - **Circle/Swatch:** background color from list.
    - **Label:** color name (i18n).
    - **Highlight** if currently selected (`txColor`).
  - Button click changes current color.

---

## 4. **Text Input and Controls**
- **Font Selector:**
  - Font name with icon.
  - Click to select font.
- **File Add Button:** Round button with icon for adding files.
- **Text Input:**
  - One-line input, bound to current group text.
  - Placeholder from `inputNote`.
  - RTL text-align if active.
- **Add Group Button:** (if `!features.textModeFix01`)
  - Appears next to input.
  - Click to add new text group.

---

## 5. **Canvas Preview**
- **Canvas** (`id="myCanvas"`)
  - Visible if `canvasShow`.

---

## 6. **Text Group List**
- **Each Group Item:**
  - Label: "第N组文字" ("Group N Text")
  - Text preview (truncated with "..." if too long).
  - Highlighted background if active.
  - Click to select/edit group.
  - **Playback Time:** 
    - Label: "播放时长" ("Playback Duration")
    - Value: `textTime[N]` (in seconds)
    - Click to edit time with popup.
  - **Delete Button:** 
    - Shown if more than one group.
    - Removes group.

---

## 7. **Popup: Edit Group Time**
- **Popup Dialog (uni-popup)**
  - Shows when editing playback time.
  - **Minus and Plus buttons** to adjust time.
  - **Input** for time (decimal if feature enabled).
  - Blur/input events update value.

---

## 8. **Text Property Sliders**
- **Text Precision Slider** (if `features.textStopTime`)
  - Label: "文字精度" ("Text Precision")
  - Slider for `txPointTime`
- **Text Size Slider** (if `!features.textModeFix01`)
  - Label: "文字大小" ("Text Size")
  - Slider for `txSize`
- **Display Distance Slider**
  - Label: "显示视距" ("Display Distance")
  - Slider for `txDist`
- **Speed Slider**
  - Label: "流水速度" ("Scrolling Speed")
  - Slider for `runSpeed`

---

## 9. **Run Direction Controls**
- **Direction Radio Group** (if `features.arbPlay` or `features.textUpDown`)
  - Label: "流水方向" ("Scrolling Direction")
  - **Left/Right/Up/Down** options (with icons).
  - Radio + image for each.
  - Only show right/up/down if feature enabled.
  - Disabled while sending.

---

## 10. **Action Buttons**
- **Preview Button:** "预览" ("Preview") – triggers preview.
- **Send Button:** "发送" ("Send") – triggers send, grayed out if not ready.

---

## 11. **Sending Progress**
- **Progress Canvas** (`id="progressCanvas"`)
  - Visible if `showSending`.

---

## 12. **Floating XY Config Button**
- **Floating Button** (if `features.xyCnf`)
  - Movable, positioned absolutely.
  - Icon (`barsPng`), responds to drag/click.

---

### **Notes**
- All text labels use i18n (`$t`).
- Layout is mobile-first, using rem units.
- Feature flags control which controls/sliders appear.
- All controls support RTL if enabled.
```





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
    - [Label] "DMX地址码" (DMX Address Code)
    - [Decrement Button] (supports long press, touch end, click)
    - [Input] (bound to `valArr[0]`)
    - [Increment Button] (supports long press, touch end, click)

- If **`dmx` is absent**:
  - [Button Group 1]
    - [Label] "地址码" (Address Code)
    - [Decrement Button]
    - [Input] (bound to `valArr[0]`)
    - [Increment Button]

  - [Button Group 2]
    - [Label] "显示范围" (Display Range)
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


# Play List Manager UI Mock

---

## 1. **Meta and Navigation**
- **Meta:** Sets page to `overflow: hidden`; `root-font-size` is responsive.
- **Navigation Bar:** Shows page title (`ntitle`), background-color `#10161C`, animated color.

---

## 2. **Main Layout (RTL support)**
- Main container supports RTL layout (`rtl` class if enabled).

---

## 3. **Playlist List Section**
- **If No Playlists:**
  - Centered message: “-- 列表空空如也 --” (List is empty)

- **If Playlists Exist:**
  - **Scrollable List** (`scroll-view`, id="scroll_view_playList"):
    - For each playlist (in reverse order):
      - **Playlist Item (`display-btn-group`):**
        - Background highlight if selected.
        - **Click** selects playlist (`playListFileSelectClick`).
        - **Label:** "[index]. [playlist name]"
        - **Eye Icon Button:** Opens playlist preview (`playListViewClick`)
        - **Play Icon Button:** Starts playlist (`playListFileClick`)
        - **Delete Icon Button:** Deletes playlist (`deleteListFileClick`)
        - **Rename Icon Button:** (only for selected playlist) Renames playlist (`editListFileClick`)
        - **Info Icon Button:** Edit playlist info (`playListEdit`)

---

## 4. **Add Playlist Button**
- **Button:** "新增列表" (Add New List)
- Prominent placement at bottom of list.
- **Click** triggers playlist creation (`playListAdd`).

---

## 5. **Popup: Playlist Name Input**
- **Popup Dialog** (`uni-popup`, ref="listNameInput"):
  - **Label:** "请输入列表名称" (Please enter playlist name)
  - **Text Input:** For playlist name (`playListNewName`)
  - **Cancel Button:** (`listNameInputCancelClick`)
  - **OK Button:** (`listNameInputOkClick`)
  - Clicking outside closes popup.

---

## 6. **Popup: Playlist Preview**
- **Popup Dialog** (`uni-popup`, ref="playListView"):
  - **Title:** Playlist name
  - **Image Preview:** (`viewImgPath`)
    - Clickable to trigger `playListViewImgClick`
  - **Speed Control:**
    - Label: "速度 [current value]"
    - Slider for speed (`slviewImgTime`, handler: `slViewTimeChange`)
  - **Navigation:**
    - **Back Button** (prev item, `playListViewItemClick(-1)`)
    - **Label:** "[current item] / [total]" (e.g., 2/10)
    - **Forward Button** (next item, `playListViewItemClick(1)`)

---

## 7. **Canvas Preview (Hidden)**
- **Canvas** (`id="drawCanvas"`)
  - Only shown if `drawCanvasShow` is true.
  - Positioned off-screen for drawing/processing.

---

### **Notes**
- All labels use i18n (`$t`).
- All action buttons have click handlers.
- All list and popup layouts support RTL (right-to-left) if enabled.
- Mobile-friendly sizing (uses "rem" units).
- All popups close when clicked outside.
- Designed for playlist management: create, view, delete, play, rename, and edit playlists.


The groupList array holds objects with the following structure:
{
    text: "",         // The text content for this group
    update: 0,        // Update flag (number)
    color: 9,         // Color index (number)
    fontIdex: e,      // Font index (number)
    time: 5,          // Time value (number)
    xys: [],          // Array of coordinate data
    XysRight: [],     // Array of coordinate data for "right" orientation
    XysUp: [],        // Array of coordinate data for "up" orientation
    XysDown: []       // Array of coordinate data for "down" orientation
}