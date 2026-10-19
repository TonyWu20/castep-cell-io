#!/usr/bin/env bash
# Task child: wait for oracle job 3592 to leave the queue, then evaluate.
JOBID=3592
while squeue -j "$JOBID" --me --noheader 2>/dev/null | rg -q "^[0-9]"; do
  sleep 15
done
JOB_DIR=/export/public_castep_jobs/tony/castep_check_io_oracle/cu111_spin
# slurm resolves --output against the submit CWD (the repo root), so look
# in both places.
LATEST=$(ls -1 /home/tony/programming/castep-cell-io/slurm_output_3592.txt \
  "$JOB_DIR"/slurm_output_3592.txt 2>/dev/null | sort | tail -1)
echo "job left the queue. log: $LATEST"
echo "--- reuse and error lines ---"
rg -in "ground.state|found.*model|model.*found|error" "$LATEST" | head -20 || true
echo "--- final lines ---"
tail -12 "$LATEST"
