"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunkmonolith_renderer_app"] = self["webpackChunkmonolith_renderer_app"] || []).push([["tile_worker_js"],{

/***/ "../pkg/monolith_renderer.js":
/*!***********************************!*\
  !*** ../pkg/monolith_renderer.js ***!
  \***********************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   fill_tile: () => (/* reexport safe */ _monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_0__.fill_tile),\n/* harmony export */   get_result_data: () => (/* reexport safe */ _monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_0__.get_result_data),\n/* harmony export */   get_result_len: () => (/* reexport safe */ _monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_0__.get_result_len)\n/* harmony export */ });\n/* harmony import */ var _monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./monolith_renderer_bg.wasm */ \"../pkg/monolith_renderer_bg.wasm\");\n/* harmony import */ var _monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./monolith_renderer_bg.js */ \"../pkg/monolith_renderer_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n(0,_monolith_renderer_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://monolith-renderer-app/../pkg/monolith_renderer.js?");

/***/ }),

/***/ "../pkg/monolith_renderer_bg.js":
/*!**************************************!*\
  !*** ../pkg/monolith_renderer_bg.js ***!
  \**************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   fill_tile: () => (/* binding */ fill_tile),\n/* harmony export */   get_result_data: () => (/* binding */ get_result_data),\n/* harmony export */   get_result_len: () => (/* binding */ get_result_len)\n/* harmony export */ });\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n/**\n* @returns {number}\n*/\nfunction get_result_data() {\n    const ret = wasm.get_result_data();\n    return ret >>> 0;\n}\n\n/**\n* @returns {number}\n*/\nfunction get_result_len() {\n    const ret = wasm.get_result_len();\n    return ret >>> 0;\n}\n\n/**\n* @param {bigint} seed\n* @param {number} tile_x\n* @param {number} tile_y\n* @param {number} tile_z\n*/\nfunction fill_tile(seed, tile_x, tile_y, tile_z) {\n    wasm.fill_tile(seed, tile_x, tile_y, tile_z);\n}\n\n\n\n//# sourceURL=webpack://monolith-renderer-app/../pkg/monolith_renderer_bg.js?");

/***/ }),

/***/ "./tile.worker.js":
/*!************************!*\
  !*** ./tile.worker.js ***!
  \************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   onmessage: () => (/* binding */ onmessage)\n/* harmony export */ });\n/* harmony import */ var monolith_renderer__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! monolith-renderer */ \"../pkg/monolith_renderer.js\");\n/* harmony import */ var monolith_renderer_monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! monolith-renderer/monolith_renderer_bg.wasm */ \"../pkg/monolith_renderer_bg.wasm\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([monolith_renderer__WEBPACK_IMPORTED_MODULE_0__, monolith_renderer_monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n([monolith_renderer__WEBPACK_IMPORTED_MODULE_0__, monolith_renderer_monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_1__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\r\n\r\n\r\nconst result_ptr = monolith_renderer__WEBPACK_IMPORTED_MODULE_0__.get_result_data();\r\nconst result_len = monolith_renderer__WEBPACK_IMPORTED_MODULE_0__.get_result_len();\r\n\r\nfunction onmessage(e) {\r\n    const data = e.data;\r\n    monolith_renderer__WEBPACK_IMPORTED_MODULE_0__.fill_tile(data.seed, data.tile_x, data.tile_y, data.tile_z);\r\n    const img = new ImageData(256, 256);\r\n    // Does this copy ?!?\r\n    const wasm_bytes = new Uint8Array(monolith_renderer_monolith_renderer_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.memory.buffer);\r\n    const result_bytes = wasm_bytes.slice(result_ptr, result_ptr + result_len);\r\n    // Copy 1\r\n    img.data.set(result_bytes);\r\n    // Copy 2 -_-\r\n    postMessage({id: data.id, img: img});\r\n}\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://monolith-renderer-app/./tile.worker.js?");

/***/ }),

/***/ "../pkg/monolith_renderer_bg.wasm":
/*!****************************************!*\
  !*** ../pkg/monolith_renderer_bg.wasm ***!
  \****************************************/
/***/ ((module, exports, __webpack_require__) => {

eval("module.exports = __webpack_require__.v(exports, module.id, \"4f3cee612b3ea1a5\");\n\n//# sourceURL=webpack://monolith-renderer-app/../pkg/monolith_renderer_bg.wasm?");

/***/ })

}]);