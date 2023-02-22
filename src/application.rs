use eframe::egui;
use chrono::DateTime;

use crate::server_interface::ServerInterface;
use crate::message::Message;
use crate::user_state::{UserState, UserStatus};



// Application
pub struct Application{
    server_interface: ServerInterface,
    user_state: UserState,
    current_message: String,
}

impl Default for Application{
    fn default() -> Self {
        Self {
            server_interface: ServerInterface::default(),
            user_state: UserState::default(),
            current_message: "".to_owned()
        }
    }
}

impl Application{
    fn create_message_from_state(&self) -> Message {
        Message {
            sender: self.user_state.name.clone(),
            contents: self.current_message.clone(),
            time: chrono::offset::Utc::now(),
        }
    }

    fn log_in(&mut self){
        self.user_state.user_status = UserStatus::LoggedIn;
        self.server_interface.user_list.push(self.user_state.clone());
    }

    fn draw_login_ui(&mut self, ctx: &egui::Context, _frame: &eframe::Frame){
        egui::CentralPanel::default().show(&ctx, |ui|{
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui|{
                ui.label("Select a Username");
                let _output = egui::TextEdit::singleline(&mut self.user_state.name).id("UsernameInput".into()).show(ui);
                ctx.memory_mut(|m|{
                    m.request_focus("UsernameInput".into());
                    m.lock_focus("UsernameInput".into(), true);
                });
                let enter_pressed =  ctx.input(|i| i.key_pressed(egui::Key::Enter));
                let login_clicked = ui.button("Login").clicked();
                let name_is_something = !self.user_state.name.is_empty();
                if (enter_pressed || login_clicked) && name_is_something {
                    self.log_in();
                }
            });

            ui.allocate_space(ui.available_size());
        });
    }

    fn draw_chat_ui(&mut self, ctx: &egui::Context, _frame: &eframe::Frame){
        let mut message_sent = false;

        egui::SidePanel::right("ActiveUsersPanle").show(ctx, |ui|{
            ui.colored_label(
                egui::Color32::LIGHT_BLUE,
                "Actice Users"
            );
            for user_state in self.server_interface.user_list.iter() {
                ui.horizontal(|ui|{
                    ui.label(format!("{} - Status {}", user_state.name, user_state.user_status.as_str()));
                    ui.painter().circle_filled(
                        ui.next_widget_position(),
                        4.0,
                        user_state.user_status.color()
                    );
                });
            }
            ui.allocate_space(ui.available_size());
        });
        
        egui::TopBottomPanel::bottom("TextEntryPanel").show(ctx, |ui|{
            ui.add_space(4.0);
            ui.horizontal(|ui|{
                let _output = egui::TextEdit::singleline(&mut self.current_message)
                    .hint_text("Say Hello!")
                    .id("TextEntry".into())
                    .show(ui);
                ctx.memory_mut(|m|{
                    m.request_focus("TextEntry".into());
                    m.lock_focus("TextEntry".into(), true);
                });
                if ctx.input(|i| i.key_pressed(egui::Key::Enter)){
                    message_sent = true;
                }
                if ui.button("Send").clicked(){
                    message_sent = true;
                }
                 ui.allocate_space(ui.available_size());
            });
            ui.allocate_space(ui.available_size());
        });

        egui::CentralPanel::default().show(ctx, |ui|{
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui|{
                    for msg in self.server_interface.message_history.iter(){
                        let msg_local_time: DateTime<chrono::Local> = DateTime::from(msg.time);
                        let formatted_time = format!("{}", msg_local_time.format("%H:%M"));
                        let text = format!("{} - {} : {}", formatted_time, msg.sender.clone(), msg.contents.clone());
                        ui.add(egui::Label::new(text).wrap(true));
                    }
                    ui.allocate_space(ui.available_size());
                });
            ui.allocate_space(ui.available_size());
        });

        // process
        if message_sent {  
            let msg = self.create_message_from_state();
            self.server_interface.send_message(msg);
            self.current_message = "".to_owned();
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.user_state.user_status {
            UserStatus::LoggedOut => {
                self.draw_login_ui(ctx, _frame);
            },
            _ => {
                self.draw_chat_ui(ctx, _frame);
            }
        }
    }
}