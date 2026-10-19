#!/usr/bin/env bash
# Task child: wait for oracle job 3595 to leave the queue, then evaluate.
JOBID=3595
while squeue -j "$JOBID" --me --noheader 2>/dev/null | rg -q "^[0-9]"; do
  sleep 15
done
JOB_DIR=/export/public_castep_jobs/tony/castep_check_io_oracle/cu111_spin
# submitted from the job dir, so the relative --output lands there
LATEST="$JOB_DIR/slurm_output_3595.txt"
echo "job left the queue. log: $LATEST"
echo "--- reuse and error lines ---"
rg -in "ground.state|found.*model|model.*found|error" "$LATEST" | head -20 || true
echo "--- final lines ---"
tail -12 "$LATEST"
