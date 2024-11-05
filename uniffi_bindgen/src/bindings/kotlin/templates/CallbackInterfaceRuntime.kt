// Magic number for the Rust proxy to call using the same mechanism as every other method,
// to free the callback once it's dropped by Rust.
internal const val IDX_CALLBACK_FREE = 0
// Callback return codes
internal const val UNIFFI_CALLBACK_SUCCESS = 0
internal const val UNIFFI_CALLBACK_ERROR = 1
internal const val UNIFFI_CALLBACK_UNEXPECTED_ERROR = 2

/**
 * @suppress
 */
public abstract class FfiConverterCallbackInterface<CallbackInterface: Any>(
    internal val langIndex: Int
): FfiConverterRustBuffer<CallbackInterface> {
    internal val handleMap = UniffiHandleMap<CallbackInterface>()

    internal fun drop(handle: Long) {
        handleMap.remove(handle)
    }

    override fun read(buf: ByteBuffer): CallbackInterface {
        val handle = buf.getLong()
        assert(buf.getInt() == this.langIndex) { "Callback interface has been called in the wrong language" }
        return handleMap.get(handle)
    }

    override fun allocationSize(value: CallbackInterface) = 12UL

    override fun write(value: CallbackInterface, buf: ByteBuffer) {
        val handle = handleMap.insert(value)
        buf.putLong(handle)
        buf.putInt(langIndex)
    }
}
