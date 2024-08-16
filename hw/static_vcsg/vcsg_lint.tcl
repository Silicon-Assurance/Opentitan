# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

#-------------------------------------------------------------------------
# Extract bind module information
#-------------------------------------------------------------------------
echo "${env(TB_TOP)},${env(DUT_TOP)},-, -, -" 

# Read filelist
set f [open [glob *.scr]]
set lines [split [read $f] \n]
close $f
#echo $lines

# Check if filelist contains "_bind"
set matches [lsearch -all -inline -glob $lines *_bind*]
#echo $matches
set lm [llength $matches]
#echo $lm

if { $lm == 0 } {
  # No bind file
  set bind_exist 0
  set bind_module_name ""
} elseif { $lm == 1 } {
  # Read bind file
  set f2 [open $matches]
  set lines [split [read $f2] \n]
  close $f2
#  echo $lines
  # Check if file contains "module"
  set module_line [lsearch -all -inline -glob $lines *module*]
#  echo $module_line
  set lml [llength $module_line]
#  echo $lml
  if { $lml == 2 } {
    # Pair module-endmodule found
    set bind_exist 1
    set bind_module_name [string trimright [lindex [regexp -all -inline {\S+} [lindex $module_line 0] ] 1] ";"]
#    echo $bind_module_name
  } else {
    # Only two instances of "module" expected --> Should not come here --> Ignore bind
    set bind_exist -2
    set bind_module_name "err-module-name"
  }
} else {
  # More than one bind file --> Should not come here --> Ignore bind
  set bind_exist -1
  set bind_module_name "err-bind-detect"
}

if { ${env(TB_TOP)} == "*_reg_top" } {
set tbtop_exist 1
} else {
set tbtop_exist 0
}
echo "${env(TB_TOP)},${env(DUT_TOP)}, ${bind_exist} , ${bind_module_name}, -, -, -" 

#-------------------------------------------------------------------------
# Set up VC SpyGlass
#-------------------------------------------------------------------------
# Set App mode
set_app_var enable_lint true

# Configure rules
configure_lint_setup -goal lint_rtl

#-------------------------------------------------------------------------
# Read and compile design
#-------------------------------------------------------------------------
# Do not define SYNTHESIS macro by default
set_app_var define_synthesis_macro false

# Analyze design files
analyze -format sverilog -vcs "+define+EN_MASKING=1 -assert svaext \
  -f [glob *.scr]"


# Compile design with TB or DUT
 if {$tbtop_exist == 1 }  {
echo "elaborate $env(TB_TOP)"
 elaborate  $env(TB_TOP)
} elseif { $bind_exist == 1 } {
   echo "Compile with sva bind $bind_module_name,$env(DUT_TOP)"  
   elaborate -sva $env(DUT_TOP) -vcs "$bind_module_name"
   } else {
     echo "elaborate -sva $env(DUT_TOP)"
     elaborate -sva $env(DUT_TOP)
     }


# Perform Lint checks
check_lint
#
# Report Generation
report_lint -verbose -include_waived -file report_hdl.txt  -report { all }
#
# GUI invocation
start_gui
#
#
#quit
