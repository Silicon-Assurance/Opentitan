================================================================== 
Formal Security Verification (FSV) App
================================================================== 

================================================================== 
Files Description
================================================================== 

declare_modules  	List of target OpenTitan modules

build_all  		Build all modules
build_module   		Build specific module

fsv_check  		Check specific module in GUI mode

fsv.tcl                	Tcl script

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
Edit Tcl script fsv.tcl as needed before running verification.
For example, configure number of workers with set_grid_usage.

Check specific module in GUI mode (interactive session)
  % fsv_check <module name>
Example: 
  % fsv_check aes

==================================================================
Creating Security Properties
==================================================================

Command fsv_generate is used to create security properties  
by specifying source and destination.

Check command fsv_generate Tcl Help and Man pages for more details.
  vcf> fsv_generate -help
  vcf> view_help fsv_generate

Example FSV security property for module aes
  fsv_generate -src u_reg.reg2hw.key_share0 -dest aes.tl_o

Add the properties in Tcl script fsv.tcl or add them on the
command line directly.

==================================================================
