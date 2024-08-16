all:	run_vc_zoix

run_vc_zoix:   vc_comp vc_vcfcc vc_vcfdb_report vc_fcm_local verdi


clean:
	rm -rf fcm.dir* *.log ucli* csrc fdb*  verdi* ../simv*

## VC-Z01X ##

vc_comp:
	vcs -kdb -lca -fusa=portfaults strobe.v -debug_access +vcs+fsdbon +define+UVM +define+UVM_NO_DEPRECATED +define+UVM_REGEX_NO_DPI +define+UVM_REG_ADDR_WIDTH=32 +define+UVM_REG_DATA_WIDTH=32 +define+UVM_REG_BYTENABLE_WIDTH=4 +define+SIMULATION +define+DUT_HIER=tb.dut -sverilog -full64 -licqueue -ntb_opts uvm-1.2 -timescale=1ns/1ps -Mdir=$REPO_TOP/opentitan/scratch/master/hmac-sim-vcs/default/simv.csrc -o $REPO_TOP/opentitan/scratch/master/hmac-sim-vcs/default/simv -f REPO_TOP/opentitan/scratch/master/hmac-sim-vcs/default/sim-vcs/lowrisc_dv_hmac_sim_0.1.scr -lca -top hmac_bind -top sec_cm_prim_onehot_check_bind -top tb -top strobe +incdir+REPO_TOP/opentitan/scratch/master/hmac-sim-vcs/default +warn=SV-NFIVC +warn=noUII-L +warn=noLCA_FEATURES_ENABLED -assert svaext -xlrm uniq_prior_final -CFLAGS --std=c99 -CFLAGS -fno-extended-identifiers -CFLAGS --std=c++11 -LDFLAGS -Wl,--no-as-needed -debug_region=cell+lib -debug_access+f +define+VCS -error=IPDW -error=UPF_ISPND -error=IGPA -error=PCSRMIO -error=AOUP -error=ELW_UNBOUND -error=IUWI -error=INAV -error=SV-ISC -error=OSVF-NPVIUFPI -error=DPIMI -error=IPDASP -error=CM-HIER-FNF -error=CWUC -error=MATN -error=STASKW_NDTAZ1 -error=TMPO -error=SV-OHCM -error=ENUMASSIGN -error=TEIF -assert novpi+dbgopt 

vc_logic:
	../simv +smoke_test=1 +UVM_NO_RELNOTES +UVM_VERBOSITY=UVM_LOW +prim_cdc_rand_delay_mode=disable -licqueue -ucli -do $REPO_TOP/opentitan/hw/dv/tools/sim.tcl +ntb_random_seed=6188285 -assert nopostproc +UVM_TESTNAME=hmac_base_test +UVM_TEST_SEQ=hmac_smoke_vseq +fsdb+struct +fsdb+all

vc_vcfcc:
	vc_fcc -full64 -daidir ../simv.daidir -sff faults.sff -campaign demo -dut tb.dut -overwrite -l fgen_v.log


vc_fcm_local:
	vc_fcm -tcl_script ./fcm_local.tcl -connect -fc demo

verdi:
	verdi -dbdir ../simv.daidir -campaign demo
