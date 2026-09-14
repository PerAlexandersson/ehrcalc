# Wide CRT exact-strict mode for KTT

The current Euler `run-exact-strict.sh` bridge correctly certifies individual
strict-face counts only when a u64 upper bound is supplied.  KTT's strongest
zero-hole flagged candidates have ordinary values around 95 bits at dilation
25; a rigorous generic bound is about 174 bits.  Their GPU modular residues
therefore cannot currently replace a long CPU replay, even though the kernel
can count residues.

Add an opt-in exact mode accepting a decimal nonnegative bound or a bit bound,
collecting enough distinct 31-bit prime residues, and reconstructing with a
host big-integer CRT until the modulus product exceeds twice that bound.  Keep
the existing u64 command unchanged.  Record the prime list, source/binary
hashes, residue list, bound, and reconstructed integer in the JSON receipt.

For Ehrhart use, a subsequent batched-dilation wrapper would reduce bridge
overhead, but it must retain per-face identity and never sum overlapping face
unions without deduplication.
