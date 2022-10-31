// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#include "sw/device/tests/otbn_randomness_impl.h"

#include "sw/device/lib/dif/dif_base.h"
#include "sw/device/lib/dif/dif_otbn.h"
#include "sw/device/lib/runtime/log.h"
#include "sw/device/lib/runtime/otbn.h"
#include "sw/device/lib/testing/test_framework/check.h"

OTBN_DECLARE_APP_SYMBOLS(randomness);
OTBN_DECLARE_SYMBOL_ADDR(randomness, rv);
OTBN_DECLARE_SYMBOL_ADDR(randomness, fail_idx);
OTBN_DECLARE_SYMBOL_ADDR(randomness, rnd_out);
OTBN_DECLARE_SYMBOL_ADDR(randomness, urnd_out);

static const otbn_app_t kOtbnAppRandomnessApp = OTBN_APP_T_INIT(randomness);
static const otbn_addr_t kVarRv = OTBN_ADDR_T_INIT(randomness, rv);
static const otbn_addr_t kVarFailIdx = OTBN_ADDR_T_INIT(randomness, fail_idx);
static const otbn_addr_t kVarRndOut = OTBN_ADDR_T_INIT(randomness, rnd_out);
static const otbn_addr_t kVarUrndOut = OTBN_ADDR_T_INIT(randomness, urnd_out);

/**
 * LOG_INFO with a 256b unsigned integer as hexadecimal number with a prefix.
 */
static void print_uint256(otbn_t *otbn, const otbn_addr_t var,
                          const char *prefix) {
  uint32_t data[32 / sizeof(uint32_t)];
  CHECK(otbn_copy_data_from_otbn(otbn, /*len_bytes=*/32, var, &data) ==
        kOtbnOk);
  LOG_INFO("%s0x%08x%08x%08x%08x%08x%08x%08x%08x", prefix, data[7], data[6],
           data[5], data[4], data[3], data[2], data[1], data[0]);
}

void otbn_randomness_test_start(otbn_t *otbn) {
  // Reset the `kVarRv` value to ensure the result loaded by
  // `otbn_randomness_test_end()` is the one generated by OTBN.
  uint32_t rv = UINT32_MAX;
  otbn_copy_data_to_otbn(otbn, sizeof(uint32_t), &rv, kVarRv);

  CHECK(otbn_load_app(otbn, kOtbnAppRandomnessApp) == kOtbnOk);
  CHECK(otbn_execute(otbn) == kOtbnOk);
}

bool otbn_randomness_test_end(otbn_t *otbn, bool skip_otbn_done_check) {
  if (!skip_otbn_done_check) {
    CHECK(otbn_busy_wait_for_done(otbn) == kOtbnOk);
  }
  uint32_t rv;
  CHECK(otbn_copy_data_from_otbn(otbn, /*len_bytes=*/4, kVarRv, &rv) ==
        kOtbnOk);
  if (rv != 0) {
    uint32_t fail_idx;
    CHECK(otbn_copy_data_from_otbn(otbn, /*len_bytes=*/4, kVarFailIdx,
                                   &fail_idx) == kOtbnOk);
    LOG_ERROR("ERROR: Test with index %d failed.", fail_idx);
    return false;
  }
  return true;
}

void otbn_randomness_test_log_results(otbn_t *otbn) {
  print_uint256(otbn, kVarRndOut, "rnd = ");
  print_uint256(otbn, kVarUrndOut, "urnd = ");
}