// Stub implementation of the uni module for testing/build purposes
const uni = {
  getSystemInfoSync: () => ({ statusBarHeight: 20 }),
  setKeepScreenOn: () => {},
  getLocale: () => 'en',
  setLocale: () => {},
  showLoading: () => {},
  hideLoading: () => {},
  showToast: () => {},
  showModal: () => {},
  createSelectorQuery: () => ({ in: () => ({ select: () => ({ boundingClientRect: (cb) => ({ exec: () => cb({ width: 100, height: 100 }) }) }) }) }),
  createCanvasContext: () => ({
    setFillStyle: () => {},
    beginPath: () => {},
    moveTo: () => {},
    arc: () => {},
    rect: () => {},
    fill: () => {},
    setFontSize: () => {},
    setShadow: () => {},
    fillText: () => {},
    measureText: (text) => ({ width: text.length * 10 }),
    draw: () => {},
    createLinearGradient: () => ({ addColorStop: () => {} })
  }),
  getStorageSync: () => undefined,
  setStorageSync: () => {},
  removeStorageSync: () => {},
  reLaunch: () => {},
  navigateTo: () => {},
  nextTick: (cb) => cb(),
  getCurrentPages: () => [{ route: 'pages/main/main' }],
};

module.exports = uni;
