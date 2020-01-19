# lightsapp-cli

Control LightsApp based BLE lights using rust based cli

## Useful commands

- Compile for Pi Zero W using [cross](https://github.com/rust-embedded/cross)

        cross build --target arm-unknown-linux-gnueabi

```
sudo setcap 'cap_net_raw,cap_net_admin+eip' `which hcitool`
```
