以下是 Rust 打包 WebAssembly（WASM）的完整流程及关键要点，结合最新工具链和实践经验整理：

---

### 一、环境准备
1. **安装 Rust 工具链**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown  # 添加 WASM 编译目标
   ```
   - 若需要 WASI 支持（如文件系统访问），可添加 `wasm32-wasi` 目标

2. **安装核心工具**
   ```bash
   cargo install wasm-pack  # 核心打包工具
   cargo install wasm-bindgen-cli  # 用于 JS-Rust 交互
   ```

---

### 二、项目配置
1. **创建项目**
   ```bash
   cargo new --lib wasm-demo  # 创建库类型项目
   ```
   或使用模板快速初始化：
   ```bash
   cargo generate --git https://github.com/rustwasm/wasm-pack-template
   ```

2. **配置 Cargo.toml**
   ```toml
   [package]
   name = "wasm-demo"
   version = "0.1.0"
   
   [lib]
   crate-type = ["cdylib"]  # 必须设置为 cdylib
   
   [dependencies]
   wasm-bindgen = "0.2"  # 基础绑定库
   js-sys = "0.3"        # JavaScript 类型支持
   web-sys = { version = "0.3", features = ["Window"] }  # Web API 支持
   ```

---

### 三、代码开发
1. **基础函数导出**（src/lib.rs）
   ```rust
   use wasm_bindgen::prelude::*;
   
   #[wasm_bindgen]
   pub fn add(a: i32, b: i32) -> i32 {
       a + b
   }
   ```

2. **调用 JavaScript API**
   ```rust
   #[wasm_bindgen]
   extern "C" {
       #[wasm_bindgen(js_namespace = console)]
       fn log(s: &str);  // 绑定 console.log
   }
   
   #[wasm_bindgen]
   pub fn greet(name: &str) {
       log(&format!("Hello, {}!", name));  // 调用 JS 方法
   }
   ```

---

### 四、构建与优化
1. **基础构建命令**
   ```bash
   wasm-pack build --target web  # 生成 Web 兼容包
   ```
   - 输出目录 `pkg/` 包含：
     - `.wasm` 二进制文件
     - `.js` 胶水代码
     - `.d.ts` TypeScript 类型声明

2. **高级优化选项**
   ```bash
   wasm-pack build --release --features console_error_panic_hook
   wasm-opt -O3 pkg/*.wasm -o optimized.wasm  # 使用 Binaryen 优化
   ```

---

### 五、前端集成
1. **HTML 调用示例**
   ```html
   <script type="module">
     import init, { add } from './pkg/wasm_demo.js';
     
     async function run() {
       await init();
       console.log(add(2, 3));  // 输出 5
     }
     run();
   </script>
   ```

2. **框架集成（以 Vite 为例）**
   ```javascript
   // vite.config.js
   import { defineConfig } from 'vite'
   import wasmPack from 'vite-plugin-wasm-pack'
   
   export default defineConfig({
     plugins: [wasmPack(['wasm-demo'])]  // 自动加载 WASM 包
   })
   ```

---

### 六、部署注意事项
1. **服务器配置**
   - 需设置 MIME 类型 `application/wasm`
   - Apache/Nginx 示例配置：
     ```nginx
     location ~ \.wasm$ {
       add_header Content-Type application/wasm;
     }
     ```

2. **跨语言交互优化**
   - 使用 `serde` 处理复杂数据结构序列化
   - 通过 `web-sys` 直接操作 DOM 避免性能损耗[9](@ref)

---

### 常见问题解决
| 问题现象                  | 解决方案                                                                 |
|--------------------------|------------------------------------------------------------------------|
| 函数未导出到 JS          | 检查 `#[wasm_bindgen]` 注解和 `crate-type` 配置[15](@ref)        |
| 内存访问错误             | 使用 `js-sys` 的 `ArrayBuffer` 进行安全内存操作[9](@ref)                 |
| 性能低于预期            | 启用 LTO 优化：在 `Cargo.toml` 添加 `[profile.release] lto = true`[15](@ref) |
| 类型转换错误            | 用 `JsValue::from_str()` 显式转换字符串                        |

完整工具链和案例可参考 Mozilla 官方指南  和 wasm-bindgen 文档 。