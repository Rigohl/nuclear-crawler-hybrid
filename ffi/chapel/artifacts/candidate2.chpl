use CTypes;

proc main() {
  var s: string = "hello";
  const p: c_ptr(c_char) = c_ptrToConst(s);
  writeln("pointer: ", p);
}
