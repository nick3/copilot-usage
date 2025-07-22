# GitHub Copilot 使用情况查看器（Rust 版本）

本命令行工具用于获取并展示指定 GitHub 组织的 Copilot 席位使用情况。该项目为原 TypeScript 版本的 Rust 重写版，具备更高性能和可靠性。

## 功能简介

- 获取 GitHub 组织分配的所有 Copilot 席位
- 以表格形式展示席位详细信息
- 统计并显示活跃席位与总席位数量
- 自动处理 GitHub API 分页

## 环境要求

- Rust 及 Cargo（建议使用最新稳定版）
- 具备 `read:org` 权限的 GitHub 个人访问令牌（PAT）

## 安装与配置

1. **克隆仓库：**
    ```bash
    git clone https://github.com/nick3/copilot-usage.git
    cd copilot-usage
    ```

2. **创建环境变量文件：**
    在项目根目录新建 `.env` 文件，并添加你的 GitHub 个人访问令牌：
    ```
    GITHUB_PAT=你的_github_pat
    ```

## 使用方法

### 运行程序

可直接通过 Cargo 运行，`<your_org>` 替换为你的 GitHub 组织名：

```bash
cargo run --organization <your_org>
```

或使用简写：

```bash
cargo run -o <your_org>
```

### 构建发布版

生成优化后的可执行文件：

```bash
cargo build --release
```

可执行文件位于 `target/release/rust_rewrite`，可直接运行：

```bash
./target/release/rust_rewrite -o <your_org>
```

### 测试

运行测试用例：

```bash
cargo test
```

## 主要模块说明

- `main.rs`：程序入口，参数解析与主流程控制
- `github_client.rs`：GitHub API 通信与数据获取
- `processor.rs`：业务逻辑与数据处理
- `formatter.rs`：表格展示与统计输出
- `types.rs`：核心数据结构定义
- `error.rs`：自定义错误类型

## 贡献指南

欢迎提交 Issue 或 PR 改进本项目。请确保代码风格统一，并附带必要的说明和测试。

## 许可证

本项目采用 MIT 许可证。

---

如需英文文档，请参见 [README.md](./README.md)
