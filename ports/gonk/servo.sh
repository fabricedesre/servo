#!/system/bin/sh
umask 0027
export TMPDIR=/data/local/tmp
mkdir -p $TMPDIR
chmod 1777 $TMPDIR
ulimit -n 8192

if [ ! -d /system/servo ]; then

  log -p W "No /system/servo directory. Attempting recovery."
  if [ -d /system/servo.bak ]; then
    if ! mount -w -o remount /system; then
      log -p E "Failed to remount /system read-write"
    fi
    if ! mv /system/servo.bak /system/servo; then
      log -p E "Failed to rename /system/servo.bak to /system/servo"
    fi
    mount -r -o remount /system
    if [ -d /system/servo ]; then
      log "Recovery successful."
    else
      log -p E "Recovery failed."
    fi
  else
    log -p E "Recovery failed: no /system/servo.bak directory."
  fi
fi

if [ -z "$SERVO_DIR" ]; then
  SERVO_DIR="/system/servo"
fi

export LD_LIBRARY_PATH=/vendor/lib:/system/lib:"$SERVO_DIR/lib"

exec $COMMAND_PREFIX "$SERVO_DIR/servo"