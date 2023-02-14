"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.init = void 0;
var vss_1 = __importStar(require("./vss"));
function init() {
    var consumeNextDisableFullscreen = false;
    vss_1.default.addQuantListener("set_road_fullscreen", function (payload) {
        if (payload.enabled) {
            return;
        }
        if (consumeNextDisableFullscreen) {
            consumeNextDisableFullscreen = false;
            return {
                enabled: true,
            };
        }
    });
    vss_1.default.addQuantListener("send_event", function (payload) {
        if (payload.code === vss_1.actintItemEvents.ACI_UNLOCK_INTERFACE) {
            consumeNextDisableFullscreen = true;
        }
        if (payload.code === vss_1.actEventCodes.EV_CHANGE_MODE && payload.asMode === vss_1.ASMode.AS_INV_MODE) {
            vss_1.default.sendEvent(vss_1.actEventCodes.EV_FULLSCR_CHANGE);
        }
    });
    vss_1.default.addQuantListener("runtime_object", function (payload) {
        if (payload.runtimeObjectId === vss_1.RoadRuntimeObjId.RTO_GAME_QUANT_ID) {
            vss_1.default.sendEvent(vss_1.actEventCodes.EV_FULLSCR_CHANGE);
        }
    });
}
exports.init = init;
