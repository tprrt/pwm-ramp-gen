APPLICATION = pwm-ramp-gen

# If no BOARD is found in the environment, use this default:
BOARD ?= native

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/RIOT

USEMODULE += ztimer_usec

FEATURES_REQUIRED += periph_pwm
FEATURES_REQUIRED += periph_uart
FEATURES_REQUIRED += rust_target

# nucleo-f746zg (Cortex-M7) is not yet assigned a RUST_TARGET in RIOT, but is
# binary-compatible with thumbv7em-none-eabihf (same as Cortex-M4F)
ifeq ($(BOARD),nucleo-f746zg)
  RUST_TARGET = thumbv7em-none-eabihf
  FEATURES_PROVIDED += rust_target
endif

# The name of the crate (as per Cargo.toml package name, with '-' replaced by '_')
APPLICATION_RUST_MODULE = pwm_ramp_gen
BASELIBS += $(APPLICATION_RUST_MODULE).module

# Comment this out to disable code in RIOT that does safety checking
# which is not needed in a production environment but helps in the
# development process:
CFLAGS += -DDEVELHELP

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

include $(RIOTBASE)/Makefile.include
