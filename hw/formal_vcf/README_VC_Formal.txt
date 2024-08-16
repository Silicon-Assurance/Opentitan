Synopsys VC Formal provides several applications that leverage 
formal techniques for exhaustive verifications of complex problems.

For Hack@DAC 2023, the following VC Formal applications are used:
- Formal Property Verification (FPV) App
- Formal Security Verification (FSV) App

================================================================== 
Setting Up VC Formal
================================================================== 

Before starting, check if VC Formal is available.
  $ which vcf
  /global/apps/vcstatic/2023.03-1/bin/vcf
(Note that the version might be different.)

Also check that the $VC_STATIC_HOME environment variable points
to the installation of VC Formal:
  $ echo $VC_STATIC_HOME
  /global/apps/vcstatic/2023.03-1

In the case the vcf command is not found, load module vcstatic:
  $ module load vcstatic

The VC Formal User Guide along with other documentation is
available under $VC_STATIC_HOME/doc/vcst/VC_Formal_Docs/.

  $ module load acrobat
  $ acroread $VC_STATIC_HOME/doc/vcst/VC_Formal_Docs/VC_Formal_UG.pdf

================================================================== 
Formal Property Verification (FPV) App
================================================================== 

The FPV App targets user-defined properties written in SVA or with
Tcl commands for the verification of the target design.

Refer to README_FPV.txt for more information about setting up
and using the FPV App.

A lab example for the FPV App is available under:
  $VC_STATIC_HOME/doc/vcst/examples/FPV/FPV

================================================================== 
Formal Security Verification (FSV) App
================================================================== 

The FSV App targets user-defined properties written with Tcl 
commands to implement data leakage and data integrity checks 
between source and destination signals.

Data leakage checks:
  Secure data cannot reach (leak) to non-secure modules
Data integrity check
  Secure data can not be over-written (data integrity) with 
  non-secure data

Refer to README_FSV.txt for more information about setting up
and using the FSV App.

A lab example for the FSV App is available under:
  $VC_STATIC_HOME/doc/vcst/examples/FSV

================================================================== 
