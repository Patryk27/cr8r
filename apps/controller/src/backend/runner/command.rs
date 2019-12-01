use futures_channel::mpsc;

pub type RunnerCommandTx = mpsc::UnboundedSender<RunnerCommand>;
pub type RunnerCommandRx = mpsc::UnboundedReceiver<RunnerCommand>;

#[derive(Debug)]
pub enum RunnerCommand {
    //
}