#!/usr/bin/env bash
# CASTEP write-oracle for castep-check-io.
#
# Sets up a slurm job that runs CASTEP 6.11 on a job directory where the
# model file was written by CheckDocument. The seed is a finished spin
# single point. CASTEP must read our .check back, find the stored model
# matching the live cell and parameters, keep ground-state reuse true, and
# skip the SCF. A clean exit with the reuse line in the log is the oracle
# signal.
#
# Usage:
#   tools/check-oracle.sh            set up + submit
#   tools/check-oracle.sh setup      set up only
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

SEED=/export/public_castep_jobs/tony/Cu111_CO_Single_Point_0614_spin_cpu_serial
JOB_DIR=/export/public_castep_jobs/tony/castep_check_io_oracle/cu111_spin
SEEDNAME=Cu111_CO
# The ~/Downloads 6.11 build carries a local devel_code=256-byte patch and
# cannot read standard-layout model files. The eigensolve build writes and
# reads the standard 6.11 layout; the 0604 warm-start job proved
# continuation: default on it.
CASTEP_BIN=/export/castep-rust-eigensolve/castep/obj/linux_x86_64_gfortran/castep.mpi
CASTEP_NIX_TREE=/export/castep-rust-eigensolve/castep

write_job_script() {
  local out="$JOB_DIR/slurm_job.sh"
  cat > "$out" <<'JOB'
#!/usr/bin/env bash
#SBATCH --job-name=check_oracle
#SBATCH --output=slurm_output_%j.txt
#SBATCH --partition=debug
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=1
#SBATCH --cpus-per-task=1
#SBATCH --mem=30000m
cd "$(dirname "$0")" || exit 1
nix develop __CASTEP_NIX_TREE__ --command bash -c \
	"mpirun --mca plm slurm \
        -x OMPI_MCA_btl_tcp_if_include=enp6s0 \
        -x OMPI_MCA_orte_keep_fqdn_hostnames=true \
       --mca pmix s1 \
       --mca btl tcp,self \
	--map-by numa --bind-to numa \
    __CASTEP_BIN__ __SEEDNAME__"
JOB
  sed -i -e "s|__CASTEP_NIX_TREE__|$CASTEP_NIX_TREE|g" \
    -e "s|__CASTEP_BIN__|$CASTEP_BIN|g" \
    -e "s|__SEEDNAME__|$SEEDNAME|g" "$out"
  chmod +x "$out"
}

setup() {
  rm -rf "$JOB_DIR"
  mkdir -p "$JOB_DIR"
  cp "$SEED/$SEEDNAME.cell" "$JOB_DIR/$SEEDNAME.cell"
  cp "$SEED/$SEEDNAME.param" "$JOB_DIR/$SEEDNAME.param"
  # continuation: default maps to <seedname>.check in the working directory.
  if ! grep -q '^CONTINUATION' "$JOB_DIR/$SEEDNAME.param"; then
    printf '\nCONTINUATION :  default\n' >> "$JOB_DIR/$SEEDNAME.param"
  fi

  cargo build -p castep-check-io --example check_oracle --manifest-path "$REPO/Cargo.toml"
  "$REPO/target/debug/examples/check_oracle" \
    "$SEED/$SEEDNAME.check" \
    "$JOB_DIR" \
    "$SEEDNAME"

  write_job_script
  echo "oracle job ready at $JOB_DIR"
}

case "${1:-submit}" in
  setup)
    setup
    ;;
  submit)
    setup
    sbatch "$JOB_DIR/slurm_job.sh"
    ;;
  *)
    echo "unknown mode: $1" >&2
    exit 2
    ;;
esac
