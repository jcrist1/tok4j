package dev.gigapixel.tok4j
import cz.adamh.utils.NativeUtils


class Tokenizer {
    private var handle: Long = -1

    private class TokenizerCleaner(val handle: Long) : Runnable {
        override fun run() {
            dropByHandle(handle)
        }
    }

    companion object {
        @JvmStatic
        fun newFromBytes(bytes: ByteArray): Tokenizer {
            NativeUtils.loadLibraryFromJar("/libtok4jbindings.dylib")
            val model = fromBytes(bytes)
            CLEANER.register(model, TokenizerCleaner(model.handle));
            return model
        }

        @JvmStatic
        private external fun fromBytes(bytes: ByteArray): Tokenizer

        @JvmStatic
        private external fun tokenize(handle: Long, text: String): Array<String>

        @JvmStatic
        private external fun dropByHandle(handle: Long)
    }

    fun tokenize(text: String): Array<String> {
        return Companion.tokenize(handle, text)
    }
}