#!/usr/bin/env bash
# Monitor: block on the oracle task child, then poke the session once.
set -u
SESSION="binary-check-io"
SESS_ROOT=/home/tony/programming/castep-cell-io/sessions
PROJ=/home/tony/programming/castep-cell-io
LOG=/home/tony/programming/castep-cell-io/tools/monitor/oracle.log
bash /home/tony/programming/castep-cell-io/tools/monitor/oracle_child.sh > "$LOG" 2>&1 &
TASK_PID=$!
echo "$TASK_PID" > /home/tony/programming/castep-cell-io/tools/monitor/oracle.pid
wait "$TASK_PID"
RC=$?
if [ "$RC" -eq 0 ]; then
  rushi run "$SESSION" "oracle job 3595 finished. Review $LOG" \
    --sessions-root "$SESS_ROOT" --cwd "$PROJ"
else
  rushi run "$SESSION" "oracle monitor child exited $RC. Review $LOG" \
    --sessions-root "$SESS_ROOT" --cwd "$PROJ"
fi
