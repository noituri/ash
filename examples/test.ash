fun main = {
    println "test"
    println -123, "test"
    println("test", "test2")
    println (sum 1, 2), sum(3, 4)
    sum_test
    // Operator test
    (1+5)+3*-2 == 1
}

fun sum(a: I32, b: I32): I32 = a + b

fun sum_test = {
    sum(1, 2)
    sum(3, 4)
    sum 5, 6
}