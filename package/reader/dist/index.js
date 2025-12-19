import { readFile as $e, readdir as xe } from "node:fs/promises";
import { join as x, extname as ye } from "node:path";
function Be(a) {
  let e = typeof a;
  if (e == "object") {
    if (Array.isArray(a))
      return "array";
    if (a === null)
      return "null";
  }
  return e;
}
function Ve(a) {
  return a !== null && typeof a == "object" && !Array.isArray(a);
}
let O = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".split(""), M = [];
for (let a = 0; a < O.length; a++)
  M[O[a].charCodeAt(0)] = a;
M[45] = O.indexOf("+");
M[95] = O.indexOf("/");
function We(a) {
  let e = a.length * 3 / 4;
  a[a.length - 2] == "=" ? e -= 2 : a[a.length - 1] == "=" && (e -= 1);
  let t = new Uint8Array(e), i = 0, r = 0, n, o = 0;
  for (let s = 0; s < a.length; s++) {
    if (n = M[a.charCodeAt(s)], n === void 0)
      switch (a[s]) {
        case "=":
          r = 0;
        // reset state when padding found
        case `
`:
        case "\r":
        case "	":
        case " ":
          continue;
        // skip white-space, and padding
        default:
          throw Error("invalid base64 string.");
      }
    switch (r) {
      case 0:
        o = n, r = 1;
        break;
      case 1:
        t[i++] = o << 2 | (n & 48) >> 4, o = n, r = 2;
        break;
      case 2:
        t[i++] = (o & 15) << 4 | (n & 60) >> 2, o = n, r = 3;
        break;
      case 3:
        t[i++] = (o & 3) << 6 | n, r = 0;
        break;
    }
  }
  if (r == 1)
    throw Error("invalid base64 string.");
  return t.subarray(0, i);
}
function Ke(a) {
  let e = "", t = 0, i, r = 0;
  for (let n = 0; n < a.length; n++)
    switch (i = a[n], t) {
      case 0:
        e += O[i >> 2], r = (i & 3) << 4, t = 1;
        break;
      case 1:
        e += O[r | i >> 4], r = (i & 15) << 2, t = 2;
        break;
      case 2:
        e += O[r | i >> 6], e += O[i & 63], t = 0;
        break;
    }
  return t && (e += O[r], e += "=", t == 1 && (e += "=")), e;
}
var y;
(function(a) {
  a.symbol = /* @__PURE__ */ Symbol.for("protobuf-ts/unknown"), a.onRead = (t, i, r, n, o) => {
    (e(i) ? i[a.symbol] : i[a.symbol] = []).push({ no: r, wireType: n, data: o });
  }, a.onWrite = (t, i, r) => {
    for (let { no: n, wireType: o, data: s } of a.list(i))
      r.tag(n, o).raw(s);
  }, a.list = (t, i) => {
    if (e(t)) {
      let r = t[a.symbol];
      return i ? r.filter((n) => n.no == i) : r;
    }
    return [];
  }, a.last = (t, i) => a.list(t, i).slice(-1)[0];
  const e = (t) => t && Array.isArray(t[a.symbol]);
})(y || (y = {}));
var u;
(function(a) {
  a[a.Varint = 0] = "Varint", a[a.Bit64 = 1] = "Bit64", a[a.LengthDelimited = 2] = "LengthDelimited", a[a.StartGroup = 3] = "StartGroup", a[a.EndGroup = 4] = "EndGroup", a[a.Bit32 = 5] = "Bit32";
})(u || (u = {}));
function je() {
  let a = 0, e = 0;
  for (let i = 0; i < 28; i += 7) {
    let r = this.buf[this.pos++];
    if (a |= (r & 127) << i, (r & 128) == 0)
      return this.assertBounds(), [a, e];
  }
  let t = this.buf[this.pos++];
  if (a |= (t & 15) << 28, e = (t & 112) >> 4, (t & 128) == 0)
    return this.assertBounds(), [a, e];
  for (let i = 3; i <= 31; i += 7) {
    let r = this.buf[this.pos++];
    if (e |= (r & 127) << i, (r & 128) == 0)
      return this.assertBounds(), [a, e];
  }
  throw new Error("invalid varint");
}
function J(a, e, t) {
  for (let n = 0; n < 28; n = n + 7) {
    const o = a >>> n, s = !(!(o >>> 7) && e == 0), l = (s ? o | 128 : o) & 255;
    if (t.push(l), !s)
      return;
  }
  const i = a >>> 28 & 15 | (e & 7) << 4, r = e >> 3 != 0;
  if (t.push((r ? i | 128 : i) & 255), !!r) {
    for (let n = 3; n < 31; n = n + 7) {
      const o = e >>> n, s = !!(o >>> 7), l = (s ? o | 128 : o) & 255;
      if (t.push(l), !s)
        return;
    }
    t.push(e >>> 31 & 1);
  }
}
const A = 65536 * 65536;
function Ie(a) {
  let e = a[0] == "-";
  e && (a = a.slice(1));
  const t = 1e6;
  let i = 0, r = 0;
  function n(o, s) {
    const l = Number(a.slice(o, s));
    r *= t, i = i * t + l, i >= A && (r = r + (i / A | 0), i = i % A);
  }
  return n(-24, -18), n(-18, -12), n(-12, -6), n(-6), [e, i, r];
}
function le(a, e) {
  if (e >>> 0 <= 2097151)
    return "" + (A * e + (a >>> 0));
  let t = a & 16777215, i = (a >>> 24 | e << 8) >>> 0 & 16777215, r = e >> 16 & 65535, n = t + i * 6777216 + r * 6710656, o = i + r * 8147497, s = r * 2, l = 1e7;
  n >= l && (o += Math.floor(n / l), n %= l), o >= l && (s += Math.floor(o / l), o %= l);
  function f(h, k) {
    let p = h ? String(h) : "";
    return k ? "0000000".slice(p.length) + p : p;
  }
  return f(
    s,
    /*needLeadingZeros=*/
    0
  ) + f(
    o,
    /*needLeadingZeros=*/
    s
  ) + // If the final 1e7 digit didn't need leading zeros, we would have
  // returned via the trivial code path at the top.
  f(
    n,
    /*needLeadingZeros=*/
    1
  );
}
function pe(a, e) {
  if (a >= 0) {
    for (; a > 127; )
      e.push(a & 127 | 128), a = a >>> 7;
    e.push(a);
  } else {
    for (let t = 0; t < 9; t++)
      e.push(a & 127 | 128), a = a >> 7;
    e.push(1);
  }
}
function Ae() {
  let a = this.buf[this.pos++], e = a & 127;
  if ((a & 128) == 0)
    return this.assertBounds(), e;
  if (a = this.buf[this.pos++], e |= (a & 127) << 7, (a & 128) == 0)
    return this.assertBounds(), e;
  if (a = this.buf[this.pos++], e |= (a & 127) << 14, (a & 128) == 0)
    return this.assertBounds(), e;
  if (a = this.buf[this.pos++], e |= (a & 127) << 21, (a & 128) == 0)
    return this.assertBounds(), e;
  a = this.buf[this.pos++], e |= (a & 15) << 28;
  for (let t = 5; (a & 128) !== 0 && t < 10; t++)
    a = this.buf[this.pos++];
  if ((a & 128) != 0)
    throw new Error("invalid varint");
  return this.assertBounds(), e >>> 0;
}
let b;
function Ce() {
  const a = new DataView(new ArrayBuffer(8));
  b = globalThis.BigInt !== void 0 && typeof a.getBigInt64 == "function" && typeof a.getBigUint64 == "function" && typeof a.setBigInt64 == "function" && typeof a.setBigUint64 == "function" ? {
    MIN: BigInt("-9223372036854775808"),
    MAX: BigInt("9223372036854775807"),
    UMIN: BigInt("0"),
    UMAX: BigInt("18446744073709551615"),
    C: BigInt,
    V: a
  } : void 0;
}
Ce();
function Re(a) {
  if (!a)
    throw new Error("BigInt unavailable, see https://github.com/timostamm/protobuf-ts/blob/v1.0.8/MANUAL.md#bigint-support");
}
const De = /^-?[0-9]+$/, C = 4294967296, K = 2147483648;
class Fe {
  /**
   * Create a new instance with the given bits.
   */
  constructor(e, t) {
    this.lo = e | 0, this.hi = t | 0;
  }
  /**
   * Is this instance equal to 0?
   */
  isZero() {
    return this.lo == 0 && this.hi == 0;
  }
  /**
   * Convert to a native number.
   */
  toNumber() {
    let e = this.hi * C + (this.lo >>> 0);
    if (!Number.isSafeInteger(e))
      throw new Error("cannot convert to safe number");
    return e;
  }
}
class I extends Fe {
  /**
   * Create instance from a `string`, `number` or `bigint`.
   */
  static from(e) {
    if (b)
      switch (typeof e) {
        case "string":
          if (e == "0")
            return this.ZERO;
          if (e == "")
            throw new Error("string is no integer");
          e = b.C(e);
        case "number":
          if (e === 0)
            return this.ZERO;
          e = b.C(e);
        case "bigint":
          if (!e)
            return this.ZERO;
          if (e < b.UMIN)
            throw new Error("signed value for ulong");
          if (e > b.UMAX)
            throw new Error("ulong too large");
          return b.V.setBigUint64(0, e, !0), new I(b.V.getInt32(0, !0), b.V.getInt32(4, !0));
      }
    else
      switch (typeof e) {
        case "string":
          if (e == "0")
            return this.ZERO;
          if (e = e.trim(), !De.test(e))
            throw new Error("string is no integer");
          let [t, i, r] = Ie(e);
          if (t)
            throw new Error("signed value for ulong");
          return new I(i, r);
        case "number":
          if (e == 0)
            return this.ZERO;
          if (!Number.isSafeInteger(e))
            throw new Error("number is no integer");
          if (e < 0)
            throw new Error("signed value for ulong");
          return new I(e, e / C);
      }
    throw new Error("unknown value " + typeof e);
  }
  /**
   * Convert to decimal string.
   */
  toString() {
    return b ? this.toBigInt().toString() : le(this.lo, this.hi);
  }
  /**
   * Convert to native bigint.
   */
  toBigInt() {
    return Re(b), b.V.setInt32(0, this.lo, !0), b.V.setInt32(4, this.hi, !0), b.V.getBigUint64(0, !0);
  }
}
I.ZERO = new I(0, 0);
class N extends Fe {
  /**
   * Create instance from a `string`, `number` or `bigint`.
   */
  static from(e) {
    if (b)
      switch (typeof e) {
        case "string":
          if (e == "0")
            return this.ZERO;
          if (e == "")
            throw new Error("string is no integer");
          e = b.C(e);
        case "number":
          if (e === 0)
            return this.ZERO;
          e = b.C(e);
        case "bigint":
          if (!e)
            return this.ZERO;
          if (e < b.MIN)
            throw new Error("signed long too small");
          if (e > b.MAX)
            throw new Error("signed long too large");
          return b.V.setBigInt64(0, e, !0), new N(b.V.getInt32(0, !0), b.V.getInt32(4, !0));
      }
    else
      switch (typeof e) {
        case "string":
          if (e == "0")
            return this.ZERO;
          if (e = e.trim(), !De.test(e))
            throw new Error("string is no integer");
          let [t, i, r] = Ie(e);
          if (t) {
            if (r > K || r == K && i != 0)
              throw new Error("signed long too small");
          } else if (r >= K)
            throw new Error("signed long too large");
          let n = new N(i, r);
          return t ? n.negate() : n;
        case "number":
          if (e == 0)
            return this.ZERO;
          if (!Number.isSafeInteger(e))
            throw new Error("number is no integer");
          return e > 0 ? new N(e, e / C) : new N(-e, -e / C).negate();
      }
    throw new Error("unknown value " + typeof e);
  }
  /**
   * Do we have a minus sign?
   */
  isNegative() {
    return (this.hi & K) !== 0;
  }
  /**
   * Negate two's complement.
   * Invert all the bits and add one to the result.
   */
  negate() {
    let e = ~this.hi, t = this.lo;
    return t ? t = ~t + 1 : e += 1, new N(t, e);
  }
  /**
   * Convert to decimal string.
   */
  toString() {
    if (b)
      return this.toBigInt().toString();
    if (this.isNegative()) {
      let e = this.negate();
      return "-" + le(e.lo, e.hi);
    }
    return le(this.lo, this.hi);
  }
  /**
   * Convert to native bigint.
   */
  toBigInt() {
    return Re(b), b.V.setInt32(0, this.lo, !0), b.V.setInt32(4, this.hi, !0), b.V.getBigInt64(0, !0);
  }
}
N.ZERO = new N(0, 0);
const ge = {
  readUnknownField: !0,
  readerFactory: (a) => new Se(a)
};
function _e(a) {
  return a ? Object.assign(Object.assign({}, ge), a) : ge;
}
class Se {
  constructor(e, t) {
    this.varint64 = je, this.uint32 = Ae, this.buf = e, this.len = e.length, this.pos = 0, this.view = new DataView(e.buffer, e.byteOffset, e.byteLength), this.textDecoder = t ?? new TextDecoder("utf-8", {
      fatal: !0,
      ignoreBOM: !0
    });
  }
  /**
   * Reads a tag - field number and wire type.
   */
  tag() {
    let e = this.uint32(), t = e >>> 3, i = e & 7;
    if (t <= 0 || i < 0 || i > 5)
      throw new Error("illegal tag: field no " + t + " wire type " + i);
    return [t, i];
  }
  /**
   * Skip one element on the wire and return the skipped data.
   * Supports WireType.StartGroup since v2.0.0-alpha.23.
   */
  skip(e) {
    let t = this.pos;
    switch (e) {
      case u.Varint:
        for (; this.buf[this.pos++] & 128; )
          ;
        break;
      case u.Bit64:
        this.pos += 4;
      case u.Bit32:
        this.pos += 4;
        break;
      case u.LengthDelimited:
        let i = this.uint32();
        this.pos += i;
        break;
      case u.StartGroup:
        let r;
        for (; (r = this.tag()[1]) !== u.EndGroup; )
          this.skip(r);
        break;
      default:
        throw new Error("cant skip wire type " + e);
    }
    return this.assertBounds(), this.buf.subarray(t, this.pos);
  }
  /**
   * Throws error if position in byte array is out of range.
   */
  assertBounds() {
    if (this.pos > this.len)
      throw new RangeError("premature EOF");
  }
  /**
   * Read a `int32` field, a signed 32 bit varint.
   */
  int32() {
    return this.uint32() | 0;
  }
  /**
   * Read a `sint32` field, a signed, zigzag-encoded 32-bit varint.
   */
  sint32() {
    let e = this.uint32();
    return e >>> 1 ^ -(e & 1);
  }
  /**
   * Read a `int64` field, a signed 64-bit varint.
   */
  int64() {
    return new N(...this.varint64());
  }
  /**
   * Read a `uint64` field, an unsigned 64-bit varint.
   */
  uint64() {
    return new I(...this.varint64());
  }
  /**
   * Read a `sint64` field, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64() {
    let [e, t] = this.varint64(), i = -(e & 1);
    return e = (e >>> 1 | (t & 1) << 31) ^ i, t = t >>> 1 ^ i, new N(e, t);
  }
  /**
   * Read a `bool` field, a variant.
   */
  bool() {
    let [e, t] = this.varint64();
    return e !== 0 || t !== 0;
  }
  /**
   * Read a `fixed32` field, an unsigned, fixed-length 32-bit integer.
   */
  fixed32() {
    return this.view.getUint32((this.pos += 4) - 4, !0);
  }
  /**
   * Read a `sfixed32` field, a signed, fixed-length 32-bit integer.
   */
  sfixed32() {
    return this.view.getInt32((this.pos += 4) - 4, !0);
  }
  /**
   * Read a `fixed64` field, an unsigned, fixed-length 64 bit integer.
   */
  fixed64() {
    return new I(this.sfixed32(), this.sfixed32());
  }
  /**
   * Read a `fixed64` field, a signed, fixed-length 64-bit integer.
   */
  sfixed64() {
    return new N(this.sfixed32(), this.sfixed32());
  }
  /**
   * Read a `float` field, 32-bit floating point number.
   */
  float() {
    return this.view.getFloat32((this.pos += 4) - 4, !0);
  }
  /**
   * Read a `double` field, a 64-bit floating point number.
   */
  double() {
    return this.view.getFloat64((this.pos += 8) - 8, !0);
  }
  /**
   * Read a `bytes` field, length-delimited arbitrary data.
   */
  bytes() {
    let e = this.uint32(), t = this.pos;
    return this.pos += e, this.assertBounds(), this.buf.subarray(t, t + e);
  }
  /**
   * Read a `string` field, length-delimited data converted to UTF-8 text.
   */
  string() {
    return this.textDecoder.decode(this.bytes());
  }
}
function g(a, e) {
  if (!a)
    throw new Error(e);
}
const Pe = 34028234663852886e22, Me = -34028234663852886e22, Xe = 4294967295, Je = 2147483647, Ge = -2147483648;
function V(a) {
  if (typeof a != "number")
    throw new Error("invalid int 32: " + typeof a);
  if (!Number.isInteger(a) || a > Je || a < Ge)
    throw new Error("invalid int 32: " + a);
}
function _(a) {
  if (typeof a != "number")
    throw new Error("invalid uint 32: " + typeof a);
  if (!Number.isInteger(a) || a > Xe || a < 0)
    throw new Error("invalid uint 32: " + a);
}
function de(a) {
  if (typeof a != "number")
    throw new Error("invalid float 32: " + typeof a);
  if (Number.isFinite(a) && (a > Pe || a < Me))
    throw new Error("invalid float 32: " + a);
}
const me = {
  writeUnknownFields: !0,
  writerFactory: () => new Ye()
};
function Ze(a) {
  return a ? Object.assign(Object.assign({}, me), a) : me;
}
class Ye {
  constructor(e) {
    this.stack = [], this.textEncoder = e ?? new TextEncoder(), this.chunks = [], this.buf = [];
  }
  /**
   * Return all bytes written and reset this writer.
   */
  finish() {
    this.chunks.push(new Uint8Array(this.buf));
    let e = 0;
    for (let r = 0; r < this.chunks.length; r++)
      e += this.chunks[r].length;
    let t = new Uint8Array(e), i = 0;
    for (let r = 0; r < this.chunks.length; r++)
      t.set(this.chunks[r], i), i += this.chunks[r].length;
    return this.chunks = [], t;
  }
  /**
   * Start a new fork for length-delimited data like a message
   * or a packed repeated field.
   *
   * Must be joined later with `join()`.
   */
  fork() {
    return this.stack.push({ chunks: this.chunks, buf: this.buf }), this.chunks = [], this.buf = [], this;
  }
  /**
   * Join the last fork. Write its length and bytes, then
   * return to the previous state.
   */
  join() {
    let e = this.finish(), t = this.stack.pop();
    if (!t)
      throw new Error("invalid state, fork stack empty");
    return this.chunks = t.chunks, this.buf = t.buf, this.uint32(e.byteLength), this.raw(e);
  }
  /**
   * Writes a tag (field number and wire type).
   *
   * Equivalent to `uint32( (fieldNo << 3 | type) >>> 0 )`.
   *
   * Generated code should compute the tag ahead of time and call `uint32()`.
   */
  tag(e, t) {
    return this.uint32((e << 3 | t) >>> 0);
  }
  /**
   * Write a chunk of raw bytes.
   */
  raw(e) {
    return this.buf.length && (this.chunks.push(new Uint8Array(this.buf)), this.buf = []), this.chunks.push(e), this;
  }
  /**
   * Write a `uint32` value, an unsigned 32 bit varint.
   */
  uint32(e) {
    for (_(e); e > 127; )
      this.buf.push(e & 127 | 128), e = e >>> 7;
    return this.buf.push(e), this;
  }
  /**
   * Write a `int32` value, a signed 32 bit varint.
   */
  int32(e) {
    return V(e), pe(e, this.buf), this;
  }
  /**
   * Write a `bool` value, a variant.
   */
  bool(e) {
    return this.buf.push(e ? 1 : 0), this;
  }
  /**
   * Write a `bytes` value, length-delimited arbitrary data.
   */
  bytes(e) {
    return this.uint32(e.byteLength), this.raw(e);
  }
  /**
   * Write a `string` value, length-delimited data converted to UTF-8 text.
   */
  string(e) {
    let t = this.textEncoder.encode(e);
    return this.uint32(t.byteLength), this.raw(t);
  }
  /**
   * Write a `float` value, 32-bit floating point number.
   */
  float(e) {
    de(e);
    let t = new Uint8Array(4);
    return new DataView(t.buffer).setFloat32(0, e, !0), this.raw(t);
  }
  /**
   * Write a `double` value, a 64-bit floating point number.
   */
  double(e) {
    let t = new Uint8Array(8);
    return new DataView(t.buffer).setFloat64(0, e, !0), this.raw(t);
  }
  /**
   * Write a `fixed32` value, an unsigned, fixed-length 32-bit integer.
   */
  fixed32(e) {
    _(e);
    let t = new Uint8Array(4);
    return new DataView(t.buffer).setUint32(0, e, !0), this.raw(t);
  }
  /**
   * Write a `sfixed32` value, a signed, fixed-length 32-bit integer.
   */
  sfixed32(e) {
    V(e);
    let t = new Uint8Array(4);
    return new DataView(t.buffer).setInt32(0, e, !0), this.raw(t);
  }
  /**
   * Write a `sint32` value, a signed, zigzag-encoded 32-bit varint.
   */
  sint32(e) {
    return V(e), e = (e << 1 ^ e >> 31) >>> 0, pe(e, this.buf), this;
  }
  /**
   * Write a `fixed64` value, a signed, fixed-length 64-bit integer.
   */
  sfixed64(e) {
    let t = new Uint8Array(8), i = new DataView(t.buffer), r = N.from(e);
    return i.setInt32(0, r.lo, !0), i.setInt32(4, r.hi, !0), this.raw(t);
  }
  /**
   * Write a `fixed64` value, an unsigned, fixed-length 64 bit integer.
   */
  fixed64(e) {
    let t = new Uint8Array(8), i = new DataView(t.buffer), r = I.from(e);
    return i.setInt32(0, r.lo, !0), i.setInt32(4, r.hi, !0), this.raw(t);
  }
  /**
   * Write a `int64` value, a signed 64-bit varint.
   */
  int64(e) {
    let t = N.from(e);
    return J(t.lo, t.hi, this.buf), this;
  }
  /**
   * Write a `sint64` value, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64(e) {
    let t = N.from(e), i = t.hi >> 31, r = t.lo << 1 ^ i, n = (t.hi << 1 | t.lo >>> 31) ^ i;
    return J(r, n, this.buf), this;
  }
  /**
   * Write a `uint64` value, an unsigned 64-bit varint.
   */
  uint64(e) {
    let t = I.from(e);
    return J(t.lo, t.hi, this.buf), this;
  }
}
const ke = {
  emitDefaultValues: !1,
  enumAsInteger: !1,
  useProtoFieldName: !1,
  prettySpaces: 0
}, be = {
  ignoreUnknownFields: !1
};
function qe(a) {
  return a ? Object.assign(Object.assign({}, be), a) : be;
}
function ze(a) {
  return a ? Object.assign(Object.assign({}, ke), a) : ke;
}
const Ee = /* @__PURE__ */ Symbol.for("protobuf-ts/message-type");
function Te(a) {
  let e = !1;
  const t = [];
  for (let i = 0; i < a.length; i++) {
    let r = a.charAt(i);
    r == "_" ? e = !0 : /\d/.test(r) ? (t.push(r), e = !0) : e ? (t.push(r.toUpperCase()), e = !1) : i == 0 ? t.push(r.toLowerCase()) : t.push(r);
  }
  return t.join("");
}
var c;
(function(a) {
  a[a.DOUBLE = 1] = "DOUBLE", a[a.FLOAT = 2] = "FLOAT", a[a.INT64 = 3] = "INT64", a[a.UINT64 = 4] = "UINT64", a[a.INT32 = 5] = "INT32", a[a.FIXED64 = 6] = "FIXED64", a[a.FIXED32 = 7] = "FIXED32", a[a.BOOL = 8] = "BOOL", a[a.STRING = 9] = "STRING", a[a.BYTES = 12] = "BYTES", a[a.UINT32 = 13] = "UINT32", a[a.SFIXED32 = 15] = "SFIXED32", a[a.SFIXED64 = 16] = "SFIXED64", a[a.SINT32 = 17] = "SINT32", a[a.SINT64 = 18] = "SINT64";
})(c || (c = {}));
var F;
(function(a) {
  a[a.BIGINT = 0] = "BIGINT", a[a.STRING = 1] = "STRING", a[a.NUMBER = 2] = "NUMBER";
})(F || (F = {}));
var S;
(function(a) {
  a[a.NO = 0] = "NO", a[a.PACKED = 1] = "PACKED", a[a.UNPACKED = 2] = "UNPACKED";
})(S || (S = {}));
function Qe(a) {
  var e, t, i, r;
  return a.localName = (e = a.localName) !== null && e !== void 0 ? e : Te(a.name), a.jsonName = (t = a.jsonName) !== null && t !== void 0 ? t : Te(a.name), a.repeat = (i = a.repeat) !== null && i !== void 0 ? i : S.NO, a.opt = (r = a.opt) !== null && r !== void 0 ? r : a.repeat || a.oneof ? !1 : a.kind == "message", a;
}
function He(a) {
  if (typeof a != "object" || a === null || !a.hasOwnProperty("oneofKind"))
    return !1;
  switch (typeof a.oneofKind) {
    case "string":
      return a[a.oneofKind] === void 0 ? !1 : Object.keys(a).length == 2;
    case "undefined":
      return Object.keys(a).length == 1;
    default:
      return !1;
  }
}
class ve {
  constructor(e) {
    var t;
    this.fields = (t = e.fields) !== null && t !== void 0 ? t : [];
  }
  prepare() {
    if (this.data)
      return;
    const e = [], t = [], i = [];
    for (let r of this.fields)
      if (r.oneof)
        i.includes(r.oneof) || (i.push(r.oneof), e.push(r.oneof), t.push(r.oneof));
      else
        switch (t.push(r.localName), r.kind) {
          case "scalar":
          case "enum":
            (!r.opt || r.repeat) && e.push(r.localName);
            break;
          case "message":
            r.repeat && e.push(r.localName);
            break;
          case "map":
            e.push(r.localName);
            break;
        }
    this.data = { req: e, known: t, oneofs: Object.values(i) };
  }
  /**
   * Is the argument a valid message as specified by the
   * reflection information?
   *
   * Checks all field types recursively. The `depth`
   * specifies how deep into the structure the check will be.
   *
   * With a depth of 0, only the presence of fields
   * is checked.
   *
   * With a depth of 1 or more, the field types are checked.
   *
   * With a depth of 2 or more, the members of map, repeated
   * and message fields are checked.
   *
   * Message fields will be checked recursively with depth - 1.
   *
   * The number of map entries / repeated values being checked
   * is < depth.
   */
  is(e, t, i = !1) {
    if (t < 0)
      return !0;
    if (e == null || typeof e != "object")
      return !1;
    this.prepare();
    let r = Object.keys(e), n = this.data;
    if (r.length < n.req.length || n.req.some((o) => !r.includes(o)) || !i && r.some((o) => !n.known.includes(o)))
      return !1;
    if (t < 1)
      return !0;
    for (const o of n.oneofs) {
      const s = e[o];
      if (!He(s))
        return !1;
      if (s.oneofKind === void 0)
        continue;
      const l = this.fields.find((f) => f.localName === s.oneofKind);
      if (!l || !this.field(s[s.oneofKind], l, i, t))
        return !1;
    }
    for (const o of this.fields)
      if (o.oneof === void 0 && !this.field(e[o.localName], o, i, t))
        return !1;
    return !0;
  }
  field(e, t, i, r) {
    let n = t.repeat;
    switch (t.kind) {
      case "scalar":
        return e === void 0 ? t.opt : n ? this.scalars(e, t.T, r, t.L) : this.scalar(e, t.T, t.L);
      case "enum":
        return e === void 0 ? t.opt : n ? this.scalars(e, c.INT32, r) : this.scalar(e, c.INT32);
      case "message":
        return e === void 0 ? !0 : n ? this.messages(e, t.T(), i, r) : this.message(e, t.T(), i, r);
      case "map":
        if (typeof e != "object" || e === null)
          return !1;
        if (r < 2)
          return !0;
        if (!this.mapKeys(e, t.K, r))
          return !1;
        switch (t.V.kind) {
          case "scalar":
            return this.scalars(Object.values(e), t.V.T, r, t.V.L);
          case "enum":
            return this.scalars(Object.values(e), c.INT32, r);
          case "message":
            return this.messages(Object.values(e), t.V.T(), i, r);
        }
        break;
    }
    return !0;
  }
  message(e, t, i, r) {
    return i ? t.isAssignable(e, r) : t.is(e, r);
  }
  messages(e, t, i, r) {
    if (!Array.isArray(e))
      return !1;
    if (r < 2)
      return !0;
    if (i) {
      for (let n = 0; n < e.length && n < r; n++)
        if (!t.isAssignable(e[n], r - 1))
          return !1;
    } else
      for (let n = 0; n < e.length && n < r; n++)
        if (!t.is(e[n], r - 1))
          return !1;
    return !0;
  }
  scalar(e, t, i) {
    let r = typeof e;
    switch (t) {
      case c.UINT64:
      case c.FIXED64:
      case c.INT64:
      case c.SFIXED64:
      case c.SINT64:
        switch (i) {
          case F.BIGINT:
            return r == "bigint";
          case F.NUMBER:
            return r == "number" && !isNaN(e);
          default:
            return r == "string";
        }
      case c.BOOL:
        return r == "boolean";
      case c.STRING:
        return r == "string";
      case c.BYTES:
        return e instanceof Uint8Array;
      case c.DOUBLE:
      case c.FLOAT:
        return r == "number" && !isNaN(e);
      default:
        return r == "number" && Number.isInteger(e);
    }
  }
  scalars(e, t, i, r) {
    if (!Array.isArray(e))
      return !1;
    if (i < 2)
      return !0;
    if (Array.isArray(e)) {
      for (let n = 0; n < e.length && n < i; n++)
        if (!this.scalar(e[n], t, r))
          return !1;
    }
    return !0;
  }
  mapKeys(e, t, i) {
    let r = Object.keys(e);
    switch (t) {
      case c.INT32:
      case c.FIXED32:
      case c.SFIXED32:
      case c.SINT32:
      case c.UINT32:
        return this.scalars(r.slice(0, i).map((n) => parseInt(n)), t, i);
      case c.BOOL:
        return this.scalars(r.slice(0, i).map((n) => n == "true" ? !0 : n == "false" ? !1 : n), t, i);
      default:
        return this.scalars(r, t, i, F.STRING);
    }
  }
}
function D(a, e) {
  switch (e) {
    case F.BIGINT:
      return a.toBigInt();
    case F.NUMBER:
      return a.toNumber();
    default:
      return a.toString();
  }
}
class et {
  constructor(e) {
    this.info = e;
  }
  prepare() {
    var e;
    if (this.fMap === void 0) {
      this.fMap = {};
      const t = (e = this.info.fields) !== null && e !== void 0 ? e : [];
      for (const i of t)
        this.fMap[i.name] = i, this.fMap[i.jsonName] = i, this.fMap[i.localName] = i;
    }
  }
  // Cannot parse JSON <type of jsonValue> for <type name>#<fieldName>.
  assert(e, t, i) {
    if (!e) {
      let r = Be(i);
      throw (r == "number" || r == "boolean") && (r = i.toString()), new Error(`Cannot parse JSON ${r} for ${this.info.typeName}#${t}`);
    }
  }
  /**
   * Reads a message from canonical JSON format into the target message.
   *
   * Repeated fields are appended. Map entries are added, overwriting
   * existing keys.
   *
   * If a message field is already present, it will be merged with the
   * new data.
   */
  read(e, t, i) {
    this.prepare();
    const r = [];
    for (const [n, o] of Object.entries(e)) {
      const s = this.fMap[n];
      if (!s) {
        if (!i.ignoreUnknownFields)
          throw new Error(`Found unknown field while reading ${this.info.typeName} from JSON format. JSON key: ${n}`);
        continue;
      }
      const l = s.localName;
      let f;
      if (s.oneof) {
        if (o === null && (s.kind !== "enum" || s.T()[0] !== "google.protobuf.NullValue"))
          continue;
        if (r.includes(s.oneof))
          throw new Error(`Multiple members of the oneof group "${s.oneof}" of ${this.info.typeName} are present in JSON.`);
        r.push(s.oneof), f = t[s.oneof] = {
          oneofKind: l
        };
      } else
        f = t;
      if (s.kind == "map") {
        if (o === null)
          continue;
        this.assert(Ve(o), s.name, o);
        const h = f[l];
        for (const [k, p] of Object.entries(o)) {
          this.assert(p !== null, s.name + " map value", null);
          let R;
          switch (s.V.kind) {
            case "message":
              R = s.V.T().internalJsonRead(p, i);
              break;
            case "enum":
              if (R = this.enum(s.V.T(), p, s.name, i.ignoreUnknownFields), R === !1)
                continue;
              break;
            case "scalar":
              R = this.scalar(p, s.V.T, s.V.L, s.name);
              break;
          }
          this.assert(R !== void 0, s.name + " map value", p);
          let E = k;
          s.K == c.BOOL && (E = E == "true" ? !0 : E == "false" ? !1 : E), E = this.scalar(E, s.K, F.STRING, s.name).toString(), h[E] = R;
        }
      } else if (s.repeat) {
        if (o === null)
          continue;
        this.assert(Array.isArray(o), s.name, o);
        const h = f[l];
        for (const k of o) {
          this.assert(k !== null, s.name, null);
          let p;
          switch (s.kind) {
            case "message":
              p = s.T().internalJsonRead(k, i);
              break;
            case "enum":
              if (p = this.enum(s.T(), k, s.name, i.ignoreUnknownFields), p === !1)
                continue;
              break;
            case "scalar":
              p = this.scalar(k, s.T, s.L, s.name);
              break;
          }
          this.assert(p !== void 0, s.name, o), h.push(p);
        }
      } else
        switch (s.kind) {
          case "message":
            if (o === null && s.T().typeName != "google.protobuf.Value") {
              this.assert(s.oneof === void 0, s.name + " (oneof member)", null);
              continue;
            }
            f[l] = s.T().internalJsonRead(o, i, f[l]);
            break;
          case "enum":
            if (o === null)
              continue;
            let h = this.enum(s.T(), o, s.name, i.ignoreUnknownFields);
            if (h === !1)
              continue;
            f[l] = h;
            break;
          case "scalar":
            if (o === null)
              continue;
            f[l] = this.scalar(o, s.T, s.L, s.name);
            break;
        }
    }
  }
  /**
   * Returns `false` for unrecognized string representations.
   *
   * google.protobuf.NullValue accepts only JSON `null` (or the old `"NULL_VALUE"`).
   */
  enum(e, t, i, r) {
    if (e[0] == "google.protobuf.NullValue" && g(t === null || t === "NULL_VALUE", `Unable to parse field ${this.info.typeName}#${i}, enum ${e[0]} only accepts null.`), t === null)
      return 0;
    switch (typeof t) {
      case "number":
        return g(Number.isInteger(t), `Unable to parse field ${this.info.typeName}#${i}, enum can only be integral number, got ${t}.`), t;
      case "string":
        let n = t;
        e[2] && t.substring(0, e[2].length) === e[2] && (n = t.substring(e[2].length));
        let o = e[1][n];
        return typeof o > "u" && r ? !1 : (g(typeof o == "number", `Unable to parse field ${this.info.typeName}#${i}, enum ${e[0]} has no value for "${t}".`), o);
    }
    g(!1, `Unable to parse field ${this.info.typeName}#${i}, cannot parse enum value from ${typeof t}".`);
  }
  scalar(e, t, i, r) {
    let n;
    try {
      switch (t) {
        // float, double: JSON value will be a number or one of the special string values "NaN", "Infinity", and "-Infinity".
        // Either numbers or strings are accepted. Exponent notation is also accepted.
        case c.DOUBLE:
        case c.FLOAT:
          if (e === null)
            return 0;
          if (e === "NaN")
            return Number.NaN;
          if (e === "Infinity")
            return Number.POSITIVE_INFINITY;
          if (e === "-Infinity")
            return Number.NEGATIVE_INFINITY;
          if (e === "") {
            n = "empty string";
            break;
          }
          if (typeof e == "string" && e.trim().length !== e.length) {
            n = "extra whitespace";
            break;
          }
          if (typeof e != "string" && typeof e != "number")
            break;
          let o = Number(e);
          if (Number.isNaN(o)) {
            n = "not a number";
            break;
          }
          if (!Number.isFinite(o)) {
            n = "too large or small";
            break;
          }
          return t == c.FLOAT && de(o), o;
        // int32, fixed32, uint32: JSON value will be a decimal number. Either numbers or strings are accepted.
        case c.INT32:
        case c.FIXED32:
        case c.SFIXED32:
        case c.SINT32:
        case c.UINT32:
          if (e === null)
            return 0;
          let s;
          if (typeof e == "number" ? s = e : e === "" ? n = "empty string" : typeof e == "string" && (e.trim().length !== e.length ? n = "extra whitespace" : s = Number(e)), s === void 0)
            break;
          return t == c.UINT32 ? _(s) : V(s), s;
        // int64, fixed64, uint64: JSON value will be a decimal string. Either numbers or strings are accepted.
        case c.INT64:
        case c.SFIXED64:
        case c.SINT64:
          if (e === null)
            return D(N.ZERO, i);
          if (typeof e != "number" && typeof e != "string")
            break;
          return D(N.from(e), i);
        case c.FIXED64:
        case c.UINT64:
          if (e === null)
            return D(I.ZERO, i);
          if (typeof e != "number" && typeof e != "string")
            break;
          return D(I.from(e), i);
        // bool:
        case c.BOOL:
          if (e === null)
            return !1;
          if (typeof e != "boolean")
            break;
          return e;
        // string:
        case c.STRING:
          if (e === null)
            return "";
          if (typeof e != "string") {
            n = "extra whitespace";
            break;
          }
          try {
            encodeURIComponent(e);
          } catch (l) {
            l = "invalid UTF8";
            break;
          }
          return e;
        // bytes: JSON value will be the data encoded as a string using standard base64 encoding with paddings.
        // Either standard or URL-safe base64 encoding with/without paddings are accepted.
        case c.BYTES:
          if (e === null || e === "")
            return new Uint8Array(0);
          if (typeof e != "string")
            break;
          return We(e);
      }
    } catch (o) {
      n = o.message;
    }
    this.assert(!1, r + (n ? " - " + n : ""), e);
  }
}
class tt {
  constructor(e) {
    var t;
    this.fields = (t = e.fields) !== null && t !== void 0 ? t : [];
  }
  /**
   * Converts the message to a JSON object, based on the field descriptors.
   */
  write(e, t) {
    const i = {}, r = e;
    for (const n of this.fields) {
      if (!n.oneof) {
        let f = this.field(n, r[n.localName], t);
        f !== void 0 && (i[t.useProtoFieldName ? n.name : n.jsonName] = f);
        continue;
      }
      const o = r[n.oneof];
      if (o.oneofKind !== n.localName)
        continue;
      const s = n.kind == "scalar" || n.kind == "enum" ? Object.assign(Object.assign({}, t), { emitDefaultValues: !0 }) : t;
      let l = this.field(n, o[n.localName], s);
      g(l !== void 0), i[t.useProtoFieldName ? n.name : n.jsonName] = l;
    }
    return i;
  }
  field(e, t, i) {
    let r;
    if (e.kind == "map") {
      g(typeof t == "object" && t !== null);
      const n = {};
      switch (e.V.kind) {
        case "scalar":
          for (const [l, f] of Object.entries(t)) {
            const h = this.scalar(e.V.T, f, e.name, !1, !0);
            g(h !== void 0), n[l.toString()] = h;
          }
          break;
        case "message":
          const o = e.V.T();
          for (const [l, f] of Object.entries(t)) {
            const h = this.message(o, f, e.name, i);
            g(h !== void 0), n[l.toString()] = h;
          }
          break;
        case "enum":
          const s = e.V.T();
          for (const [l, f] of Object.entries(t)) {
            g(f === void 0 || typeof f == "number");
            const h = this.enum(s, f, e.name, !1, !0, i.enumAsInteger);
            g(h !== void 0), n[l.toString()] = h;
          }
          break;
      }
      (i.emitDefaultValues || Object.keys(n).length > 0) && (r = n);
    } else if (e.repeat) {
      g(Array.isArray(t));
      const n = [];
      switch (e.kind) {
        case "scalar":
          for (let l = 0; l < t.length; l++) {
            const f = this.scalar(e.T, t[l], e.name, e.opt, !0);
            g(f !== void 0), n.push(f);
          }
          break;
        case "enum":
          const o = e.T();
          for (let l = 0; l < t.length; l++) {
            g(t[l] === void 0 || typeof t[l] == "number");
            const f = this.enum(o, t[l], e.name, e.opt, !0, i.enumAsInteger);
            g(f !== void 0), n.push(f);
          }
          break;
        case "message":
          const s = e.T();
          for (let l = 0; l < t.length; l++) {
            const f = this.message(s, t[l], e.name, i);
            g(f !== void 0), n.push(f);
          }
          break;
      }
      (i.emitDefaultValues || n.length > 0 || i.emitDefaultValues) && (r = n);
    } else
      switch (e.kind) {
        case "scalar":
          r = this.scalar(e.T, t, e.name, e.opt, i.emitDefaultValues);
          break;
        case "enum":
          r = this.enum(e.T(), t, e.name, e.opt, i.emitDefaultValues, i.enumAsInteger);
          break;
        case "message":
          r = this.message(e.T(), t, e.name, i);
          break;
      }
    return r;
  }
  /**
   * Returns `null` as the default for google.protobuf.NullValue.
   */
  enum(e, t, i, r, n, o) {
    if (e[0] == "google.protobuf.NullValue")
      return !n && !r ? void 0 : null;
    if (t === void 0) {
      g(r);
      return;
    }
    if (!(t === 0 && !n && !r))
      return g(typeof t == "number"), g(Number.isInteger(t)), o || !e[1].hasOwnProperty(t) ? t : e[2] ? e[2] + e[1][t] : e[1][t];
  }
  message(e, t, i, r) {
    return t === void 0 ? r.emitDefaultValues ? null : void 0 : e.internalJsonWrite(t, r);
  }
  scalar(e, t, i, r, n) {
    if (t === void 0) {
      g(r);
      return;
    }
    const o = n || r;
    switch (e) {
      // int32, fixed32, uint32: JSON value will be a decimal number. Either numbers or strings are accepted.
      case c.INT32:
      case c.SFIXED32:
      case c.SINT32:
        return t === 0 ? o ? 0 : void 0 : (V(t), t);
      case c.FIXED32:
      case c.UINT32:
        return t === 0 ? o ? 0 : void 0 : (_(t), t);
      // float, double: JSON value will be a number or one of the special string values "NaN", "Infinity", and "-Infinity".
      // Either numbers or strings are accepted. Exponent notation is also accepted.
      case c.FLOAT:
        de(t);
      case c.DOUBLE:
        return t === 0 ? o ? 0 : void 0 : (g(typeof t == "number"), Number.isNaN(t) ? "NaN" : t === Number.POSITIVE_INFINITY ? "Infinity" : t === Number.NEGATIVE_INFINITY ? "-Infinity" : t);
      // string:
      case c.STRING:
        return t === "" ? o ? "" : void 0 : (g(typeof t == "string"), t);
      // bool:
      case c.BOOL:
        return t === !1 ? o ? !1 : void 0 : (g(typeof t == "boolean"), t);
      // JSON value will be a decimal string. Either numbers or strings are accepted.
      case c.UINT64:
      case c.FIXED64:
        g(typeof t == "number" || typeof t == "string" || typeof t == "bigint");
        let s = I.from(t);
        return s.isZero() && !o ? void 0 : s.toString();
      // JSON value will be a decimal string. Either numbers or strings are accepted.
      case c.INT64:
      case c.SFIXED64:
      case c.SINT64:
        g(typeof t == "number" || typeof t == "string" || typeof t == "bigint");
        let l = N.from(t);
        return l.isZero() && !o ? void 0 : l.toString();
      // bytes: JSON value will be the data encoded as a string using standard base64 encoding with paddings.
      // Either standard or URL-safe base64 encoding with/without paddings are accepted.
      case c.BYTES:
        return g(t instanceof Uint8Array), t.byteLength ? Ke(t) : o ? "" : void 0;
    }
  }
}
function fe(a, e = F.STRING) {
  switch (a) {
    case c.BOOL:
      return !1;
    case c.UINT64:
    case c.FIXED64:
      return D(I.ZERO, e);
    case c.INT64:
    case c.SFIXED64:
    case c.SINT64:
      return D(N.ZERO, e);
    case c.DOUBLE:
    case c.FLOAT:
      return 0;
    case c.BYTES:
      return new Uint8Array(0);
    case c.STRING:
      return "";
    default:
      return 0;
  }
}
class nt {
  constructor(e) {
    this.info = e;
  }
  prepare() {
    var e;
    if (!this.fieldNoToField) {
      const t = (e = this.info.fields) !== null && e !== void 0 ? e : [];
      this.fieldNoToField = new Map(t.map((i) => [i.no, i]));
    }
  }
  /**
   * Reads a message from binary format into the target message.
   *
   * Repeated fields are appended. Map entries are added, overwriting
   * existing keys.
   *
   * If a message field is already present, it will be merged with the
   * new data.
   */
  read(e, t, i, r) {
    this.prepare();
    const n = r === void 0 ? e.len : e.pos + r;
    for (; e.pos < n; ) {
      const [o, s] = e.tag(), l = this.fieldNoToField.get(o);
      if (!l) {
        let p = i.readUnknownField;
        if (p == "throw")
          throw new Error(`Unknown field ${o} (wire type ${s}) for ${this.info.typeName}`);
        let R = e.skip(s);
        p !== !1 && (p === !0 ? y.onRead : p)(this.info.typeName, t, o, s, R);
        continue;
      }
      let f = t, h = l.repeat, k = l.localName;
      switch (l.oneof && (f = f[l.oneof], f.oneofKind !== k && (f = t[l.oneof] = {
        oneofKind: k
      })), l.kind) {
        case "scalar":
        case "enum":
          let p = l.kind == "enum" ? c.INT32 : l.T, R = l.kind == "scalar" ? l.L : void 0;
          if (h) {
            let W = f[k];
            if (s == u.LengthDelimited && p != c.STRING && p != c.BYTES) {
              let X = e.uint32() + e.pos;
              for (; e.pos < X; )
                W.push(this.scalar(e, p, R));
            } else
              W.push(this.scalar(e, p, R));
          } else
            f[k] = this.scalar(e, p, R);
          break;
        case "message":
          if (h) {
            let W = f[k], X = l.T().internalBinaryRead(e, e.uint32(), i);
            W.push(X);
          } else
            f[k] = l.T().internalBinaryRead(e, e.uint32(), i, f[k]);
          break;
        case "map":
          let [E, Ue] = this.mapEntry(l, e, i);
          f[k][E] = Ue;
          break;
      }
    }
  }
  /**
   * Read a map field, expecting key field = 1, value field = 2
   */
  mapEntry(e, t, i) {
    let r = t.uint32(), n = t.pos + r, o, s;
    for (; t.pos < n; ) {
      let [l, f] = t.tag();
      switch (l) {
        case 1:
          e.K == c.BOOL ? o = t.bool().toString() : o = this.scalar(t, e.K, F.STRING);
          break;
        case 2:
          switch (e.V.kind) {
            case "scalar":
              s = this.scalar(t, e.V.T, e.V.L);
              break;
            case "enum":
              s = t.int32();
              break;
            case "message":
              s = e.V.T().internalBinaryRead(t, t.uint32(), i);
              break;
          }
          break;
        default:
          throw new Error(`Unknown field ${l} (wire type ${f}) in map entry for ${this.info.typeName}#${e.name}`);
      }
    }
    if (o === void 0) {
      let l = fe(e.K);
      o = e.K == c.BOOL ? l.toString() : l;
    }
    if (s === void 0)
      switch (e.V.kind) {
        case "scalar":
          s = fe(e.V.T, e.V.L);
          break;
        case "enum":
          s = 0;
          break;
        case "message":
          s = e.V.T().create();
          break;
      }
    return [o, s];
  }
  scalar(e, t, i) {
    switch (t) {
      case c.INT32:
        return e.int32();
      case c.STRING:
        return e.string();
      case c.BOOL:
        return e.bool();
      case c.DOUBLE:
        return e.double();
      case c.FLOAT:
        return e.float();
      case c.INT64:
        return D(e.int64(), i);
      case c.UINT64:
        return D(e.uint64(), i);
      case c.FIXED64:
        return D(e.fixed64(), i);
      case c.FIXED32:
        return e.fixed32();
      case c.BYTES:
        return e.bytes();
      case c.UINT32:
        return e.uint32();
      case c.SFIXED32:
        return e.sfixed32();
      case c.SFIXED64:
        return D(e.sfixed64(), i);
      case c.SINT32:
        return e.sint32();
      case c.SINT64:
        return D(e.sint64(), i);
    }
  }
}
class it {
  constructor(e) {
    this.info = e;
  }
  prepare() {
    if (!this.fields) {
      const e = this.info.fields ? this.info.fields.concat() : [];
      this.fields = e.sort((t, i) => t.no - i.no);
    }
  }
  /**
   * Writes the message to binary format.
   */
  write(e, t, i) {
    this.prepare();
    for (const n of this.fields) {
      let o, s, l = n.repeat, f = n.localName;
      if (n.oneof) {
        const h = e[n.oneof];
        if (h.oneofKind !== f)
          continue;
        o = h[f], s = !0;
      } else
        o = e[f], s = !1;
      switch (n.kind) {
        case "scalar":
        case "enum":
          let h = n.kind == "enum" ? c.INT32 : n.T;
          if (l)
            if (g(Array.isArray(o)), l == S.PACKED)
              this.packed(t, h, n.no, o);
            else
              for (const k of o)
                this.scalar(t, h, n.no, k, !0);
          else o === void 0 ? g(n.opt) : this.scalar(t, h, n.no, o, s || n.opt);
          break;
        case "message":
          if (l) {
            g(Array.isArray(o));
            for (const k of o)
              this.message(t, i, n.T(), n.no, k);
          } else
            this.message(t, i, n.T(), n.no, o);
          break;
        case "map":
          g(typeof o == "object" && o !== null);
          for (const [k, p] of Object.entries(o))
            this.mapEntry(t, i, n, k, p);
          break;
      }
    }
    let r = i.writeUnknownFields;
    r !== !1 && (r === !0 ? y.onWrite : r)(this.info.typeName, e, t);
  }
  mapEntry(e, t, i, r, n) {
    e.tag(i.no, u.LengthDelimited), e.fork();
    let o = r;
    switch (i.K) {
      case c.INT32:
      case c.FIXED32:
      case c.UINT32:
      case c.SFIXED32:
      case c.SINT32:
        o = Number.parseInt(r);
        break;
      case c.BOOL:
        g(r == "true" || r == "false"), o = r == "true";
        break;
    }
    switch (this.scalar(e, i.K, 1, o, !0), i.V.kind) {
      case "scalar":
        this.scalar(e, i.V.T, 2, n, !0);
        break;
      case "enum":
        this.scalar(e, c.INT32, 2, n, !0);
        break;
      case "message":
        this.message(e, t, i.V.T(), 2, n);
        break;
    }
    e.join();
  }
  message(e, t, i, r, n) {
    n !== void 0 && (i.internalBinaryWrite(n, e.tag(r, u.LengthDelimited).fork(), t), e.join());
  }
  /**
   * Write a single scalar value.
   */
  scalar(e, t, i, r, n) {
    let [o, s, l] = this.scalarInfo(t, r);
    (!l || n) && (e.tag(i, o), e[s](r));
  }
  /**
   * Write an array of scalar values in packed format.
   */
  packed(e, t, i, r) {
    if (!r.length)
      return;
    g(t !== c.BYTES && t !== c.STRING), e.tag(i, u.LengthDelimited), e.fork();
    let [, n] = this.scalarInfo(t);
    for (let o = 0; o < r.length; o++)
      e[n](r[o]);
    e.join();
  }
  /**
   * Get information for writing a scalar value.
   *
   * Returns tuple:
   * [0]: appropriate WireType
   * [1]: name of the appropriate method of IBinaryWriter
   * [2]: whether the given value is a default value
   *
   * If argument `value` is omitted, [2] is always false.
   */
  scalarInfo(e, t) {
    let i = u.Varint, r, n = t === void 0, o = t === 0;
    switch (e) {
      case c.INT32:
        r = "int32";
        break;
      case c.STRING:
        o = n || !t.length, i = u.LengthDelimited, r = "string";
        break;
      case c.BOOL:
        o = t === !1, r = "bool";
        break;
      case c.UINT32:
        r = "uint32";
        break;
      case c.DOUBLE:
        i = u.Bit64, r = "double";
        break;
      case c.FLOAT:
        i = u.Bit32, r = "float";
        break;
      case c.INT64:
        o = n || N.from(t).isZero(), r = "int64";
        break;
      case c.UINT64:
        o = n || I.from(t).isZero(), r = "uint64";
        break;
      case c.FIXED64:
        o = n || I.from(t).isZero(), i = u.Bit64, r = "fixed64";
        break;
      case c.BYTES:
        o = n || !t.byteLength, i = u.LengthDelimited, r = "bytes";
        break;
      case c.FIXED32:
        i = u.Bit32, r = "fixed32";
        break;
      case c.SFIXED32:
        i = u.Bit32, r = "sfixed32";
        break;
      case c.SFIXED64:
        o = n || N.from(t).isZero(), i = u.Bit64, r = "sfixed64";
        break;
      case c.SINT32:
        r = "sint32";
        break;
      case c.SINT64:
        o = n || N.from(t).isZero(), r = "sint64";
        break;
    }
    return [i, r, n || o];
  }
}
function rt(a) {
  const e = a.messagePrototype ? Object.create(a.messagePrototype) : Object.defineProperty({}, Ee, { value: a });
  for (let t of a.fields) {
    let i = t.localName;
    if (!t.opt)
      if (t.oneof)
        e[t.oneof] = { oneofKind: void 0 };
      else if (t.repeat)
        e[i] = [];
      else
        switch (t.kind) {
          case "scalar":
            e[i] = fe(t.T, t.L);
            break;
          case "enum":
            e[i] = 0;
            break;
          case "map":
            e[i] = {};
            break;
        }
  }
  return e;
}
function m(a, e, t) {
  let i, r = t, n;
  for (let o of a.fields) {
    let s = o.localName;
    if (o.oneof) {
      const l = r[o.oneof];
      if (l?.oneofKind == null)
        continue;
      if (i = l[s], n = e[o.oneof], n.oneofKind = l.oneofKind, i == null) {
        delete n[s];
        continue;
      }
    } else if (i = r[s], n = e, i == null)
      continue;
    switch (o.repeat && (n[s].length = i.length), o.kind) {
      case "scalar":
      case "enum":
        if (o.repeat)
          for (let f = 0; f < i.length; f++)
            n[s][f] = i[f];
        else
          n[s] = i;
        break;
      case "message":
        let l = o.T();
        if (o.repeat)
          for (let f = 0; f < i.length; f++)
            n[s][f] = l.create(i[f]);
        else n[s] === void 0 ? n[s] = l.create(i) : l.mergePartial(n[s], i);
        break;
      case "map":
        switch (o.V.kind) {
          case "scalar":
          case "enum":
            Object.assign(n[s], i);
            break;
          case "message":
            let f = o.V.T();
            for (let h of Object.keys(i))
              n[s][h] = f.create(i[h]);
            break;
        }
        break;
    }
  }
}
function at(a, e, t) {
  if (e === t)
    return !0;
  if (!e || !t)
    return !1;
  for (let i of a.fields) {
    let r = i.localName, n = i.oneof ? e[i.oneof][r] : e[r], o = i.oneof ? t[i.oneof][r] : t[r];
    switch (i.kind) {
      case "enum":
      case "scalar":
        let s = i.kind == "enum" ? c.INT32 : i.T;
        if (!(i.repeat ? we(s, n, o) : Oe(s, n, o)))
          return !1;
        break;
      case "map":
        if (!(i.V.kind == "message" ? Ne(i.V.T(), j(n), j(o)) : we(i.V.kind == "enum" ? c.INT32 : i.V.T, j(n), j(o))))
          return !1;
        break;
      case "message":
        let l = i.T();
        if (!(i.repeat ? Ne(l, n, o) : l.equals(n, o)))
          return !1;
        break;
    }
  }
  return !0;
}
const j = Object.values;
function Oe(a, e, t) {
  if (e === t)
    return !0;
  if (a !== c.BYTES)
    return !1;
  let i = e, r = t;
  if (i.length !== r.length)
    return !1;
  for (let n = 0; n < i.length; n++)
    if (i[n] != r[n])
      return !1;
  return !0;
}
function we(a, e, t) {
  if (e.length !== t.length)
    return !1;
  for (let i = 0; i < e.length; i++)
    if (!Oe(a, e[i], t[i]))
      return !1;
  return !0;
}
function Ne(a, e, t) {
  if (e.length !== t.length)
    return !1;
  for (let i = 0; i < e.length; i++)
    if (!a.equals(e[i], t[i]))
      return !1;
  return !0;
}
const Le = Object.getOwnPropertyDescriptors(Object.getPrototypeOf({})), st = Le[Ee] = {};
class T {
  constructor(e, t, i) {
    this.defaultCheckDepth = 16, this.typeName = e, this.fields = t.map(Qe), this.options = i ?? {}, st.value = this, this.messagePrototype = Object.create(null, Le), this.refTypeCheck = new ve(this), this.refJsonReader = new et(this), this.refJsonWriter = new tt(this), this.refBinReader = new nt(this), this.refBinWriter = new it(this);
  }
  create(e) {
    let t = rt(this);
    return e !== void 0 && m(this, t, e), t;
  }
  /**
   * Clone the message.
   *
   * Unknown fields are discarded.
   */
  clone(e) {
    let t = this.create();
    return m(this, t, e), t;
  }
  /**
   * Determines whether two message of the same type have the same field values.
   * Checks for deep equality, traversing repeated fields, oneof groups, maps
   * and messages recursively.
   * Will also return true if both messages are `undefined`.
   */
  equals(e, t) {
    return at(this, e, t);
  }
  /**
   * Is the given value assignable to our message type
   * and contains no [excess properties](https://www.typescriptlang.org/docs/handbook/interfaces.html#excess-property-checks)?
   */
  is(e, t = this.defaultCheckDepth) {
    return this.refTypeCheck.is(e, t, !1);
  }
  /**
   * Is the given value assignable to our message type,
   * regardless of [excess properties](https://www.typescriptlang.org/docs/handbook/interfaces.html#excess-property-checks)?
   */
  isAssignable(e, t = this.defaultCheckDepth) {
    return this.refTypeCheck.is(e, t, !0);
  }
  /**
   * Copy partial data into the target message.
   */
  mergePartial(e, t) {
    m(this, e, t);
  }
  /**
   * Create a new message from binary format.
   */
  fromBinary(e, t) {
    let i = _e(t);
    return this.internalBinaryRead(i.readerFactory(e), e.byteLength, i);
  }
  /**
   * Read a new message from a JSON value.
   */
  fromJson(e, t) {
    return this.internalJsonRead(e, qe(t));
  }
  /**
   * Read a new message from a JSON string.
   * This is equivalent to `T.fromJson(JSON.parse(json))`.
   */
  fromJsonString(e, t) {
    let i = JSON.parse(e);
    return this.fromJson(i, t);
  }
  /**
   * Write the message to canonical JSON value.
   */
  toJson(e, t) {
    return this.internalJsonWrite(e, ze(t));
  }
  /**
   * Convert the message to canonical JSON string.
   * This is equivalent to `JSON.stringify(T.toJson(t))`
   */
  toJsonString(e, t) {
    var i;
    let r = this.toJson(e, t);
    return JSON.stringify(r, null, (i = t?.prettySpaces) !== null && i !== void 0 ? i : 0);
  }
  /**
   * Write the message to binary format.
   */
  toBinary(e, t) {
    let i = Ze(t);
    return this.internalBinaryWrite(e, i.writerFactory(), i).finish();
  }
  /**
   * This is an internal method. If you just want to read a message from
   * JSON, use `fromJson()` or `fromJsonString()`.
   *
   * Reads JSON value and merges the fields into the target
   * according to protobuf rules. If the target is omitted,
   * a new instance is created first.
   */
  internalJsonRead(e, t, i) {
    if (e !== null && typeof e == "object" && !Array.isArray(e)) {
      let r = i ?? this.create();
      return this.refJsonReader.read(e, r, t), r;
    }
    throw new Error(`Unable to parse message ${this.typeName} from JSON ${Be(e)}.`);
  }
  /**
   * This is an internal method. If you just want to write a message
   * to JSON, use `toJson()` or `toJsonString().
   *
   * Writes JSON value and returns it.
   */
  internalJsonWrite(e, t) {
    return this.refJsonWriter.write(e, t);
  }
  /**
   * This is an internal method. If you just want to write a message
   * in binary format, use `toBinary()`.
   *
   * Serializes the message in binary format and appends it to the given
   * writer. Returns passed writer.
   */
  internalBinaryWrite(e, t, i) {
    return this.refBinWriter.write(e, t, i), t;
  }
  /**
   * This is an internal method. If you just want to read a message from
   * binary data, use `fromBinary()`.
   *
   * Reads data from binary format and merges the fields into
   * the target according to protobuf rules. If the target is
   * omitted, a new instance is created first.
   */
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create();
    return this.refBinReader.read(e, n, i, t), n;
  }
}
var ue;
(function(a) {
  a[a.NULL_VALUE = 0] = "NULL_VALUE";
})(ue || (ue = {}));
class ot extends T {
  constructor() {
    super("shared.Struct", [
      { no: 1, name: "fields", kind: "map", K: 9, V: { kind: "message", T: () => B } }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.fields = {}, e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* map<string, shared.Value> fields */
        1:
          this.binaryReadMap1(n.fields, e, i);
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  binaryReadMap1(e, t, i) {
    let r = t.uint32(), n = t.pos + r, o, s;
    for (; t.pos < n; ) {
      let [l, f] = t.tag();
      switch (l) {
        case 1:
          o = t.string();
          break;
        case 2:
          s = B.internalBinaryRead(t, t.uint32(), i);
          break;
        default:
          throw new globalThis.Error("unknown map entry field for shared.Struct.fields");
      }
    }
    e[o ?? ""] = s ?? B.create();
  }
  internalBinaryWrite(e, t, i) {
    for (let n of globalThis.Object.keys(e.fields))
      t.tag(1, u.LengthDelimited).fork().tag(1, u.LengthDelimited).string(n), t.tag(2, u.LengthDelimited).fork(), B.internalBinaryWrite(e.fields[n], t, i), t.join().join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const G = new ot();
class lt extends T {
  constructor() {
    super("shared.Value", [
      { no: 1, name: "null_value", kind: "enum", oneof: "kind", T: () => ["shared.NullValue", ue] },
      {
        no: 2,
        name: "number_value",
        kind: "scalar",
        oneof: "kind",
        T: 1
        /*ScalarType.DOUBLE*/
      },
      {
        no: 3,
        name: "string_value",
        kind: "scalar",
        oneof: "kind",
        T: 9
        /*ScalarType.STRING*/
      },
      {
        no: 4,
        name: "bool_value",
        kind: "scalar",
        oneof: "kind",
        T: 8
        /*ScalarType.BOOL*/
      },
      { no: 5, name: "struct_value", kind: "message", oneof: "kind", T: () => G },
      { no: 6, name: "list_value", kind: "message", oneof: "kind", T: () => Z }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.kind = { oneofKind: void 0 }, e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* shared.NullValue null_value */
        1:
          n.kind = {
            oneofKind: "nullValue",
            nullValue: e.int32()
          };
          break;
        case /* double number_value */
        2:
          n.kind = {
            oneofKind: "numberValue",
            numberValue: e.double()
          };
          break;
        case /* string string_value */
        3:
          n.kind = {
            oneofKind: "stringValue",
            stringValue: e.string()
          };
          break;
        case /* bool bool_value */
        4:
          n.kind = {
            oneofKind: "boolValue",
            boolValue: e.bool()
          };
          break;
        case /* shared.Struct struct_value */
        5:
          n.kind = {
            oneofKind: "structValue",
            structValue: G.internalBinaryRead(e, e.uint32(), i, n.kind.structValue)
          };
          break;
        case /* shared.ListValue list_value */
        6:
          n.kind = {
            oneofKind: "listValue",
            listValue: Z.internalBinaryRead(e, e.uint32(), i, n.kind.listValue)
          };
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.kind.oneofKind === "nullValue" && t.tag(1, u.Varint).int32(e.kind.nullValue), e.kind.oneofKind === "numberValue" && t.tag(2, u.Bit64).double(e.kind.numberValue), e.kind.oneofKind === "stringValue" && t.tag(3, u.LengthDelimited).string(e.kind.stringValue), e.kind.oneofKind === "boolValue" && t.tag(4, u.Varint).bool(e.kind.boolValue), e.kind.oneofKind === "structValue" && G.internalBinaryWrite(e.kind.structValue, t.tag(5, u.LengthDelimited).fork(), i).join(), e.kind.oneofKind === "listValue" && Z.internalBinaryWrite(e.kind.listValue, t.tag(6, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const B = new lt();
class ft extends T {
  constructor() {
    super("shared.ListValue", [
      { no: 1, name: "values", kind: "message", repeat: 2, T: () => B }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.values = [], e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* repeated shared.Value values */
        1:
          n.values.push(B.internalBinaryRead(e, e.uint32(), i));
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    for (let n = 0; n < e.values.length; n++)
      B.internalBinaryWrite(e.values[n], t.tag(1, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const Z = new ft();
class ut extends T {
  constructor() {
    super("shared.Translation", [
      {
        no: 1,
        name: "code",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      {
        no: 2,
        name: "content",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.code = "", t.content = "", e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string code */
        1:
          n.code = e.string();
          break;
        case /* string content */
        2:
          n.content = e.string();
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.code !== "" && t.tag(1, u.LengthDelimited).string(e.code), e.content !== "" && t.tag(2, u.LengthDelimited).string(e.content);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const d = new ut();
class ct extends T {
  constructor() {
    super("shared.FlowType", [
      {
        no: 1,
        name: "identifier",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 2, name: "settings", kind: "message", repeat: 2, T: () => Y },
      {
        no: 3,
        name: "input_type_identifier",
        kind: "scalar",
        opt: !0,
        T: 9
        /*ScalarType.STRING*/
      },
      {
        no: 4,
        name: "return_type_identifier",
        kind: "scalar",
        opt: !0,
        T: 9
        /*ScalarType.STRING*/
      },
      {
        no: 5,
        name: "editable",
        kind: "scalar",
        T: 8
        /*ScalarType.BOOL*/
      },
      { no: 6, name: "name", kind: "message", repeat: 2, T: () => d },
      { no: 7, name: "description", kind: "message", repeat: 2, T: () => d },
      { no: 8, name: "documentation", kind: "message", repeat: 2, T: () => d },
      { no: 9, name: "display_message", kind: "message", repeat: 2, T: () => d },
      { no: 10, name: "alias", kind: "message", repeat: 2, T: () => d },
      {
        no: 11,
        name: "version",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.identifier = "", t.settings = [], t.editable = !1, t.name = [], t.description = [], t.documentation = [], t.displayMessage = [], t.alias = [], t.version = "", e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string identifier */
        1:
          n.identifier = e.string();
          break;
        case /* repeated shared.FlowTypeSetting settings */
        2:
          n.settings.push(Y.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* optional string input_type_identifier */
        3:
          n.inputTypeIdentifier = e.string();
          break;
        case /* optional string return_type_identifier */
        4:
          n.returnTypeIdentifier = e.string();
          break;
        case /* bool editable */
        5:
          n.editable = e.bool();
          break;
        case /* repeated shared.Translation name */
        6:
          n.name.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation description */
        7:
          n.description.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation documentation */
        8:
          n.documentation.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation display_message */
        9:
          n.displayMessage.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation alias */
        10:
          n.alias.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* string version */
        11:
          n.version = e.string();
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.identifier !== "" && t.tag(1, u.LengthDelimited).string(e.identifier);
    for (let n = 0; n < e.settings.length; n++)
      Y.internalBinaryWrite(e.settings[n], t.tag(2, u.LengthDelimited).fork(), i).join();
    e.inputTypeIdentifier !== void 0 && t.tag(3, u.LengthDelimited).string(e.inputTypeIdentifier), e.returnTypeIdentifier !== void 0 && t.tag(4, u.LengthDelimited).string(e.returnTypeIdentifier), e.editable !== !1 && t.tag(5, u.Varint).bool(e.editable);
    for (let n = 0; n < e.name.length; n++)
      d.internalBinaryWrite(e.name[n], t.tag(6, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.description.length; n++)
      d.internalBinaryWrite(e.description[n], t.tag(7, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.documentation.length; n++)
      d.internalBinaryWrite(e.documentation[n], t.tag(8, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.displayMessage.length; n++)
      d.internalBinaryWrite(e.displayMessage[n], t.tag(9, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.alias.length; n++)
      d.internalBinaryWrite(e.alias[n], t.tag(10, u.LengthDelimited).fork(), i).join();
    e.version !== "" && t.tag(11, u.LengthDelimited).string(e.version);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const ht = new ct();
class dt extends T {
  constructor() {
    super("shared.FlowTypeSetting", [
      {
        no: 1,
        name: "identifier",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      {
        no: 2,
        name: "unique",
        kind: "scalar",
        T: 8
        /*ScalarType.BOOL*/
      },
      {
        no: 3,
        name: "data_type_identifier",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 4, name: "default_value", kind: "message", T: () => B },
      { no: 5, name: "name", kind: "message", repeat: 2, T: () => d },
      { no: 6, name: "description", kind: "message", repeat: 2, T: () => d }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.identifier = "", t.unique = !1, t.dataTypeIdentifier = "", t.name = [], t.description = [], e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string identifier */
        1:
          n.identifier = e.string();
          break;
        case /* bool unique */
        2:
          n.unique = e.bool();
          break;
        case /* string data_type_identifier */
        3:
          n.dataTypeIdentifier = e.string();
          break;
        case /* optional shared.Value default_value */
        4:
          n.defaultValue = B.internalBinaryRead(e, e.uint32(), i, n.defaultValue);
          break;
        case /* repeated shared.Translation name */
        5:
          n.name.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation description */
        6:
          n.description.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.identifier !== "" && t.tag(1, u.LengthDelimited).string(e.identifier), e.unique !== !1 && t.tag(2, u.Varint).bool(e.unique), e.dataTypeIdentifier !== "" && t.tag(3, u.LengthDelimited).string(e.dataTypeIdentifier), e.defaultValue && B.internalBinaryWrite(e.defaultValue, t.tag(4, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.name.length; n++)
      d.internalBinaryWrite(e.name[n], t.tag(5, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.description.length; n++)
      d.internalBinaryWrite(e.description[n], t.tag(6, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const Y = new dt();
var ce;
(function(a) {
  a[a.UNKNOWN = 0] = "UNKNOWN", a[a.PRIMITIVE = 1] = "PRIMITIVE", a[a.TYPE = 2] = "TYPE", a[a.OBJECT = 3] = "OBJECT", a[a.DATATYPE = 4] = "DATATYPE", a[a.ARRAY = 5] = "ARRAY", a[a.ERROR = 6] = "ERROR", a[a.NODE = 7] = "NODE";
})(ce || (ce = {}));
var he;
(function(a) {
  a[a.UNKNOWN = 0] = "UNKNOWN", a[a.AND = 1] = "AND", a[a.OR = 2] = "OR";
})(he || (he = {}));
class yt extends T {
  constructor() {
    super("shared.DefinitionDataType", [
      { no: 1, name: "variant", kind: "enum", T: () => ["shared.DefinitionDataType.Variant", ce] },
      {
        no: 2,
        name: "identifier",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 3, name: "name", kind: "message", repeat: 2, T: () => d },
      { no: 4, name: "display_message", kind: "message", repeat: 2, T: () => d },
      { no: 5, name: "alias", kind: "message", repeat: 2, T: () => d },
      { no: 6, name: "rules", kind: "message", repeat: 2, T: () => q },
      {
        no: 7,
        name: "generic_keys",
        kind: "scalar",
        repeat: 2,
        T: 9
        /*ScalarType.STRING*/
      },
      {
        no: 8,
        name: "version",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.variant = 0, t.identifier = "", t.name = [], t.displayMessage = [], t.alias = [], t.rules = [], t.genericKeys = [], t.version = "", e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* shared.DefinitionDataType.Variant variant */
        1:
          n.variant = e.int32();
          break;
        case /* string identifier */
        2:
          n.identifier = e.string();
          break;
        case /* repeated shared.Translation name */
        3:
          n.name.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation display_message */
        4:
          n.displayMessage.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation alias */
        5:
          n.alias.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.DefinitionDataTypeRule rules */
        6:
          n.rules.push(q.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated string generic_keys */
        7:
          n.genericKeys.push(e.string());
          break;
        case /* string version */
        8:
          n.version = e.string();
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.variant !== 0 && t.tag(1, u.Varint).int32(e.variant), e.identifier !== "" && t.tag(2, u.LengthDelimited).string(e.identifier);
    for (let n = 0; n < e.name.length; n++)
      d.internalBinaryWrite(e.name[n], t.tag(3, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.displayMessage.length; n++)
      d.internalBinaryWrite(e.displayMessage[n], t.tag(4, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.alias.length; n++)
      d.internalBinaryWrite(e.alias[n], t.tag(5, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.rules.length; n++)
      q.internalBinaryWrite(e.rules[n], t.tag(6, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.genericKeys.length; n++)
      t.tag(7, u.LengthDelimited).string(e.genericKeys[n]);
    e.version !== "" && t.tag(8, u.LengthDelimited).string(e.version);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const pt = new yt();
class gt extends T {
  constructor() {
    super("shared.ExecutionDataType", [
      {
        no: 1,
        name: "identifier",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 2, name: "rules", kind: "message", repeat: 2, T: () => z }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.identifier = "", t.rules = [], e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string identifier */
        1:
          n.identifier = e.string();
          break;
        case /* repeated shared.ExecutionDataTypeRule rules */
        2:
          n.rules.push(z.internalBinaryRead(e, e.uint32(), i));
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.identifier !== "" && t.tag(1, u.LengthDelimited).string(e.identifier);
    for (let n = 0; n < e.rules.length; n++)
      z.internalBinaryWrite(e.rules[n], t.tag(2, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
new gt();
class mt extends T {
  constructor() {
    super("shared.DefinitionDataTypeRule", [
      { no: 1, name: "contains_key", kind: "message", oneof: "config", T: () => H },
      { no: 2, name: "contains_type", kind: "message", oneof: "config", T: () => ee },
      { no: 3, name: "item_of_collection", kind: "message", oneof: "config", T: () => L },
      { no: 4, name: "number_range", kind: "message", oneof: "config", T: () => U },
      { no: 5, name: "regex", kind: "message", oneof: "config", T: () => $ },
      { no: 6, name: "input_types", kind: "message", oneof: "config", T: () => ne },
      { no: 7, name: "return_type", kind: "message", oneof: "config", T: () => re },
      { no: 8, name: "parent_type", kind: "message", oneof: "config", T: () => Q }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.config = { oneofKind: void 0 }, e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* shared.DefinitionDataTypeContainsKeyRuleConfig contains_key */
        1:
          n.config = {
            oneofKind: "containsKey",
            containsKey: H.internalBinaryRead(e, e.uint32(), i, n.config.containsKey)
          };
          break;
        case /* shared.DefinitionDataTypeContainsTypeRuleConfig contains_type */
        2:
          n.config = {
            oneofKind: "containsType",
            containsType: ee.internalBinaryRead(e, e.uint32(), i, n.config.containsType)
          };
          break;
        case /* shared.DataTypeItemOfCollectionRuleConfig item_of_collection */
        3:
          n.config = {
            oneofKind: "itemOfCollection",
            itemOfCollection: L.internalBinaryRead(e, e.uint32(), i, n.config.itemOfCollection)
          };
          break;
        case /* shared.DataTypeNumberRangeRuleConfig number_range */
        4:
          n.config = {
            oneofKind: "numberRange",
            numberRange: U.internalBinaryRead(e, e.uint32(), i, n.config.numberRange)
          };
          break;
        case /* shared.DataTypeRegexRuleConfig regex */
        5:
          n.config = {
            oneofKind: "regex",
            regex: $.internalBinaryRead(e, e.uint32(), i, n.config.regex)
          };
          break;
        case /* shared.DefinitionDataTypeInputTypesRuleConfig input_types */
        6:
          n.config = {
            oneofKind: "inputTypes",
            inputTypes: ne.internalBinaryRead(e, e.uint32(), i, n.config.inputTypes)
          };
          break;
        case /* shared.DefinitionDataTypeReturnTypeRuleConfig return_type */
        7:
          n.config = {
            oneofKind: "returnType",
            returnType: re.internalBinaryRead(e, e.uint32(), i, n.config.returnType)
          };
          break;
        case /* shared.DefinitionDataTypeParentTypeRuleConfig parent_type */
        8:
          n.config = {
            oneofKind: "parentType",
            parentType: Q.internalBinaryRead(e, e.uint32(), i, n.config.parentType)
          };
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.config.oneofKind === "containsKey" && H.internalBinaryWrite(e.config.containsKey, t.tag(1, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "containsType" && ee.internalBinaryWrite(e.config.containsType, t.tag(2, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "itemOfCollection" && L.internalBinaryWrite(e.config.itemOfCollection, t.tag(3, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "numberRange" && U.internalBinaryWrite(e.config.numberRange, t.tag(4, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "regex" && $.internalBinaryWrite(e.config.regex, t.tag(5, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "inputTypes" && ne.internalBinaryWrite(e.config.inputTypes, t.tag(6, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "returnType" && re.internalBinaryWrite(e.config.returnType, t.tag(7, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "parentType" && Q.internalBinaryWrite(e.config.parentType, t.tag(8, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const q = new mt();
class kt extends T {
  constructor() {
    super("shared.ExecutionDataTypeRule", [
      { no: 1, name: "contains_key", kind: "message", oneof: "config", T: () => v },
      { no: 2, name: "contains_type", kind: "message", oneof: "config", T: () => te },
      { no: 3, name: "item_of_collection", kind: "message", oneof: "config", T: () => L },
      { no: 4, name: "number_range", kind: "message", oneof: "config", T: () => U },
      { no: 5, name: "regex", kind: "message", oneof: "config", T: () => $ }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.config = { oneofKind: void 0 }, e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* shared.ExecutionDataTypeContainsKeyRuleConfig contains_key */
        1:
          n.config = {
            oneofKind: "containsKey",
            containsKey: v.internalBinaryRead(e, e.uint32(), i, n.config.containsKey)
          };
          break;
        case /* shared.ExecutionDataTypeContainsTypeRuleConfig contains_type */
        2:
          n.config = {
            oneofKind: "containsType",
            containsType: te.internalBinaryRead(e, e.uint32(), i, n.config.containsType)
          };
          break;
        case /* shared.DataTypeItemOfCollectionRuleConfig item_of_collection */
        3:
          n.config = {
            oneofKind: "itemOfCollection",
            itemOfCollection: L.internalBinaryRead(e, e.uint32(), i, n.config.itemOfCollection)
          };
          break;
        case /* shared.DataTypeNumberRangeRuleConfig number_range */
        4:
          n.config = {
            oneofKind: "numberRange",
            numberRange: U.internalBinaryRead(e, e.uint32(), i, n.config.numberRange)
          };
          break;
        case /* shared.DataTypeRegexRuleConfig regex */
        5:
          n.config = {
            oneofKind: "regex",
            regex: $.internalBinaryRead(e, e.uint32(), i, n.config.regex)
          };
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.config.oneofKind === "containsKey" && v.internalBinaryWrite(e.config.containsKey, t.tag(1, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "containsType" && te.internalBinaryWrite(e.config.containsType, t.tag(2, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "itemOfCollection" && L.internalBinaryWrite(e.config.itemOfCollection, t.tag(3, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "numberRange" && U.internalBinaryWrite(e.config.numberRange, t.tag(4, u.LengthDelimited).fork(), i).join(), e.config.oneofKind === "regex" && $.internalBinaryWrite(e.config.regex, t.tag(5, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const z = new kt();
class bt extends T {
  constructor() {
    super("shared.DefinitionDataTypeParentTypeRuleConfig", [
      { no: 1, name: "parent_type", kind: "message", T: () => w }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* shared.DataTypeIdentifier parent_type */
        1:
          n.parentType = w.internalBinaryRead(e, e.uint32(), i, n.parentType);
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.parentType && w.internalBinaryWrite(e.parentType, t.tag(1, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const Q = new bt();
class Tt extends T {
  constructor() {
    super("shared.DefinitionDataTypeContainsKeyRuleConfig", [
      {
        no: 1,
        name: "key",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 2, name: "data_type_identifier", kind: "message", T: () => w }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.key = "", e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string key */
        1:
          n.key = e.string();
          break;
        case /* shared.DataTypeIdentifier data_type_identifier */
        2:
          n.dataTypeIdentifier = w.internalBinaryRead(e, e.uint32(), i, n.dataTypeIdentifier);
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.key !== "" && t.tag(1, u.LengthDelimited).string(e.key), e.dataTypeIdentifier && w.internalBinaryWrite(e.dataTypeIdentifier, t.tag(2, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const H = new Tt();
class wt extends T {
  constructor() {
    super("shared.ExecutionDataTypeContainsKeyRuleConfig", [
      {
        no: 1,
        name: "key",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      {
        no: 2,
        name: "data_type_identifier",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.key = "", t.dataTypeIdentifier = "", e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string key */
        1:
          n.key = e.string();
          break;
        case /* string data_type_identifier */
        2:
          n.dataTypeIdentifier = e.string();
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.key !== "" && t.tag(1, u.LengthDelimited).string(e.key), e.dataTypeIdentifier !== "" && t.tag(2, u.LengthDelimited).string(e.dataTypeIdentifier);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const v = new wt();
class Nt extends T {
  constructor() {
    super("shared.DefinitionDataTypeContainsTypeRuleConfig", [
      { no: 1, name: "data_type_identifier", kind: "message", T: () => w }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* shared.DataTypeIdentifier data_type_identifier */
        1:
          n.dataTypeIdentifier = w.internalBinaryRead(e, e.uint32(), i, n.dataTypeIdentifier);
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.dataTypeIdentifier && w.internalBinaryWrite(e.dataTypeIdentifier, t.tag(1, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const ee = new Nt();
class Bt extends T {
  constructor() {
    super("shared.ExecutionDataTypeContainsTypeRuleConfig", [
      {
        no: 1,
        name: "data_type_identifier",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.dataTypeIdentifier = "", e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string data_type_identifier */
        1:
          n.dataTypeIdentifier = e.string();
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.dataTypeIdentifier !== "" && t.tag(1, u.LengthDelimited).string(e.dataTypeIdentifier);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const te = new Bt();
class It extends T {
  constructor() {
    super("shared.DataTypeItemOfCollectionRuleConfig", [
      { no: 1, name: "items", kind: "message", repeat: 2, T: () => B }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.items = [], e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* repeated shared.Value items */
        1:
          n.items.push(B.internalBinaryRead(e, e.uint32(), i));
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    for (let n = 0; n < e.items.length; n++)
      B.internalBinaryWrite(e.items[n], t.tag(1, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const L = new It();
class Rt extends T {
  constructor() {
    super("shared.DataTypeNumberRangeRuleConfig", [
      {
        no: 1,
        name: "from",
        kind: "scalar",
        T: 3,
        L: 0
        /*LongType.BIGINT*/
      },
      {
        no: 2,
        name: "to",
        kind: "scalar",
        T: 3,
        L: 0
        /*LongType.BIGINT*/
      },
      {
        no: 3,
        name: "steps",
        kind: "scalar",
        opt: !0,
        T: 3,
        L: 0
        /*LongType.BIGINT*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.from = 0n, t.to = 0n, e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* int64 from */
        1:
          n.from = e.int64().toBigInt();
          break;
        case /* int64 to */
        2:
          n.to = e.int64().toBigInt();
          break;
        case /* optional int64 steps */
        3:
          n.steps = e.int64().toBigInt();
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.from !== 0n && t.tag(1, u.Varint).int64(e.from), e.to !== 0n && t.tag(2, u.Varint).int64(e.to), e.steps !== void 0 && t.tag(3, u.Varint).int64(e.steps);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const U = new Rt();
class Dt extends T {
  constructor() {
    super("shared.DataTypeRegexRuleConfig", [
      {
        no: 1,
        name: "pattern",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.pattern = "", e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string pattern */
        1:
          n.pattern = e.string();
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.pattern !== "" && t.tag(1, u.LengthDelimited).string(e.pattern);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const $ = new Dt();
class Ft extends T {
  constructor() {
    super("shared.DefinitionDataTypeInputTypesRuleConfig", [
      { no: 1, name: "input_types", kind: "message", repeat: 2, T: () => ie }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.inputTypes = [], e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* repeated shared.DefinitionDataTypeInputTypesRuleConfig.DataTypeInputType input_types */
        1:
          n.inputTypes.push(ie.internalBinaryRead(e, e.uint32(), i));
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    for (let n = 0; n < e.inputTypes.length; n++)
      ie.internalBinaryWrite(e.inputTypes[n], t.tag(1, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const ne = new Ft();
class Et extends T {
  constructor() {
    super("shared.DefinitionDataTypeInputTypesRuleConfig.DataTypeInputType", [
      { no: 1, name: "data_type_identifier", kind: "message", T: () => w },
      {
        no: 2,
        name: "input_identifier",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.inputIdentifier = "", e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* shared.DataTypeIdentifier data_type_identifier */
        1:
          n.dataTypeIdentifier = w.internalBinaryRead(e, e.uint32(), i, n.dataTypeIdentifier);
          break;
        case /* string input_identifier */
        2:
          n.inputIdentifier = e.string();
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.dataTypeIdentifier && w.internalBinaryWrite(e.dataTypeIdentifier, t.tag(1, u.LengthDelimited).fork(), i).join(), e.inputIdentifier !== "" && t.tag(2, u.LengthDelimited).string(e.inputIdentifier);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const ie = new Et();
class Ot extends T {
  constructor() {
    super("shared.DefinitionDataTypeReturnTypeRuleConfig", [
      { no: 1, name: "data_type_identifier", kind: "message", T: () => w }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* shared.DataTypeIdentifier data_type_identifier */
        1:
          n.dataTypeIdentifier = w.internalBinaryRead(e, e.uint32(), i, n.dataTypeIdentifier);
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.dataTypeIdentifier && w.internalBinaryWrite(e.dataTypeIdentifier, t.tag(1, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const re = new Ot();
class Lt extends T {
  constructor() {
    super("shared.DataTypeIdentifier", [
      {
        no: 1,
        name: "data_type_identifier",
        kind: "scalar",
        oneof: "type",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 2, name: "generic_type", kind: "message", oneof: "type", T: () => ae },
      {
        no: 3,
        name: "generic_key",
        kind: "scalar",
        oneof: "type",
        T: 9
        /*ScalarType.STRING*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.type = { oneofKind: void 0 }, e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string data_type_identifier */
        1:
          n.type = {
            oneofKind: "dataTypeIdentifier",
            dataTypeIdentifier: e.string()
          };
          break;
        case /* shared.GenericType generic_type */
        2:
          n.type = {
            oneofKind: "genericType",
            genericType: ae.internalBinaryRead(e, e.uint32(), i, n.type.genericType)
          };
          break;
        case /* string generic_key */
        3:
          n.type = {
            oneofKind: "genericKey",
            genericKey: e.string()
          };
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.type.oneofKind === "dataTypeIdentifier" && t.tag(1, u.LengthDelimited).string(e.type.dataTypeIdentifier), e.type.oneofKind === "genericType" && ae.internalBinaryWrite(e.type.genericType, t.tag(2, u.LengthDelimited).fork(), i).join(), e.type.oneofKind === "genericKey" && t.tag(3, u.LengthDelimited).string(e.type.genericKey);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const w = new Lt();
class Ut extends T {
  constructor() {
    super("shared.GenericType", [
      {
        no: 1,
        name: "data_type_identifier",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 2, name: "generic_mappers", kind: "message", repeat: 2, T: () => se }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.dataTypeIdentifier = "", t.genericMappers = [], e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string data_type_identifier */
        1:
          n.dataTypeIdentifier = e.string();
          break;
        case /* repeated shared.GenericMapper generic_mappers */
        2:
          n.genericMappers.push(se.internalBinaryRead(e, e.uint32(), i));
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.dataTypeIdentifier !== "" && t.tag(1, u.LengthDelimited).string(e.dataTypeIdentifier);
    for (let n = 0; n < e.genericMappers.length; n++)
      se.internalBinaryWrite(e.genericMappers[n], t.tag(2, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const ae = new Ut();
class $t extends T {
  constructor() {
    super("shared.GenericMapper", [
      { no: 1, name: "source", kind: "message", repeat: 2, T: () => w },
      {
        no: 2,
        name: "target",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 3, name: "generic_combinations", kind: "enum", repeat: 1, T: () => ["shared.GenericMapper.GenericCombinationStrategy", he] }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.source = [], t.target = "", t.genericCombinations = [], e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* repeated shared.DataTypeIdentifier source */
        1:
          n.source.push(w.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* string target */
        2:
          n.target = e.string();
          break;
        case /* repeated shared.GenericMapper.GenericCombinationStrategy generic_combinations */
        3:
          if (l === u.LengthDelimited)
            for (let k = e.int32() + e.pos; e.pos < k; )
              n.genericCombinations.push(e.int32());
          else
            n.genericCombinations.push(e.int32());
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    for (let n = 0; n < e.source.length; n++)
      w.internalBinaryWrite(e.source[n], t.tag(1, u.LengthDelimited).fork(), i).join();
    if (e.target !== "" && t.tag(2, u.LengthDelimited).string(e.target), e.genericCombinations.length) {
      t.tag(3, u.LengthDelimited).fork();
      for (let n = 0; n < e.genericCombinations.length; n++)
        t.int32(e.genericCombinations[n]);
      t.join();
    }
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const se = new $t();
class xt extends T {
  constructor() {
    super("shared.RuntimeFunctionDefinition", [
      {
        no: 1,
        name: "runtime_name",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 2, name: "runtime_parameter_definitions", kind: "message", repeat: 2, T: () => oe },
      { no: 3, name: "return_type_identifier", kind: "message", T: () => w },
      {
        no: 4,
        name: "throws_error",
        kind: "scalar",
        T: 8
        /*ScalarType.BOOL*/
      },
      {
        no: 5,
        name: "generic_keys",
        kind: "scalar",
        repeat: 2,
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 6, name: "name", kind: "message", repeat: 2, T: () => d },
      { no: 7, name: "description", kind: "message", repeat: 2, T: () => d },
      { no: 8, name: "documentation", kind: "message", repeat: 2, T: () => d },
      { no: 9, name: "deprecation_message", kind: "message", repeat: 2, T: () => d },
      { no: 10, name: "display_message", kind: "message", repeat: 2, T: () => d },
      { no: 11, name: "alias", kind: "message", repeat: 2, T: () => d },
      {
        no: 12,
        name: "version",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.runtimeName = "", t.runtimeParameterDefinitions = [], t.throwsError = !1, t.genericKeys = [], t.name = [], t.description = [], t.documentation = [], t.deprecationMessage = [], t.displayMessage = [], t.alias = [], t.version = "", e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* string runtime_name */
        1:
          n.runtimeName = e.string();
          break;
        case /* repeated shared.RuntimeParameterDefinition runtime_parameter_definitions */
        2:
          n.runtimeParameterDefinitions.push(oe.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* optional shared.DataTypeIdentifier return_type_identifier */
        3:
          n.returnTypeIdentifier = w.internalBinaryRead(e, e.uint32(), i, n.returnTypeIdentifier);
          break;
        case /* bool throws_error */
        4:
          n.throwsError = e.bool();
          break;
        case /* repeated string generic_keys */
        5:
          n.genericKeys.push(e.string());
          break;
        case /* repeated shared.Translation name */
        6:
          n.name.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation description */
        7:
          n.description.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation documentation */
        8:
          n.documentation.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation deprecation_message */
        9:
          n.deprecationMessage.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation display_message */
        10:
          n.displayMessage.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation alias */
        11:
          n.alias.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* string version */
        12:
          n.version = e.string();
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.runtimeName !== "" && t.tag(1, u.LengthDelimited).string(e.runtimeName);
    for (let n = 0; n < e.runtimeParameterDefinitions.length; n++)
      oe.internalBinaryWrite(e.runtimeParameterDefinitions[n], t.tag(2, u.LengthDelimited).fork(), i).join();
    e.returnTypeIdentifier && w.internalBinaryWrite(e.returnTypeIdentifier, t.tag(3, u.LengthDelimited).fork(), i).join(), e.throwsError !== !1 && t.tag(4, u.Varint).bool(e.throwsError);
    for (let n = 0; n < e.genericKeys.length; n++)
      t.tag(5, u.LengthDelimited).string(e.genericKeys[n]);
    for (let n = 0; n < e.name.length; n++)
      d.internalBinaryWrite(e.name[n], t.tag(6, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.description.length; n++)
      d.internalBinaryWrite(e.description[n], t.tag(7, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.documentation.length; n++)
      d.internalBinaryWrite(e.documentation[n], t.tag(8, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.deprecationMessage.length; n++)
      d.internalBinaryWrite(e.deprecationMessage[n], t.tag(9, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.displayMessage.length; n++)
      d.internalBinaryWrite(e.displayMessage[n], t.tag(10, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.alias.length; n++)
      d.internalBinaryWrite(e.alias[n], t.tag(11, u.LengthDelimited).fork(), i).join();
    e.version !== "" && t.tag(12, u.LengthDelimited).string(e.version);
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const Vt = new xt();
class Wt extends T {
  constructor() {
    super("shared.RuntimeParameterDefinition", [
      { no: 1, name: "data_type_identifier", kind: "message", T: () => w },
      {
        no: 2,
        name: "runtime_name",
        kind: "scalar",
        T: 9
        /*ScalarType.STRING*/
      },
      { no: 3, name: "default_value", kind: "message", T: () => B },
      { no: 4, name: "name", kind: "message", repeat: 2, T: () => d },
      { no: 5, name: "description", kind: "message", repeat: 2, T: () => d },
      { no: 6, name: "documentation", kind: "message", repeat: 2, T: () => d }
    ]);
  }
  create(e) {
    const t = globalThis.Object.create(this.messagePrototype);
    return t.runtimeName = "", t.name = [], t.description = [], t.documentation = [], e !== void 0 && m(this, t, e), t;
  }
  internalBinaryRead(e, t, i, r) {
    let n = r ?? this.create(), o = e.pos + t;
    for (; e.pos < o; ) {
      let [s, l] = e.tag();
      switch (s) {
        case /* shared.DataTypeIdentifier data_type_identifier */
        1:
          n.dataTypeIdentifier = w.internalBinaryRead(e, e.uint32(), i, n.dataTypeIdentifier);
          break;
        case /* string runtime_name */
        2:
          n.runtimeName = e.string();
          break;
        case /* optional shared.Value default_value */
        3:
          n.defaultValue = B.internalBinaryRead(e, e.uint32(), i, n.defaultValue);
          break;
        case /* repeated shared.Translation name */
        4:
          n.name.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation description */
        5:
          n.description.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        case /* repeated shared.Translation documentation */
        6:
          n.documentation.push(d.internalBinaryRead(e, e.uint32(), i));
          break;
        default:
          let f = i.readUnknownField;
          if (f === "throw")
            throw new globalThis.Error(`Unknown field ${s} (wire type ${l}) for ${this.typeName}`);
          let h = e.skip(l);
          f !== !1 && (f === !0 ? y.onRead : f)(this.typeName, n, s, l, h);
      }
    }
    return n;
  }
  internalBinaryWrite(e, t, i) {
    e.dataTypeIdentifier && w.internalBinaryWrite(e.dataTypeIdentifier, t.tag(1, u.LengthDelimited).fork(), i).join(), e.runtimeName !== "" && t.tag(2, u.LengthDelimited).string(e.runtimeName), e.defaultValue && B.internalBinaryWrite(e.defaultValue, t.tag(3, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.name.length; n++)
      d.internalBinaryWrite(e.name[n], t.tag(4, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.description.length; n++)
      d.internalBinaryWrite(e.description[n], t.tag(5, u.LengthDelimited).fork(), i).join();
    for (let n = 0; n < e.documentation.length; n++)
      d.internalBinaryWrite(e.documentation[n], t.tag(6, u.LengthDelimited).fork(), i).join();
    let r = i.writeUnknownFields;
    return r !== !1 && (r == !0 ? y.onWrite : r)(this.typeName, e, t), t;
  }
}
const oe = new Wt();
var Kt = /* @__PURE__ */ ((a) => (a.FlowType = "FlowType", a.DataType = "DataType", a.RuntimeFunction = "RuntimeFunction", a))(Kt || {});
class Xt {
  static async fromPath(e) {
    const t = /* @__PURE__ */ new Map();
    for (const i of await P(e)) {
      if (!i.isDirectory()) continue;
      const r = i.name, n = x(e, r);
      for (const o of await P(n)) {
        const s = jt(o.name);
        if (!s) continue;
        const l = x(n, o.name), f = await Ct(l);
        for (const h of f) {
          const k = await $e(h, "utf8"), p = t.get(r) ?? At(r);
          _t(p, k, s), t.set(r, p);
        }
      }
    }
    return Array.from(t.values());
  }
}
const jt = (a) => ({
  flow_type: "FlowType",
  data_type: "DataType",
  runtime_definition: "RuntimeFunction"
  /* RuntimeFunction */
})[a] ?? null, At = (a) => ({
  name: a,
  data_types: [],
  flow_types: [],
  runtime_functions: [],
  errors: []
}), P = async (a) => {
  try {
    return await xe(a, { withFileTypes: !0 });
  } catch {
    return [];
  }
}, Ct = async (a) => {
  const e = await P(a), t = e.filter((i) => i.isFile() && ye(i.name) === ".json").map((i) => x(a, i.name));
  for (const i of e.filter((r) => r.isDirectory())) {
    const r = (await P(x(a, i.name))).filter((n) => n.isFile() && ye(n.name) === ".json").map((n) => x(a, i.name, n.name));
    t.push(...r);
  }
  return t;
}, _t = (a, e, t) => {
  try {
    t === "DataType" ? a.data_types.push(pt.fromJsonString(e)) : t === "FlowType" ? a.flow_types.push(ht.fromJsonString(e)) : a.runtime_functions.push(Vt.fromJsonString(e));
  } catch (i) {
    a.errors.push({
      definition: St(e, t),
      definition_type: t,
      error: i instanceof Error ? i.message : String(i)
    });
  }
}, St = (a, e) => {
  const t = e === "RuntimeFunction" ? "runtime_name" : "identifier";
  return a.match(new RegExp(`"${t}"\\s*:\\s*"([^"]+)"`))?.[1] ?? a;
};
export {
  Kt as MetaType,
  Xt as Reader
};
