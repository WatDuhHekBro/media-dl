use std::process::{ExitStatus, Stdio};
use std::sync::mpsc::{channel, Sender};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub enum Mode {
    Default,
    Flags,
    Processing,
    Done,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Default
    }
}

pub enum Message {
    Update(String),
    Exit(ExitStatus),
}

// --all-subs --skip-download
// youtube-dl --all-subs --skip-download
// https://github.com/ytdl-org/youtube-dl/issues/8114

pub async fn spawn_downloader(
    tx: Sender<Message>,
    args: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("youtube-dl");

    // Add the args in a loop. Apparently using args() with the vector itself won't work.
    for arg in args {
        child.arg(arg);
    }

    // Intercept the output from stdout to this program (stdin).
    let mut child = child.stdout(Stdio::piped()).spawn().expect(
        "Failed to spawn command, you most likely don't have \"youtube-dl\" in your PATH variable.",
    );

    let stdout = child
        .stdout
        .take()
        .expect("Child did not have a handle to stdout.");

    let mut reader = BufReader::new(stdout).lines();

    // Spawn a child to continue processing the command.
    // This is needed to move on and read the output from stdin.
    // Create another channel to signal to this thread that time's up.
    let (sender, receiver) = channel();

    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        sender.send(Message::Exit(status)).unwrap();
    });

    // Continue sending updates from youtube-dl's output in the meantime.
    while let Some(line) = reader.next_line().await? {
        tx.send(Message::Update(line)).unwrap();
    }

    // After the output finishes, wait for the child process to finish and send its exit status, then relay it to the GUI.
    let result = receiver.recv().unwrap();
    tx.send(result).unwrap();

    Ok(())
}
