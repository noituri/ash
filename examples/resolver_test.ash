@[builtin]
fun println(msg: String)

fun tt = a + b

let b = a
let a = 2 + d
// let a = c
let c = b

let d = 1

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
