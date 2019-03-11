/* -*- Mode: C++; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 2 -*-
 * vim: set ts=8 sts=2 et sw=2 tw=80:
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef js_Id_h
#define js_Id_h

// A jsid is an identifier for a property or method of an object which is
// either a 31-bit unsigned integer, interned string or symbol.
//
// Also, there is an additional jsid value, JSID_VOID, which does not occur in
// JS scripts but may be used to indicate the absence of a valid jsid.  A void
// jsid is not a valid id and only arises as an exceptional API return value,
// such as in JS_NextProperty. Embeddings must not pass JSID_VOID into JSAPI
// entry points expecting a jsid and do not need to handle JSID_VOID in hooks
// receiving a jsid except when explicitly noted in the API contract.
//
// A jsid is not implicitly convertible to or from a Value; JS_ValueToId or
// JS_IdToValue must be used instead.

#include "jstypes.h"

#include "js/HeapAPI.h"
#include "js/RootingAPI.h"
#include "js/TypeDecls.h"
#include "js/Utility.h"

// All jsids with the low bit set are integer ids. This means the other type
// tags must all be even.
#define JSID_TYPE_INT_BIT 0x1

// Use 0 for JSID_TYPE_STRING to avoid a bitwise op for atom <-> id conversions.
#define JSID_TYPE_STRING 0x0
#define JSID_TYPE_VOID 0x2
#define JSID_TYPE_SYMBOL 0x4
#define JSID_TYPE_EMPTY 0x6
#define JSID_TYPE_MASK 0x7

namespace JS {

struct PropertyKey {
  size_t asBits;

  constexpr PropertyKey() : asBits(JSID_TYPE_VOID) {}

  static constexpr MOZ_ALWAYS_INLINE PropertyKey fromRawBits(size_t bits) {
    PropertyKey id;
    id.asBits = bits;
    return id;
  }

  bool operator==(const PropertyKey& rhs) const { return asBits == rhs.asBits; }
  bool operator!=(const PropertyKey& rhs) const { return asBits != rhs.asBits; }

  MOZ_ALWAYS_INLINE bool isInt() const {
    return !!(asBits & JSID_TYPE_INT_BIT);
  }

} JS_HAZ_GC_POINTER;

}  // namespace JS

using jsid = JS::PropertyKey;

#define JSID_BITS(id) (id.asBits)

// Avoid using canonical 'id' for jsid parameters since this is a magic word in
// Objective-C++ which, apparently, wants to be able to #include jsapi.h.
#define id iden

static MOZ_ALWAYS_INLINE bool JSID_IS_STRING(jsid id) {
  return (JSID_BITS(id) & JSID_TYPE_MASK) == JSID_TYPE_STRING;
}

static MOZ_ALWAYS_INLINE JSString* JSID_TO_STRING(jsid id) {
  // Use XOR instead of `& ~JSID_TYPE_MASK` because small immediates can be
  // encoded more efficiently on some platorms.
  MOZ_ASSERT(JSID_IS_STRING(id));
  return (JSString*)(JSID_BITS(id) ^ JSID_TYPE_STRING);
}

/**
 * Only JSStrings that have been interned via the JSAPI can be turned into
 * jsids by API clients.
 *
 * N.B. if a jsid is backed by a string which has not been interned, that
 * string must be appropriately rooted to avoid being collected by the GC.
 */
JS_PUBLIC_API jsid INTERNED_STRING_TO_JSID(JSContext* cx, JSString* str);

static MOZ_ALWAYS_INLINE bool JSID_IS_INT(jsid id) {
  return !!(JSID_BITS(id) & JSID_TYPE_INT_BIT);
}

static MOZ_ALWAYS_INLINE int32_t JSID_TO_INT(jsid id) {
  MOZ_ASSERT(JSID_IS_INT(id));
  MOZ_ASSERT(id.isInt());
  uint32_t bits = static_cast<uint32_t>(JSID_BITS(id)) >> 1;
  return static_cast<int32_t>(bits);
}

#define JSID_INT_MIN 0
#define JSID_INT_MAX INT32_MAX

static MOZ_ALWAYS_INLINE bool INT_FITS_IN_JSID(int32_t i) { return i >= 0; }

static MOZ_ALWAYS_INLINE jsid INT_TO_JSID(int32_t i) {
  jsid id;
  MOZ_ASSERT(INT_FITS_IN_JSID(i));
  uint32_t bits = (static_cast<uint32_t>(i) << 1) | JSID_TYPE_INT_BIT;
  JSID_BITS(id) = static_cast<size_t>(bits);
  return id;
}

static MOZ_ALWAYS_INLINE bool JSID_IS_SYMBOL(jsid id) {
  return (JSID_BITS(id) & JSID_TYPE_MASK) == JSID_TYPE_SYMBOL;
}

