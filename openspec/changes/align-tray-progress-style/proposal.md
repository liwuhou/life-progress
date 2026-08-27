## Why

当前 Life Progress 的托盘横向进度条直接填满整张低高度图像，宽度调大后会出现直角、边缘和比例异常，视觉上没有与 `xe-ai-usage` 的成熟实现对齐。与此同时，固定启用 macOS template icon 模式会将 RGBA 颜色当作单色蒙版处理，无法正确呈现阈值色和灰色轨道。

## What Changes

- 将横向进度条改为透明画布上的内缩圆角轨道，统一宽度、高度、边距、填充比例和边界行为。
- 引入可持久化的颜色模式：单色模式和阈值模式。
- 在阈值模式下按可配置的下限/上限显示红、橙、绿三段颜色，并支持彩虹阈值条样式。
- 为图形样式统一颜色与圆角绘制规则，使进度条、圆环和柱状图可以共享颜色语义。
- 更新设置页面，允许选择颜色模式并配置阈值边界；保留旧配置的默认兼容行为。
- 调整 macOS 托盘图标集成：不再无条件启用 template 模式；彩色样式使用真实 RGBA，单色样式按平台行为验证。
- 增加渲染边界测试，并在 macOS 实机验证不同宽度、进度值和颜色模式下的托盘显示。

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `tray-display-preferences`: 扩展托盘图形显示的颜色模式、阈值边界、圆角布局和 macOS 图标呈现行为。

## Impact

- Rust 托盘显示配置、持久化模型和图标渲染逻辑。
- `src-tauri/assets/index.html` 设置页面及其 Tauri 调用参数。
- macOS 托盘图标初始化与运行时更新逻辑。
- 现有 `desktop.toml` 配置需要通过 serde 默认值平滑读取；不改变 profile 数据结构。
