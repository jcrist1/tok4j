package dev.gigapixel.tok4j

val CLEANER = java.lang.ref.Cleaner.create()

object Main {
    @JvmStatic
    fun main(args: Array<String>) {
        System.loadLibrary("tok4jbindings")


        (0..100).forEach {
            println(it)
            val model= Model.newFromStatic();
            println("$model");
            System.gc()
        }

        val model = Model.newFromStatic();

        val input = "\"Hello Mr. Fox\", said Mrs. Badger. \"How are you today?\""
        model.tokenize(input).forEach {
            print("[$it]")
        }
    }
}