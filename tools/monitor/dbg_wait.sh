#!/usr/bin/env bash
# One-shot: wait for slurm dbg job 3594 to leave the queue, then poke.
set -u
LOG=/export/public_castep_jobs/tony/castep_check_io_oracle/cu111_spin/slurm_dbg_3594.txt
while squeue -j 3594 --me --noheader 2>/dev/null | rg -q '^[0-9]'; do
  sleep 5
done
sleep 2
OUT=$(cat "$LOG" 2>/dev/null)
rushi run binary-check-io "slurm CWD probe finished. Probe output: $OUT" \
  --sessions-root /home/tony/programming/castep-cell-io/sessions \
  --cwd /home/tony/programming/castep-cell-io
