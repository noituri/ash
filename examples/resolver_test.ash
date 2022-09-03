@[builtin]
fun println(msg: String)

// fun tt = a + b

// let b = a
// let a = 2 + d
// let a = c
// let c = b

// let d = 1

fun main = {
    println "Hello World"
    let a = a
    a = 4
    a = {
        let a = a
        let b = 3
        sum a, sum b, 1
    }
    // let b = {}
}

fun sum(a: I32, b: I32): I32 = a + b
