"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.FileOpenFlags = exports.ASFlag = exports.ASMode = exports.actEventCodes = exports.actintItemEvents = exports.iScreenOptionId = exports.SDLScanCode = exports.RoadRuntimeObjId = exports.VssMath = void 0;
var global = new Function("return this;")();
var VssMath = /** @class */ (function () {
    function VssMath() {
        this.PI = 1 << 11;
        this.PI_2 = this.PI / 2;
    }
    VssMath.prototype.angleToRadians = function (angle) {
        return angle * Math.PI / this.PI;
    };
    VssMath.prototype.radiansToAngle = function (radians) {
        return Math.round(radians * this.PI / Math.PI);
    };
    return VssMath;
}());
exports.VssMath = VssMath;
var Vss = /** @class */ (function () {
    function Vss() {
        // game math
        this.math = new VssMath();
        this.quantListeners = {};
        this.getScriptsFolder = bridge.getScriptsFolder;
        this.initScripts = bridge.initScripts;
        this.sendEvent = bridge.sendEvent;
        this.isKeyPressed = bridge.isKeyPressed;
        this.isFileExists = bridge.isFileExists;
        this.scripts = bridge.scripts().filter(function (value) {
            return value !== "vss.js" && value !== "main.js" && value.endsWith(".js");
        }).map(function (value) { return value.substring(0, value.length - 3); });
        global.onVssQuant = this.onVssQuant.bind(this);
    }
    Vss.prototype.fatal = function (msg) {
        bridge.fatal("vss: " + msg);
    };
    Vss.prototype.addQuantListener = function (quant, listener) {
        if (this.quantListeners[quant] === undefined) {
            this.quantListeners[quant] = [];
        }
        this.quantListeners[quant].push(listener);
    };
    Vss.prototype.removeQuantListener = function (quant, listener) {
        var _a;
        var index = (_a = this.quantListeners[quant]) === null || _a === void 0 ? void 0 : _a.indexOf(listener);
        if (index !== undefined && index !== -1) {
            this.quantListeners[quant].splice(index, 1);
        }
    };
    Vss.prototype.onVssQuant = function (quant, payload) {
        var listeners = this.quantListeners[quant];
        if (listeners === undefined || listeners.length === 0) {
            return undefined;
        }
        var resultRef = {
            result: {
                preventDefault: false,
            },
            runNext: true,
            preventDefault: false,
        };
        var stopPropogation = function () {
            resultRef.runNext = false;
        };
        for (var _i = 0, listeners_1 = listeners; _i < listeners_1.length; _i++) {
            var next = listeners_1[_i];
            var result = next(payload, stopPropogation, quant);
            if (result === "preventDefault") {
                resultRef.result.preventDefault = true;
            }
            else if (result !== undefined) {
                resultRef.result = __assign(__assign({}, resultRef.result), result);
            }
            if (!resultRef.runNext) {
                break;
            }
        }
        return resultRef.result;
    };
    return Vss;
}());
/**
 * Vangers Scripting Subsystem exports `vss` object as entry point to game API
 * you must use it to interact with game.
 */
