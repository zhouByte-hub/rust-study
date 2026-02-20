use gpui::{
    px, Context, Entity, EventEmitter, InteractiveElement, ParentElement, Render, StatefulInteractiveElement, Styled,
    Subscription, Window,
};
use gpui_component::{
    h_flex,
    input::{Input, InputEvent, InputState},
    scroll::ScrollableElement,
    v_flex,
    StyledExt,
};

#[derive(Clone, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum AppEvent {
    SearchRequested,
    SearchCompleted(Vec<User>),
    SearchFailed(String),
}

pub struct SimpleApp {
    users: Vec<User>,
    search_query: String,
    is_loading: bool,
    error_message: Option<String>,
    search_input: Option<Entity<InputState>>,
    input_subscription: Option<Subscription>,
}

impl SimpleApp {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            search_query: String::new(),
            is_loading: false,
            error_message: None,
            search_input: None,
            input_subscription: None,
        }
    }

    pub fn handle_search(&mut self, cx: &mut Context<Self>) {
        if self.is_loading {
            return;
        }

        self.is_loading = true;
        self.users.clear();
        self.error_message = None;
        cx.emit(AppEvent::SearchRequested);
        cx.notify();

        let query = self.search_query.trim().to_string();
        
        cx.spawn(|this: gpui::WeakEntity<Self>, cx: &mut gpui::AsyncApp| {
            let mut cx = cx.clone();
            async move {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                
                let mock_users = vec![
                    User { id: 1, name: "张三".to_string(), email: "zhangsan@example.com".to_string() },
                    User { id: 2, name: "李四".to_string(), email: "lisi@example.com".to_string() },
                    User { id: 3, name: "王五".to_string(), email: "wangwu@example.com".to_string() },
                    User { id: 4, name: "赵六".to_string(), email: "zhaoliu@example.com".to_string() },
                    User { id: 5, name: "孙七".to_string(), email: "sunqi@example.com".to_string() },
                ];

                let filtered: Vec<User> = if query.is_empty() {
                    mock_users
                } else {
                    mock_users.into_iter()
                        .filter(|u| u.name.contains(&query) || u.email.contains(&query))
                        .collect()
                };

                this.update(&mut cx, |app, cx| {
                    app.is_loading = false;
                    app.users = filtered.clone();
                    
                    if filtered.is_empty() && !query.is_empty() {
                        app.error_message = Some(format!("未找到匹配 '{}' 的用户", query));
                        cx.emit(AppEvent::SearchFailed(format!("未找到匹配 '{}' 的用户", query)));
                    } else {
                        cx.emit(AppEvent::SearchCompleted(filtered));
                    }
                    
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    pub fn handle_input_change(&mut self, new_query: String, cx: &mut Context<Self>) {
        self.search_query = new_query;
        cx.notify();
    }

    fn ensure_input_state(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Entity<InputState> {
        let input = window.use_keyed_state("complete-search-input", cx, |window, cx| {
            InputState::new(window, cx).placeholder("输入用户名或邮箱搜索...")
        });

        if self.search_input.as_ref().map(|state| state.entity_id()) != Some(input.entity_id()) {
            self.search_input = Some(input.clone());
        }

        if self.input_subscription.is_none() {
            let input_clone = input.clone();
            self.input_subscription = Some(cx.subscribe(&input_clone, |this, input, event: &InputEvent, cx| {
                match event {
                    InputEvent::Change => {
                        let value = input.read_with(cx, |state, _| state.value());
                        this.handle_input_change(value.to_string(), cx);
                    }
                    InputEvent::PressEnter { .. } => {
                        this.handle_search(cx);
                    }
                    _ => {}
                }
            }));
        }

        input
    }
}

impl EventEmitter<AppEvent> for SimpleApp {}

impl Render for SimpleApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .size_full()
            .bg(gpui::rgb(0x1e1e1e))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p(px(20.0))
            .gap(px(16.0))
            .child(self.render_title())
            .child(self.render_search_bar(window, cx))
            .child(self.render_content())
    }
}

impl SimpleApp {
    fn render_title(&self) -> impl gpui::IntoElement {
        gpui::div()
            .text_2xl()
            .font_semibold()
            .text_color(gpui::rgb(0xffffff))
            .child("用户搜索系统")
    }

    fn render_search_bar(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let input = self.ensure_input_state(window, cx);

        h_flex()
            .w(px(500.0))
            .h(px(44.0))
            .gap(px(12.0))
            .child(
                Input::new(&input)
                    .h_full()
                    .w_full()
                    .cleanable(true),
            )
            .child(
                gpui::div()
                    .w(px(100.0))
                    .h_full()
                    .bg(if self.is_loading {
                        gpui::rgb(0x005a9e)
                    } else {
                        gpui::rgb(0x007acc)
                    })
                    .rounded_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(gpui::rgb(0xffffff))
                    .font_semibold()
                    .child(if self.is_loading { "搜索中..." } else { "搜索" })
                    .id("complete-search-button")
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.handle_search(cx);
                    })),
            )
    }

    fn render_content(&self) -> impl gpui::IntoElement {
        gpui::div()
            .w(px(500.0))
            .h(px(400.0))
            .bg(gpui::rgb(0x2d2d2d))
            .rounded_md()
            .border_1()
            .border_color(gpui::rgb(0x3d3d3d))
            .flex()
            .flex_col()
            .child(self.render_content_header())
            .child(self.render_user_list())
    }

    fn render_content_header(&self) -> impl gpui::IntoElement {
        gpui::div()
            .w_full()
            .h(px(50.0))
            .px(px(16.0))
            .flex()
            .items_center()
            .border_b_1()
            .border_color(gpui::rgb(0x3d3d3d))
            .child(
                gpui::div()
                    .text_base()
                    .font_semibold()
                    .text_color(gpui::rgb(0xffffff))
                    .child(format!("用户列表 ({})", self.users.len())),
            )
    }

    fn render_user_list(&self) -> impl gpui::IntoElement {
        gpui::div()
            .flex_1()
            .w_full()
            .overflow_y_scrollbar()
            .p(px(16.0))
            .child(
                v_flex()
                    .w_full()
                    .gap(px(12.0))
                    .children(self.render_state_content()),
            )
    }

    fn render_state_content(&self) -> Vec<gpui::Div> {
        if self.is_loading {
            vec![
                gpui::div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(40.0))
                    .child(
                        v_flex()
                            .gap(px(12.0))
                            .items_center()
                            .child(gpui::div().text_2xl().child("⏳"))
                            .child(
                                gpui::div()
                                    .text_color(gpui::rgb(0x999999))
                                    .child("正在搜索..."),
                            ),
                    ),
            ]
        } else if let Some(error) = &self.error_message {
            vec![
                gpui::div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(40.0))
                    .child(
                        v_flex()
                            .gap(px(12.0))
                            .items_center()
                            .child(gpui::div().text_2xl().child("❌"))
                            .child(
                                gpui::div()
                                    .text_color(gpui::rgb(0xff6b6b))
                                    .child(error.clone()),
                            ),
                    ),
            ]
        } else if self.users.is_empty() {
            vec![
                gpui::div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(40.0))
                    .child(
                        v_flex()
                            .gap(px(12.0))
                            .items_center()
                            .child(gpui::div().text_2xl().child("🔍"))
                            .child(
                                gpui::div()
                                    .text_color(gpui::rgb(0x999999))
                                    .child("点击搜索按钮开始查询"),
                            ),
                    ),
            ]
        } else {
            self.users
                .iter()
                .map(|user| self.render_user_item(user))
                .collect()
        }
    }

    fn render_user_item(&self, user: &User) -> gpui::Div {
        let initial = user.name.chars().next().unwrap_or('?').to_string();
        
        gpui::div()
            .w_full()
            .p(px(16.0))
            .bg(gpui::rgb(0x3d3d3d))
            .rounded_md()
            .border_1()
            .border_color(gpui::rgb(0x4d4d4d))
            .hover(|style| {
                style
                    .bg(gpui::rgb(0x4d4d4d))
                    .border_color(gpui::rgb(0x007acc))
            })
            .child(
                h_flex()
                    .w_full()
                    .gap(px(12.0))
                    .items_center()
                    .child(
                        gpui::div()
                            .w(px(40.0))
                            .h(px(40.0))
                            .bg(gpui::rgb(0x007acc))
                            .rounded_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(gpui::rgb(0xffffff))
                            .font_semibold()
                            .child(initial),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .gap(px(4.0))
                            .child(
                                gpui::div()
                                    .text_base()
                                    .font_semibold()
                                    .text_color(gpui::rgb(0xffffff))
                                    .child(user.name.clone()),
                            )
                            .child(
                                gpui::div()
                                    .text_sm()
                                    .text_color(gpui::rgb(0x999999))
                                    .child(user.email.clone()),
                            ),
                    )
                    .child(
                        gpui::div()
                            .text_xs()
                            .text_color(gpui::rgb(0x666666))
                            .child(format!("ID: {}", user.id)),
                    ),
            )
    }
}
