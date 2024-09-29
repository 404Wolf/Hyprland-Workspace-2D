{
  writeShellApplication,
  jq,
  busybox,
}:
writeShellApplication {
  name = "workspace2d";
  runtimeInputs = [
    jq
    busybox
  ];
  excludeShellChecks = [
    "SC2086"
    "SC2046"
  ];
  text = builtins.readFile ./workspace2d.sh;
}
