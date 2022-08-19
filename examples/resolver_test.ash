@[builtin]
fun println(msg: String)

fun main = {
    println "Hello World"
    let a = 3
    a = 4
    a = {
        let b = 3
        sum 3, sum b, 1
    }
    // let b = {}
}

fun sum(a: I32, b: I32): I32 = a + b
