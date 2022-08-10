fun main = {
    println "test"
    println -123, "test"
    println("test", "test2")
    let test_var: F64 = 123.0
    println (sum 1, 2), sum(3, 4)
    test_var = {
        let a = 1
        a + 1
    }
    sum_test
    // Operator test
    let v = (1+5)+3*-2 == 1
}

fun sum(a: I32, b: I32): I32 = a + b

fun sum_test = {
    sum(1, 2)
    sum(3, 4)
    sum 5, 6
}

fun some_num: I32 = {
    fun inner = println "INNER"
    inner
    return 3 + 5
}