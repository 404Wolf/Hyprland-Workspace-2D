{
  writeShellScriptBin,
  jq,
  busybox,
}:
writeShellScriptBin {
  name = "workspace2d";
  runtimeInputs = [
    jq
    busybox
  ];
  text = builtins.readFile ./workspace2d.sh;
}
