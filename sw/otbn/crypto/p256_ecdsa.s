/* Copyright lowRISC contributors. */
/* Licensed under the Apache License, Version 2.0, see LICENSE for details. */
/* SPDX-License-Identifier: Apache-2.0 */

/**
 * Elliptic curve P-256 ECDSA
 *
 * Uses OTBN ECC P-256 lib to perform an ECDSA operations.
 */

.section .text.start
.globl start
start:
  /* Read mode, then tail-call either p256_ecdsa_sign or p256_ecdsa_verify */
  la    x2, mode
  lw    x2, 0(x2)

  li    x3, 1
  beq   x2, x3, p256_ecdsa_sign

  li    x3, 2
  beq   x2, x3, p256_ecdsa_verify

  /* Mode is neither 1 (= sign) nor 2 (= verify). Fail. */
  unimp

.text
p256_ecdsa_sign:
  jal      x1, p256_generate_k
  jal      x1, p256_sign
  ecall

p256_ecdsa_verify:
  jal      x1, p256_verify
  ecall


.bss

/* Operation mode (1 = sign; 2 = verify) */
.globl mode
.balign 4
mode:
  .zero 4

/* Message digest. */
.globl msg
.balign 32
msg:
  .zero 32

/* Signature R. */
.globl r
.balign 32
r:
  .zero 32

/* Signature S. */
.globl s
.balign 32
s:
  .zero 32

/* Public key x-coordinate. */
.globl x
.balign 32
x:
  .zero 32

/* Public key y-coordinate. */
.globl y
.balign 32
y:
  .zero 32

/* Private key (d) in two shares: d = (d0 + d1) mod n. */
.globl d0
.balign 32
d0:
  .zero 64
.globl d1
.balign 32
d1:
  .zero 64

/* Verification result x_r (aka x_1). */
.globl x_r
.balign 32
x_r:
  .zero 32

.section .scratchpad

/* Secret scalar (k) in two shares: k = (k0 + k1) mod n */
.globl k0
.balign 32
k0:
  .zero 64

.globl k1
.balign 32
k1:
  .zero 64
