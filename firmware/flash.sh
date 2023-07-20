
set -e

# clean files to make sure we dont reuse them
rm -rf app.hex
rm -rf complete.hex
rm -rf settings.hex



echo "Building and flashing in complete mode (MTK)"
nrfjprog --recover
nrfjprog --eraseall


# create hex file from cargo executable
DEFMT_LOG=off cargo objcopy --bin app  --release --no-default-features --features nrf52dk  -- -O ihex app.hex


# merge secure bootloader, its settings, softdevice and application
# mergehex --merge EDH_BleBootSDK15.hex settings.hex  s132_nrf52_7.3.0_softdevice.hex  app.hex --output complete.hex
mergehex --merge s132_nrf52_7.3.0_softdevice.hex  app.hex --output complete.hex


# program the whole hex
nrfjprog -f nrf52 --program complete.hex --verify  --reset
