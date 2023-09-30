package dev.gigapixel.tok4j


class Tokenizer {
    // The native library will handle setting this value which is a pointer to the Rust tokenizer
    private var handle: Long = -1


    private class TokenizerCleaner(val handle: Long): Runnable {
        override fun run() {
            dropByHandle(handle)
        }
    }

    companion object{
        fun newFromStatic(): Tokenizer {
            val tokenizer = loadStatic("")
            CLEANER.register(tokenizer, TokenizerCleaner(tokenizer.handle));
            return tokenizer
        }

        @JvmStatic
        private external fun loadStatic(_str: String): Tokenizer

        @JvmStatic
        external fun tokenize(str: String): Array<String>


        @JvmStatic
        private external fun tokenizeFromHandle(handle: Long, str: String): Array<String>

        @JvmStatic
        private external fun dropByHandle(handle: Long)

    }
    fun tokenizeFromClass(str: String): Array<String>  {
        return tokenizeFromHandle(handle, str)
    }


}




