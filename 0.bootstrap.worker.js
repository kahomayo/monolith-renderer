self["webpackChunk"]([0],{

/***/ "../pkg/monolith_renderer.js":
/*!***********************************!*\
  !*** ../pkg/monolith_renderer.js ***!
  \***********************************/
/*! exports provided: get_result_data, get_result_len, fill_tile */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./monolith_renderer_bg.wasm */ \"../pkg/monolith_renderer_bg.wasm\");\n/* harmony import */ var _monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./monolith_renderer_bg.js */ \"../pkg/monolith_renderer_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"get_result_data\", function() { return _monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"get_result_data\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"get_result_len\", function() { return _monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"get_result_len\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"fill_tile\", function() { return _monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"fill_tile\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/monolith_renderer.js?");

/***/ }),

/***/ "../pkg/monolith_renderer_bg.js":
/*!**************************************!*\
  !*** ../pkg/monolith_renderer_bg.js ***!
  \**************************************/
/*! exports provided: get_result_data, get_result_len, fill_tile */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"get_result_data\", function() { return get_result_data; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"get_result_len\", function() { return get_result_len; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"fill_tile\", function() { return fill_tile; });\n/* harmony import */ var _monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./monolith_renderer_bg.wasm */ \"../pkg/monolith_renderer_bg.wasm\");\n\n\n/**\n* @returns {number}\n*/\nfunction get_result_data() {\n    var ret = _monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"get_result_data\"]();\n    return ret;\n}\n\n/**\n* @returns {number}\n*/\nfunction get_result_len() {\n    var ret = _monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"get_result_len\"]();\n    return ret >>> 0;\n}\n\nconst u32CvtShim = new Uint32Array(2);\n\nconst uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);\n/**\n* @param {BigInt} seed\n* @param {number} tile_x\n* @param {number} tile_y\n* @param {number} tile_z\n*/\nfunction fill_tile(seed, tile_x, tile_y, tile_z) {\n    uint64CvtShim[0] = seed;\n    const low0 = u32CvtShim[0];\n    const high0 = u32CvtShim[1];\n    _monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"fill_tile\"](low0, high0, tile_x, tile_y, tile_z);\n}\n\n\n\n//# sourceURL=webpack:///../pkg/monolith_renderer_bg.js?");

/***/ }),

/***/ "../pkg/monolith_renderer_bg.wasm":
/*!****************************************!*\
  !*** ../pkg/monolith_renderer_bg.wasm ***!
  \****************************************/
/*! exports provided: memory, get_result_data, get_result_len, fill_tile */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/monolith_renderer_bg.wasm?");

/***/ }),

/***/ "./tile.worker.js":
/*!************************!*\
  !*** ./tile.worker.js ***!
  \************************/
/*! exports provided: onmessage */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"onmessage\", function() { return onmessage; });\n/* harmony import */ var monolith_renderer__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! monolith-renderer */ \"../pkg/monolith_renderer.js\");\n/* harmony import */ var monolith_renderer_monolith_renderer_bg__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! monolith-renderer/monolith_renderer_bg */ \"../pkg/monolith_renderer_bg.wasm\");\n\r\n\r\n\r\nconst result_ptr = monolith_renderer__WEBPACK_IMPORTED_MODULE_0__[\"get_result_data\"]();\r\nconst result_len = monolith_renderer__WEBPACK_IMPORTED_MODULE_0__[\"get_result_len\"]();\r\n\r\nfunction onmessage(e) {\r\n    const data = e.data;\r\n    monolith_renderer__WEBPACK_IMPORTED_MODULE_0__[\"fill_tile\"](data.seed, data.tile_x, data.tile_y, data.tile_z);\r\n    const img = new ImageData(256, 256);\r\n    // Does this copy ?!?\r\n    const wasm_bytes = new Uint8Array(monolith_renderer_monolith_renderer_bg__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer);\r\n    const result_bytes = wasm_bytes.slice(result_ptr, result_ptr + result_len);\r\n    // Copy 1\r\n    img.data.set(result_bytes);\r\n    // Copy 2 -_-\r\n    postMessage({id: data.id, img: img});\r\n}\n\n//# sourceURL=webpack:///./tile.worker.js?");

/***/ })

});