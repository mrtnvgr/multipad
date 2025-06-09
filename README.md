# MultiPad

Control multiple devices with one gamepad.

This application suite consists of:

- A server that relays input events from a connected controller to a currently selected client.
- A client that receives inputs and sends them to running applications.

This was written as a PoW for [easytcp](https://github.com/mrtnvgr/easytcp).

## Important note for client devices

Currently, **clients are Linux only.**

In order for application to work, you need to have a `uinput` kernel module enabled.
You can use `modprobe uinput` for a temporary solution.

If the `/dev/uinput` ownership is `root:root`, add a udev rule to your system:
`/etc/udev/rules.d/uinput.rules`: `SUBSYSTEM=="uinput", GROUP="input"`

Reload udev rules with `udevadm control --reload-rules`.
