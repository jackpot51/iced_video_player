// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use cosmic::app::{Command, Core, Settings};
use cosmic::iced_core::{Length, Size};
use cosmic::widget::{self, Column, Row, Slider};
use cosmic::{executor, Element};
use iced_video_player::{Video, VideoPlayer};
use std::time::Duration;

/// Runs application with these settings
#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default()
        .size(Size::new(1024., 768.));

        let video = Video::new(
            &url::Url::from_file_path(
                std::env::args().nth(1).unwrap()
            )
            .unwrap(),
        )
        .unwrap();

    cosmic::app::run::<App>(settings, video)?;

    Ok(())
}

/// Messages that are used specifically by our [`App`].
#[derive(Clone, Debug)]
pub enum Message {
    TogglePause,
    ToggleLoop,
    Seek(f64),
    SeekRelease,
    EndOfStream,
    NewFrame,
}

/// The [`App`] stores application-specific state.
pub struct App {
    core: Core,
    video: Video,
    position: f64,
    dragging: bool,
}

/// Implement [`cosmic::Application`] to integrate with COSMIC.
impl cosmic::Application for App {
    /// Default async executor to use with the app.
    type Executor = executor::Default;

    /// Argument received [`cosmic::Application::new`].
    type Flags = Video;

    /// Message type specific to our [`App`].
    type Message = Message;

    /// The unique application ID to supply to the window manager.
    const APP_ID: &'static str = "org.cosmic.AppDemo";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Creates the application, and optionally emits command on initialize.
    fn init(core: Core, video: Self::Flags) -> (Self, Command<Self::Message>) {
        (App { core, video, position: 0.0, dragging: false }, Command::none())
    }

    /// Handle application events here.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TogglePause => {
                self.video.set_paused(!self.video.paused());
            }
            Message::ToggleLoop => {
                self.video.set_looping(!self.video.looping());
            }
            Message::Seek(secs) => {
                self.dragging = true;
                self.position = secs;
                self.video
                    .seek(Duration::from_secs_f64(self.position), false)
                    .expect("seek");
                self.video.set_paused(false);
            }
            Message::SeekRelease => {
                self.dragging = false;
                self.video
                    .seek(Duration::from_secs_f64(self.position), true)
                    .expect("seek");
                self.video.set_paused(false);
            }
            Message::EndOfStream => {
                println!("end of stream");
            }
            Message::NewFrame => {
                if self.dragging {
                    self.video.set_paused(true);
                } else {
                    self.position = self.video.position().as_secs_f64();
                }
            }
        }
        Command::none()
    }

    /// Creates a view after each update.
    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(widget::vertical_space(Length::Fill))
            .push(
                VideoPlayer::new(&self.video)
                    .on_end_of_stream(Message::EndOfStream)
                    .on_new_frame(Message::NewFrame)
                    .width(Length::Fill)
            )
            .push(widget::vertical_space(Length::Fill))
            .push(
                Row::new()
                    .height(Length::Fixed(16.0))
                    .spacing(8)
                    .push(
                        widget::button::icon(if self.video.paused() {
                            widget::icon::from_name("media-playback-start-symbolic").size(16)
                        } else {
                            widget::icon::from_name("media-playback-pause-symbolic").size(16)
                        })
                        .on_press(Message::TogglePause),
                    )
                    .push(widget::text(format!(
                        "{:#?}s / {:#?}s",
                        self.position as u64,
                        self.video.duration().as_secs()
                    )))
                    .push(
                        Slider::new(
                            0.0..=self.video.duration().as_secs_f64(),
                            self.position,
                            Message::Seek,
                        )
                        .step(0.1)
                        .on_release(Message::SeekRelease),
                    ),
            )
            .into()
    }
}