static MOZ_ALWAYS_INLINE JS::Symbol* JSID_TO_SYMBOL(jsid id) {
  MOZ_ASSERT(JSID_IS_SYMBOL(id));
  return (JS::Symbol*)(JSID_BITS(id) ^ JSID_TYPE_SYMBOL);
}

static MOZ_ALWAYS_INLINE jsid SYMBOL_TO_JSID(JS::Symbol* sym) {
  jsid id;
  MOZ_ASSERT(sym != nullptr);
  MOZ_ASSERT((size_t(sym) & JSID_TYPE_MASK) == 0);
  MOZ_ASSERT(!js::gc::IsInsideNursery(reinterpret_cast<js::gc::Cell*>(sym)));
  JSID_BITS(id) = (size_t(sym) | JSID_TYPE_SYMBOL);
  return id;
}

static MOZ_ALWAYS_INLINE bool JSID_IS_GCTHING(jsid id) {
  return JSID_IS_STRING(id) || JSID_IS_SYMBOL(id);
}

static MOZ_ALWAYS_INLINE JS::GCCellPtr JSID_TO_GCTHING(jsid id) {
  void* thing = (void*)(JSID_BITS(id) & ~(size_t)JSID_TYPE_MASK);
  if (JSID_IS_STRING(id)) {
    return JS::GCCellPtr(thing, JS::TraceKind::String);
  }
  MOZ_ASSERT(JSID_IS_SYMBOL(id));
  return JS::GCCellPtr(thing, JS::TraceKind::Symbol);
}

static MOZ_ALWAYS_INLINE bool JSID_IS_VOID(const jsid id) {
  MOZ_ASSERT_IF((JSID_BITS(id) & JSID_TYPE_MASK) == JSID_TYPE_VOID,
                JSID_BITS(id) == JSID_TYPE_VOID);
  return JSID_BITS(id) == JSID_TYPE_VOID;
}

static MOZ_ALWAYS_INLINE bool JSID_IS_EMPTY(const jsid id) {
  MOZ_ASSERT_IF((JSID_BITS(id) & JSID_TYPE_MASK) == JSID_TYPE_EMPTY,
                JSID_BITS(id) == JSID_TYPE_EMPTY);
  return JSID_BITS(id) == JSID_TYPE_EMPTY;
}

constexpr const jsid JSID_VOID;
constexpr const jsid JSID_EMPTY = jsid::fromRawBits(JSID_TYPE_EMPTY);

extern JS_PUBLIC_DATA const JS::HandleId JSID_VOIDHANDLE;
extern JS_PUBLIC_DATA const JS::HandleId JSID_EMPTYHANDLE;

namespace JS {

template <>
struct GCPolicy<jsid> {
  static void trace(JSTracer* trc, jsid* idp, const char* name) {
    js::UnsafeTraceManuallyBarrieredEdge(trc, idp, name);
  }
  static bool isValid(jsid id) {
    return !JSID_IS_GCTHING(id) ||
           js::gc::IsCellPointerValid(JSID_TO_GCTHING(id).asCell());
  }
};

#ifdef DEBUG
MOZ_ALWAYS_INLINE void AssertIdIsNotGray(jsid id) {
  if (JSID_IS_GCTHING(id)) {
    AssertCellIsNotGray(JSID_TO_GCTHING(id).asCell());
  }
}
#endif

}  // namespace JS

namespace js {

template <>
struct BarrierMethods<jsid> {
  static gc::Cell* asGCThingOrNull(jsid id) {
    if (JSID_IS_STRING(id)) {
      return reinterpret_cast<gc::Cell*>(JSID_TO_STRING(id));
    }
    if (JSID_IS_SYMBOL(id)) {
      return reinterpret_cast<gc::Cell*>(JSID_TO_SYMBOL(id));
    }
    return nullptr;
  }
  static void postBarrier(jsid* idp, jsid prev, jsid next) {}
  static void exposeToJS(jsid id) {
    if (JSID_IS_GCTHING(id)) {
      js::gc::ExposeGCThingToActiveJS(JSID_TO_GCTHING(id));
    }
  }
};

// If the jsid is a GC pointer type, convert to that type and call |f| with
// the pointer. If the jsid is not a GC type, calls F::defaultValue.
template <typename F, typename... Args>
auto DispatchTyped(F f, const jsid& id, Args&&... args)
    -> decltype(f(static_cast<JSString*>(nullptr),
                  std::forward<Args>(args)...)) {
  if (JSID_IS_STRING(id)) {
    return f(JSID_TO_STRING(id), std::forward<Args>(args)...);
  }
  if (JSID_IS_SYMBOL(id)) {
    return f(JSID_TO_SYMBOL(id), std::forward<Args>(args)...);
  }
  MOZ_ASSERT(!JSID_IS_GCTHING(id));
  return F::defaultValue(id);
}

#undef id

}  // namespace js

#endif /* js_Id_h */
