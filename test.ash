@[builtin]
fun printf(format: str, v: i32)

fun main() {
    printf("test %d", 3 + 2 * 5);
}