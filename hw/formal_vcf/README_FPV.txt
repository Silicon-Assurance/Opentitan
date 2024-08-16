================================================================== 
Formal Property Verification (FPV) App
================================================================== 

================================================================== 
Files Description
================================================================== 

declare_modules  	List of target OpenTitan modules

build_all  		Build all modules
build_module   		Build specific module

fpv_check  		Check specific module in GUI mode

fpv.tcl                	Tcl script

================================================================== 
Building OpenTitan Modules
================================================================== 

Building modules will create a directory under the build directory
for every target module. The files can then be edited e.g., new
SVA properties can be added.

Option 1: Build ALL modules from declare_modules
  % build_all

Option 2: Build specific module
  % build_module <module name>
Example: 
  % build_module aes

Note: There is no need to build a module again if it is available
  in the build directory.

================================================================== 
Checking OpenTitan Modules
================================================================== 

*IMPORTANT*
Edit Tcl script fpv.tcl as needed before running verification.
For example, configure number of workers with set_grid_usage.

Check specific module in GUI mode (interactive session)
  % fpv_check <module name>
Example: 
  % fpv_check aes

================================================================== 
Creating Properties
================================================================== 

OPTION 1: Tcl commands

Command fvassert is used to create assertions.
Commands fvcover and fvassume are used to create cover properties
and constraints respectively.

Check command fvassert Tcl Help and Man pages for more details.
  vcf> fvassert -help
  vcf> view_help fvassert

Example FPV property for module aes
  fvassert key0_leak_violation -expr {u_reg.reg_re |-> u_reg.reg_rdata != u_reg.reg2hw.key_share0[0].q}

Add the properties in Tcl script fpv.tcl or add them on the
command line directly.

OPTION 2: Add SVA properties to target modules or bind files

SVA properties can be added to the files resulting from building
target modules directly. Use the filelist and source files to 
select the best location to add SVA properties.

For module aes:
Filelist: build/lowrisc_dv_aes_sva_0.1/formal-icarus/lowrisc_dv_aes_sva_0.1.scr 
Source directory: build/lowrisc_dv_aes_sva_0.1/src

================================================================== 
