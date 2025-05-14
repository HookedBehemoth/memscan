use egui_extras::{Column, TableBuilder};

#[derive(Clone)]
pub struct Process {
    pub pid: i32,
    pub uid: u32,
    pub cmd: String,
    pub selected: bool,
}

impl Process {
    fn from(proc: procfs::process::Process) -> Self {
        let stat = proc.stat().unwrap();
        let pid = proc.pid;
        let uid = proc.uid().unwrap_or_default();
        let cmd = stat.comm.clone();

        Self {
            pid,
            uid,
            cmd,
            selected: false,
        }
    }
}

#[derive(Default)]
pub struct ProcessPicker {
    processes: Vec<Process>,
}

impl ProcessPicker {
    pub fn new() -> Self {
        Self {
            processes: Self::fetch_processes(),
        }
    }

    fn fetch_processes() -> Vec<Process> {
        let mut processes = vec![];
        for prc in procfs::process::all_processes().unwrap() {
            let Ok(prc) = prc else {
                continue;
            };

            processes.push(Process::from(prc));
        }
        processes
    }

    pub fn selected_processes(&self) -> Vec<Process> {
        self.processes
            .iter()
            .filter(|prc| prc.selected)
            .cloned()
            .collect()
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.button("Refresh").clicked().then(|| {
            self.processes = Self::fetch_processes();
        });
        ui.label("Running processes:");
        
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().at_least(50.0))
            .column(Column::auto().at_least(50.0))
            .column(Column::auto().at_least(50.0))
            .column(
                Column::remainder(),
            )
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label("Selected");
                });
                header.col(|ui| {
                    ui.label("PID");
                });
                header.col(|ui| {
                    ui.label("UID");
                });
                header.col(|ui| {
                    ui.label("CMD");
                });
            })
            .body(|mut body| {
                for prc in self.processes.iter_mut().rev() {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            ui.checkbox(&mut prc.selected, "").clicked().then(|| {});
                        });
                        row.col(|ui| {
                            ui.label(prc.pid.to_string());
                        });
                        row.col(|ui| {
                            ui.label(prc.uid.to_string());
                        });
                        row.col(|ui| {
                            ui.label(&prc.cmd);
                        });
                    });
                }
            });
    }
}
