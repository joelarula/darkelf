l = {
    data: function() {…},
    created: function() {…},
    onLoad: function() {…},
    onShow: function() {…},
    onReady: function() {…},
    computed: {
        functionsShow: function() {…}
    },
    methods: {
        bluInitPro: function() {… o.cnnPreBlu(); },
        clearBluTimer: function() {…},
        goQueryCmd: function() {… o.gosend(false, c.getQueryCmd(this.randomCheck)); },
        blu_cnn_call_back: function(t, r) {… o.setCanSend(true); o.setCmdData(t); },
        gotoMain: function() {…},
        testShow: function(t) {…},
        getCurPage: function() {…},
        blu_rec_call_back: function(t) {…},
        genRandomCheck: function() {…},
        checkRcvData: function(e, t) {…},
        voteTitle: function(e) {…},
        t: function(e) {…},
        cnnLaser: function() { o.cnnLaser() },
        settingClick: function(e) {…},
        onOffChange: function(t) {…
            if (o.getCanSend()) {… o.gosend(false, r); }
        },
        testFunc: function() {… o.setCmdData("E0E1E2E3B0B1B2B300B4B5B6B7C0C1C2C30400098080800080003309FFFFFF320000000000000000000000000000000000000000000000000000000000000000000000000000FF035393C06600000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001000A00FFFFFF020000000000000004050607D0D1D2D3820000FF28000000000000000000003200FF00FF28000000000000000000FF3200FFD4D5D6D7F0F1F2F300000000070102030405060700004466F4F5F6F743E3A317F0000000E4E5E6E7"); },
        sendCmd: function() {…},
        doSendCmd: function() {…
            if (this.modeCmdSend !== "") {
                var e = o.gosend(false, this.modeCmdSend);
            }
        },
        prjClick: function(e) {…},
        sendCmd2: function(t) {…
            var i = o.gosend(false, h);
        },
        sendLastCmd: function(e) {…},
        chDrawInit: function() {…},
        refreshAllChDraw: function() {…},
        refreshChDraw: function() {…},
        radioPhaseChange: function(e) {…},
        radioChange: function(e) {…},
        drawChCanvas: function(e, t, r, n, h, a) {…},
        addCnfValusAndSend: function(e) {…},
        chTouchstart: function(e) {…},
        chTouchmove: function(e) {…},
        chTouchend: function(e) {…}
    }
};


Therefore, the program is searching for services with the following UUID:

6e400001-b5a3-f393-e0a9-e50e24dcca9e
In addition to mserviceuuids, the n.globalData object also defines the following characteristics UUIDs:

mcharacteristicWrite: "6e400002-b5a3-f393-e0a9-e50e24dcca9e"
mcharacteristicNotify: "6e400003-b5a3-f393-e0a9-e50e24dcca9e"
mcharacteristicIndicate: "6e400003-b5a3-f393-e0a9-e50e24dcca9e"
mcharacteristictRead: "6e400003-b5a3-f393-e0a9-e50e24dcca9e"
mcharacteristictRssi: "2a56"