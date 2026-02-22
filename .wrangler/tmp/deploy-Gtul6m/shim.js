var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// build/index.js
import { WorkerEntrypoint as ct } from "cloudflare:workers";
import N from "./2bcb701ddcea8a4d05b653777def17a63fb1532d-index_bg.wasm";
var m = class {
  static {
    __name(this, "m");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, Q.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    i.__wbg_containerstartupoptions_free(t, 0);
  }
  get enableInternet() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let t = i.__wbg_get_containerstartupoptions_enableInternet(this.__wbg_ptr);
    return t === 16777215 ? void 0 : t !== 0;
  }
  get entrypoint() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let t = i.__wbg_get_containerstartupoptions_entrypoint(this.__wbg_ptr);
    var e = nt(t[0], t[1]).slice();
    return i.__wbindgen_free(t[0], t[1] * 4, 4), e;
  }
  get env() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    return i.__wbg_get_containerstartupoptions_env(this.__wbg_ptr);
  }
  set enableInternet(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_containerstartupoptions_enableInternet(this.__wbg_ptr, a(t) ? 16777215 : t ? 1 : 0);
  }
  set entrypoint(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let e = _t(t, i.__wbindgen_malloc), r = w;
    i.__wbg_set_containerstartupoptions_entrypoint(this.__wbg_ptr, e, r);
  }
  set env(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_containerstartupoptions_env(this.__wbg_ptr, t);
  }
};
Symbol.dispose && (m.prototype[Symbol.dispose] = m.prototype.free);
var v = class {
  static {
    __name(this, "v");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, X.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    i.__wbg_intounderlyingbytesource_free(t, 0);
  }
  get autoAllocateChunkSize() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    return i.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  cancel() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let t = this.__destroy_into_raw();
    i.intounderlyingbytesource_cancel(t);
  }
  pull(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    return i.intounderlyingbytesource_pull(this.__wbg_ptr, t);
  }
  start(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    i.intounderlyingbytesource_start(this.__wbg_ptr, t);
  }
  get type() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let t = i.intounderlyingbytesource_type(this.__wbg_ptr);
    return K[t];
  }
};
Symbol.dispose && (v.prototype[Symbol.dispose] = v.prototype.free);
var x = class {
  static {
    __name(this, "x");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, Y.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    i.__wbg_intounderlyingsink_free(t, 0);
  }
  abort(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let e = this.__destroy_into_raw();
    return i.intounderlyingsink_abort(e, t);
  }
  close() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let t = this.__destroy_into_raw();
    return i.intounderlyingsink_close(t);
  }
  write(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    return i.intounderlyingsink_write(this.__wbg_ptr, t);
  }
};
Symbol.dispose && (x.prototype[Symbol.dispose] = x.prototype.free);
var I = class {
  static {
    __name(this, "I");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, Z.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    i.__wbg_intounderlyingsource_free(t, 0);
  }
  cancel() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let t = this.__destroy_into_raw();
    i.intounderlyingsource_cancel(t);
  }
  pull(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    return i.intounderlyingsource_pull(this.__wbg_ptr, t);
  }
};
Symbol.dispose && (I.prototype[Symbol.dispose] = I.prototype.free);
var E = class {
  static {
    __name(this, "E");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, tt.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    i.__wbg_minifyconfig_free(t, 0);
  }
  get css() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    return i.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  get html() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    return i.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  get js() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    return i.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set css(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_minifyconfig_css(this.__wbg_ptr, t);
  }
  set html(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_minifyconfig_html(this.__wbg_ptr, t);
  }
  set js(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_minifyconfig_js(this.__wbg_ptr, t);
  }
};
Symbol.dispose && (E.prototype[Symbol.dispose] = E.prototype.free);
var R = class {
  static {
    __name(this, "R");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, et.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    i.__wbg_r2range_free(t, 0);
  }
  get length() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let t = i.__wbg_get_r2range_length(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  get offset() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let t = i.__wbg_get_r2range_offset(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  get suffix() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    let t = i.__wbg_get_r2range_suffix(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  set length(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_r2range_length(this.__wbg_ptr, !a(t), a(t) ? 0 : t);
  }
  set offset(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_r2range_offset(this.__wbg_ptr, !a(t), a(t) ? 0 : t);
  }
  set suffix(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== o) throw new Error("Invalid stale object from previous Wasm instance");
    i.__wbg_set_r2range_suffix(this.__wbg_ptr, !a(t), a(t) ? 0 : t);
  }
};
Symbol.dispose && (R.prototype[Symbol.dispose] = R.prototype.free);
function B() {
  o++, p = null, j = null, typeof numBytesDecoded < "u" && (numBytesDecoded = 0), typeof w < "u" && (w = 0), i = new WebAssembly.Instance(N, V()).exports, i.__wbindgen_start();
}
__name(B, "B");
function D(n, t, e) {
  return i.fetch(n, t, e);
}
__name(D, "D");
function M(n) {
  i.setPanicHook(n);
}
__name(M, "M");
function V() {
  return { __proto__: null, "./index_bg.js": { __proto__: null, __wbg_Error_8c4e43fe74559d73: /* @__PURE__ */ __name(function(t, e) {
    return Error(d(t, e));
  }, "__wbg_Error_8c4e43fe74559d73"), __wbg_Number_04624de7d0e8332d: /* @__PURE__ */ __name(function(t) {
    return Number(t);
  }, "__wbg_Number_04624de7d0e8332d"), __wbg_String_8f0eb39a4a4c2f66: /* @__PURE__ */ __name(function(t, e) {
    let r = String(e), _ = h(r, i.__wbindgen_malloc, i.__wbindgen_realloc), s = w;
    b().setInt32(t + 4, s, true), b().setInt32(t + 0, _, true);
  }, "__wbg_String_8f0eb39a4a4c2f66"), __wbg___wbindgen_bigint_get_as_i64_8fcf4ce7f1ca72a2: /* @__PURE__ */ __name(function(t, e) {
    let r = e, _ = typeof r == "bigint" ? r : void 0;
    b().setBigInt64(t + 8, a(_) ? BigInt(0) : _, true), b().setInt32(t + 0, !a(_), true);
  }, "__wbg___wbindgen_bigint_get_as_i64_8fcf4ce7f1ca72a2"), __wbg___wbindgen_boolean_get_bbbb1c18aa2f5e25: /* @__PURE__ */ __name(function(t) {
    let e = t, r = typeof e == "boolean" ? e : void 0;
    return a(r) ? 16777215 : r ? 1 : 0;
  }, "__wbg___wbindgen_boolean_get_bbbb1c18aa2f5e25"), __wbg___wbindgen_debug_string_0bc8482c6e3508ae: /* @__PURE__ */ __name(function(t, e) {
    let r = T(e), _ = h(r, i.__wbindgen_malloc, i.__wbindgen_realloc), s = w;
    b().setInt32(t + 4, s, true), b().setInt32(t + 0, _, true);
  }, "__wbg___wbindgen_debug_string_0bc8482c6e3508ae"), __wbg___wbindgen_in_47fa6863be6f2f25: /* @__PURE__ */ __name(function(t, e) {
    return t in e;
  }, "__wbg___wbindgen_in_47fa6863be6f2f25"), __wbg___wbindgen_is_bigint_31b12575b56f32fc: /* @__PURE__ */ __name(function(t) {
    return typeof t == "bigint";
  }, "__wbg___wbindgen_is_bigint_31b12575b56f32fc"), __wbg___wbindgen_is_function_0095a73b8b156f76: /* @__PURE__ */ __name(function(t) {
    return typeof t == "function";
  }, "__wbg___wbindgen_is_function_0095a73b8b156f76"), __wbg___wbindgen_is_object_5ae8e5880f2c1fbd: /* @__PURE__ */ __name(function(t) {
    let e = t;
    return typeof e == "object" && e !== null;
  }, "__wbg___wbindgen_is_object_5ae8e5880f2c1fbd"), __wbg___wbindgen_is_string_cd444516edc5b180: /* @__PURE__ */ __name(function(t) {
    return typeof t == "string";
  }, "__wbg___wbindgen_is_string_cd444516edc5b180"), __wbg___wbindgen_is_undefined_9e4d92534c42d778: /* @__PURE__ */ __name(function(t) {
    return t === void 0;
  }, "__wbg___wbindgen_is_undefined_9e4d92534c42d778"), __wbg___wbindgen_jsval_eq_11888390b0186270: /* @__PURE__ */ __name(function(t, e) {
    return t === e;
  }, "__wbg___wbindgen_jsval_eq_11888390b0186270"), __wbg___wbindgen_jsval_loose_eq_9dd77d8cd6671811: /* @__PURE__ */ __name(function(t, e) {
    return t == e;
  }, "__wbg___wbindgen_jsval_loose_eq_9dd77d8cd6671811"), __wbg___wbindgen_number_get_8ff4255516ccad3e: /* @__PURE__ */ __name(function(t, e) {
    let r = e, _ = typeof r == "number" ? r : void 0;
    b().setFloat64(t + 8, a(_) ? 0 : _, true), b().setInt32(t + 0, !a(_), true);
  }, "__wbg___wbindgen_number_get_8ff4255516ccad3e"), __wbg___wbindgen_string_get_72fb696202c56729: /* @__PURE__ */ __name(function(t, e) {
    let r = e, _ = typeof r == "string" ? r : void 0;
    var s = a(_) ? 0 : h(_, i.__wbindgen_malloc, i.__wbindgen_realloc), c = w;
    b().setInt32(t + 4, c, true), b().setInt32(t + 0, s, true);
  }, "__wbg___wbindgen_string_get_72fb696202c56729"), __wbg___wbindgen_throw_be289d5034ed271b: /* @__PURE__ */ __name(function(t, e) {
    throw new Error(d(t, e));
  }, "__wbg___wbindgen_throw_be289d5034ed271b"), __wbg__wbg_cb_unref_d9b87ff7982e3b21: /* @__PURE__ */ __name(function(t) {
    t._wbg_cb_unref();
  }, "__wbg__wbg_cb_unref_d9b87ff7982e3b21"), __wbg_buffer_26d0910f3a5bc899: /* @__PURE__ */ __name(function(t) {
    return t.buffer;
  }, "__wbg_buffer_26d0910f3a5bc899"), __wbg_byobRequest_80e594e6da4e1af7: /* @__PURE__ */ __name(function(t) {
    let e = t.byobRequest;
    return a(e) ? 0 : l(e);
  }, "__wbg_byobRequest_80e594e6da4e1af7"), __wbg_byteLength_3417f266f4bf562a: /* @__PURE__ */ __name(function(t) {
    return t.byteLength;
  }, "__wbg_byteLength_3417f266f4bf562a"), __wbg_byteOffset_f88547ca47c86358: /* @__PURE__ */ __name(function(t) {
    return t.byteOffset;
  }, "__wbg_byteOffset_f88547ca47c86358"), __wbg_call_389efe28435a9388: /* @__PURE__ */ __name(function() {
    return f(function(t, e) {
      return t.call(e);
    }, arguments);
  }, "__wbg_call_389efe28435a9388"), __wbg_call_4708e0c13bdc8e95: /* @__PURE__ */ __name(function() {
    return f(function(t, e, r) {
      return t.call(e, r);
    }, arguments);
  }, "__wbg_call_4708e0c13bdc8e95"), __wbg_cause_0fc168d4eaec87cc: /* @__PURE__ */ __name(function(t) {
    return t.cause;
  }, "__wbg_cause_0fc168d4eaec87cc"), __wbg_cf_b8165e79377eeebd: /* @__PURE__ */ __name(function() {
    return f(function(t) {
      let e = t.cf;
      return a(e) ? 0 : l(e);
    }, arguments);
  }, "__wbg_cf_b8165e79377eeebd"), __wbg_close_06dfa0a815b9d71f: /* @__PURE__ */ __name(function() {
    return f(function(t) {
      t.close();
    }, arguments);
  }, "__wbg_close_06dfa0a815b9d71f"), __wbg_close_a79afee31de55b36: /* @__PURE__ */ __name(function() {
    return f(function(t) {
      t.close();
    }, arguments);
  }, "__wbg_close_a79afee31de55b36"), __wbg_crypto_86f2631e91b51511: /* @__PURE__ */ __name(function(t) {
    return t.crypto;
  }, "__wbg_crypto_86f2631e91b51511"), __wbg_done_57b39ecd9addfe81: /* @__PURE__ */ __name(function(t) {
    return t.done;
  }, "__wbg_done_57b39ecd9addfe81"), __wbg_enqueue_2c63f2044f257c3e: /* @__PURE__ */ __name(function() {
    return f(function(t, e) {
      t.enqueue(e);
    }, arguments);
  }, "__wbg_enqueue_2c63f2044f257c3e"), __wbg_entries_04679661ea6e74fc: /* @__PURE__ */ __name(function(t) {
    return t.entries();
  }, "__wbg_entries_04679661ea6e74fc"), __wbg_error_9a7fe3f932034cde: /* @__PURE__ */ __name(function(t) {
    console.error(t);
  }, "__wbg_error_9a7fe3f932034cde"), __wbg_error_f852e41c69b0bd84: /* @__PURE__ */ __name(function(t, e) {
    console.error(t, e);
  }, "__wbg_error_f852e41c69b0bd84"), __wbg_getRandomValues_9c5c1b115e142bb8: /* @__PURE__ */ __name(function() {
    return f(function(t, e) {
      globalThis.crypto.getRandomValues(A(t, e));
    }, arguments);
  }, "__wbg_getRandomValues_9c5c1b115e142bb8"), __wbg_getRandomValues_b3f15fcbfabb0f8b: /* @__PURE__ */ __name(function() {
    return f(function(t, e) {
      t.getRandomValues(e);
    }, arguments);
  }, "__wbg_getRandomValues_b3f15fcbfabb0f8b"), __wbg_getTime_1e3cd1391c5c3995: /* @__PURE__ */ __name(function(t) {
    return t.getTime();
  }, "__wbg_getTime_1e3cd1391c5c3995"), __wbg_get_941633a1d2f510cb: /* @__PURE__ */ __name(function() {
    return f(function(t, e, r, _) {
      let s = e.get(d(r, _));
      var c = a(s) ? 0 : h(s, i.__wbindgen_malloc, i.__wbindgen_realloc), u = w;
      b().setInt32(t + 4, u, true), b().setInt32(t + 0, c, true);
    }, arguments);
  }, "__wbg_get_941633a1d2f510cb"), __wbg_get_9b94d73e6221f75c: /* @__PURE__ */ __name(function(t, e) {
    return t[e >>> 0];
  }, "__wbg_get_9b94d73e6221f75c"), __wbg_get_b3ed3ad4be2bc8ac: /* @__PURE__ */ __name(function() {
    return f(function(t, e) {
      return Reflect.get(t, e);
    }, arguments);
  }, "__wbg_get_b3ed3ad4be2bc8ac"), __wbg_get_with_ref_key_1dc361bd10053bfe: /* @__PURE__ */ __name(function(t, e) {
    return t[e];
  }, "__wbg_get_with_ref_key_1dc361bd10053bfe"), __wbg_headers_5a897f7fee9a0571: /* @__PURE__ */ __name(function(t) {
    return t.headers;
  }, "__wbg_headers_5a897f7fee9a0571"), __wbg_instanceof_ArrayBuffer_c367199e2fa2aa04: /* @__PURE__ */ __name(function(t) {
    let e;
    try {
      e = t instanceof ArrayBuffer;
    } catch {
      e = false;
    }
    return e;
  }, "__wbg_instanceof_ArrayBuffer_c367199e2fa2aa04"), __wbg_instanceof_Error_8573fe0b0b480f46: /* @__PURE__ */ __name(function(t) {
    let e;
    try {
      e = t instanceof Error;
    } catch {
      e = false;
    }
    return e;
  }, "__wbg_instanceof_Error_8573fe0b0b480f46"), __wbg_instanceof_Uint8Array_9b9075935c74707c: /* @__PURE__ */ __name(function(t) {
    let e;
    try {
      e = t instanceof Uint8Array;
    } catch {
      e = false;
    }
    return e;
  }, "__wbg_instanceof_Uint8Array_9b9075935c74707c"), __wbg_isArray_d314bb98fcf08331: /* @__PURE__ */ __name(function(t) {
    return Array.isArray(t);
  }, "__wbg_isArray_d314bb98fcf08331"), __wbg_isSafeInteger_bfbc7332a9768d2a: /* @__PURE__ */ __name(function(t) {
    return Number.isSafeInteger(t);
  }, "__wbg_isSafeInteger_bfbc7332a9768d2a"), __wbg_iterator_6ff6560ca1568e55: /* @__PURE__ */ __name(function() {
    return Symbol.iterator;
  }, "__wbg_iterator_6ff6560ca1568e55"), __wbg_json_086b635bd30e59b5: /* @__PURE__ */ __name(function() {
    return f(function(t) {
      return t.json();
    }, arguments);
  }, "__wbg_json_086b635bd30e59b5"), __wbg_length_32ed9a279acd054c: /* @__PURE__ */ __name(function(t) {
    return t.length;
  }, "__wbg_length_32ed9a279acd054c"), __wbg_length_35a7bace40f36eac: /* @__PURE__ */ __name(function(t) {
    return t.length;
  }, "__wbg_length_35a7bace40f36eac"), __wbg_method_a9e9b2fcba5440fb: /* @__PURE__ */ __name(function(t, e) {
    let r = e.method, _ = h(r, i.__wbindgen_malloc, i.__wbindgen_realloc), s = w;
    b().setInt32(t + 4, s, true), b().setInt32(t + 0, _, true);
  }, "__wbg_method_a9e9b2fcba5440fb"), __wbg_msCrypto_d562bbe83e0d4b91: /* @__PURE__ */ __name(function(t) {
    return t.msCrypto;
  }, "__wbg_msCrypto_d562bbe83e0d4b91"), __wbg_new_0_73afc35eb544e539: /* @__PURE__ */ __name(function() {
    return /* @__PURE__ */ new Date();
  }, "__wbg_new_0_73afc35eb544e539"), __wbg_new_361308b2356cecd0: /* @__PURE__ */ __name(function() {
    return new Object();
  }, "__wbg_new_361308b2356cecd0"), __wbg_new_64284bd487f9d239: /* @__PURE__ */ __name(function() {
    return f(function() {
      return new Headers();
    }, arguments);
  }, "__wbg_new_64284bd487f9d239"), __wbg_new_72b49615380db768: /* @__PURE__ */ __name(function(t, e) {
    return new Error(d(t, e));
  }, "__wbg_new_72b49615380db768"), __wbg_new_b5d9e2fb389fef91: /* @__PURE__ */ __name(function(t, e) {
    try {
      var r = { a: t, b: e }, _ = /* @__PURE__ */ __name((c, u) => {
        let g = r.a;
        r.a = 0;
        try {
          return G(g, r.b, c, u);
        } finally {
          r.a = g;
        }
      }, "_");
      return new Promise(_);
    } finally {
      r.a = r.b = 0;
    }
  }, "__wbg_new_b5d9e2fb389fef91"), __wbg_new_dd2b680c8bf6ae29: /* @__PURE__ */ __name(function(t) {
    return new Uint8Array(t);
  }, "__wbg_new_dd2b680c8bf6ae29"), __wbg_new_no_args_1c7c842f08d00ebb: /* @__PURE__ */ __name(function(t, e) {
    return new Function(d(t, e));
  }, "__wbg_new_no_args_1c7c842f08d00ebb"), __wbg_new_with_byte_offset_and_length_aa261d9c9da49eb1: /* @__PURE__ */ __name(function(t, e, r) {
    return new Uint8Array(t, e >>> 0, r >>> 0);
  }, "__wbg_new_with_byte_offset_and_length_aa261d9c9da49eb1"), __wbg_new_with_length_a2c39cbe88fd8ff1: /* @__PURE__ */ __name(function(t) {
    return new Uint8Array(t >>> 0);
  }, "__wbg_new_with_length_a2c39cbe88fd8ff1"), __wbg_new_with_opt_buffer_source_and_init_8c10f2615c78809b: /* @__PURE__ */ __name(function() {
    return f(function(t, e) {
      return new Response(t, e);
    }, arguments);
  }, "__wbg_new_with_opt_buffer_source_and_init_8c10f2615c78809b"), __wbg_new_with_opt_readable_stream_and_init_8a044befefe6d8bb: /* @__PURE__ */ __name(function() {
    return f(function(t, e) {
      return new Response(t, e);
    }, arguments);
  }, "__wbg_new_with_opt_readable_stream_and_init_8a044befefe6d8bb"), __wbg_new_with_opt_str_and_init_4fbb71523b271b6e: /* @__PURE__ */ __name(function() {
    return f(function(t, e, r) {
      return new Response(t === 0 ? void 0 : d(t, e), r);
    }, arguments);
  }, "__wbg_new_with_opt_str_and_init_4fbb71523b271b6e"), __wbg_next_3482f54c49e8af19: /* @__PURE__ */ __name(function() {
    return f(function(t) {
      return t.next();
    }, arguments);
  }, "__wbg_next_3482f54c49e8af19"), __wbg_next_418f80d8f5303233: /* @__PURE__ */ __name(function(t) {
    return t.next;
  }, "__wbg_next_418f80d8f5303233"), __wbg_node_e1f24f89a7336c2e: /* @__PURE__ */ __name(function(t) {
    return t.node;
  }, "__wbg_node_e1f24f89a7336c2e"), __wbg_process_3975fd6c72f520aa: /* @__PURE__ */ __name(function(t) {
    return t.process;
  }, "__wbg_process_3975fd6c72f520aa"), __wbg_prototypesetcall_bdcdcc5842e4d77d: /* @__PURE__ */ __name(function(t, e, r) {
    Uint8Array.prototype.set.call(A(t, e), r);
  }, "__wbg_prototypesetcall_bdcdcc5842e4d77d"), __wbg_queueMicrotask_0aa0a927f78f5d98: /* @__PURE__ */ __name(function(t) {
    return t.queueMicrotask;
  }, "__wbg_queueMicrotask_0aa0a927f78f5d98"), __wbg_queueMicrotask_5bb536982f78a56f: /* @__PURE__ */ __name(function(t) {
    queueMicrotask(t);
  }, "__wbg_queueMicrotask_5bb536982f78a56f"), __wbg_randomFillSync_f8c153b79f285817: /* @__PURE__ */ __name(function() {
    return f(function(t, e) {
      t.randomFillSync(e);
    }, arguments);
  }, "__wbg_randomFillSync_f8c153b79f285817"), __wbg_require_b74f47fc2d022fd6: /* @__PURE__ */ __name(function() {
    return f(function() {
      return module.require;
    }, arguments);
  }, "__wbg_require_b74f47fc2d022fd6"), __wbg_resolve_002c4b7d9d8f6b64: /* @__PURE__ */ __name(function(t) {
    return Promise.resolve(t);
  }, "__wbg_resolve_002c4b7d9d8f6b64"), __wbg_respond_bf6ab10399ca8722: /* @__PURE__ */ __name(function() {
    return f(function(t, e) {
      t.respond(e >>> 0);
    }, arguments);
  }, "__wbg_respond_bf6ab10399ca8722"), __wbg_set_6cb8631f80447a67: /* @__PURE__ */ __name(function() {
    return f(function(t, e, r) {
      return Reflect.set(t, e, r);
    }, arguments);
  }, "__wbg_set_6cb8631f80447a67"), __wbg_set_cc56eefd2dd91957: /* @__PURE__ */ __name(function(t, e, r) {
    t.set(A(e, r));
  }, "__wbg_set_cc56eefd2dd91957"), __wbg_set_db769d02949a271d: /* @__PURE__ */ __name(function() {
    return f(function(t, e, r, _, s) {
      t.set(d(e, r), d(_, s));
    }, arguments);
  }, "__wbg_set_db769d02949a271d"), __wbg_set_headers_bbdfebba19309590: /* @__PURE__ */ __name(function(t, e) {
    t.headers = e;
  }, "__wbg_set_headers_bbdfebba19309590"), __wbg_set_status_fa41f71c4575bca5: /* @__PURE__ */ __name(function(t, e) {
    t.status = e;
  }, "__wbg_set_status_fa41f71c4575bca5"), __wbg_static_accessor_GLOBAL_12837167ad935116: /* @__PURE__ */ __name(function() {
    let t = typeof global > "u" ? null : global;
    return a(t) ? 0 : l(t);
  }, "__wbg_static_accessor_GLOBAL_12837167ad935116"), __wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f: /* @__PURE__ */ __name(function() {
    let t = typeof globalThis > "u" ? null : globalThis;
    return a(t) ? 0 : l(t);
  }, "__wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f"), __wbg_static_accessor_SELF_a621d3dfbb60d0ce: /* @__PURE__ */ __name(function() {
    let t = typeof self > "u" ? null : self;
    return a(t) ? 0 : l(t);
  }, "__wbg_static_accessor_SELF_a621d3dfbb60d0ce"), __wbg_static_accessor_WINDOW_f8727f0cf888e0bd: /* @__PURE__ */ __name(function() {
    let t = typeof window > "u" ? null : window;
    return a(t) ? 0 : l(t);
  }, "__wbg_static_accessor_WINDOW_f8727f0cf888e0bd"), __wbg_subarray_a96e1fef17ed23cb: /* @__PURE__ */ __name(function(t, e, r) {
    return t.subarray(e >>> 0, r >>> 0);
  }, "__wbg_subarray_a96e1fef17ed23cb"), __wbg_then_0d9fe2c7b1857d32: /* @__PURE__ */ __name(function(t, e, r) {
    return t.then(e, r);
  }, "__wbg_then_0d9fe2c7b1857d32"), __wbg_then_b9e7b3b5f1a9e1b5: /* @__PURE__ */ __name(function(t, e) {
    return t.then(e);
  }, "__wbg_then_b9e7b3b5f1a9e1b5"), __wbg_toString_029ac24421fd7a24: /* @__PURE__ */ __name(function(t) {
    return t.toString();
  }, "__wbg_toString_029ac24421fd7a24"), __wbg_url_36c39f6580d05409: /* @__PURE__ */ __name(function(t, e) {
    let r = e.url, _ = h(r, i.__wbindgen_malloc, i.__wbindgen_realloc), s = w;
    b().setInt32(t + 4, s, true), b().setInt32(t + 0, _, true);
  }, "__wbg_url_36c39f6580d05409"), __wbg_value_0546255b415e96c1: /* @__PURE__ */ __name(function(t) {
    return t.value;
  }, "__wbg_value_0546255b415e96c1"), __wbg_versions_4e31226f5e8dc909: /* @__PURE__ */ __name(function(t) {
    return t.versions;
  }, "__wbg_versions_4e31226f5e8dc909"), __wbg_view_6c32e7184b8606ad: /* @__PURE__ */ __name(function(t) {
    let e = t.view;
    return a(e) ? 0 : l(e);
  }, "__wbg_view_6c32e7184b8606ad"), __wbindgen_cast_0000000000000001: /* @__PURE__ */ __name(function(t, e) {
    return rt(t, e, i.wasm_bindgen__closure__destroy__hb97640ea62ec9bdb, J);
  }, "__wbindgen_cast_0000000000000001"), __wbindgen_cast_0000000000000002: /* @__PURE__ */ __name(function(t, e) {
    return A(t, e);
  }, "__wbindgen_cast_0000000000000002"), __wbindgen_cast_0000000000000003: /* @__PURE__ */ __name(function(t, e) {
    return d(t, e);
  }, "__wbindgen_cast_0000000000000003"), __wbindgen_cast_0000000000000004: /* @__PURE__ */ __name(function(t) {
    return BigInt.asUintN(64, t);
  }, "__wbindgen_cast_0000000000000004"), __wbindgen_init_externref_table: /* @__PURE__ */ __name(function() {
    let t = i.__wbindgen_externrefs, e = t.grow(4);
    t.set(0, void 0), t.set(e + 0, void 0), t.set(e + 1, null), t.set(e + 2, true), t.set(e + 3, false);
  }, "__wbindgen_init_externref_table") } };
}
__name(V, "V");
function J(n, t, e) {
  i.wasm_bindgen__convert__closures_____invoke__h224bf158d3e59c76(n, t, e);
}
__name(J, "J");
function G(n, t, e, r) {
  i.wasm_bindgen__convert__closures_____invoke__h4ef83b79f6408a62(n, t, e, r);
}
__name(G, "G");
var K = ["bytes"];
var o = 0;
var Q = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: t }) => {
  t === o && i.__wbg_containerstartupoptions_free(n >>> 0, 1);
});
var X = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: t }) => {
  t === o && i.__wbg_intounderlyingbytesource_free(n >>> 0, 1);
});
var Y = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: t }) => {
  t === o && i.__wbg_intounderlyingsink_free(n >>> 0, 1);
});
var Z = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: t }) => {
  t === o && i.__wbg_intounderlyingsource_free(n >>> 0, 1);
});
var tt = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: t }) => {
  t === o && i.__wbg_minifyconfig_free(n >>> 0, 1);
});
var et = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: t }) => {
  t === o && i.__wbg_r2range_free(n >>> 0, 1);
});
function l(n) {
  let t = i.__externref_table_alloc();
  return i.__wbindgen_externrefs.set(t, n), t;
}
__name(l, "l");
var q = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((n) => {
  n.instance === o && n.dtor(n.a, n.b);
});
function T(n) {
  let t = typeof n;
  if (t == "number" || t == "boolean" || n == null) return `${n}`;
  if (t == "string") return `"${n}"`;
  if (t == "symbol") {
    let _ = n.description;
    return _ == null ? "Symbol" : `Symbol(${_})`;
  }
  if (t == "function") {
    let _ = n.name;
    return typeof _ == "string" && _.length > 0 ? `Function(${_})` : "Function";
  }
  if (Array.isArray(n)) {
    let _ = n.length, s = "[";
    _ > 0 && (s += T(n[0]));
    for (let c = 1; c < _; c++) s += ", " + T(n[c]);
    return s += "]", s;
  }
  let e = /\[object ([^\]]+)\]/.exec(toString.call(n)), r;
  if (e && e.length > 1) r = e[1];
  else return toString.call(n);
  if (r == "Object") try {
    return "Object(" + JSON.stringify(n) + ")";
  } catch {
    return "Object";
  }
  return n instanceof Error ? `${n.name}: ${n.message}
${n.stack}` : r;
}
__name(T, "T");
function nt(n, t) {
  n = n >>> 0;
  let e = b(), r = [];
  for (let _ = n; _ < n + 4 * t; _ += 4) r.push(i.__wbindgen_externrefs.get(e.getUint32(_, true)));
  return i.__externref_drop_slice(n, t), r;
}
__name(nt, "nt");
function A(n, t) {
  return n = n >>> 0, S().subarray(n / 1, n / 1 + t);
}
__name(A, "A");
var p = null;
function b() {
  return (p === null || p.buffer.detached === true || p.buffer.detached === void 0 && p.buffer !== i.memory.buffer) && (p = new DataView(i.memory.buffer)), p;
}
__name(b, "b");
function d(n, t) {
  return n = n >>> 0, it(n, t);
}
__name(d, "d");
var j = null;
function S() {
  return (j === null || j.byteLength === 0) && (j = new Uint8Array(i.memory.buffer)), j;
}
__name(S, "S");
function f(n, t) {
  try {
    return n.apply(this, t);
  } catch (e) {
    let r = l(e);
    i.__wbindgen_exn_store(r);
  }
}
__name(f, "f");
function a(n) {
  return n == null;
}
__name(a, "a");
function rt(n, t, e, r) {
  let _ = { a: n, b: t, cnt: 1, dtor: e, instance: o }, s = /* @__PURE__ */ __name((...c) => {
    if (_.instance !== o) throw new Error("Cannot invoke closure from previous WASM instance");
    _.cnt++;
    let u = _.a;
    _.a = 0;
    try {
      return r(u, _.b, ...c);
    } finally {
      _.a = u, s._wbg_cb_unref();
    }
  }, "s");
  return s._wbg_cb_unref = () => {
    --_.cnt === 0 && (_.dtor(_.a, _.b), _.a = 0, q.unregister(_));
  }, q.register(s, _, _), s;
}
__name(rt, "rt");
function _t(n, t) {
  let e = t(n.length * 4, 4) >>> 0;
  for (let r = 0; r < n.length; r++) {
    let _ = l(n[r]);
    b().setUint32(e + 4 * r, _, true);
  }
  return w = n.length, e;
}
__name(_t, "_t");
function h(n, t, e) {
  if (e === void 0) {
    let u = W.encode(n), g = t(u.length, 1) >>> 0;
    return S().subarray(g, g + u.length).set(u), w = u.length, g;
  }
  let r = n.length, _ = t(r, 1) >>> 0, s = S(), c = 0;
  for (; c < r; c++) {
    let u = n.charCodeAt(c);
    if (u > 127) break;
    s[_ + c] = u;
  }
  if (c !== r) {
    c !== 0 && (n = n.slice(c)), _ = e(_, r, r = c + n.length * 3, 1) >>> 0;
    let u = S().subarray(_ + c, _ + r), g = W.encodeInto(n, u);
    c += g.written, _ = e(_, r, c, 1) >>> 0;
  }
  return w = c, _;
}
__name(h, "h");
var H = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
H.decode();
function it(n, t) {
  return H.decode(S().subarray(n, n + t));
}
__name(it, "it");
var W = new TextEncoder();
"encodeInto" in W || (W.encodeInto = function(n, t) {
  let e = W.encode(n);
  return t.set(e), { read: n.length, written: e.length };
});
var w = 0;
var ot = new WebAssembly.Instance(N, V());
var i = ot.exports;
i.__wbindgen_start();
Error.stackTraceLimit = 100;
var k = false;
function $() {
  M && M(function(n) {
    let t = new Error("Rust panic: " + n);
    console.error("Critical", t), k = true;
  });
}
__name($, "$");
$();
var P = 0;
function C() {
  k && (console.log("Reinitializing Wasm application"), B(), k = false, $(), P++);
}
__name(C, "C");
addEventListener("error", (n) => {
  L(n.error);
});
function L(n) {
  n instanceof WebAssembly.RuntimeError && (console.error("Critical", n), k = true);
}
__name(L, "L");
var z = class extends ct {
  static {
    __name(this, "z");
  }
};
z.prototype.fetch = function(t) {
  return D.call(this, t, this.env, this.ctx);
};
var ut = { set: /* @__PURE__ */ __name((n, t, e, r) => Reflect.set(n.instance, t, e, r), "set"), has: /* @__PURE__ */ __name((n, t) => Reflect.has(n.instance, t), "has"), deleteProperty: /* @__PURE__ */ __name((n, t) => Reflect.deleteProperty(n.instance, t), "deleteProperty"), apply: /* @__PURE__ */ __name((n, t, e) => Reflect.apply(n.instance, t, e), "apply"), construct: /* @__PURE__ */ __name((n, t, e) => Reflect.construct(n.instance, t, e), "construct"), getPrototypeOf: /* @__PURE__ */ __name((n) => Reflect.getPrototypeOf(n.instance), "getPrototypeOf"), setPrototypeOf: /* @__PURE__ */ __name((n, t) => Reflect.setPrototypeOf(n.instance, t), "setPrototypeOf"), isExtensible: /* @__PURE__ */ __name((n) => Reflect.isExtensible(n.instance), "isExtensible"), preventExtensions: /* @__PURE__ */ __name((n) => Reflect.preventExtensions(n.instance), "preventExtensions"), getOwnPropertyDescriptor: /* @__PURE__ */ __name((n, t) => Reflect.getOwnPropertyDescriptor(n.instance, t), "getOwnPropertyDescriptor"), defineProperty: /* @__PURE__ */ __name((n, t, e) => Reflect.defineProperty(n.instance, t, e), "defineProperty"), ownKeys: /* @__PURE__ */ __name((n) => Reflect.ownKeys(n.instance), "ownKeys") };
var y = { construct(n, t, e) {
  try {
    C();
    let r = { instance: Reflect.construct(n, t, e), instanceId: P, ctor: n, args: t, newTarget: e };
    return new Proxy(r, { ...ut, get(_, s, c) {
      _.instanceId !== P && (_.instance = Reflect.construct(_.ctor, _.args, _.newTarget), _.instanceId = P);
      let u = Reflect.get(_.instance, s, c);
      return typeof u != "function" ? u : u.constructor === Function ? new Proxy(u, { apply(g, O, U) {
        C();
        try {
          return g.apply(O, U);
        } catch (F) {
          throw L(F), F;
        }
      } }) : new Proxy(u, { async apply(g, O, U) {
        C();
        try {
          return await g.apply(O, U);
        } catch (F) {
          throw L(F), F;
        }
      } });
    } });
  } catch (r) {
    throw k = true, r;
  }
} };
var bt = new Proxy(z, y);
var gt = new Proxy(m, y);
var wt = new Proxy(v, y);
var dt = new Proxy(x, y);
var lt = new Proxy(I, y);
var pt = new Proxy(E, y);
var yt = new Proxy(R, y);
export {
  gt as ContainerStartupOptions,
  wt as IntoUnderlyingByteSource,
  dt as IntoUnderlyingSink,
  lt as IntoUnderlyingSource,
  pt as MinifyConfig,
  yt as R2Range,
  bt as default
};
//# sourceMappingURL=shim.js.map
