package dev.gigapixel.tok4j

class Model {
    private var handle: Long = -1
    private class ModelCleaner(val handle: Long): Runnable {
        override fun run() {
            Model.dropByHandle(handle)
        }
    }
    companion object {

        fun newFromStatic(): Model {
            val model = newModel("")
            CLEANER.register(model, ModelCleaner(model.handle));
            return model
        }
        @JvmStatic
        private external fun tokenize(handle: Long, str: String): Array<String>

        @JvmStatic
        private external fun newModel(_str: String): Model

        @JvmStatic
        private external fun dropByHandle(handle: Long)
    }
    fun tokenize(str: String): Array<String>  {
        return Companion.tokenize(handle, str)
    }
}