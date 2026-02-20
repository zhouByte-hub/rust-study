use gpui::{
    px, Context, Entity, InteractiveElement, ParentElement, Render, StatefulInteractiveElement, Styled, Subscription,
    Window,
};
use gpui_component::{
    h_flex,
    input::{Input, InputEvent, InputState},
    scroll::ScrollableElement,
    v_flex,
    StyledExt,
};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub struct SimpleApp {
    users: Vec<User>,
    search_query: String,
    is_loading: bool,
    search_input: Option<Entity<InputState>>,
    input_subscription: Option<Subscription>,
}

impl SimpleApp {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            search_query: String::new(),
            is_loading: false,
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
                    app.users = filtered;
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    fn ensure_input_state(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Entity<InputState> {
        let input = window.use_keyed_state("simple-search-input", cx, |window, cx| {
            InputState::new(window, cx).placeholder("输入用户名或邮箱...")
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
                        this.search_query = value.to_string();
                        cx.notify();
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

impl Render for SimpleApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .size_full()
            .bg(gpui::rgb(0x1e1e1e))
            .flex()
            .flex_col()
            .p(px(20.0))
            .gap(px(16.0))
            .child(self.render_search_bar(window, cx))
            .child(self.render_user_list())
    }
}

impl SimpleApp {
    fn render_search_bar(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let input = self.ensure_input_state(window, cx);

        h_flex()
            .w(px(400.0))
            .h(px(40.0))
            .gap(px(8.0))
            .child(
                Input::new(&input)
                    .h_full()
                    .w_full()
                    .cleanable(true),
            )
            .child(
                gpui::div()
                    .w(px(80.0))
                    .h_full()
                    .bg(gpui::rgb(0x007acc))
                    .rounded_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(gpui::rgb(0xffffff))
                    .child(if self.is_loading { "搜索中..." } else { "搜索" })
                    .id("simple-search-button")
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.handle_search(cx);
                    })),
            )
    }

    fn render_user_list(&self) -> impl gpui::IntoElement {
        gpui::div()
            .flex_1()
            .w(px(400.0))
            .bg(gpui::rgb(0x2d2d2d))
            .rounded_md()
            .overflow_y_scrollbar()
            .child(
                v_flex()
                    .w_full()
                    .p(px(16.0))
                    .gap(px(12.0))
                    .children(self.users.iter().map(|user| {
                        self.render_user_item(user)
                    })),
            )
    }

    fn render_user_item(&self, user: &User) -> impl gpui::IntoElement {
        gpui::div()
            .w_full()
            .p(px(12.0))
            .bg(gpui::rgb(0x3d3d3d))
            .rounded_md()
            .child(
                v_flex()
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
    }
}
