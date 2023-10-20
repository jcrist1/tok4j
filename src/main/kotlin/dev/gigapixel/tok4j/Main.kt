package dev.gigapixel.tok4j
import org.apache.commons.io.IOUtils.resourceToByteArray

val CLEANER = java.lang.ref.Cleaner.create()

object Main {
    @JvmStatic
    fun main(args: Array<String>) {
        // System.loadLibrary("tok4jbindings")
        val bytes = resourceToByteArray("/tokenizer.json")


        (0..100).forEach {
            println(it)
            val tokenizer = Tokenizer.newFromBytes(bytes);
            println("$tokenizer");
            System.gc()
        }

        val tokenizer = Tokenizer.newFromBytes(bytes)

        val input = "\"Hello Mr. Fox\", said Mrs. Badger. \"How are you today?\""
        tokenizer.tokenize(input).forEach {
            print("[$it]")
        }
    }
}