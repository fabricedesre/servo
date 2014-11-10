CARGO_OPTS ?=

CARGO_OPTS += --features gonk

.PHONY: build
build:
	../../mach cargo build --target=arm-linux-androideabi $(CARGO_OPTS)

.PHONY: app
app:
	cd app; \
	../../../mach cargo build --target=arm-linux-androideabi $(CARGO_OPTS)

.PHONY: flash
flash:
	adb push ../../target/arm-linux-androideabi/servo /system/servo/servo
