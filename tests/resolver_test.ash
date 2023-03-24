@[builtin]
fun println(msg: str)


// val b = a;
// val a = 2 + d;
// val a = c;
// val c = b;
val a = d;

val d = 1;

fun main() {
    println("Hello World");
    var a = a;
    a = 4;
    a = {
        val a = a + 1;
        val b = 3;
        break sum(a, b);
    };
    // val b = {};
}

fun sum(a: i32, b: i32) > i32 => a + b;
