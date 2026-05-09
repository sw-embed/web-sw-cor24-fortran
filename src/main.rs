//! FORTRAN Hello World on COR24 — multi-stage in-browser pipeline.
//!
//!   .f source  --[Compile  -> snobol4.lgo + fortran.sno (nested emu)]--> .s
//!      .s     --[Assemble -> cor24-assembler                        ]--> bytes + listing
//!     bytes   --[Run      -> cor24-emulator                         ]--> UART output

mod compiler;
mod demos;
mod editor;
mod help;
mod highlight;
mod panels;

use std::cell::RefCell;
use std::rc::Rc;

use cor24_assembler::AssembledLine;
use cor24_emulator::{EmulatorCore, StopReason};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use editor::Editor;
use help::HelpModal;
use panels::UartPanel;

const TICK_INTERVAL_MS: u32 = 16;
const RUN_BATCH: u64 = 100_000;

#[function_component(App)]
fn app() -> Html {
    let source = use_state(|| demos::DEFAULT_SOURCE.to_string());

    let asm = use_state(String::new);
    let driver_log = use_state(String::new);
    let via_path_a = use_state(|| false);
    let compile_error = use_state(|| None::<compiler::CompileError>);

    let listing = use_state(Vec::<AssembledLine>::new);
    let bytes_out = use_state(Vec::<u8>::new);
    let assemble_error = use_state(|| None::<String>);

    let uart_output = use_state(String::new);
    let running = use_state(|| false);
    let halted = use_state(|| false);
    let instr_count = use_state(|| 0u64);
    let runtime_error = use_state(|| None::<String>);

    let status_msg = use_state(|| String::from("Ready. Edit the source, then click Compile."));
    let help_open = use_state(|| false);

    let emu: Rc<RefCell<EmulatorCore>> = use_mut_ref(EmulatorCore::new);
    let interval_handle = use_mut_ref(|| None::<gloo_timers::callback::Interval>);

    let clear_downstream = {
        let asm = asm.clone();
        let driver_log = driver_log.clone();
        let via_path_a = via_path_a.clone();
        let compile_error = compile_error.clone();
        let listing = listing.clone();
        let bytes_out = bytes_out.clone();
        let assemble_error = assemble_error.clone();
        let uart_output = uart_output.clone();
        let running = running.clone();
        let halted = halted.clone();
        let runtime_error = runtime_error.clone();
        let interval_handle = interval_handle.clone();
        move || {
            *interval_handle.borrow_mut() = None;
            asm.set(String::new());
            driver_log.set(String::new());
            via_path_a.set(false);
            compile_error.set(None);
            listing.set(Vec::new());
            bytes_out.set(Vec::new());
            assemble_error.set(None);
            uart_output.set(String::new());
            running.set(false);
            halted.set(false);
            runtime_error.set(None);
        }
    };

    let on_source_change = {
        let source = source.clone();
        let clear_downstream = clear_downstream.clone();
        let status_msg = status_msg.clone();
        Callback::from(move |value: String| {
            source.set(value);
            clear_downstream();
            status_msg.set("Edited \u{2014} click Compile.".into());
        })
    };

    let on_demo_select = {
        let source = source.clone();
        let clear_downstream = clear_downstream.clone();
        let status_msg = status_msg.clone();
        Callback::from(move |e: Event| {
            let Some(select) = e.target().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            else {
                return;
            };
            let value = select.value();
            if value.is_empty() {
                return;
            }
            select.set_value("");
            if let Some(src) = demos::lookup(&value) {
                source.set(src.to_string());
                clear_downstream();
                status_msg.set(format!("Loaded {value}. Click Compile."));
            }
        })
    };

    let do_compile = {
        let source = source.clone();
        let asm = asm.clone();
        let driver_log = driver_log.clone();
        let via_path_a = via_path_a.clone();
        let compile_error = compile_error.clone();
        let status_msg = status_msg.clone();
        move || -> Option<String> {
            status_msg.set("Compiling (running snobol4.lgo + fortran.sno)...".into());
            let r = compiler::compile(&source);
            driver_log.set(r.driver_log);
            via_path_a.set(r.via_path_a);
            if let Some(err) = r.error {
                asm.set(String::new());
                status_msg.set("Compile failed (driver did not emit assembly)".into());
                compile_error.set(Some(err));
                None
            } else {
                compile_error.set(None);
                asm.set(r.asm.clone());
                status_msg.set(if r.via_path_a {
                    "Compiled (Path-A: dcftn's hello.s)".into()
                } else {
                    format!("Compiled ({} lines of .s)", r.asm.lines().count())
                });
                Some(r.asm)
            }
        }
    };

    let do_assemble = {
        let listing = listing.clone();
        let bytes_out = bytes_out.clone();
        let assemble_error = assemble_error.clone();
        let status_msg = status_msg.clone();
        move |asm_text: &str| -> Option<Vec<u8>> {
            let r = compiler::assemble(asm_text);
            listing.set(r.listing.clone());
            if let Some(e) = r.error {
                assemble_error.set(Some(e));
                bytes_out.set(Vec::new());
                status_msg.set("Assembler error".into());
                None
            } else {
                assemble_error.set(None);
                bytes_out.set(r.bytes.clone());
                status_msg.set(format!("Assembled ({} bytes)", r.bytes.len()));
                Some(r.bytes)
            }
        }
    };

    let on_compile = {
        let do_compile = do_compile.clone();
        Callback::from(move |_: MouseEvent| {
            do_compile();
        })
    };

    let on_assemble = {
        let do_compile = do_compile.clone();
        let do_assemble = do_assemble.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(asm_text) = do_compile() {
                do_assemble(&asm_text);
            }
        })
    };

    let on_run = {
        let do_compile = do_compile.clone();
        let do_assemble = do_assemble.clone();
        let emu = emu.clone();
        let interval_handle = interval_handle.clone();
        let uart_output = uart_output.clone();
        let running = running.clone();
        let halted = halted.clone();
        let instr_count = instr_count.clone();
        let runtime_error = runtime_error.clone();
        let status_msg = status_msg.clone();
        Callback::from(move |_: MouseEvent| {
            *interval_handle.borrow_mut() = None;

            let Some(asm_text) = do_compile() else { return };
            let Some(prog_bytes) = do_assemble(&asm_text) else { return };

            {
                let mut e = emu.borrow_mut();
                *e = EmulatorCore::new();
                e.load_program(0, &prog_bytes);
                e.load_program_extent(prog_bytes.len() as u32);
                e.resume();
            }
            uart_output.set(String::new());
            runtime_error.set(None);
            instr_count.set(0);
            halted.set(false);
            running.set(true);
            status_msg.set("Running".into());

            let emu = emu.clone();
            let uart_output = uart_output.clone();
            let running = running.clone();
            let halted = halted.clone();
            let instr_count = instr_count.clone();
            let runtime_error = runtime_error.clone();
            let status_msg = status_msg.clone();
            let interval_handle2 = interval_handle.clone();

            let interval = gloo_timers::callback::Interval::new(TICK_INTERVAL_MS, move || {
                let mut e = emu.borrow_mut();
                let batch = e.run_batch(RUN_BATCH);
                uart_output.set(e.get_uart_output().to_string());
                instr_count.set(e.instructions_count());

                let stop = match batch.reason {
                    StopReason::Halted => {
                        halted.set(true);
                        status_msg.set("Halted".into());
                        true
                    }
                    StopReason::InvalidInstruction(op) => {
                        halted.set(true);
                        runtime_error.set(Some(format!(
                            "Invalid instruction: {op:#04x} at PC={:#06x}",
                            e.pc()
                        )));
                        status_msg.set("Runtime error".into());
                        true
                    }
                    StopReason::Paused => {
                        status_msg.set("Paused".into());
                        true
                    }
                    _ => false,
                };
                if stop {
                    running.set(false);
                    *interval_handle2.borrow_mut() = None;
                }
            });
            *interval_handle.borrow_mut() = Some(interval);
        })
    };

    let on_stop = {
        let emu = emu.clone();
        let interval_handle = interval_handle.clone();
        let running = running.clone();
        let status_msg = status_msg.clone();
        Callback::from(move |_: MouseEvent| {
            emu.borrow_mut().pause();
            *interval_handle.borrow_mut() = None;
            running.set(false);
            status_msg.set("Stopped".into());
        })
    };

    let open_help = {
        let help_open = help_open.clone();
        Callback::from(move |_: MouseEvent| help_open.set(true))
    };
    let close_help = {
        let help_open = help_open.clone();
        Callback::from(move |_: MouseEvent| help_open.set(false))
    };

    html! {
        <main style="display:flex; flex-direction:column; height:100vh; padding:12px; gap:8px; box-sizing:border-box;">
            <div style="display:flex; align-items:center; gap:10px;">
                <h1 style="font-size:1.2rem; color:#89b4fa; margin:0;">
                    {"FORTRAN Hello World on COR24"}
                </h1>
                <span style="font-size:0.78rem; color:#bac2de;">
                    {"cor24-emulator \u{2192} snobol4.lgo \u{2192} fortran.sno \u{2192} <demo>.f \u{2192} cor24-assembler \u{2192} cor24-emulator. Live."}
                </span>
                <button onclick={open_help} title="Help"
                    style="margin-left:auto; padding:4px 10px; \
                           background:#313244; color:#cdd6f4; \
                           border:1px solid #585b70; border-radius:5px; \
                           font-size:0.85rem; cursor:pointer;">
                    {"[?]"}
                </button>
            </div>

            <div style="display:flex; flex:1; gap:10px; min-height:0;">
                <div style="flex:1; min-width:0; display:flex; flex-direction:column; gap:4px;">
                    <label style="color:#cdd6f4; font-weight:600; font-size:0.85rem;">
                        {"FTI-0 source (.f)"}
                    </label>
                    <Editor value={AttrValue::from((*source).clone())} on_change={on_source_change} />
                </div>

                <div style="flex:1; min-width:0; display:flex; flex-direction:column; gap:8px;">
                    <div style="flex:1; min-height:0; display:flex; flex-direction:column; gap:4px;">
                        <label style="color:#cdd6f4; font-weight:600; font-size:0.85rem;">
                            {"Compiler output: COR24 assembly (.s)"}
                            if *via_path_a {
                                <span style="color:#f9e2af; font-weight:400; margin-left:8px;">
                                    {"\u{00b7} Path-A short-circuit"}
                                </span>
                            }
                        </label>
                        if let Some(err) = compile_error.as_ref() {
                            <pre style="margin:0; padding:10px; background:#181825; \
                                        border:1px solid #f38ba8; border-radius:6px; \
                                        color:#f38ba8; font-size:0.82rem; \
                                        white-space:pre-wrap; overflow:auto; flex:1; min-height:0;">
                                { &err.message }
                            </pre>
                        } else {
                            <pre style="margin:0; padding:10px; background:#181825; \
                                        border:1px solid #313244; border-radius:6px; \
                                        color:#cdd6f4; font-family:monospace; font-size:11.5px; \
                                        line-height:1.45; white-space:pre; overflow:auto; \
                                        flex:1; min-height:0;">
                                { if asm.is_empty() {
                                    html! { <span style="color:#a6adc8;">{"(click Compile)"}</span> }
                                } else {
                                    html! { (*asm).clone() }
                                }}
                            </pre>
                        }
                    </div>

                    <div style="flex:1; min-height:0; display:flex; flex-direction:column; gap:4px;">
                        <label style="color:#cdd6f4; font-weight:600; font-size:0.85rem;">
                            {"Assembler output: listing"}
                        </label>
                        if let Some(err) = assemble_error.as_ref() {
                            <pre style="margin:0; padding:10px; background:#181825; \
                                        border:1px solid #f38ba8; border-radius:6px; \
                                        color:#f38ba8; font-size:0.82rem; \
                                        white-space:pre-wrap; overflow:auto; flex:1; min-height:0;">
                                { err }
                            </pre>
                        } else {
                            { panels::listing::render(&listing, None) }
                        }
                    </div>
                </div>

                <div style="flex:1; min-width:0; display:flex; flex-direction:column; gap:4px;">
                    <label style="color:#cdd6f4; font-weight:600; font-size:0.85rem;">
                        {"Run output: UART"}
                    </label>
                    if let Some(err) = runtime_error.as_ref() {
                        <pre style="margin:0; padding:10px; background:#181825; \
                                    border:1px solid #f38ba8; border-radius:6px; \
                                    color:#f38ba8; font-size:0.82rem; \
                                    white-space:pre-wrap; overflow:auto;">
                            { err }
                        </pre>
                    }
                    <UartPanel
                        output={AttrValue::from((*uart_output).clone())}
                        running={*running}
                        halted={*halted}
                        placeholder={AttrValue::from("(click Run \u{2014} executes on the COR24 emulator)")}
                    />
                </div>
            </div>

            <div style="display:flex; gap:8px; align-items:center; flex-wrap:wrap;">
                <button onclick={on_compile}
                    style="padding:6px 14px; background:#cba6f7; color:#1e1e2e; \
                           border:none; border-radius:5px; font-size:0.9rem; font-weight:600; cursor:pointer;">
                    {"Compile"}
                </button>
                <button onclick={on_assemble}
                    style="padding:6px 14px; background:#f9e2af; color:#1e1e2e; \
                           border:none; border-radius:5px; font-size:0.9rem; font-weight:600; cursor:pointer;">
                    {"Assemble"}
                </button>
                <button onclick={on_run}
                    style="padding:6px 14px; background:#89b4fa; color:#1e1e2e; \
                           border:none; border-radius:5px; font-size:0.9rem; font-weight:600; cursor:pointer;">
                    {"Run"}
                </button>
                if *running {
                    <button onclick={on_stop}
                        style="padding:6px 14px; background:#f38ba8; color:#1e1e2e; \
                               border:none; border-radius:5px; font-size:0.9rem; font-weight:600; cursor:pointer;">
                        {"Stop"}
                    </button>
                }
                <select onchange={on_demo_select}
                    style="padding:6px 12px; background:#313244; color:#cdd6f4; \
                           border:1px solid #585b70; border-radius:5px; \
                           font-size:0.85rem; cursor:pointer; margin-left:6px;">
                    <option value="" selected=true disabled=true>{"Load demo..."}</option>
                    { for demos::DEMOS.iter().map(|d| html! {
                        <option value={d.id}>{format!("{} \u{2014} {}", d.id, d.label)}</option>
                    }) }
                </select>
                <span style="color:#bac2de; font-size:0.82rem; margin-left:6px;">{ &*status_msg }</span>
                <span style="color:#a6adc8; font-size:0.82rem; margin-left:auto;">
                    { format!("{} instructions", *instr_count) }
                </span>
            </div>

            if !driver_log.is_empty() {
                <details style="font-size:0.82rem;">
                    <summary style="color:#bac2de; cursor:pointer; user-select:none;">
                        {"SNOBOL4 driver output (snobol4.lgo running fortran.sno on your .f)"}
                    </summary>
                    <pre style="margin:6px 0 0 0; padding:8px; background:#11111b; \
                                border:1px solid #313244; border-radius:4px; \
                                color:#cdd6f4; font-family:monospace; font-size:11.5px; \
                                line-height:1.4; white-space:pre-wrap; \
                                max-height:160px; overflow:auto;">
                        { (*driver_log).clone() }
                    </pre>
                </details>
            }

            <div style="display:flex; gap:6px; align-items:center; flex-wrap:wrap; \
                        font-size:0.75rem; color:#bac2de; padding-top:6px; border-top:1px solid #313244;">
                <span>{"\u{00a9} 2026 Michael A. Wright"}</span>
                <span>{"\u{00b7}"}</span>
                <span>{"MIT"}</span>
                <span>{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/web-sw-cor24-fortran" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"Source"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/sw-cor24-fortran" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"Upstream Fortran compiler"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/sw-cor24-snobol4" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"SNOBOL4"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://makerlisp.com" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"COR24-TB"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://software-wrighter-lab.github.io/" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"Blog"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://discord.com/invite/Ctzk5uHggZ" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"Discord"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://www.youtube.com/@SoftwareWrighter" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"YouTube"}</a>
                <span>{"\u{00b7}"}</span>
                <span>{ format!("{} \u{00b7} {} \u{00b7} {}",
                    env!("BUILD_HOST"), env!("BUILD_SHA"), env!("BUILD_TIMESTAMP"),
                ) }</span>
            </div>

            <HelpModal open={*help_open} on_close={close_help} />
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
