@[builtin]
fun printf(format: str, d: i32)

fun sum(a: i32, b: i32) > i32 => a + b;

fun writeln(msg: str) {
    printf(msg, 0);
}

fun main() {
    writeln("Hello World!");
    printf("%d", sum(1, 3));
}
