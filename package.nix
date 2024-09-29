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
  text = builtins.readFile ./workspace2d.sh;
}
