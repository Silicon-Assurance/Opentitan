module strobe();
initial begin
#1110078ps	$fs_inject();
end


always @(posedge tb.dut.clk_i) begin
	#19000ps $fs_strobe(tb.dut);
end


endmodule
