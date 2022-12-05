@[builtin]
fun printf(format: String, d: I32)

fun sum(a: I32, b: I32): I32 = a + b

fun writeln(msg: String) = {
    printf(msg, 0)
}

fun main = {
    printf("%d", sum(1, 3))
}
