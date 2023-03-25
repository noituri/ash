@[builtin]
fun printf(format: str, v: i32)

fun main() {
    printf("test %d", 2 * 3 + 2 + 1000 * (3 + 1));
}