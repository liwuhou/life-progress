# Product

<!-- impeccable:product-schema 1 -->

## Platform

adaptive

## Stack

Rust/Tauri 2 with a static HTML/CSS/JavaScript settings WebView and no frontend build server.

## Users

个人用户，通常让应用长期运行在菜单栏中，通过托盘持续观察自己的生命进度。

## Product Purpose

Life Progress 以用户的生日、性别和国家寿命数据计算生命进度，并通过持续可见的托盘指标提醒用户珍惜和安排时间。成功意味着用户无需打开主窗口，也能快速感知时间状态。

## Positioning

它把生命时间变成低打扰、持续可见的系统托盘进度，而不是需要主动打开的统计页面或一次性提醒。

## Operating Context

应用以菜单栏/系统托盘为主要入口；设置页仅在首次配置或用户主动打开时出现。用户可以维护生日资料、选择剩余或已过人生，并调整托盘展示方式与右侧文案。

## Capabilities and Constraints

- Rust/Tauri 负责寿命数据、计算、持久化、刷新和托盘呈现。
- 用户资料与 CLI 共用 `~/.config/life_progress/profile.toml`。
- 寿命数据缓存独立保存为 `~/.config/life_progress/.tmp_expectancy.json`。
- 默认寿命数据必须支持离线使用。
- 正常状态无常驻主窗口；设置 WebView 按需打开。
- 个人数据仅保存在本地，持久化最少必要字段。
- 设置页与托盘体验以中文为主。

## Brand Commitments

产品名为 Life Progress；语气应直接、克制、具有时间意识，不将生命进度游戏化或制造焦虑。

## Evidence on Hand

- `src-tauri/crates/life-progress-core/`：生日、性别、国家和生命进度领域逻辑。
- `src-tauri/crates/lifespan-crawler/`：寿命数据获取、缓存和 bundled fallback。
- `src-tauri/assets/index.html`：当前设置页原型。
- `../../xiaoe/xe-ai-usage/`：托盘展示样式与动态更新参考工程。

## Product Principles

- 托盘优先：信息应在用户不主动操作时仍可感知。
- 低打扰：提醒时间，但不把生命统计变成游戏或压力机制。
- 本地优先：个人资料本地保存，离线时仍保持核心功能可用。
- 诚实呈现：显示模式、颜色和文案必须与实际生命语义一致。

## Accessibility & Inclusion

设置表单应提供清晰标签、键盘可操作控件、可读错误信息，并避免仅依赖颜色表达生命状态。
