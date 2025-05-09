use crate::driver_config::DriverConfig;
use std::{fs::File, io::Write};

pub fn write_config(config: &DriverConfig, file: &mut File) -> std::io::Result<()> {
    let _ = file.write(
b"# Do not modify this file!  It was generated by 'winteros-generate-config'
# and may be overwritten by future invocations.  Please make changes
# to /etc/nixos/configuration.nix instead.
{ lib, ... }:
let
\tnixos-hardware = builtins.fetchTarball \"https://github.com/NixOS/nixos-hardware/archive/master.tar.gz\";
in
{
\timports = [
")?;

    for module in config.get_module() {
        write!(file, "\t\t(import \"${{nixos-hardware}}/{}\")\n", module)?;
    }
    let _ = file.write(b"\t];\n")?;
    if config.get_iio_sensor() {
        let _ = file.write(b"\thardware.sensor.iio.enable = lib.mkDefault true;\n")?;
    }
    if config.get_fingerprint() {
        let _ = file.write(b"\tservices.fprintd.enable = lib.mkDefault true;\n")?;
    }
    let _ = file.write(b"}")?;
    Ok(())
}
