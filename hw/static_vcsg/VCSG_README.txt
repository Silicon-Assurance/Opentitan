================================================================== 
Google OpenTitan - Open Source Silicon Root of Trust
================================================================== 

GitHub repository: 
https://github.com/lowRISC/opentitan

Documentation and website: 
https://docs.opentitan.org/

Modules information: 
https://docs.opentitan.org/hw/
https://docs.opentitan.org/hw/top_earlgrey/doc/

Goal: Integration of VC SpyGlass into the dvsim environment with other verification solutions

================================================================== 
Special Considerations
================================================================== 

. This is a “Work in Progress”: There are many moving pieces and OpenTitan is still changing
. The list of OpenTitan modules for VCSG is updated as the RTL code changes
. Top modules top_earlgrey and chip_earlgrey_asic are buildable but not when targeting VCSG for now
. Some modules do not compile and result in errors
. Configuration for each module is lacking (clocks period information, resets, constraints, etc.)
  - Currently assuming clocks to have same period and phase
  - Not applying any constraint but the ones set in original script

================================================================== 
Contents of File static_vcsg.zip
================================================================== 

vcsg_declare_modules  			List of target OpenTitan modules
vcsg_build_all  				Build all modules
vcsg_build      				Build specific module
vcsg_check_all  				Run VC SpyGlass on all modules
vcsg_check      				Run VC SpyGlass on specific module
vcsg.tcl                 		VC SpyGlass VCSG script
					(relies on vcf_dvsim_report.tcl formatting)
log_example/.log		Sample VCSG run log for module hmac
report_example/*rpt             Sample VCSG reports for module hmac

================================================================== 
Step 1: Setting Up Environment
================================================================== 

% module load vcstatic	  
[Validated with VC SpyGlass 2023.03-1 and Google OpenTitan 6/21/2023]
Git clone Google OpenTitan
% cd opentitan-master
% pip3 install --user -r python-requirements.txt
% which python3
% setenv PATH /u/<YOUR USERNAME>/.local/bin:$PATH
% which fusesoc
% cd hw
% unzip static_vcsg.zip
% cd static_vcsg

================================================================== 
Step 2: Building OpenTitan Modules
================================================================== 

In directory static_vcsg:

Option 1: Build ALL design modules
% vcsg_build_all

Option 2: Build specific design module
% vcsg_build <module name>
Example: 
% vcsg_build aes

================================================================== 
Step 3: Running VCSG on OpenTitan Modules
================================================================== 

In directory static_vcsg:

Option 1: Check ALL design modules
% vcsg_check_all

Option 2: Check specific design module
% vcsg_check <module name>
Example: 
% vcsg_check aes

check specific top level testbench
% vcsg_check <module name>  <Top-level name>
Example:
% vcsg_check hmac hmac_reg_top



================================================================== 
Step 4: Reviewing Results
================================================================== 
a. Reviewing run logs

      - VCST Session log will be "vcsg_lint.log" 
      - The complete Lint run log will be in respective run dir path/vcst_rtdb/reports/run.log

    Example : $REPO_TOP/hw/static_vcsg/build/lowrisc_dv_hmac_sva_0.1/formal-icarus/vcst_rtdb/reports/run.log

b. Reviewing Lint Reports

      - There are 2 report formats 
          b.1 : report_hdl.txt
                 Example : $REPO_TOP/hw/static_vcsg/build/lowrisc_dv_hmac_sva_0.1/formal-icarus/report_hdl.txt

          b.2 :  OLD spyglass report format
                 Example : $REPO_TOP/hw/static_vcsg/build/lowrisc_dv_hmac_sva_0.1/formal-icarus/vcst_rtdb/reports/moresimple.rpt



      - The complete Lint run summary report will be at :
   Example : $REPO_TOP/hw/static_vcsg/build/lowrisc_dv_hmac_sva_0.1/formal-icarus/vcst_rtdb/reports/summary.rpt


================================================================== 
