use CTypes;

proc main() {
  var s: string = "hello";
  var p = c_ptrToConst(s);
  writeln("pointer: ", p);
}
