extern crate iui;
use self::iui::prelude::*;
use self::iui::controls::{Label, Button, VerticalBox, Group, MultilineEntry, HorizontalBox, HorizontalSeparator};

use controller::Controller;
use parser;

pub fn init() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");

    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, "Rasm", 1000, 700, WindowType::HasMenubar);
    
    // Create a vertical layout to hold the controls
    let mut hbox = HorizontalBox::new(&ui);
    hbox.set_padded(&ui, true);
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);
    let mut machine_rows = VerticalBox::new(&ui);
    machine_rows.set_padded(&ui, true);
    let mut machine_row_1 = HorizontalBox::new(&ui);
    machine_row_1.set_padded(&ui, true);
    let mut machine_row_2 = HorizontalBox::new(&ui);
    machine_row_2.set_padded(&ui, true);
    let mut machine_row_3 = HorizontalBox::new(&ui);
    machine_row_3.set_padded(&ui, true);
    let mut machine_row_4 = HorizontalBox::new(&ui);
    machine_row_4.set_padded(&ui, true);
    let mut machine_row_5 = HorizontalBox::new(&ui);
    machine_row_5.set_padded(&ui, true);
    let mut machine_row_6 = HorizontalBox::new(&ui);
    machine_row_6.set_padded(&ui, true);

    // Create a new label. Note that labels don't auto-wrap!
    let mut label_text = String::new();
    label_text.push_str("Type your program here");
    let mut label = Label::new(&ui, &label_text);

    let mut entry = MultilineEntry::new(&ui);

    let mut separator = HorizontalSeparator::new(&ui);

    let mut button = Button::new(&ui, "run");

    label_text = String::new();
    label_text.push_str("accumulator");
    let mut label_1_1 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("instriuction_counter");
    let mut label_1_2 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R0");
    let mut label_2_1 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R1");
    let mut label_2_2 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R2");
    let mut label_2_3 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R3");
    let mut label_3_1 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R4");
    let mut label_3_2 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R5");
    let mut label_3_3 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R6");
    let mut label_4_1 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R7");
    let mut label_4_2 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R8");
    let mut label_4_3 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R9");
    let mut label_5_1 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R10");
    let mut label_5_2 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R11");
    let mut label_5_3 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R12");
    let mut label_6_1 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R13");
    let mut label_6_2 = Label::new(&ui, &label_text);
    label_text = String::new();
    label_text.push_str("R14");
    let mut label_6_3 = Label::new(&ui, &label_text);

    button.on_clicked(&ui, {
        let ui = ui.clone();
        let mut label_1_1 = label_1_1.clone();
        let mut label_1_2 = label_1_2.clone();
        let mut label_2_1 = label_2_1.clone();
        let mut label_2_2 = label_2_2.clone();
        let mut label_2_3 = label_2_3.clone();
        let mut label_3_1 = label_3_1.clone();
        let mut label_3_2 = label_3_2.clone();
        let mut label_3_3 = label_3_3.clone();
        let mut label_4_1 = label_4_1.clone();
        let mut label_4_2 = label_4_2.clone();
        let mut label_4_3 = label_4_3.clone();
        let mut label_5_1 = label_5_1.clone();
        let mut label_5_2 = label_5_2.clone();
        let mut label_5_3 = label_5_3.clone();
        let mut label_6_1 = label_6_1.clone();
        let mut label_6_2 = label_6_2.clone();
        let mut label_6_3 = label_6_3.clone();
        let mut entry = entry.clone();
        move |_| {
            label_1_1.set_text(&ui, "successfully clicked");
            let text = entry.value(&ui);
            let commands = parser::parse(&text);
            if commands.0 {
                let mut controller = Controller::new(commands.1);
                let end_state = controller.run();
                label_1_1.set_text(&ui, &end_state.get_registers_to_string()[0]);
                label_1_2.set_text(&ui, &end_state.get_registers_to_string()[1]);
                label_2_1.set_text(&ui, &end_state.get_registers_to_string()[2]);
                label_2_2.set_text(&ui, &end_state.get_registers_to_string()[3]);
                label_2_3.set_text(&ui, &end_state.get_registers_to_string()[4]);
                label_3_1.set_text(&ui, &end_state.get_registers_to_string()[5]);
                label_3_2.set_text(&ui, &end_state.get_registers_to_string()[6]);
                label_3_3.set_text(&ui, &end_state.get_registers_to_string()[7]);
                label_4_1.set_text(&ui, &end_state.get_registers_to_string()[8]);
                label_4_2.set_text(&ui, &end_state.get_registers_to_string()[9]);
                label_4_3.set_text(&ui, &end_state.get_registers_to_string()[10]);
                label_5_1.set_text(&ui, &end_state.get_registers_to_string()[11]);
                label_5_2.set_text(&ui, &end_state.get_registers_to_string()[12]);
                label_5_3.set_text(&ui, &end_state.get_registers_to_string()[13]);
                label_6_1.set_text(&ui, &end_state.get_registers_to_string()[14]);
                label_6_2.set_text(&ui, &end_state.get_registers_to_string()[15]);
                label_6_3.set_text(&ui, &end_state.get_registers_to_string()[16]);
            } else {
                label_1_1.set_text(&ui, "An error occured");
            }
        }
    });


    vbox.append(&ui, label, LayoutStrategy::Compact);
    vbox.append(&ui, entry, LayoutStrategy::Stretchy);
    vbox.append(&ui, button, LayoutStrategy::Compact);

    machine_row_1.append(&ui, label_1_1, LayoutStrategy::Stretchy);
    machine_row_1.append(&ui, label_1_2, LayoutStrategy::Stretchy);
    machine_row_2.append(&ui, label_2_1, LayoutStrategy::Stretchy);
    machine_row_2.append(&ui, label_2_2, LayoutStrategy::Stretchy);
    machine_row_2.append(&ui, label_2_3, LayoutStrategy::Stretchy);
    machine_row_3.append(&ui, label_3_1, LayoutStrategy::Stretchy);
    machine_row_3.append(&ui, label_3_2, LayoutStrategy::Stretchy);
    machine_row_3.append(&ui, label_3_3, LayoutStrategy::Stretchy);
    machine_row_4.append(&ui, label_4_1, LayoutStrategy::Stretchy);
    machine_row_4.append(&ui, label_4_2, LayoutStrategy::Stretchy);
    machine_row_4.append(&ui, label_4_3, LayoutStrategy::Stretchy);
    machine_row_5.append(&ui, label_5_1, LayoutStrategy::Stretchy);
    machine_row_5.append(&ui, label_5_2, LayoutStrategy::Stretchy);
    machine_row_5.append(&ui, label_5_3, LayoutStrategy::Stretchy);
    machine_row_6.append(&ui, label_6_1, LayoutStrategy::Stretchy);
    machine_row_6.append(&ui, label_6_2, LayoutStrategy::Stretchy);
    machine_row_6.append(&ui, label_6_3, LayoutStrategy::Stretchy);

    machine_rows.append(&ui, machine_row_1, LayoutStrategy::Stretchy);
    machine_rows.append(&ui, machine_row_2, LayoutStrategy::Stretchy);
    machine_rows.append(&ui, machine_row_3, LayoutStrategy::Stretchy);
    machine_rows.append(&ui, machine_row_4, LayoutStrategy::Stretchy);
    machine_rows.append(&ui, machine_row_5, LayoutStrategy::Stretchy);
    machine_rows.append(&ui, machine_row_6, LayoutStrategy::Stretchy);

    hbox.append(&ui, vbox, LayoutStrategy::Stretchy);
    hbox.append(&ui, separator, LayoutStrategy::Compact);
    hbox.append(&ui, machine_rows, LayoutStrategy::Stretchy);

    // Actually put the button in the window
    win.set_child(&ui, hbox);
    // Show the window
    win.show(&ui);
    // Run the application
    ui.main();
}