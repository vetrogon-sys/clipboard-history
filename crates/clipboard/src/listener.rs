pub trait ClipboardListener {
    fn start(&mut self) -> anyhow::Result<()>;
}