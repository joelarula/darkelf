// UnifiedModule: Merged data, methods, and lifecycle hooks from mainPageComponent, appStateManager, DeviceConfigPageController, and bleDeviceProjectConfigPageComponent

const UnifiedModule = {
  data: function() {
    return {
      // mainPageComponent data
      modeCmdSend: "",
      functions: [
        { tag: 0, name: "DMX", show: true },
        { tag: 1, name: "Random playback", show: true },
        { tag: 2, name: "Timeline playback", show: true },
        { tag: 3, name: "Animation playback", show: true },
        { tag: 4, name: "Text playback", show: true },
        { tag: 5, name: "Christmas broadcast", show: true },
        { tag: 5, name: "ILDA", show: false },
        { tag: 6, name: "Outdoor playback", show: true },
        { tag: 7, name: "Personalized programming", show: true },
        { tag: 8, name: "Hand-drawn doodle", show: true },
        { tag: 9, name: "Playlist", show: true }
      ],
      features: {},
      deviceOn: false,
      prjIndex: -1,
      cnnDevice: "Not connected",
      cnnState: false,
      randomCheck: [],
      initShow: false,
      xyCnf: {
        auto: true,
        autoValue: 0,
        phase: 0,
        xy: [
          { title: "X-axis coarse tuning", name: "xBig", value: 0 },
          { title: "X-axis fine tuning", name: "xSmall", value: 0 },
          { title: "Y-axis coarse tuning", name: "yBig", value: 0 },
          { title: "Y-axis fine tuning", name: "ySmall", value: 0 }
        ]
      },
      // DeviceConfigPageController data
      showCtr: { light1: true, light2: true, light3: true, lightExt: false },
      deviceInfo: 0,
      SetCmdSend: "",
      version: "",
      machine: "",
      dmx: 0,
      ch: 0,
      xy: 0,
      light: 1,
      cfg: 0,
      valArr: [1, 10, 10, 10, 10],
      valRange: [ [1,512], [10,100], [0,255], [0,255], [0,255] ],
      // bleDeviceProjectConfigPageComponent data
      sendCmdParmsTimer: null,
      showOutDoorTips: false,
      colorDisplayOrder: [
        { name: "Red", color: "red", order: 0, idx: 1 },
        { name: "yellow", color: "yellow", order: 1, idx: 4 },
        { name: "green", color: "green", order: 2, idx: 2 },
        { name: "Cyan", color: "#00FFFF", order: 3, idx: 5 },
        { name: "blue", color: "blue", order: 4, idx: 3 },
        { name: "purple", color: "purple", order: 5, idx: 6 },
        { name: "white", color: "white", order: 6, idx: 7 },
        { name: "Jump", color: "transparent", order: 7, idx: 8 },
        { name: "RGB", color: "transparent", order: 8, idx: 9 }
      ],
      public: { txColor: 0, rdMode: 0, runSpeed: 10, soundVal: 20 },
      item: { pyMode: 0, prjSelected: [0,0,0,0], ckValues: [] }
    };
  },
  // --- Lifecycle hooks ---
  created: function() {
    // mainPageComponent created
    if (this && this.$options && this.$options.methods && this.$options.methods.setMainPage) {
      this.$options.methods.setMainPage.call(this, this);
    }
    // appStateManager onLaunch
    if (this && this.$options && this.$options.methods && this.$options.methods.getDeviceInfo) {
      this.$options.methods.getDeviceInfo.call(this);
    }
    if (this && this.$options && this.$options.methods && this.$options.methods.getSysinfo) {
      this.$options.methods.getSysinfo.call(this);
    }
  },
  onLoad: function(e) {
    // mainPageComponent onLoad
    if (this && this.$options && this.$options.methods && this.$options.methods.genRandomCheck) {
      this.$options.methods.genRandomCheck.call(this);
    }
    // DeviceConfigPageController onLoad
    if (e && e.dmx !== undefined) {
      this.version = (this.version || '').trim();
      let settingData = this.getCmdData('settingData');
      settingData.dmx = e.dmx;
      Object.keys(settingData).forEach(key => {
        this[key] = settingData[key];
      });
      this.deviceInfo = this.getDeviceInfo();
      this.machine = ("0" + this.deviceInfo.deviceType).slice(-2) + " - " + ("0" + this.deviceInfo.version).slice(-2);
      if (this.initData) this.initData();
    }
    // bleDeviceProjectConfigPageComponent onLoad
    if (e && e.tag !== undefined) {
      let prjIndex = e.tag;
      let prjData = this.getCmdData('prjData');
      let projectConfig = {};
      let publicSettings = prjData.public;
      if (prjIndex == 1) {
        projectConfig = { public: publicSettings, prjIndex };
      } else {
        let index = prjData.prjItem[prjIndex + ""];
        projectConfig = { public: publicSettings, item: index, prjIndex };
        let selectionBits = this.getCkValues(projectConfig.item.prjSelected);
        projectConfig.item["ckValues"] = selectionBits;
      }
      this.prjIndex = projectConfig.prjIndex;
      this.public = projectConfig.public;
      this.item = projectConfig.item;
      if (this.prjIndex == 6 && this.features.showOutDoorTips) this.showOutDoorTips = true;
    }
  },
  onShow: function() {
    // mainPageComponent onShow
    this.features = this.getDeviceFeatures ? this.getDeviceFeatures() : {};
    if (!this.initShow) {
      this.initShow = true;
      if (this.bluInitPro) this.bluInitPro();
    }
    // appStateManager onShow
    if (this.blu_connected === 0 && this.mainPage && this.mainPage.gotoMain) {
      this.mainPage.gotoMain(true);
    }
  },
  onHide: function() {

    if (this.closeBLEConnection) {
      this.closeBLEConnection(() => {
        this.blu_state = 0;
        if (this.setBluCnnState) this.setBluCnnState(0, false);
        if (this.closeBluetoothAdapter) this.closeBluetoothAdapter();
      });
    }
  },
  onUnload: function() {

    let settingData = {
      ch: this.ch,
      dmx: this.dmx,
      xy: this.xy,
      light: this.light,
      cfg: this.cfg,
      lang: this.lang,
      valArr: this.valArr
    };
    if (this.setCmdData) this.setCmdData("settingData", settingData);
  },

  methods: {
    // mainPageComponent methods
    bluInitPro: function() {
      if (this.setBluCnnCallBack) this.setBluCnnCallBack(this.blu_cnn_call_back);
      if (this.setBluRecCallBack) this.setBluRecCallBack(this.blu_rec_call_back);
      if (this.cnnPreBlu) this.cnnPreBlu();
    },
    goQueryCmd: function() {
      if (this.gosend && this.getQueryCmd) this.gosend(false, this.getQueryCmd(this.randomCheck));
    },
    blu_cnn_call_back: function(connectionStatus, resultCode) {
      if (connectionStatus !== 1) {
        let device = this.ble_device;
        if (connectionStatus && device && "characteristicId" in device) {
          this.cnnDevice = device.name;
          this.cnnState = true;
          if (this.goQueryCmd) this.goQueryCmd();
        } else {
          this.cnnState = false;
          this.deviceOn = false;
          this.prjIndex = -1;
        }
      }
    },
    blu_rec_call_back: function(data) {
      if (this.checkRcvData && this.randomCheck) {
        if (this.checkRcvData(data, this.randomCheck)) {
          if (this.setCanSend) this.setCanSend(true);
          if (this.setCmdData) this.setCmdData(data);
          this.prjIndex = this.curMode;
        }
      }
    },
    genRandomCheck: function() {
      this.randomCheck = Array.from({ length: 4 }, () => Math.floor(256 * Math.random()));
    },
    checkRcvData: function(data, randomVerify) {
      if (!randomVerify || randomVerify.length !== 4 || !data || data.length < 24) return false;
      let r = data.substr(data.length - 24, 8);
      let n = [];
      for (let h = 0; h < 4; h++) {
        let i = 0, c = randomVerify[h];
        if (h === 0) i = (c + 55 >> 1) - 10 & 255;
        if (h === 1) i = 7 + (c - 68 << 1) & 255;
        if (h === 2) i = 15 + (c + 97 >> 1) & 255;
        if (h === 3) i = 87 + (c - 127 >> 1) & 255;
        n.push(i);
      }
      let o = [];
      for (let s = 0; s < 4; s++) {
        let l = r[2 * s] + r[2 * s + 1];
        let p = parseInt(l, 16);
        o.push(p);
      }
      for (let d = 0; d < 4; d++) if (o[d] !== n[d]) return false;
      let b = data.substr(data.length - 16, 2);
      this.deviceOn = parseInt(b, 16) !== 0;
      let deviceType = data.substr(data.length - 14, 2);
      let version = data.substr(data.length - 12, 2);
      let userType = data.substr(data.length - 10, 2);
      if (this.setDeviceInfo) this.setDeviceInfo(deviceType, version, userType);
      if (this.getDeviceFeatures) this.features = this.getDeviceFeatures();
      return true;
    },
    cnnLaser: function() {
      if (this.cnnLaser) this.cnnLaser();
    },
    settingClick: function(e) {
      let tag = e.currentTarget && e.currentTarget.dataset ? e.currentTarget.dataset.tag : undefined;
      if (tag !== 0 || this.deviceOn) {
        if (this.prjIndex !== tag && tag === 0) {
          this.prjIndex = tag;
          if (this.setCmdMode) this.setCmdMode(tag);
          if (this.sendCmd) this.sendCmd();
        } else if (this.navigateTo) {
          this.navigateTo({ url: `/pages/setting/setting?dmx=${tag}` });
        }
      }
    },
    onOffChange: function(t) {
      if (this.getCanSend && this.getCanSend()) {
        this.deviceOn = !this.deviceOn;
        let command = this.deviceOn ? "B0B1B2B3FFB4B5B6B7" : "B0B1B2B300B4B5B6B7";
        if (this.gosend) this.gosend(false, command);
      } else if (this.cnnState) {
        if (this.showModalTips) this.showModalTips(this.$t("The current device cannot be identified"), true);
      } else {
        if (this.showModalTips) this.showModalTips(this.$t("Please connect first 5Bluetooth"), true);
      }
    },
    testFunc: function() {
      if (this.setCmdData) this.setCmdData("E0E1E2E3B0B1B2B300B4B5B6B7C0C1C2C30400098080800080003309FFFFFF320000000000000000000000000000000000000000000000000000000000000000000000000000FF035393C06600000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001000A00FFFFFF020000000000000004050607D0D1D2D3820000FF28000000000000000000003200FF00FF28000000000000000000FF3200FFD4D5D6D7F0F1F2F300000000070102030405060700004466F4F5F6F743E3A317F0000000E4E5E6E7");
      this.prjIndex = this.curMode;
    },
    sendCmd: function() {
      if (this.getCmdStr && this.getDeviceFeatures) {
        let command = this.getCmdStr(this.cmd, { features: this.getDeviceFeatures() });
        this.modeCmdSend = command;
        if (this.doSendCmd) this.doSendCmd();
      }
    },
    doSendCmd: function() {
      if (this.modeCmdSend && this.modeCmdSend !== "") {
        let result = this.gosend ? this.gosend(false, this.modeCmdSend) : false;
        if (result) {
          this.modeCmdSend = "";
        } else {
          setTimeout(() => { if (this.doSendCmd) this.doSendCmd(); }, 100);
        }
      }
    },
    prjClick: function(e) {
      let mode = e.currentTarget && e.currentTarget.dataset ? e.currentTarget.dataset.tag : undefined;
      if (mode !== 0) {
        if (this.deviceOn) {
          if (this.prjIndex !== mode || (mode === 5 && this.features.ilda)) {
            this.prjIndex = mode;
            if (this.setCmdMode) this.setCmdMode(mode);
            if (this.sendCmd) this.sendCmd();
            return;
          }
          if (this.sendCmd) this.sendCmd();
          if (mode === 4 && this.navigateTo) this.navigateTo({ url: "/sub/pages/text/text" });
          if (mode === 7 && this.navigateTo) this.navigateTo({ url: "/sub2/pages/pgs/pgs" });
          if (mode === 8 && this.navigateTo) this.navigateTo({ url: "/sub/pages/draw/draw" });
          if (mode === 9 && this.navigateTo) this.navigateTo({ url: "/sub/pages/listMaster/listMaster" });
          if (mode >= 1 && mode <= 6 && mode !== 4 && this.navigateTo) this.navigateTo({ url: `/pages/prj/prj?tag=${mode}` });
        } else {
          if (this.settingClick) this.settingClick(e);
        }
      }
    },
    sendCmd2: function(xyConf) {
      if (this.blu_data_cmdSending) {
        if (!this.sendTimer) {
          this.sendTimer = setTimeout(() => {
            this.sendTimer = null;
            if (this.sendCmd2) this.sendCmd2(xyConf);
          }, 100);
        }
      } else if (!(this.lastCmdTime < this.lastSendTime)) {
        if (this.getDeviceFeatures && this.getShakeCmdStr && this.gosend) {
          let n = this.getDeviceFeatures();
          let command = this.getShakeCmdStr(this.cmd, { features: n, xyCnfSave: xyConf });
          let i = this.gosend(false, command);
          if (i) this.lastSendTime = (new Date()).getTime();
        }
      }
    },
    sendLastCmd: function(e) {
      this.lastCmdTime = (new Date()).getTime();
      if (this.sendCmd2) this.sendCmd2(e);
    },
    // --- DeviceConfigPageController methods ---
    sendCmdDeviceConfig: function() {
      let settingData = {
        ch: this.ch,
        dmx: this.dmx,
        xy: this.xy,
        light: this.light,
        cfg: this.cfg,
        lang: this.lang,
        valArr: this.valArr
      };
      if (this.setCmdData) this.setCmdData("settingData", settingData);
      if (this.getSettingCmd && this.gosend) {
        let command = this.getSettingCmd(this.cmd.settingData);
        this.SetCmdSend = command;
        if (this.doSendCmd) this.doSendCmd();
      }
    },
    doSendCmdDeviceConfig: function() {
      if (this.SetCmdSend && this.SetCmdSend !== "") {
        let commandResult = this.gosend ? this.gosend(false, this.SetCmdSend) : false;
        if (commandResult) {
          this.SetCmdSend = "";
        } else {
          setTimeout(() => { if (this.doSendCmdDeviceConfig) this.doSendCmdDeviceConfig(); }, 100);
        }
      }
    },
    initData: function() {
      if (this.deviceInfo && this.deviceInfo.deviceType === 1) {
        this.valRange = [ [1,512], [10,100], [0,100], [0,100], [0,100] ];
        for (let e = 2; e < 5; e++) if (this.valArr[e] > this.valRange[e][1]) this.valArr[e] = this.valRange[e][1];
      }
      if ((this.deviceInfo && this.deviceInfo.deviceType === 1) || (this.deviceInfo && this.deviceInfo.deviceType === 0 && this.deviceInfo.version >= 1)) {
        this.showCtr = { light1: false, light2: false, light3: false, lightExt: true };
      }
    },
    // --- bleDeviceProjectConfigPageComponent methods ---
    sendCmdProjectConfig: function(commandParams = null) {
      if (this.refreshShow) this.refreshShow();
      if (this.setCmdData) this.setCmdData("prjData", { prjIndex: this.prjIndex, public: this.public, item: this.item });
      if (!commandParams) commandParams = {};
      if (this.getDeviceFeatures) commandParams["features"] = this.getDeviceFeatures();
      if (this.getCmdStr && this.gosend) {
        let command = this.getCmdStr(this.cmd, commandParams);
        return this.gosend(false, command);
      }
      return false;
    },
    getCkValues: function(e) {
      let t = [];
      for (let r = 0; r < e.length; r++) {
        let n = e[r];
        for (let h = 0; h < 16; h++) {
          let a = n >> h & 1;
          t.push(a);
        }
      }
      return t;
    },
    getprjSelected: function(e) {
      let t = 0, r = [0, 0, 0, 0];
      for (let n = 0; n < e.length; n++) {
        let h = n % 16;
        if (e[n] === 1) t += Math.pow(2, h);
        if (h === 15) {
          r[(n + 1) / 16 - 1] = t;
          t = 0;
        }
      }
      return r;
    },
    selectAutoBtnClick: function(selectionAction) {
      let r = selectionAction, selectionBits = this.item.ckValues;
      for (let h = 0; h < selectionBits.length; h++) {
        if (r === 2) selectionBits[h] = selectionBits[h] === 1 ? 0 : 1;
        else selectionBits[h] = r === 3 ? 0 : 1;
      }
      let packedBits = this.getprjSelected(selectionBits);
      this.item.prjSelected = packedBits;
      this.item.ckValues = selectionBits;
      if (this.sendCmd) this.sendCmd();
    },
    checkboxChange: function(indices) {
      let packedBits = [0, 0, 0, 0];
      let n = indices.detail.value;
      for (let h = 0; h < n.length; h++) {
        let a = n[h] - 1,
          i = Math.floor(a / 16),
          c = a % 16,
          o = 1 << c;
        packedBits[i] = packedBits[i] | o;
      }
      this.item.prjSelected = packedBits;
      if (this.sendCmd) this.sendCmd();
    },
    btnSelectClick: function(selectedIndex) {
      if (this.item.pyMode !== 0) {
        let selectionBits = this.item.ckValues, commandParams = null;
        if (selectionBits[selectedIndex] === 1) selectionBits[selectedIndex] = 0;
        else {
          selectionBits[selectedIndex] = 1;
          commandParams = { prjParm: { prjIndex: this.prjIndex, selIndex: selectedIndex + 1 } };
        }
        let packedBits = this.getprjSelected(selectionBits);
        this.item.prjSelected = packedBits;
        this.item.ckValues = selectionBits;
        if (this.sendCmdParms) this.sendCmdParms(commandParams);
      }
    },
    sendCmdParms: function(commandParams) {
      if (this.sendCmd) this.sendCmd(commandParams);
    }
  }
};

export default UnifiedModule;
