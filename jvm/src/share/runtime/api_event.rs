pub enum ApiEvent {
    ShutDownEvent,
    CallMainMethodEvent { init_class: String },
}