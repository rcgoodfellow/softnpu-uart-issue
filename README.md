# Softnpu UART issue repro

## Instructions

1. [Install falcon](https://github.com/oxidecomputer/falcon#installing).
2. Build the falcon topology `cargo build`.
3. Run the topology `pfexec ./softnpu-uart-issue launch`.
4. Start to tail the propolis log for the `scrimlet` vm: `tail -f
   .falcon/scrimlet.out | looker -l error`.
4. On a separate terminal, get a console on the scrimlet: `./softnpu-uart-issue
   serial scrimlet`, and press `<enter>`. Username is `root` password is empty.
6. Run the repro script: `cd /opt/cargo-bay; chmod +x loop.sh; ./loop.sh`. 
7. Observe errors in tail session that look like this

```
20:17:44.140Z ERRO propolis-server: mgmt message deser: expected value at line 1 column 1
20:17:44.140Z ERRO propolis-server: [44, 75, 6d, 70, 52, 65, 71, 75, 65, 73, 74, 22]
20:17:44.140Z ERRO propolis-server: DumpRequest"
```

## Testing fixes

Falcon can use a custom built propolis server via the `--propolis` flag in
combination with the `launch` command. If you want avoid rebuilding the topology
while taking laps, falcon has `hyperstop` and `hyperstart` commands to stop and
start VMs. The `hyperstart` command also takes a `--propolis` flag to use a
custom `propolis-server` build.
