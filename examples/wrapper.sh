#!/usr/bin/env bash
set -Eeuo pipefail

# Customize the BIN variable to point to the correct path of the built binary.
BIN="${BIN:-$HOME/Projects/night-light-dialog/target/release/night-light-dialog}"

if pgrep -u "$USER" -f "$BIN" >/dev/null; then
  echo "the process is already launched: $BIN"
  exit 1
fi

corner="${1:-tr}"      # tr|tl|br|bl
dx="${2:-10}"
dy="${3:-28}"

command -v wmctrl >/dev/null 2>&1 || { echo "wmctrl not dound"; exit 1; }
command -v xwininfo >/dev/null 2>&1 || { echo "xwininfo not dound"; exit 1; }

"$BIN" & disown
PID=$!

wid=""
for _ in $(seq 1 200); do
  wid="$(wmctrl -lp | awk -v pid="$PID" '$3==pid {print $1; exit}')"
  [[ -n "$wid" ]] && break
  sleep 0.01
done
[[ -z "$wid" ]] && { echo "Window PID $PID not found"; exit 2; }

# screen
read sw sh < <(xwininfo -root | awk '/Width:/ {w=$2} /Height:/ {h=$2} END{print w, h}')
# window
read ww wh < <(xwininfo -id "$wid" | awk '/Width:/ {w=$2} /Height:/ {h=$2} END{print w, h}')

case "$corner" in
  tr) x=$(( sw - ww - dx )); y=$(( dy )) ;;
  tl) x=$(( dx ));          y=$(( dy )) ;;
  br) x=$(( sw - ww - dx )); y=$(( sh - wh - dy )) ;;
  bl) x=$(( dx ));           y=$(( sh - wh - dy )) ;;
  *)  x=$(( sw - ww - dx )); y=$(( dy )) ;;
esac

wmctrl -i -r "$wid" -e "0,$x,$y,-1,-1"
wmctrl -i -a "$wid"