var vss = new Vss();
exports.default = vss;
// == in game definitions
var RoadRuntimeObjId;
(function (RoadRuntimeObjId) {
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_GAME_QUANT_ID"] = 1] = "RTO_GAME_QUANT_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_LOADING1_ID"] = 2] = "RTO_LOADING1_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_LOADING2_ID"] = 3] = "RTO_LOADING2_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_LOADING3_ID"] = 4] = "RTO_LOADING3_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_MAIN_MENU_ID"] = 5] = "RTO_MAIN_MENU_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_FIRST_ESCAVE_ID"] = 6] = "RTO_FIRST_ESCAVE_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_FIRST_ESCAVE_OUT_ID"] = 7] = "RTO_FIRST_ESCAVE_OUT_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_ESCAVE_ID"] = 8] = "RTO_ESCAVE_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_ESCAVE_OUT_ID"] = 9] = "RTO_ESCAVE_OUT_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_PALETTE_TRANSFORM_ID"] = 10] = "RTO_PALETTE_TRANSFORM_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_SHOW_IMAGE_ID"] = 11] = "RTO_SHOW_IMAGE_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_SHOW_AVI_ID"] = 12] = "RTO_SHOW_AVI_ID";
    RoadRuntimeObjId[RoadRuntimeObjId["RTO_MAX_ID"] = 13] = "RTO_MAX_ID";
})(RoadRuntimeObjId = exports.RoadRuntimeObjId || (exports.RoadRuntimeObjId = {}));
;
var SDLScanCode;
(function (SDLScanCode) {
    SDLScanCode[SDLScanCode["SDL_SCANCODE_UNKNOWN"] = 0] = "SDL_SCANCODE_UNKNOWN";
    /**
     *  \name Usage page 0x07
     *
     *  These values are from usage page 0x07 (USB keyboard page).
     */
    /* @{ */
    SDLScanCode[SDLScanCode["SDL_SCANCODE_A"] = 4] = "SDL_SCANCODE_A";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_B"] = 5] = "SDL_SCANCODE_B";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_C"] = 6] = "SDL_SCANCODE_C";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_D"] = 7] = "SDL_SCANCODE_D";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_E"] = 8] = "SDL_SCANCODE_E";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F"] = 9] = "SDL_SCANCODE_F";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_G"] = 10] = "SDL_SCANCODE_G";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_H"] = 11] = "SDL_SCANCODE_H";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_I"] = 12] = "SDL_SCANCODE_I";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_J"] = 13] = "SDL_SCANCODE_J";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_K"] = 14] = "SDL_SCANCODE_K";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_L"] = 15] = "SDL_SCANCODE_L";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_M"] = 16] = "SDL_SCANCODE_M";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_N"] = 17] = "SDL_SCANCODE_N";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_O"] = 18] = "SDL_SCANCODE_O";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_P"] = 19] = "SDL_SCANCODE_P";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_Q"] = 20] = "SDL_SCANCODE_Q";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_R"] = 21] = "SDL_SCANCODE_R";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_S"] = 22] = "SDL_SCANCODE_S";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_T"] = 23] = "SDL_SCANCODE_T";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_U"] = 24] = "SDL_SCANCODE_U";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_V"] = 25] = "SDL_SCANCODE_V";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_W"] = 26] = "SDL_SCANCODE_W";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_X"] = 27] = "SDL_SCANCODE_X";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_Y"] = 28] = "SDL_SCANCODE_Y";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_Z"] = 29] = "SDL_SCANCODE_Z";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_1"] = 30] = "SDL_SCANCODE_1";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_2"] = 31] = "SDL_SCANCODE_2";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_3"] = 32] = "SDL_SCANCODE_3";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_4"] = 33] = "SDL_SCANCODE_4";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_5"] = 34] = "SDL_SCANCODE_5";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_6"] = 35] = "SDL_SCANCODE_6";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_7"] = 36] = "SDL_SCANCODE_7";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_8"] = 37] = "SDL_SCANCODE_8";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_9"] = 38] = "SDL_SCANCODE_9";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_0"] = 39] = "SDL_SCANCODE_0";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_RETURN"] = 40] = "SDL_SCANCODE_RETURN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_ESCAPE"] = 41] = "SDL_SCANCODE_ESCAPE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_BACKSPACE"] = 42] = "SDL_SCANCODE_BACKSPACE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_TAB"] = 43] = "SDL_SCANCODE_TAB";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_SPACE"] = 44] = "SDL_SCANCODE_SPACE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_MINUS"] = 45] = "SDL_SCANCODE_MINUS";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_EQUALS"] = 46] = "SDL_SCANCODE_EQUALS";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LEFTBRACKET"] = 47] = "SDL_SCANCODE_LEFTBRACKET";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_RIGHTBRACKET"] = 48] = "SDL_SCANCODE_RIGHTBRACKET";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_BACKSLASH"] = 49] = "SDL_SCANCODE_BACKSLASH";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_NONUSHASH"] = 50] = "SDL_SCANCODE_NONUSHASH";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_SEMICOLON"] = 51] = "SDL_SCANCODE_SEMICOLON";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_APOSTROPHE"] = 52] = "SDL_SCANCODE_APOSTROPHE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_GRAVE"] = 53] = "SDL_SCANCODE_GRAVE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_COMMA"] = 54] = "SDL_SCANCODE_COMMA";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_PERIOD"] = 55] = "SDL_SCANCODE_PERIOD";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_SLASH"] = 56] = "SDL_SCANCODE_SLASH";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_CAPSLOCK"] = 57] = "SDL_SCANCODE_CAPSLOCK";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F1"] = 58] = "SDL_SCANCODE_F1";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F2"] = 59] = "SDL_SCANCODE_F2";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F3"] = 60] = "SDL_SCANCODE_F3";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F4"] = 61] = "SDL_SCANCODE_F4";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F5"] = 62] = "SDL_SCANCODE_F5";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F6"] = 63] = "SDL_SCANCODE_F6";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F7"] = 64] = "SDL_SCANCODE_F7";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F8"] = 65] = "SDL_SCANCODE_F8";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F9"] = 66] = "SDL_SCANCODE_F9";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F10"] = 67] = "SDL_SCANCODE_F10";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F11"] = 68] = "SDL_SCANCODE_F11";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F12"] = 69] = "SDL_SCANCODE_F12";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_PRINTSCREEN"] = 70] = "SDL_SCANCODE_PRINTSCREEN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_SCROLLLOCK"] = 71] = "SDL_SCANCODE_SCROLLLOCK";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_PAUSE"] = 72] = "SDL_SCANCODE_PAUSE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INSERT"] = 73] = "SDL_SCANCODE_INSERT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_HOME"] = 74] = "SDL_SCANCODE_HOME";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_PAGEUP"] = 75] = "SDL_SCANCODE_PAGEUP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_DELETE"] = 76] = "SDL_SCANCODE_DELETE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_END"] = 77] = "SDL_SCANCODE_END";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_PAGEDOWN"] = 78] = "SDL_SCANCODE_PAGEDOWN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_RIGHT"] = 79] = "SDL_SCANCODE_RIGHT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LEFT"] = 80] = "SDL_SCANCODE_LEFT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_DOWN"] = 81] = "SDL_SCANCODE_DOWN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_UP"] = 82] = "SDL_SCANCODE_UP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_NUMLOCKCLEAR"] = 83] = "SDL_SCANCODE_NUMLOCKCLEAR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_DIVIDE"] = 84] = "SDL_SCANCODE_KP_DIVIDE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_MULTIPLY"] = 85] = "SDL_SCANCODE_KP_MULTIPLY";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_MINUS"] = 86] = "SDL_SCANCODE_KP_MINUS";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_PLUS"] = 87] = "SDL_SCANCODE_KP_PLUS";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_ENTER"] = 88] = "SDL_SCANCODE_KP_ENTER";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_1"] = 89] = "SDL_SCANCODE_KP_1";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_2"] = 90] = "SDL_SCANCODE_KP_2";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_3"] = 91] = "SDL_SCANCODE_KP_3";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_4"] = 92] = "SDL_SCANCODE_KP_4";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_5"] = 93] = "SDL_SCANCODE_KP_5";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_6"] = 94] = "SDL_SCANCODE_KP_6";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_7"] = 95] = "SDL_SCANCODE_KP_7";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_8"] = 96] = "SDL_SCANCODE_KP_8";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_9"] = 97] = "SDL_SCANCODE_KP_9";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_0"] = 98] = "SDL_SCANCODE_KP_0";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_PERIOD"] = 99] = "SDL_SCANCODE_KP_PERIOD";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_NONUSBACKSLASH"] = 100] = "SDL_SCANCODE_NONUSBACKSLASH";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_APPLICATION"] = 101] = "SDL_SCANCODE_APPLICATION";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_POWER"] = 102] = "SDL_SCANCODE_POWER";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_EQUALS"] = 103] = "SDL_SCANCODE_KP_EQUALS";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F13"] = 104] = "SDL_SCANCODE_F13";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F14"] = 105] = "SDL_SCANCODE_F14";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F15"] = 106] = "SDL_SCANCODE_F15";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F16"] = 107] = "SDL_SCANCODE_F16";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F17"] = 108] = "SDL_SCANCODE_F17";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F18"] = 109] = "SDL_SCANCODE_F18";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F19"] = 110] = "SDL_SCANCODE_F19";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F20"] = 111] = "SDL_SCANCODE_F20";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F21"] = 112] = "SDL_SCANCODE_F21";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F22"] = 113] = "SDL_SCANCODE_F22";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F23"] = 114] = "SDL_SCANCODE_F23";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_F24"] = 115] = "SDL_SCANCODE_F24";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_EXECUTE"] = 116] = "SDL_SCANCODE_EXECUTE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_HELP"] = 117] = "SDL_SCANCODE_HELP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_MENU"] = 118] = "SDL_SCANCODE_MENU";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_SELECT"] = 119] = "SDL_SCANCODE_SELECT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_STOP"] = 120] = "SDL_SCANCODE_STOP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AGAIN"] = 121] = "SDL_SCANCODE_AGAIN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_UNDO"] = 122] = "SDL_SCANCODE_UNDO";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_CUT"] = 123] = "SDL_SCANCODE_CUT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_COPY"] = 124] = "SDL_SCANCODE_COPY";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_PASTE"] = 125] = "SDL_SCANCODE_PASTE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_FIND"] = 126] = "SDL_SCANCODE_FIND";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_MUTE"] = 127] = "SDL_SCANCODE_MUTE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_VOLUMEUP"] = 128] = "SDL_SCANCODE_VOLUMEUP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_VOLUMEDOWN"] = 129] = "SDL_SCANCODE_VOLUMEDOWN";
    /* not sure whether there's a reason to enable these */
    /*     SDL_SCANCODE_LOCKINGCAPSLOCK = 130,  */
    /*     SDL_SCANCODE_LOCKINGNUMLOCK = 131, */
    /*     SDL_SCANCODE_LOCKINGSCROLLLOCK = 132, */
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_COMMA"] = 133] = "SDL_SCANCODE_KP_COMMA";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_EQUALSAS400"] = 134] = "SDL_SCANCODE_KP_EQUALSAS400";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INTERNATIONAL1"] = 135] = "SDL_SCANCODE_INTERNATIONAL1";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INTERNATIONAL2"] = 136] = "SDL_SCANCODE_INTERNATIONAL2";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INTERNATIONAL3"] = 137] = "SDL_SCANCODE_INTERNATIONAL3";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INTERNATIONAL4"] = 138] = "SDL_SCANCODE_INTERNATIONAL4";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INTERNATIONAL5"] = 139] = "SDL_SCANCODE_INTERNATIONAL5";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INTERNATIONAL6"] = 140] = "SDL_SCANCODE_INTERNATIONAL6";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INTERNATIONAL7"] = 141] = "SDL_SCANCODE_INTERNATIONAL7";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INTERNATIONAL8"] = 142] = "SDL_SCANCODE_INTERNATIONAL8";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_INTERNATIONAL9"] = 143] = "SDL_SCANCODE_INTERNATIONAL9";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LANG1"] = 144] = "SDL_SCANCODE_LANG1";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LANG2"] = 145] = "SDL_SCANCODE_LANG2";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LANG3"] = 146] = "SDL_SCANCODE_LANG3";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LANG4"] = 147] = "SDL_SCANCODE_LANG4";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LANG5"] = 148] = "SDL_SCANCODE_LANG5";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LANG6"] = 149] = "SDL_SCANCODE_LANG6";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LANG7"] = 150] = "SDL_SCANCODE_LANG7";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LANG8"] = 151] = "SDL_SCANCODE_LANG8";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LANG9"] = 152] = "SDL_SCANCODE_LANG9";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_ALTERASE"] = 153] = "SDL_SCANCODE_ALTERASE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_SYSREQ"] = 154] = "SDL_SCANCODE_SYSREQ";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_CANCEL"] = 155] = "SDL_SCANCODE_CANCEL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_CLEAR"] = 156] = "SDL_SCANCODE_CLEAR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_PRIOR"] = 157] = "SDL_SCANCODE_PRIOR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_RETURN2"] = 158] = "SDL_SCANCODE_RETURN2";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_SEPARATOR"] = 159] = "SDL_SCANCODE_SEPARATOR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_OUT"] = 160] = "SDL_SCANCODE_OUT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_OPER"] = 161] = "SDL_SCANCODE_OPER";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_CLEARAGAIN"] = 162] = "SDL_SCANCODE_CLEARAGAIN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_CRSEL"] = 163] = "SDL_SCANCODE_CRSEL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_EXSEL"] = 164] = "SDL_SCANCODE_EXSEL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_00"] = 176] = "SDL_SCANCODE_KP_00";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_000"] = 177] = "SDL_SCANCODE_KP_000";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_THOUSANDSSEPARATOR"] = 178] = "SDL_SCANCODE_THOUSANDSSEPARATOR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_DECIMALSEPARATOR"] = 179] = "SDL_SCANCODE_DECIMALSEPARATOR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_CURRENCYUNIT"] = 180] = "SDL_SCANCODE_CURRENCYUNIT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_CURRENCYSUBUNIT"] = 181] = "SDL_SCANCODE_CURRENCYSUBUNIT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_LEFTPAREN"] = 182] = "SDL_SCANCODE_KP_LEFTPAREN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_RIGHTPAREN"] = 183] = "SDL_SCANCODE_KP_RIGHTPAREN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_LEFTBRACE"] = 184] = "SDL_SCANCODE_KP_LEFTBRACE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_RIGHTBRACE"] = 185] = "SDL_SCANCODE_KP_RIGHTBRACE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_TAB"] = 186] = "SDL_SCANCODE_KP_TAB";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_BACKSPACE"] = 187] = "SDL_SCANCODE_KP_BACKSPACE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_A"] = 188] = "SDL_SCANCODE_KP_A";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_B"] = 189] = "SDL_SCANCODE_KP_B";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_C"] = 190] = "SDL_SCANCODE_KP_C";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_D"] = 191] = "SDL_SCANCODE_KP_D";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_E"] = 192] = "SDL_SCANCODE_KP_E";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_F"] = 193] = "SDL_SCANCODE_KP_F";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_XOR"] = 194] = "SDL_SCANCODE_KP_XOR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_POWER"] = 195] = "SDL_SCANCODE_KP_POWER";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_PERCENT"] = 196] = "SDL_SCANCODE_KP_PERCENT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_LESS"] = 197] = "SDL_SCANCODE_KP_LESS";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_GREATER"] = 198] = "SDL_SCANCODE_KP_GREATER";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_AMPERSAND"] = 199] = "SDL_SCANCODE_KP_AMPERSAND";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_DBLAMPERSAND"] = 200] = "SDL_SCANCODE_KP_DBLAMPERSAND";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_VERTICALBAR"] = 201] = "SDL_SCANCODE_KP_VERTICALBAR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_DBLVERTICALBAR"] = 202] = "SDL_SCANCODE_KP_DBLVERTICALBAR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_COLON"] = 203] = "SDL_SCANCODE_KP_COLON";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_HASH"] = 204] = "SDL_SCANCODE_KP_HASH";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_SPACE"] = 205] = "SDL_SCANCODE_KP_SPACE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_AT"] = 206] = "SDL_SCANCODE_KP_AT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_EXCLAM"] = 207] = "SDL_SCANCODE_KP_EXCLAM";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_MEMSTORE"] = 208] = "SDL_SCANCODE_KP_MEMSTORE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_MEMRECALL"] = 209] = "SDL_SCANCODE_KP_MEMRECALL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_MEMCLEAR"] = 210] = "SDL_SCANCODE_KP_MEMCLEAR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_MEMADD"] = 211] = "SDL_SCANCODE_KP_MEMADD";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_MEMSUBTRACT"] = 212] = "SDL_SCANCODE_KP_MEMSUBTRACT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_MEMMULTIPLY"] = 213] = "SDL_SCANCODE_KP_MEMMULTIPLY";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_MEMDIVIDE"] = 214] = "SDL_SCANCODE_KP_MEMDIVIDE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_PLUSMINUS"] = 215] = "SDL_SCANCODE_KP_PLUSMINUS";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_CLEAR"] = 216] = "SDL_SCANCODE_KP_CLEAR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_CLEARENTRY"] = 217] = "SDL_SCANCODE_KP_CLEARENTRY";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_BINARY"] = 218] = "SDL_SCANCODE_KP_BINARY";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_OCTAL"] = 219] = "SDL_SCANCODE_KP_OCTAL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_DECIMAL"] = 220] = "SDL_SCANCODE_KP_DECIMAL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KP_HEXADECIMAL"] = 221] = "SDL_SCANCODE_KP_HEXADECIMAL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LCTRL"] = 224] = "SDL_SCANCODE_LCTRL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LSHIFT"] = 225] = "SDL_SCANCODE_LSHIFT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LALT"] = 226] = "SDL_SCANCODE_LALT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_LGUI"] = 227] = "SDL_SCANCODE_LGUI";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_RCTRL"] = 228] = "SDL_SCANCODE_RCTRL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_RSHIFT"] = 229] = "SDL_SCANCODE_RSHIFT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_RALT"] = 230] = "SDL_SCANCODE_RALT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_RGUI"] = 231] = "SDL_SCANCODE_RGUI";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_MODE"] = 257] = "SDL_SCANCODE_MODE";
    /* @} */ /* Usage page 0x07 */
    /**
     *  \name Usage page 0x0C
     *
     *  These values are mapped from usage page 0x0C (USB consumer page).
     */
    /* @{ */
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AUDIONEXT"] = 258] = "SDL_SCANCODE_AUDIONEXT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AUDIOPREV"] = 259] = "SDL_SCANCODE_AUDIOPREV";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AUDIOSTOP"] = 260] = "SDL_SCANCODE_AUDIOSTOP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AUDIOPLAY"] = 261] = "SDL_SCANCODE_AUDIOPLAY";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AUDIOMUTE"] = 262] = "SDL_SCANCODE_AUDIOMUTE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_MEDIASELECT"] = 263] = "SDL_SCANCODE_MEDIASELECT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_WWW"] = 264] = "SDL_SCANCODE_WWW";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_MAIL"] = 265] = "SDL_SCANCODE_MAIL";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_CALCULATOR"] = 266] = "SDL_SCANCODE_CALCULATOR";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_COMPUTER"] = 267] = "SDL_SCANCODE_COMPUTER";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AC_SEARCH"] = 268] = "SDL_SCANCODE_AC_SEARCH";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AC_HOME"] = 269] = "SDL_SCANCODE_AC_HOME";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AC_BACK"] = 270] = "SDL_SCANCODE_AC_BACK";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AC_FORWARD"] = 271] = "SDL_SCANCODE_AC_FORWARD";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AC_STOP"] = 272] = "SDL_SCANCODE_AC_STOP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AC_REFRESH"] = 273] = "SDL_SCANCODE_AC_REFRESH";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AC_BOOKMARKS"] = 274] = "SDL_SCANCODE_AC_BOOKMARKS";
    /* @} */ /* Usage page 0x0C */
    /**
     *  \name Walther keys
     *
     *  These are values that Christian Walther added (for mac keyboard?).
     */
    /* @{ */
    SDLScanCode[SDLScanCode["SDL_SCANCODE_BRIGHTNESSDOWN"] = 275] = "SDL_SCANCODE_BRIGHTNESSDOWN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_BRIGHTNESSUP"] = 276] = "SDL_SCANCODE_BRIGHTNESSUP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_DISPLAYSWITCH"] = 277] = "SDL_SCANCODE_DISPLAYSWITCH";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KBDILLUMTOGGLE"] = 278] = "SDL_SCANCODE_KBDILLUMTOGGLE";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KBDILLUMDOWN"] = 279] = "SDL_SCANCODE_KBDILLUMDOWN";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_KBDILLUMUP"] = 280] = "SDL_SCANCODE_KBDILLUMUP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_EJECT"] = 281] = "SDL_SCANCODE_EJECT";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_SLEEP"] = 282] = "SDL_SCANCODE_SLEEP";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_APP1"] = 283] = "SDL_SCANCODE_APP1";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_APP2"] = 284] = "SDL_SCANCODE_APP2";
    /* @} */ /* Walther keys */
    /**
     *  \name Usage page 0x0C (additional media keys)
     *
     *  These values are mapped from usage page 0x0C (USB consumer page).
     */
    /* @{ */
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AUDIOREWIND"] = 285] = "SDL_SCANCODE_AUDIOREWIND";
    SDLScanCode[SDLScanCode["SDL_SCANCODE_AUDIOFASTFORWARD"] = 286] = "SDL_SCANCODE_AUDIOFASTFORWARD";
    /* @} */ /* Usage page 0x0C (additional media keys) */
    /* Add any other keys here. */
    SDLScanCode[SDLScanCode["SDL_NUM_SCANCODES"] = 512] = "SDL_NUM_SCANCODES"; /** < not a key, just marks the number of scancodes
                                 for array bounds */
})(SDLScanCode = exports.SDLScanCode || (exports.SDLScanCode = {}));
;
var iScreenOptionId;
(function (iScreenOptionId) {
    iScreenOptionId[iScreenOptionId["iSOUND_ON"] = 0] = "iSOUND_ON";
    iScreenOptionId[iScreenOptionId["iSOUND_VOLUME_CUR"] = 1] = "iSOUND_VOLUME_CUR";
    iScreenOptionId[iScreenOptionId["iSOUND_VOLUME_MAX"] = 2] = "iSOUND_VOLUME_MAX";
    iScreenOptionId[iScreenOptionId["iMUSIC_ON"] = 3] = "iMUSIC_ON";
    iScreenOptionId[iScreenOptionId["iMUSIC_VOLUME_CUR"] = 4] = "iMUSIC_VOLUME_CUR";
    iScreenOptionId[iScreenOptionId["iMUSIC_VOLUME_MAX"] = 5] = "iMUSIC_VOLUME_MAX";
    iScreenOptionId[iScreenOptionId["iTUTORIAL_ON"] = 6] = "iTUTORIAL_ON";
    iScreenOptionId[iScreenOptionId["iDETAIL_SETTING"] = 7] = "iDETAIL_SETTING";
    iScreenOptionId[iScreenOptionId["iPLAYER_COLOR3"] = 8] = "iPLAYER_COLOR3";
    iScreenOptionId[iScreenOptionId["iPLAYER_COLOR"] = 9] = "iPLAYER_COLOR";
    iScreenOptionId[iScreenOptionId["iSERVER_NAME"] = 10] = "iSERVER_NAME";
    iScreenOptionId[iScreenOptionId["iSERVER_NAME2"] = 11] = "iSERVER_NAME2";
    iScreenOptionId[iScreenOptionId["iPLAYER_PASSWORD"] = 12] = "iPLAYER_PASSWORD";
    iScreenOptionId[iScreenOptionId["iMPGAME0_ID"] = 13] = "iMPGAME0_ID";
    iScreenOptionId[iScreenOptionId["iMPGAME1_ID"] = 14] = "iMPGAME1_ID";
    iScreenOptionId[iScreenOptionId["iMPGAME2_ID"] = 15] = "iMPGAME2_ID";
    iScreenOptionId[iScreenOptionId["iMPGAME3_ID"] = 16] = "iMPGAME3_ID";
    iScreenOptionId[iScreenOptionId["iMPGAME4_ID"] = 17] = "iMPGAME4_ID";
    iScreenOptionId[iScreenOptionId["iCUR_MPGAME_ID"] = 18] = "iCUR_MPGAME_ID";
    iScreenOptionId[iScreenOptionId["iCUR_MPGAME_ID2"] = 19] = "iCUR_MPGAME_ID2";
    iScreenOptionId[iScreenOptionId["iPLAYER_NAME2"] = 20] = "iPLAYER_NAME2";
    iScreenOptionId[iScreenOptionId["iSCREEN_RESOLUTION"] = 21] = "iSCREEN_RESOLUTION";
    iScreenOptionId[iScreenOptionId["iHOST_NAME"] = 22] = "iHOST_NAME";
    iScreenOptionId[iScreenOptionId["iKEEP_IN_USE"] = 23] = "iKEEP_IN_USE";
    iScreenOptionId[iScreenOptionId["iKEEP_CLEAN_UP"] = 24] = "iKEEP_CLEAN_UP";
    iScreenOptionId[iScreenOptionId["iKEEP_MODE"] = 25] = "iKEEP_MODE";
    iScreenOptionId[iScreenOptionId["iPANNING_ON"] = 26] = "iPANNING_ON";
    iScreenOptionId[iScreenOptionId["iDESTR_MODE"] = 27] = "iDESTR_MODE";
    iScreenOptionId[iScreenOptionId["iPLAYER_COLOR2"] = 28] = "iPLAYER_COLOR2";
    iScreenOptionId[iScreenOptionId["iPLAYER_NAME3"] = 29] = "iPLAYER_NAME3";
    iScreenOptionId[iScreenOptionId["iMECH_SOUND"] = 30] = "iMECH_SOUND";
    iScreenOptionId[iScreenOptionId["iBACK_SOUND"] = 31] = "iBACK_SOUND";
    iScreenOptionId[iScreenOptionId["iJOYSTICK_TYPE"] = 32] = "iJOYSTICK_TYPE";
    iScreenOptionId[iScreenOptionId["iPROXY_USAGE"] = 33] = "iPROXY_USAGE";
    iScreenOptionId[iScreenOptionId["iPROXY_SERVER"] = 34] = "iPROXY_SERVER";
    iScreenOptionId[iScreenOptionId["iPROXY_PORT"] = 35] = "iPROXY_PORT";
    iScreenOptionId[iScreenOptionId["iPROXY_SERVER_STR"] = 36] = "iPROXY_SERVER_STR";
    iScreenOptionId[iScreenOptionId["iPROXY_PORT_STR"] = 37] = "iPROXY_PORT_STR";
    iScreenOptionId[iScreenOptionId["iSERVER_PORT"] = 38] = "iSERVER_PORT";
    iScreenOptionId[iScreenOptionId["iPLAYER_NAME_CR"] = 39] = "iPLAYER_NAME_CR";
    iScreenOptionId[iScreenOptionId["iPLAYER_PASSWORD_CR"] = 40] = "iPLAYER_PASSWORD_CR";
    iScreenOptionId[iScreenOptionId["iIP_ADDRESS"] = 41] = "iIP_ADDRESS";
    iScreenOptionId[iScreenOptionId["iCAMERA_TURN"] = 42] = "iCAMERA_TURN";
    iScreenOptionId[iScreenOptionId["iCAMERA_SLOPE"] = 43] = "iCAMERA_SLOPE";
    iScreenOptionId[iScreenOptionId["iCAMERA_SCALE"] = 44] = "iCAMERA_SCALE";
    iScreenOptionId[iScreenOptionId["iFULLSCREEN"] = 45] = "iFULLSCREEN";
    iScreenOptionId[iScreenOptionId["iAUTO_ACCELERATION"] = 46] = "iAUTO_ACCELERATION";
    iScreenOptionId[iScreenOptionId["iMAX_OPTION_ID"] = 47] = "iMAX_OPTION_ID";
})(iScreenOptionId = exports.iScreenOptionId || (exports.iScreenOptionId = {}));
;
var actintItemEvents;
(function (actintItemEvents) {
    actintItemEvents[actintItemEvents["ACI_PUT_ITEM"] = 1] = "ACI_PUT_ITEM";
    actintItemEvents[actintItemEvents["ACI_DROP_ITEM"] = 2] = "ACI_DROP_ITEM";
    actintItemEvents[actintItemEvents["ACI_CHANGE_ITEM_DATA"] = 3] = "ACI_CHANGE_ITEM_DATA";
    actintItemEvents[actintItemEvents["ACI_ACTIVATE_ITEM"] = 4] = "ACI_ACTIVATE_ITEM";
    actintItemEvents[actintItemEvents["ACI_DEACTIVATE_ITEM"] = 5] = "ACI_DEACTIVATE_ITEM";
    actintItemEvents[actintItemEvents["ACI_GET_ITEM_DATA"] = 6] = "ACI_GET_ITEM_DATA";
    actintItemEvents[actintItemEvents["ACI_CHECK_MOUSE"] = 7] = "ACI_CHECK_MOUSE";
    actintItemEvents[actintItemEvents["ACI_SET_DROP_LEVEL"] = 8] = "ACI_SET_DROP_LEVEL";
    actintItemEvents[actintItemEvents["ACI_PUT_IN_SLOT"] = 9] = "ACI_PUT_IN_SLOT";
    actintItemEvents[actintItemEvents["ACI_LOCK_INTERFACE"] = 10] = "ACI_LOCK_INTERFACE";
    actintItemEvents[actintItemEvents["ACI_UNLOCK_INTERFACE"] = 11] = "ACI_UNLOCK_INTERFACE";
    actintItemEvents[actintItemEvents["ACI_SHOW_TEXT"] = 12] = "ACI_SHOW_TEXT";
    actintItemEvents[actintItemEvents["ACI_HIDE_TEXT"] = 13] = "ACI_HIDE_TEXT";
    actintItemEvents[actintItemEvents["ACI_SHOW_ITEM_TEXT"] = 14] = "ACI_SHOW_ITEM_TEXT";
    actintItemEvents[actintItemEvents["ACI_DROP_CONFIRM"] = 15] = "ACI_DROP_CONFIRM";
    actintItemEvents[actintItemEvents["ACI_MAX_EVENT"] = 16] = "ACI_MAX_EVENT";
})(actintItemEvents = exports.actintItemEvents || (exports.actintItemEvents = {}));
;
var actEventCodes;
(function (actEventCodes) {
    actEventCodes[actEventCodes["EV_CHANGE_MODE"] = 17] = "EV_CHANGE_MODE";
    actEventCodes[actEventCodes["EV_SET_MODE"] = 18] = "EV_SET_MODE";
    actEventCodes[actEventCodes["EV_ACTIVATE_MENU"] = 19] = "EV_ACTIVATE_MENU";
    actEventCodes[actEventCodes["EV_CHANGE_SCREEN"] = 20] = "EV_CHANGE_SCREEN";
    actEventCodes[actEventCodes["EV_FULLSCR_CHANGE"] = 21] = "EV_FULLSCR_CHANGE";
    actEventCodes[actEventCodes["EV_ACTIVATE_IINV"] = 22] = "EV_ACTIVATE_IINV";
    actEventCodes[actEventCodes["EV_DEACTIVATE_IINV"] = 23] = "EV_DEACTIVATE_IINV";
    actEventCodes[actEventCodes["EV_ACTIVATE_MATRIX"] = 24] = "EV_ACTIVATE_MATRIX";
    actEventCodes[actEventCodes["EV_DEACTIVATE_MATRIX"] = 25] = "EV_DEACTIVATE_MATRIX";
    actEventCodes[actEventCodes["EV_EVINCE_PALETTE"] = 26] = "EV_EVINCE_PALETTE";
    actEventCodes[actEventCodes["EV_INIT_MATRIX_OBJ"] = 27] = "EV_INIT_MATRIX_OBJ";
    actEventCodes[actEventCodes["EV_INIT_SC_MATRIX_OBJ"] = 28] = "EV_INIT_SC_MATRIX_OBJ";
    actEventCodes[actEventCodes["EV_REDRAW"] = 29] = "EV_REDRAW";
    actEventCodes[actEventCodes["EV_CANCEL_MATRIX"] = 30] = "EV_CANCEL_MATRIX";
    actEventCodes[actEventCodes["EV_AUTO_MOVE_ITEMS"] = 31] = "EV_AUTO_MOVE_ITEMS";
    actEventCodes[actEventCodes["EV_SET_MECH_NAME"] = 32] = "EV_SET_MECH_NAME";
    actEventCodes[actEventCodes["EV_NEXT_SHOP_AVI"] = 33] = "EV_NEXT_SHOP_AVI";
    actEventCodes[actEventCodes["EV_PREV_SHOP_AVI"] = 34] = "EV_PREV_SHOP_AVI";
    actEventCodes[actEventCodes["EV_CHANGE_AVI_LIST"] = 35] = "EV_CHANGE_AVI_LIST";
    actEventCodes[actEventCodes["EV_BUY_ITEM"] = 36] = "EV_BUY_ITEM";
    actEventCodes[actEventCodes["EV_SET_ITM_PICKUP"] = 37] = "EV_SET_ITM_PICKUP";
    actEventCodes[actEventCodes["EV_SET_WPN_PICKUP"] = 38] = "EV_SET_WPN_PICKUP";
    actEventCodes[actEventCodes["EV_ACTIVATE_SHOP_MENU"] = 39] = "EV_ACTIVATE_SHOP_MENU";
    actEventCodes[actEventCodes["EV_CHOOSE_SHOP_ITEM"] = 40] = "EV_CHOOSE_SHOP_ITEM";
    actEventCodes[actEventCodes["EV_NEXT_PHRASE"] = 41] = "EV_NEXT_PHRASE";
    actEventCodes[actEventCodes["EV_START_SPEECH"] = 42] = "EV_START_SPEECH";
    actEventCodes[actEventCodes["EV_SHOW_QUESTS"] = 43] = "EV_SHOW_QUESTS";
    actEventCodes[actEventCodes["EV_ASK_QUEST"] = 44] = "EV_ASK_QUEST";
    actEventCodes[actEventCodes["EV_TRY_2_ENTER"] = 45] = "EV_TRY_2_ENTER";
    actEventCodes[actEventCodes["EV_GET_CIRT"] = 46] = "EV_GET_CIRT";
    actEventCodes[actEventCodes["EV_TAKE_ELEECH"] = 47] = "EV_TAKE_ELEECH";
    actEventCodes[actEventCodes["EV_GET_ELEECH"] = 48] = "EV_GET_ELEECH";
    actEventCodes[actEventCodes["EV_ISCR_KEYTRAP"] = 49] = "EV_ISCR_KEYTRAP";
    actEventCodes[actEventCodes["EV_LOCK_ISCREEN"] = 50] = "EV_LOCK_ISCREEN";
    actEventCodes[actEventCodes["EV_UNLOCK_ISCREEN"] = 51] = "EV_UNLOCK_ISCREEN";
    actEventCodes[actEventCodes["EV_SELL_MOVE_ITEM"] = 52] = "EV_SELL_MOVE_ITEM";
    actEventCodes[actEventCodes["EV_CHANGE_AVI_INDEX"] = 53] = "EV_CHANGE_AVI_INDEX";
    actEventCodes[actEventCodes["EV_TELEPORT"] = 54] = "EV_TELEPORT";
    actEventCodes[actEventCodes["EV_INIT_BUTTONS"] = 55] = "EV_INIT_BUTTONS";
    actEventCodes[actEventCodes["EV_ENTER_TEXT_MODE"] = 56] = "EV_ENTER_TEXT_MODE";
    actEventCodes[actEventCodes["EV_LEAVE_TEXT_MODE"] = 57] = "EV_LEAVE_TEXT_MODE";
    actEventCodes[actEventCodes["EV_PROTRACTOR_EVENT"] = 58] = "EV_PROTRACTOR_EVENT";
    actEventCodes[actEventCodes["EV_MECH_MESSIAH_EVENT"] = 59] = "EV_MECH_MESSIAH_EVENT";
    actEventCodes[actEventCodes["EV_GET_BLOCK_PHRASE"] = 60] = "EV_GET_BLOCK_PHRASE";
    actEventCodes[actEventCodes["EV_PAUSE_AML"] = 61] = "EV_PAUSE_AML";
    actEventCodes[actEventCodes["EV_RESUME_AML"] = 62] = "EV_RESUME_AML";
    actEventCodes[actEventCodes["EV_ENTER_CHAT"] = 63] = "EV_ENTER_CHAT";
    actEventCodes[actEventCodes["EV_LEAVE_CHAT"] = 64] = "EV_LEAVE_CHAT";
    actEventCodes[actEventCodes["EV_ITEM_TEXT"] = 65] = "EV_ITEM_TEXT";
    actEventCodes[actEventCodes["EV_GET_RUBOX"] = 66] = "EV_GET_RUBOX";
    actEventCodes[actEventCodes["EV_INIT_AVI_OBJECT"] = 67] = "EV_INIT_AVI_OBJECT";
    actEventCodes[actEventCodes["EV_MAX_CODE"] = 68] = "EV_MAX_CODE";
})(actEventCodes = exports.actEventCodes || (exports.actEventCodes = {}));
;
// actInt modes
var ASMode;
(function (ASMode) {
    ASMode[ASMode["AS_INV_MODE"] = 0] = "AS_INV_MODE";
    ASMode[ASMode["AS_INFO_MODE"] = 1] = "AS_INFO_MODE";
})(ASMode = exports.ASMode || (exports.ASMode = {}));
// actInt flags
var ASFlag;
(function (ASFlag) {
    ASFlag[ASFlag["AS_FULL_REDRAW"] = 1] = "AS_FULL_REDRAW";
    ASFlag[ASFlag["aMS_LEFT_PRESS"] = 2] = "aMS_LEFT_PRESS";
    ASFlag[ASFlag["aMS_RIGHT_PRESS"] = 4] = "aMS_RIGHT_PRESS";
    ASFlag[ASFlag["aMS_MOVED"] = 8] = "aMS_MOVED";
    ASFlag[ASFlag["aMS_PRESS"] = 14] = "aMS_PRESS";
    ASFlag[ASFlag["AS_CHANGE_MODE"] = 16] = "AS_CHANGE_MODE";
    ASFlag[ASFlag["AS_INV_MOVE_ITEM"] = 32] = "AS_INV_MOVE_ITEM";
    ASFlag[ASFlag["AS_INV_SET_DROP"] = 64] = "AS_INV_SET_DROP";
    ASFlag[ASFlag["AS_FULL_FLUSH"] = 128] = "AS_FULL_FLUSH";
    ASFlag[ASFlag["AS_FULLSCR"] = 256] = "AS_FULLSCR";
    ASFlag[ASFlag["AS_ISCREEN"] = 512] = "AS_ISCREEN";
    ASFlag[ASFlag["AS_ISCREEN_INV_MODE"] = 1024] = "AS_ISCREEN_INV_MODE";
    ASFlag[ASFlag["AS_EVINCE_PALETTE"] = 2048] = "AS_EVINCE_PALETTE";
    ASFlag[ASFlag["AS_LOCKED"] = 4096] = "AS_LOCKED";
    ASFlag[ASFlag["AS_TEXT_MODE"] = 8192] = "AS_TEXT_MODE";
    ASFlag[ASFlag["AS_CHAT_MODE"] = 16384] = "AS_CHAT_MODE";
    ASFlag[ASFlag["AS_WORLDS_INIT"] = 32768] = "AS_WORLDS_INIT";
})(ASFlag = exports.ASFlag || (exports.ASFlag = {}));
var FileOpenFlags;
(function (FileOpenFlags) {
    FileOpenFlags[FileOpenFlags["XS_IN"] = 1] = "XS_IN";
    FileOpenFlags[FileOpenFlags["XS_OUT"] = 2] = "XS_OUT";
    FileOpenFlags[FileOpenFlags["XS_NOREPLACE"] = 4] = "XS_NOREPLACE";
    FileOpenFlags[FileOpenFlags["XS_APPEND"] = 8] = "XS_APPEND";
    FileOpenFlags[FileOpenFlags["XS_NOBUFFERING"] = 16] = "XS_NOBUFFERING";
    FileOpenFlags[FileOpenFlags["XS_NOSHARING"] = 32] = "XS_NOSHARING";
    FileOpenFlags[FileOpenFlags["XS_SHAREREAD"] = 64] = "XS_SHAREREAD";
    FileOpenFlags[FileOpenFlags["XS_SHAREWRITE"] = 128] = "XS_SHAREWRITE";
})(FileOpenFlags = exports.FileOpenFlags || (exports.FileOpenFlags = {}));
;
