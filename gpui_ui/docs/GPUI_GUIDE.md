# GPUI Component 完整使用指南

> 一份全面的 GPUI Component 框架文档，涵盖所有概念、组件和最佳实践

## 目录

- [简介](#简介)
- [安装与配置](#安装与配置)
- [快速开始](#快速开始)
- [核心概念](#核心概念)
- [组件库](#组件库)
  - [基础组件](#基础组件)
  - [表单组件](#表单组件)
  - [数据展示](#数据展示)
  - [导航组件](#导航组件)
  - [反馈组件](#反馈组件)
  - [布局组件](#布局组件)
- [主题系统](#主题系统)
- [布局系统](#布局系统)
- [最佳实践](#最佳实践)
- [示例代码](#示例代码)

---

## 简介

### 什么是 GPUI Component？

GPUI Component 是一个基于 GPUI 框架构建的跨平台桌面 UI 组件库，提供了 60+ 个高质量组件，用于构建功能丰富的桌面应用程序。

### 核心特性

- **60+ 组件**：全面的跨平台桌面 UI 组件库
- **高性能**：虚拟化表格和列表组件，平滑渲染大数据集
- **主题化**：内置主题系统，支持 20+ 主题，开箱即用的暗色模式
- **灵活布局**：Dock 布局、可调整面板、自由布局
- **数据可视化**：内置图表组件（折线图、柱状图、面积图、饼图）
- **代码编辑器**：高性能代码编辑器，支持 LSP、语法高亮、Tree-sitter 和 Rope

### 许可证

GPUI Component 是一个开源项目，采用 Apache-2.0 许可证，由 [Longbridge](https://longbridge.com/) 开发。

图标资源使用 [Lucide](https://lucide.dev/) 和 [Isocons](https://isocons.app/)。

---

## 安装与配置

### 系统要求

#### macOS
- macOS 15 或更高版本
- Xcode 命令行工具

#### Windows
- Windows 10 或更高版本
- 提供引导脚本安装所需工具链和依赖项
- 在 PowerShell 中运行：`.\script\install-window.ps1`

#### Linux
- 运行 `./script/bootstrap` 安装系统依赖

### Rust 环境

- **Rust 1.90** 或更高版本
- **Cargo**（随 Rust 一起安装）

### 安装步骤

在 `Cargo.toml` 文件的 `[dependencies]` 部分添加：

```toml
[dependencies]
gpui = "0.2.2"
gpui-component = "0.5.1"
```

---

## 快速开始

### Hello World 示例

创建 `src/main.rs` 文件：

```rust
use gpui::*;
use gpui_component::{button::*, *};

pub struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        // 在使用任何 GPUI Component 功能之前必须调用此方法
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| HelloWorld);
                // 窗口的第一层应该是 Root
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
```

运行程序：

```bash
$ cargo run
```

---

## 核心概念

### 1. 应用程序初始化

GPUI 应用程序需要正确初始化才能运行：

```rust
use gpui::{Application, WindowOptions};
use gpui_component::Root;

fn main() {
    // 创建应用程序实例
    let app = Application::new();
    
    app.run(|cx| {
        // 初始化 GPUI Component
        gpui_component::init(cx);
        
        // 设置全局主题
        cx.set_global(Theme::default());
        
        // 打开窗口
        cx.open_window(WindowOptions::default(), |window, cx| {
            let view = cx.new(|_| MyView);
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });
}
```

### 2. 视图（View）和渲染（Render）

每个 GPUI 组件都需要实现 `Render` trait：

```rust
use gpui::{Render, Window, Context};

pub struct MyComponent {
    // 组件状态
}

impl Render for MyComponent {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .size_full()
            .child("My Component")
    }
}
```

### 3. 状态管理

GPUI 使用实体（Entity）来管理状态：

```rust
// 创建实体
let entity = cx.new(|cx| MyState::new());

// 更新实体
entity.update(cx, |state, cx| {
    state.modify();
});

// 读取实体
let value = entity.read(cx).get_value();
```

### 4. 事件处理

GPUI 使用监听器（Listener）处理事件：

```rust
Button::new("my-button")
    .label("Click Me")
    .on_click(cx.listener(|this, event, window, cx| {
        // 处理点击事件
        this.handle_click(event, window, cx);
    }))
```

### 5. 异步操作

使用 `spawn` 进行异步操作：

```rust
cx.spawn(|cx| async move {
    // 异步操作
    let result = some_async_operation().await;
    
    // 更新 UI
    cx.update(|cx| {
        // 更新状态
    })?;
    
    Ok::<_, anyhow::Error>(())
})
.detach();
```

---

## 组件库

### 基础组件

#### 1. Button（按钮）

按钮组件支持多种变体、大小和状态。

**导入：**

```rust
use gpui_component::button::{Button, ButtonGroup};
```

**基本用法：**

```rust
Button::new("my-button")
    .label("Click me")
    .on_click(|_, _, _| {
        println!("Button clicked!");
    })
```

**变体：**

```rust
// 主要按钮
Button::new("btn-primary").primary().label("Primary")

// 次要按钮（默认）
Button::new("btn-secondary").label("Secondary")

// 危险按钮
Button::new("btn-danger").danger().label("Delete")

// 警告按钮
Button::new("btn-warning").warning().label("Warning")

// 成功按钮
Button::new("btn-success").success().label("Success")

// 信息按钮
Button::new("btn-info").info().label("Info")

// 幽灵按钮
Button::new("btn-ghost").ghost().label("Ghost")

// 链接按钮
Button::new("btn-link").link().label("Link")

// 文本按钮
Button::new("btn-text").text().label("Text")
```

**轮廓按钮：**

```rust
Button::new("btn-outline")
    .primary()
    .outline()
    .label("Outline Primary")
```

**尺寸：**

```rust
Button::new("btn-xs").xsmall().label("XSmall")
Button::new("btn-sm").small().label("Small")
Button::new("btn-md").label("Medium")  // 默认
Button::new("btn-lg").large().label("Large")
```

**带图标：**

```rust
use gpui_component::{Icon, IconName};

Button::new("btn-icon")
    .icon(IconName::Search)
    .label("Search")

Button::new("btn-icon-only")
    .icon(IconName::Plus)
```

**加载状态：**

```rust
Button::new("btn-loading")
    .loading(true)
    .label("Loading...")
```

**禁用状态：**

```rust
Button::new("btn-disabled")
    .disabled(true)
    .label("Disabled")
```

**按钮组：**

```rust
ButtonGroup::new()
    .child(Button::new("btn-1").label("Left"))
    .child(Button::new("btn-2").label("Center"))
    .child(Button::new("btn-3").label("Right"))
```

#### 2. Icon（图标）

图标组件提供大量预定义图标。

**导入：**

```rust
use gpui_component::{Icon, IconName};
```

**基本用法：**

```rust
Icon::new(IconName::Search)
Icon::new(IconName::Settings)
Icon::new(IconName::User)
```

**尺寸：**

```rust
Icon::new(IconName::Search).xsmall()
Icon::new(IconName::Search).small()
Icon::new(IconName::Search).medium()  // 默认
Icon::new(IconName::Search).large()
```

**颜色：**

```rust
Icon::new(IconName::Search).text_color(gpui::red())
Icon::new(IconName::Search).text_color(cx.theme().primary)
```

#### 3. Avatar（头像）

头像组件显示用户头像，支持回退选项。

**导入：**

```rust
use gpui_component::avatar::{Avatar, AvatarGroup};
```

**基本用法：**

```rust
// 带图片
Avatar::new()
    .name("John Doe")
    .src("https://example.com/avatar.jpg")

// 无图片，显示首字母
Avatar::new().name("John Doe")  // 显示 "JD"

// 占位符
Avatar::new().placeholder(IconName::UserCircle)
```

**尺寸：**

```rust
Avatar::new().name("John Doe").xsmall()
Avatar::new().name("John Doe").small()
Avatar::new().name("John Doe")  // 默认 medium
Avatar::new().name("John Doe").large()

// 自定义尺寸
Avatar::new().name("John Doe").with_size(px(100.))
```

**头像组：**

```rust
AvatarGroup::new()
    .limit(3)
    .ellipsis()
    .child(Avatar::new().src("https://example.com/user1.jpg"))
    .child(Avatar::new().src("https://example.com/user2.jpg"))
    .child(Avatar::new().src("https://example.com/user3.jpg"))
    .child(Avatar::new().name("John Doe"))
```

#### 4. Badge（徽章）

徽章组件显示计数、点或图标。

**导入：**

```rust
use gpui_component::badge::Badge;
```

**计数徽章：**

```rust
Badge::new()
    .count(3)
    .child(Icon::new(IconName::Bell))
```

**点徽章：**

```rust
Badge::new()
    .dot()
    .child(Icon::new(IconName::Inbox))
```

**图标徽章：**

```rust
Badge::new()
    .icon(IconName::Check)
    .child(Avatar::new().src("https://example.com/avatar.jpg"))
```

**自定义最大计数：**

```rust
Badge::new()
    .count(150)
    .max(999)  // 显示 "999+"
    .child(Icon::new(IconName::Mail))
```

#### 5. Text（文本）

文本组件提供文本显示功能。

**导入：**

```rust
use gpui_component::text::TextView;
```

**Markdown 渲染：**

```rust
TextView::markdown(
    "markdown-content",
    "# Title\n\nThis is **bold** text.",
    window,
    cx,
)
```

**HTML 渲染：**

```rust
TextView::html(
    "html-content",
    "<h1>Title</h1><p>Paragraph</p>",
    window,
    cx,
)
```

### 表单组件

#### 1. Input（输入框）

**导入：**

```rust
use gpui_component::input::Input;
```

**基本用法：**

```rust
Input::new("my-input")
    .placeholder("Enter text...")
    .on_change(cx.listener(|this, value, _, cx| {
        this.handle_input(value, cx);
    }))
```

**验证：**

```rust
Input::new("validated-input")
    .placeholder("Email")
    .validate(|value| {
        if value.contains('@') {
            Ok(())
        } else {
            Err("Invalid email")
        }
    })
```

#### 2. Checkbox（复选框）

**导入：**

```rust
use gpui_component::checkbox::Checkbox;
```

**基本用法：**

```rust
Checkbox::new("my-checkbox")
    .label("Accept terms")
    .checked(true)
    .on_click(cx.listener(|this, checked, _, cx| {
        this.toggle(checked, cx);
    }))
```

#### 3. Radio（单选框）

**导入：**

```rust
use gpui_component::radio::Radio;
```

**基本用法：**

```rust
Radio::new("option-1")
    .label("Option 1")
    .checked(true)
    .on_click(cx.listener(|this, _, _, cx| {
        this.select_option(1, cx);
    }))
```

#### 4. Switch（开关）

**导入：**

```rust
use gpui_component::switch::Switch;
```

**基本用法：**

```rust
Switch::new("my-switch")
    .checked(true)
    .on_click(cx.listener(|this, checked, _, cx| {
        this.toggle(checked, cx);
    }))
```

#### 5. Select（选择器）

**导入：**

```rust
use gpui_component::select::Select;
```

**基本用法：**

```rust
Select::new("my-select")
    .options(vec![
        ("value1", "Option 1"),
        ("value2", "Option 2"),
        ("value3", "Option 3"),
    ])
    .selected("value1")
    .on_change(cx.listener(|this, value, _, cx| {
        this.handle_selection(value, cx);
    }))
```

#### 6. Slider（滑块）

**导入：**

```rust
use gpui_component::slider::Slider;
```

**基本用法：**

```rust
Slider::new("my-slider")
    .min(0.0)
    .max(100.0)
    .value(50.0)
    .on_change(cx.listener(|this, value, _, cx| {
        this.update_value(value, cx);
    }))
```

### 数据展示

#### 1. Table（表格）

高性能虚拟化表格组件。

**导入：**

```rust
use gpui_component::table::{Table, TableDelegate};
```

**基本用法：**

```rust
pub struct MyTableDelegate {
    // 表格数据
}

impl TableDelegate for MyTableDelegate {
    fn columns(&self) -> Vec<TableColumn> {
        vec![
            TableColumn::new("name", "Name"),
            TableColumn::new("age", "Age"),
        ]
    }
    
    fn row_count(&self) -> usize {
        self.data.len()
    }
    
    fn render_cell(&self, row: usize, column: &str, cx: &mut Context<Self>) -> impl IntoElement {
        // 渲染单元格
    }
}

let delegate = MyTableDelegate::new();
Table::new(delegate)
    .flex_1()
    .w_full()
```

#### 2. List（列表）

虚拟化列表组件。

**导入：**

```rust
use gpui_component::list::List;
```

**基本用法：**

```rust
List::new("my-list")
    .items(my_items)
    .render_item(|item, cx| {
        div().child(item.name)
    })
```

#### 3. Tree（树形控件）

**导入：**

```rust
use gpui_component::tree::Tree;
```

**基本用法：**

```rust
Tree::new("my-tree")
    .items(tree_items)
    .render_item(|item, cx| {
        div().child(item.name)
    })
```

#### 4. Chart（图表）

数据可视化组件。

**导入：**

```rust
use gpui_component::chart::{Chart, ChartType, ChartData};
```

**折线图：**

```rust
Chart::new()
    .chart_type(ChartType::Line)
    .data(chart_data)
    .width(px(400.0))
    .height(px(300.0))
```

**柱状图：**

```rust
Chart::new()
    .chart_type(ChartType::Bar)
    .data(chart_data)
```

**饼图：**

```rust
Chart::new()
    .chart_type(ChartType::Pie)
    .data(chart_data)
```

#### 5. DescriptionList（描述列表）

**导入：**

```rust
use gpui_component::description_list::{DescriptionList, DescriptionItem};
```

**基本用法：**

```rust
DescriptionList::new()
    .columns(2)
    .children([
        DescriptionItem::new("Name").value("GPUI Component"),
        DescriptionItem::new("Version").value("0.5.1"),
        DescriptionItem::new("License").value("Apache-2.0"),
    ])
```

**复杂示例：**

```rust
DescriptionList::new()
    .columns(3)
    .label_width(px(150.0))
    .children([
        DescriptionItem::new("Project Name").value("GPUI Component").span(1),
        DescriptionItem::new("Version").value("0.1.0").span(1),
        DescriptionItem::new("Status").value("Active").span(1),
        
        DescriptionItem::Divider, // 全宽分隔符
        
        DescriptionItem::new("Description").value(
            "A comprehensive UI component library for building desktop applications with GPUI"
        ).span(3),
        
        DescriptionItem::new("Repository").value(
            "https://github.com/longbridge/gpui-component"
        ).span(2),
        DescriptionItem::new("License").value("Apache-2.0").span(1),
    ])
```

### 导航组件

#### 1. Menu（菜单）

**导入：**

```rust
use gpui_component::menu::{Menu, MenuItem};
```

**基本用法：**

```rust
Menu::new("my-menu")
    .items(vec![
        MenuItem::new("New File", Box::new(NewFileAction)),
        MenuItem::new("Open File", Box::new(OpenFileAction)),
        MenuItem::separator(),
        MenuItem::new("Exit", Box::new(ExitAction)),
    ])
```

#### 2. DropdownMenu（下拉菜单）

**导入：**

```rust
use gpui_component::popup_menu::{PopupMenuExt as _, PopupMenuItem};
```

**基本用法：**

```rust
use gpui::{actions, Action};

Button::new("menu-btn")
    .label("Open Menu")
    .dropdown_menu(|menu, window, cx| {
        menu.menu("New File", Box::new(NewFile))
            .menu("Open File", Box::new(OpenFile))
            .link("Documentation", "https://longbridge.github.io/gpui-component/")
            .separator()
            .item(PopupMenuItem::new("Custom Action")
                .on_click(window.listener_for(&view, |this, _, window, cx| {
                    // 自定义操作逻辑
                }))
            )
            .separator()
            .menu("Exit", Box::new(Exit))
    })
```

#### 3. Tab（标签页）

**导入：**

```rust
use gpui_component::tab::Tab;
```

**基本用法：**

```rust
Tab::new("my-tab")
    .items(vec![
        TabItem::new("Tab 1", content1),
        TabItem::new("Tab 2", content2),
    ])
    .selected(0)
    .on_change(cx.listener(|this, index, _, cx| {
        this.select_tab(index, cx);
    }))
```

#### 4. Navigation（导航栏）

**导入：**

```rust
use gpui_component::navigation::Navigation;
```

**基本用法：**

```rust
Navigation::new()
    .items(nav_items)
    .selected(0)
```

### 反馈组件

#### 1. Alert（警告）

**导入：**

```rust
use gpui_component::alert::Alert;
```

**基本用法：**

```rust
// 信息警告
Alert::info("info-alert", "This is an informational message.")
    .title("Information")

// 成功警告
Alert::success("success-alert", "Operation completed successfully.")
    .title("Success!")

// 警告
Alert::warning("warning-alert", "Please review your settings.")
    .title("Warning")

// 错误警告
Alert::error("error-alert", "An error occurred.")
    .title("Error")
```

**可关闭警告：**

```rust
Alert::info("closable-alert", "This alert can be dismissed.")
    .title("Dismissible")
    .on_close(|_event, _window, _cx| {
        println!("Alert was closed");
    })
```

**横幅模式：**

```rust
Alert::info("banner-alert", "This is a banner alert.")
    .banner()
```

**带 Markdown 内容：**

```rust
use gpui_component::text::TextView;

Alert::error(
    "error-with-markdown",
    TextView::markdown(
        "error-message",
        "Please verify your billing information:\n\
        - Check your card details\n\
        - Ensure sufficient funds\n\
        - Verify billing address",
        window,
        cx,
    ),
)
.title("Payment Failed")
```

#### 2. Notification（通知）

**导入：**

```rust
use gpui_component::notification::Notification;
```

**基本用法：**

```rust
Notification::new()
    .title("Update Available")
    .content("A new version is available.")
    .show(window, cx)
```

**自定义内容：**

```rust
let markdown_content = r#"
## Custom Notification
- **Feature**: New dashboard available
- **Status**: Ready to use
- [Learn more](https://example.com)
"#;

Notification::new()
    .content(|_, window, cx| {
        TextView::markdown(
            "custom-content",
            markdown_content,
            window,
            cx,
        )
        .into_any_element()
    })
```

#### 3. Modal（模态框）

**导入：**

```rust
use gpui_component::modal::Modal;
```

**基本用法：**

```rust
Modal::new("my-modal")
    .title("Confirm Action")
    .content(confirm_content)
    .on_confirm(|_, _, _| {
        // 确认操作
    })
    .show(window, cx)
```

#### 4. Tooltip（工具提示）

**导入：**

```rust
use gpui_component::tooltip::Tooltip;
```

**基本用法：**

```rust
Tooltip::new("This is a tooltip")
    .child(Button::new("btn").label("Hover me"))
```

#### 5. Progress（进度条）

**导入：**

```rust
use gpui_component::progress::Progress;
```

**基本用法：**

```rust
Progress::new()
    .value(0.7)  // 70%
    .width(px(200.0))
```

#### 6. Spinner（加载动画）

**导入：**

```rust
use gpui_component::spinner::Spinner;
```

**基本用法：**

```rust
Spinner::new()
    .size(px(32.0))
```

### 布局组件

#### 1. Dock（停靠布局）

**导入：**

```rust
use gpui_component::dock::Dock;
```

**基本用法：**

```rust
Dock::new("my-dock")
    .panels(panels)
    .layout(dock_layout)
```

#### 2. Panel（面板）

**导入：**

```rust
use gpui_component::panel::Panel;
```

**基本用法：**

```rust
Panel::new("my-panel")
    .title("Panel Title")
    .content(panel_content)
    .resizable(true)
```

#### 3. Collapse（折叠面板）

**导入：**

```rust
use gpui_component::collapse::Collapse;
```

**基本用法：**

```rust
Collapse::new("my-collapse")
    .header("Click to expand")
    .content(collapse_content)
    .open(false)
```

#### 4. Accordion（手风琴）

**导入：**

```rust
use gpui_component::accordion::Accordion;
```

**基本用法：**

```rust
Accordion::new("my-accordion")
    .item(|item| {
        item.title("Section 1")
            .child("Content for section 1")
    })
    .item(|item| {
        item.title("Section 2")
            .child("Content for section 2")
    })
```

**多个展开项：**

```rust
Accordion::new("my-accordion")
    .multiple(true)
    .item(|item| item.title("Section 1").child("Content 1"))
    .item(|item| item.title("Section 2").child("Content 2"))
```

#### 5. Divider（分隔线）

**导入：**

```rust
use gpui_component::divider::Divider;
```

**基本用法：**

```rust
Divider::new()
    .horizontal()

Divider::new()
    .vertical()
```

#### 6. Scroll（滚动容器）

**导入：**

```rust
use gpui_component::scroll::ScrollableElement;
```

**基本用法：**

```rust
div()
    .size_full()
    .overflow_y_scroll()
    .child(long_content)
```

---

## 主题系统

### 内置主题

GPUI Component 提供了 20+ 内置主题，支持暗色模式。

### 使用主题

**设置主题：**

```rust
use gpui_component::theme::Theme;

cx.set_global(Theme::default());
```

**切换主题：**

```rust
Theme::change(appearance, window, cx);
```

**访问主题颜色：**

```rust
let theme = cx.theme();

// 使用主题颜色
div()
    .bg(theme.background)
    .text_color(theme.foreground)
```

### 自定义主题

```rust
use gpui_component::theme::{Theme, ThemeConfig};

let custom_theme = Theme::new(ThemeConfig {
    name: "custom".into(),
    // 自定义颜色配置
});

cx.set_global(custom_theme);
```

---

## 布局系统

### Flex 布局

GPUI 提供了强大的 Flex 布局系统：

```rust
use gpui::{div, h_flex, v_flex};

// 水平布局
h_flex()
    .gap_4()
    .items_center()
    .child("Item 1")
    .child("Item 2")
    .child("Item 3")

// 垂直布局
v_flex()
    .gap_2()
    .child("Item 1")
    .child("Item 2")
    .child("Item 3")

// 使用 div
div()
    .flex()
    .flex_col()
    .gap_4()
    .child("Content")
```

### 尺寸系统

```rust
// 固定尺寸
div()
    .w(px(200.0))
    .h(px(100.0))

// 相对尺寸
div()
    .w_full()
    .h_full()
    .flex_1()

// 最小/最大尺寸
div()
    .min_w(px(100.0))
    .max_w(px(500.0))
```

### 间距系统

```rust
// Gap
div()
    .gap_1()  // 4px
    .gap_2()  // 8px
    .gap_3()  // 12px
    .gap_4()  // 16px
    .gap_5()  // 20px
    .gap_6()  // 24px

// Padding
div()
    .p_4()    // 所有方向
    .px_4()   // 水平方向
    .py_4()   // 垂直方向
    .pt_4()   // 上
    .pb_4()   // 下
    .pl_4()   // 左
    .pr_4()   // 右

// Margin
div()
    .m_4()    // 所有方向
    .mx_4()   // 水平方向
    .my_4()   // 垂直方向
```

### 对齐系统

```rust
// 主轴对齐
div()
    .justify_start()
    .justify_center()
    .justify_end()
    .justify_between()
    .justify_around()

// 交叉轴对齐
div()
    .items_start()
    .items_center()
    .items_end()
    .items_stretch()
```

---

## 最佳实践

### 1. 组件组织

将组件拆分为独立的模块：

```
src/
├── main.rs
├── components/
│   ├── header.rs
│   ├── sidebar.rs
│   ├── content.rs
│   └── mod.rs
├── data.rs
├── theme.rs
└── app.rs
```

### 2. 状态管理

使用实体管理应用状态：

```rust
pub struct AppState {
    pub user: Option<User>,
    pub settings: Settings,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            user: None,
            settings: Settings::default(),
        }
    }
}
```

### 3. 事件处理

使用监听器处理事件：

```rust
Button::new("my-button")
    .label("Click Me")
    .on_click(cx.listener(|this, _, window, cx| {
        this.handle_click(window, cx);
    }))
```

### 4. 性能优化

- 使用虚拟化列表和表格处理大数据集
- 避免在渲染方法中创建大量临时对象
- 使用 `memo` 缓存计算结果

### 5. 错误处理

使用 `Result` 处理错误：

```rust
cx.spawn(|cx| async move {
    let result = risky_operation().await?;
    
    cx.update(|cx| {
        // 更新 UI
    })?;
    
    Ok::<_, anyhow::Error>(())
})
.detach();
```

---

## 示例代码

### 完整应用示例

```rust
use gpui::*;
use gpui_component::{
    button::Button,
    input::Input,
    theme::Theme,
    Root,
};

pub struct MyApp {
    input_value: String,
    count: u32,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            input_value: String::new(),
            count: 0,
        }
    }
}

impl Render for MyApp {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .bg(Theme::global(cx).background)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                Input::new("my-input")
                    .placeholder("Enter text...")
                    .value(self.input_value.clone())
                    .on_change(cx.listener(|this, value, _, _| {
                        this.input_value = value;
                    }))
            )
            .child(
                Button::new("increment")
                    .primary()
                    .label(format!("Count: {}", self.count))
                    .on_click(cx.listener(|this, _, _, _| {
                        this.count += 1;
                    }))
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        gpui_component::init(cx);
        cx.set_global(Theme::default());
        
        cx.open_window(WindowOptions::default(), |window, cx| {
            let view = cx.new(|_| MyApp::new());
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });
}
```

### 设置面板示例

```rust
use gpui_component::{
    setting::{Settings, SettingPage, SettingGroup, SettingItem, SettingField},
    group_box::GroupBoxVariant,
};

Settings::new("my-settings")
    .with_group_variant(GroupBoxVariant::Outline)
    .pages(vec![
        SettingPage::new("General")
            .group(
                SettingGroup::new()
                    .title("Basic Options")
                    .item(
                        SettingItem::new(
                            "Enable Feature",
                            SettingField::switch(
                                |cx: &App| true,
                                |val: bool, cx: &mut App| {
                                    println!("Feature enabled: {}", val);
                                },
                            )
                        )
                    )
            )
    ])
```

---

## 常见问题

### Q: 如何初始化 GPUI Component？

**A:** 在应用程序启动时调用 `gpui_component::init(cx)`：

```rust
Application::new().run(|cx| {
    gpui_component::init(cx);
    // 其他初始化代码
});
```

### Q: 如何处理异步操作？

**A:** 使用 `cx.spawn`：

```rust
cx.spawn(|cx| async move {
    let result = async_operation().await?;
    cx.update(|cx| {
        // 更新 UI
    })?;
    Ok::<_, anyhow::Error>(())
})
.detach();
```

### Q: 如何自定义主题？

**A:** 创建自定义主题并设置：

```rust
let theme = Theme::new(ThemeConfig {
    name: "custom".into(),
    // 自定义配置
});
cx.set_global(theme);
```

### Q: 如何处理大量数据？

**A:** 使用虚拟化组件：

```rust
Table::new(delegate)  // 虚拟化表格
List::new("list").items(items)  // 虚拟化列表
```

---

## 参考资源

- [GPUI 官方网站](https://gpui.rs/)
- [GPUI Component GitHub](https://github.com/longbridge/gpui-component)
- [问题反馈](https://github.com/longbridge/gpui-component/issues)
- [讨论区](https://github.com/longbridge/gpui-component/discussions)
- [API 文档](https://docs.rs/gpui-component)

---

## 更新日志

### v0.5.1
- 新增 60+ 组件
- 改进性能
- 修复已知问题
- 优化主题系统

---

## 贡献指南

欢迎贡献代码！请查看 [GitHub 仓库](https://github.com/longbridge/gpui-component) 了解如何贡献。

---

**文档版本：** 1.0.0  
**最后更新：** 2025-02-19  
**作者：** GPUI Component 社区
