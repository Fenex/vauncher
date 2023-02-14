"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var vss_1 = __importDefault(require("./vss"));
var log = function () {
    var args = [];
    for (var _i = 0; _i < arguments.length; _i++) {
        args[_i] = arguments[_i];
    }
    console.log.apply(console, __spreadArray(["main.js (loader):"], args, false));
};
// load all scripts inside scripts folder
log("== loading", vss_1.default.getScriptsFolder(), "scripts");
for (var _i = 0, _a = vss_1.default.scripts; _i < _a.length; _i++) {
    var next = _a[_i];
    var initFn = require(next).init;
    if (initFn !== undefined) {
        initFn();
        log(next, " started");
    }
}
log("== loading ednded");
