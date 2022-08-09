fun main = {
    println "test"
    println 123, "test"
    println("test", "test2")
    println (sum 1, 2), sum(3, 4)
    sum_test
}

fun sum(a: I32, b: I32): I32 = ab // TODO: binary support a + b

fun sum_test = {
    sum(1, 2)
    sum(3, 4)
    sum 5, 6
}