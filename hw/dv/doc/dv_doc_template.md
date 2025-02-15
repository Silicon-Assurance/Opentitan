# FOO DV document

<!-- Copy this file to hw/ip/foo/doc/index.md and make changes as needed.
For convenience 'foo' in the document can be searched and replaced easily with the
desired IP (with case sensitivity!). Also, use the testbench block diagram
located at OpenTitan team drive / 'design verification'
as a starting point and modify it to reflect your foo testbench and save it
to hw/ip/foo/doc/tb.svg. It should get linked and rendered under the block
diagram section below. Please update / modify / remove sections below as
applicable. Once done, remove this comment before making a PR.

Remove "draft: true" before submission.
-->

## Goals
* **DV**
  * Verify all FOO IP features by running dynamic simulations with a SV/UVM based testbench
  * Develop and run all tests based on the [testplan](#testplan) below towards closing code and functional coverage on the IP and all of its sub-modules
* **FPV**
  * Verify TileLink device protocol compliance with an SVA based testbench

## Current status
* [Design & verification stage](../../README.md)
  * [HW development stages](../../../doc/project_governance/development_stages.md)
* [Simulation results](https://reports.opentitan.org/hw/ip/foo/dv/latest/report.html)

## Design features
<!-- TODO: uncomment link to the spec below -->
For detailed information on FOO design features, please see the [FOO  HWIP technical specification]({{</* relref "hw/ip/foo/doc" */>}}).

## Testbench architecture
FOO testbench has been constructed based on the [CIP testbench architecture](../sv/cip_lib/README.md).

### Block diagram
![Block diagram](tb.svg)

### Top level testbench
Top level testbench is located at `hw/ip/foo/dv/tb/tb.sv`. It instantiates the FOO DUT module `hw/ip/foo/rtl/foo.sv`.
In addition, it instantiates the following interfaces and sets their handle into `uvm_config_db`:
* [Clock and reset interface](../sv/common_ifs/README.md)
* [TileLink host interface](../sv/tl_agent/README.md)
* FOO IOs
* Interrupts ([`pins_if`](../sv/common_ifs/README.md))
* Alerts ([`pins_if`](../sv/common_ifs/README.md))
* Devmode ([`pins_if`](../sv/common_ifs/README.md))

### Common DV utility components
The following utilities provide generic helper tasks and functions to perform activities that are common across the project:
* [dv_utils_pkg](../sv/dv_utils/README.md)
* [csr_utils_pkg](../sv/csr_utils/README.md)

### Compile-time configurations
[list compile time configurations, if any and what are they used for]

### Global types & methods
All common types and methods defined at the package level can be found in
`foo_env_pkg`. Some of them in use are:
```systemverilog
[list a few parameters, types & methods; no need to mention all]
```
### TL_agent
FOO testbench instantiates (already handled in CIP base env) [tl_agent](../sv/tl_agent/README.md)
which provides the ability to drive and independently monitor random traffic via
TL host interface into FOO device.

### UVC/agent 1
[Describe here or add link to its README]

### UVC/agent 2
[Describe here or add link to its README]

### UVM RAL Model
The FOO RAL model is created with the [`ralgen`](../tools/ralgen/README.md) FuseSoC generator script automatically when the simulation is at the build stage.

It can be created manually (separately) by running `make` in the the `hw/` area.

### Reference models
[Describe reference models in use if applicable, example: SHA256/HMAC]

### Stimulus strategy
#### Test sequences
All test sequences reside in `hw/ip/foo/dv/env/seq_lib`.
The `foo_base_vseq` virtual sequence is extended from `cip_base_vseq` and serves as a starting point.
All test sequences are extended from `foo_base_vseq`.
It provides commonly used handles, variables, functions and tasks that the test sequences can simple use / call.
Some of the most commonly used tasks / functions are as follows:
* task 1:
* task 2:

#### Functional coverage
To ensure high quality constrained random stimulus, it is necessary to develop a functional coverage model.
The following covergroups have been developed to prove that the test intent has been adequately met:
* cg1:
* cg2:

### Self-checking strategy
#### Scoreboard
The `foo_scoreboard` is primarily used for end to end checking.
It creates the following analysis ports to retrieve the data monitored by corresponding interface agents:
* analysis port1:
* analysis port2:
<!-- explain inputs monitored, flow of data and outputs checked -->

#### Assertions
* TLUL assertions: The `tb/foo_bind.sv` binds the `tlul_assert` [assertions](../../ip/tlul/doc/TlulProtocolChecker.md) to the IP to ensure TileLink interface protocol compliance.
* Unknown checks on DUT outputs: The RTL has assertions to ensure all outputs are initialized to known values after coming out of reset.
* assert prop 1:
* assert prop 2:

## Building and running tests
We are using our in-house developed [regression tool](../../../util/dvsim/README.md) for building and running our tests and regressions.
Please take a look at the link for detailed information on the usage, capabilities, features and known issues.

Here's how to run a smoke test:

```console
$ cd hw/ip/foo/dv
$ make TEST_NAME=foo_smoke
```

## Testplan
<!-- TODO: uncomment the line below after adding the testplan -->
{{</* incGenFromIpDesc "../../data/foo_testplan.hjson" "testplan" */>}}
