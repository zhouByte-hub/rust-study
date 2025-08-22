/**
 * winapi = "0.3.9"
 *
 * winapi 是一个 Rust 语言的第三方库（crate），其主要功能是为 Rust 程序提供对 Windows 操作系统原生 API 的绑定（Bindings）。
 * 简单来说，它让你可以在 Rust 代码中调用 Windows 提供的底层 C 语言 API 函数、使用相关的数据结构和常量。
 *
 * 提供 Windows API 的 Rust 绑定:
 *      1、它将 Windows SDK（如 kernel32.dll, user32.dll, advapi32.dll 等）中定义的 C 函数（如 CreateFile, ReadFile, GetSystemInfo）翻译成 Rust 可以调用的形式。
 *      2、它定义了 Windows API 所需的结构体（如 STARTUPINFO, PROCESS_INFORMATION, FILETIME）、枚举、常量（如 INVALID_HANDLE_VALUE, ERROR_SUCCESS）和类型别名（如 HANDLE, DWORD, LPSTR）。
 * 模块化设计：
 *      1、winapi 库非常庞大，因为它覆盖了几乎所有 Windows API。为了减少编译时间和依赖体积，它采用了 功能（features） 驱动的设计。
 *      2、你不需要一次性引入所有 API，而是通过在 Cargo.toml 中指定 features 来按需加载特定头文件（header files）对应的 API 模块。
 */
#[cfg(test)]
mod winapi {}
